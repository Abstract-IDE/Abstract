//! The bridge to Neovim.
//!
//! Everything this library does to Neovim goes through the global `vim` table
//! (`vim.api`, `vim.fn`, `vim.keymap`, ...). We never touch nvim-oxi: the host
//! plugin hands us its live [`mlua::Lua`] once via [`init`], and every call here
//! reuses it.
//!
//! Neovim's Lua runs on a single thread, so the handle lives in a thread-local.
//!
//! ## Why there is a cache and a driver
//!
//! Crossing from Rust into Lua is the expensive part of every operation, so the
//! bridge keeps it to a minimum:
//!
//! * Function handles (`vim.api.nvim_*`, `vim.fn.*`, `vim.keymap.set`, ...) are
//!   resolved once and cached, instead of walking `vim` → `api` → `name` on
//!   every call.
//! * Hot, multi-step operations (painting a frame, installing a surface's key
//!   table, timers) live in a small Lua module — the *driver* — compiled once.
//!   Rust hands it the data and it performs all the API calls on the Lua side,
//!   so a whole frame costs one crossing rather than one per extmark.

use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::ManuallyDrop;
use std::rc::Rc;

use mlua::{Function, IntoLua, IntoLuaMulti, Lua, Table};

use crate::error::{Error, Result};

/// The Lua half of the bridge. Returns a table of functions; see the call
/// sites (named after each function) for the argument encodings.
const DRIVER: &str = r#"
local api = vim.api
local uv = vim.uv or vim.loop
local set_lines = api.nvim_buf_set_lines
local clear_ns = api.nvim_buf_clear_namespace
local set_extmark = api.nvim_buf_set_extmark
local get_opt = api.nvim_get_option_value
local set_opt = api.nvim_set_option_value

local M = {}

-- Paint a frame (or the changed part of one) into `buf`.
--
--   tick   changedtick the caller last left the buffer at, or -1 for "unknown"
--   nrows  number of rows the caller believes the buffer has (diff frames)
--   full   true: `runs` describes the whole buffer and replaces it
--   runs   flat list of quads { start_row, row_count, lines | false, marks }
--            lines  the run's new text, or `false` if only highlights changed
--            marks  flat list of quads { row, start_col, end_col, hl_group }
--
-- Returns the buffer's changedtick afterwards, or -1 without touching
-- anything when the buffer is not in the state the caller diffed against
-- (someone else edited it) — the caller then repaints in full.
function M.render(buf, ns, tick, nrows, full, runs)
  if not api.nvim_buf_is_valid(buf) then
    return -2
  end
  if not full then
    if api.nvim_buf_get_changedtick(buf) ~= tick or api.nvim_buf_line_count(buf) ~= nrows then
      return -1
    end
  end

  local scope = { buf = buf }
  local modifiable = get_opt("modifiable", scope)
  local readonly = get_opt("readonly", scope)
  if not modifiable then set_opt("modifiable", true, scope) end
  if readonly then set_opt("readonly", false, scope) end

  local ok, err = pcall(function()
    for i = 1, #runs, 4 do
      local start, lines, marks = runs[i], runs[i + 2], runs[i + 3]
      if full then
        set_lines(buf, 0, -1, false, lines)
        clear_ns(buf, ns, 0, -1)
      else
        local stop = start + runs[i + 1]
        if lines then
          set_lines(buf, start, stop, false, lines)
        end
        clear_ns(buf, ns, start, stop)
      end
      for j = 1, #marks, 4 do
        set_extmark(buf, ns, marks[j], marks[j + 1], { end_col = marks[j + 2], hl_group = marks[j + 3] })
      end
    end
  end)

  if not modifiable then set_opt("modifiable", false, scope) end
  if readonly then set_opt("readonly", true, scope) end
  if not ok then error(err, 0) end
  return api.nvim_buf_get_changedtick(buf)
end

-- Bind many keys of `buf` to one dispatcher. `keys` is a flat list of
-- { lhs, code } pairs; pressing `lhs` calls `dispatch(code)`.
function M.map_keys(buf, mode, keys, dispatch)
  local set = api.nvim_buf_set_keymap
  for i = 1, #keys, 2 do
    local code = keys[i + 1]
    set(buf, mode, keys[i], "", {
      noremap = true,
      silent = true,
      nowait = true,
      callback = function() dispatch(code) end,
    })
  end
end

-- libuv timers. Callbacks hop to the main loop (`vim.schedule_wrap`), since
-- uv callbacks run in a fast-event context where most of the API is off-limits.
function M.timer_start(delay, rep, cb)
  local t = uv.new_timer()
  t:start(delay, rep, vim.schedule_wrap(cb))
  return t
end

function M.timer_stop(t)
  if not t:is_closing() then t:stop() end
end

function M.timer_close(t)
  if not t:is_closing() then
    t:stop()
    t:close()
  end
end

-- Real-buffer views (embedded windows, code views).

-- Syntax-highlight `buf` as filetype `ft`: treesitter when a parser exists,
-- else Vim's regex syntax; "" turns highlighting off.
function M.set_syntax(buf, ft)
  pcall(vim.treesitter.stop, buf)
  vim.bo[buf].syntax = ""
  if ft and ft ~= "" then
    local lang = vim.treesitter.language.get_lang(ft) or ft
    if not pcall(vim.treesitter.start, buf, lang) then
      vim.bo[buf].syntax = ft
    end
  end
end

-- Put the cursor on line `lnum` of `win` and centre it (or show the top).
function M.view_line(win, lnum)
  if not api.nvim_win_is_valid(win) then return end
  local count = api.nvim_buf_line_count(api.nvim_win_get_buf(win))
  local target = math.max(1, math.min(lnum or 1, count))
  api.nvim_win_call(win, function()
    api.nvim_win_set_cursor(win, { target, 0 })
    if lnum and lnum > 1 then
      vim.cmd("normal! zz")
    else
      vim.fn.winrestview({ topline = 1, leftcol = 0 })
    end
  end)
end

-- Scroll `win` by `delta` lines.
function M.scroll_by(win, delta)
  if not api.nvim_win_is_valid(win) then return end
  api.nvim_win_call(win, function()
    local view = vim.fn.winsaveview()
    view.topline = math.max(1, math.min(vim.fn.line("$"), view.topline + delta))
    view.lnum = math.max(view.lnum, view.topline)
    vim.fn.winrestview(view)
  end)
end

-- Scroll `win` like the mouse wheel would: `rows` down (negative: up),
-- `cols` right (negative: left).
function M.scroll_win(win, rows, cols)
  if not api.nvim_win_is_valid(win) then return end
  api.nvim_win_call(win, function()
    if rows > 0 then vim.cmd("normal! " .. rows .. "\5") elseif rows < 0 then vim.cmd("normal! " .. -rows .. "\25") end
    if cols > 0 then vim.cmd("normal! " .. cols .. "zl") elseif cols < 0 then vim.cmd("normal! " .. -cols .. "zh") end
  end)
end

function M.set_mouse(win, on)
  if api.nvim_win_is_valid(win) then
    pcall(api.nvim_win_set_config, win, { mouse = on })
  end
end

-- File icons in their own colours: mini.icons when it is set up (its colour
-- groups exist only after `setup()`), else nvim-web-devicons, else an
-- un-set-up mini.icons (icons without colour); nil without any. The third
-- result says whether that is final: until a coloured provider shows up (a
-- lazy-loaded plugin, mini.icons' `setup()`), look again now and then.
local icon_provider, provider_final, probed_at = nil, false, -math.huge
local function mini_provider(mini)
  return function(n)
    local ok, icon, hl = pcall(mini.get, "file", n)
    if ok then return icon, hl end
  end
end
function M.file_icon(name)
  local now = vim.uv.now()
  if not provider_final and now - probed_at >= 2000 then
    probed_at = now
    icon_provider = nil
    local ok, mini = pcall(require, "mini.icons")
    local has_mini = ok and type(mini.get) == "function"
    if has_mini and _G.MiniIcons ~= nil then
      icon_provider, provider_final = mini_provider(mini), true
    else
      local ok2, dev = pcall(require, "nvim-web-devicons")
      if ok2 then
        icon_provider, provider_final = function(n)
          return dev.get_icon(n, vim.fn.fnamemodify(n, ":e"), { default = true })
        end, true
      elseif has_mini then
        icon_provider = mini_provider(mini)
      end
    end
  end
  if not icon_provider then return nil, nil, provider_final end
  local icon, hl = icon_provider(name)
  return icon, hl, provider_final
end

return M
"#;

/// The resolved handles for one `Lua` state.
struct Bridge {
    lua: Lua,
    vim: Table,
    api: Table,
    vfn: Table,
    driver: Table,
    /// `vim.api.*` functions by name.
    api_fns: RefCell<HashMap<Box<str>, Function>>,
    /// `vim.fn.*` functions by name.
    vim_fns: RefCell<HashMap<Box<str>, Function>>,
    /// Other functions reachable from `vim`, by dotted path.
    path_fns: RefCell<HashMap<&'static str, Function>>,
    /// Driver functions by name.
    driver_fns: RefCell<HashMap<&'static str, Function>>,
}

impl Bridge {
    fn new(lua: &Lua) -> Result<Self> {
        let vim: Table = lua.globals().get("vim")?;
        let api: Table = vim.get("api")?;
        let vfn: Table = vim.get("fn")?;
        let driver: Table = lua.load(DRIVER).set_name("=ab-nui/driver").call(())?;
        Ok(Self {
            lua: lua.clone(),
            vim,
            api,
            vfn,
            driver,
            api_fns: RefCell::new(HashMap::new()),
            vim_fns: RefCell::new(HashMap::new()),
            path_fns: RefCell::new(HashMap::new()),
            driver_fns: RefCell::new(HashMap::new()),
        })
    }
}

/// Look `name` up in `cache`, resolving it from `table` on a miss.
///
/// The handle is cloned out of the cache (cheap: it is a registry reference)
/// rather than borrowed, because the call it is about to make may re-enter the
/// bridge and need to insert into the same cache.
fn cached(cache: &RefCell<HashMap<Box<str>, Function>>, table: &Table, name: &str) -> Result<Function> {
    if let Some(f) = cache.borrow().get(name) {
        return Ok(f.clone());
    }
    let f: Function = table.get(name)?;
    cache.borrow_mut().insert(name.into(), f.clone());
    Ok(f)
}

// `ManuallyDrop`: these must not be torn down when the thread (i.e. Neovim)
// exits. Dropping them would call into a Lua state that Neovim may already be
// dismantling, and would make the bridge unavailable to other thread-locals
// whose destructors still need it.
thread_local! {
    static LUA: ManuallyDrop<RefCell<Option<Lua>>> = const { ManuallyDrop::new(RefCell::new(None)) };
    static BRIDGE: ManuallyDrop<RefCell<Option<Rc<Bridge>>>> = const { ManuallyDrop::new(RefCell::new(None)) };
}

/// Register the host's `Lua` handle. Call this exactly once during setup,
/// e.g. `ui::init(&nvim_oxi::mlua::lua())`.
pub fn init(lua: &Lua) {
    LUA.with(|cell| *cell.borrow_mut() = Some(lua.clone()));
    // Handles resolved for a previous state are meaningless for this one.
    BRIDGE.with(|cell| *cell.borrow_mut() = None);
}

/// `true` once [`init`] has been called.
pub fn is_initialized() -> bool {
    LUA.with(|cell| cell.borrow().is_some())
}

/// The bridge for the registered `Lua`, built on first use.
fn bridge() -> Result<Rc<Bridge>> {
    if let Some(b) = BRIDGE.with(|cell| cell.borrow().clone()) {
        return Ok(b);
    }
    let lua = LUA.with(|cell| cell.borrow().clone()).ok_or(Error::NotInitialized)?;
    let b = Rc::new(Bridge::new(&lua)?);
    BRIDGE.with(|cell| *cell.borrow_mut() = Some(b.clone()));
    Ok(b)
}

/// Get a clone of the live `Lua` handle.
///
/// Cheap (the handle is reference-counted). Returns an error rather than
/// panicking so callers can surface a friendly message.
pub fn lua() -> Result<Lua> {
    LUA.with(|cell| cell.borrow().clone()).ok_or(Error::NotInitialized)
}

/// Fresh, empty Lua table.
pub fn table() -> Result<Table> {
    Ok(bridge()?.lua.create_table()?)
}

/// The global `vim` table.
pub fn vim() -> Result<Table> {
    Ok(bridge()?.vim.clone())
}

/// `vim.api`.
pub fn api() -> Result<Table> {
    Ok(bridge()?.api.clone())
}

/// `vim.fn`.
pub fn vfn() -> Result<Table> {
    Ok(bridge()?.vfn.clone())
}

/// The `vim.api.*` function called `name` (resolved once, then cached).
pub fn api_fn(name: &str) -> Result<Function> {
    let b = bridge()?;
    cached(&b.api_fns, &b.api, name)
}

/// Call a `vim.api.*` function by name.
///
/// ```ignore
/// let bufnr: i64 = ui::lua::call_api("nvim_create_buf", (false, true))?;
/// ```
pub fn call_api<R: mlua::FromLuaMulti>(name: &str, args: impl IntoLuaMulti) -> Result<R> {
    Ok(api_fn(name)?.call(args)?)
}

/// Call a `vim.fn.*` (vimscript) function by name.
pub fn call_fn<R: mlua::FromLuaMulti>(name: &str, args: impl IntoLuaMulti) -> Result<R> {
    let b = bridge()?;
    Ok(cached(&b.vim_fns, &b.vfn, name)?.call(args)?)
}

/// A function reachable from `vim` by a dotted path — `"cmd"`, `"keymap.set"`,
/// `"schedule"`, ... (resolved once, then cached).
pub fn vim_fn(path: &'static str) -> Result<Function> {
    let b = bridge()?;
    if let Some(f) = b.path_fns.borrow().get(path) {
        return Ok(f.clone());
    }
    let mut segments = path.split('.').peekable();
    let mut table = b.vim.clone();
    let f = loop {
        let segment = segments.next().ok_or_else(|| Error::invalid("empty vim function path"))?;
        if segments.peek().is_none() {
            break table.get::<Function>(segment)?;
        }
        table = table.get::<Table>(segment)?;
    };
    b.path_fns.borrow_mut().insert(path, f.clone());
    Ok(f)
}

/// A function of the Lua-side driver (see the module docs).
pub(crate) fn driver_fn(name: &'static str) -> Result<Function> {
    let b = bridge()?;
    if let Some(f) = b.driver_fns.borrow().get(name) {
        return Ok(f.clone());
    }
    let f: Function = b.driver.get(name)?;
    b.driver_fns.borrow_mut().insert(name, f.clone());
    Ok(f)
}

/// Run Ex command(s) — what `vim.cmd(string)` does.
pub fn cmd(command: &str) -> Result<()> {
    // `vim.cmd` is a callable *table* on current Neovim, not a function, so
    // this calls what it calls for a string argument: `nvim_exec2`.
    call_api::<mlua::Value>("nvim_exec2", (command, table()?))?;
    Ok(())
}

/// Read a global option via `nvim_get_option_value(name, {})`.
pub fn get_option<R: mlua::FromLua>(name: &str) -> Result<R> {
    let opts = table()?;
    call_api("nvim_get_option_value", (name, opts))
}

/// Read an option with an explicit scope table (`{ buf = .. }`, `{ win = .. }`)
/// via `nvim_get_option_value`.
pub fn get_option_scoped<R: mlua::FromLua>(name: &str, scope: Table) -> Result<R> {
    call_api("nvim_get_option_value", (name, scope))
}

/// Set an option with an explicit scope table (`{ buf = .. }`, `{ win = .. }`,
/// or empty for global) via `nvim_set_option_value`.
pub fn set_option_scoped(name: &str, value: impl IntoLua, scope: Table) -> Result<()> {
    call_api::<()>("nvim_set_option_value", (name, value, scope))
}
