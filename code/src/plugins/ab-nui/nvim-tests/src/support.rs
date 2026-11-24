//! Shared helpers: initialise the library and inspect what Neovim shows.

#![allow(dead_code)]

use wp_ui::lua;
use wp_ui::nvim::{Buffer, Window};

/// Hand ab-nui the live `Lua` and install the default highlight groups.
/// Call at the top of every test.
pub fn setup() {
    wp_ui::init(&nvim_oxi::mlua::lua());
    wp_ui::theme::setup().expect("theme setup");
}

/// All lines of `buf`.
pub fn lines(buf: Buffer) -> Vec<String> {
    buf.get_lines(0, -1).expect("get_lines")
}

/// All lines of `buf` with trailing whitespace removed.
pub fn trimmed(buf: Buffer) -> Vec<String> {
    lines(buf).into_iter().map(|l| l.trim_end().to_string()).collect()
}

/// The whole buffer as one string (lines joined with `\n`).
pub fn text(buf: Buffer) -> String {
    lines(buf).join("\n")
}

/// Every highlight extmark in `buf` (all namespaces) as
/// `(row, start_col, end_col, hl_group)`, sorted. Columns are byte offsets.
pub fn highlights(buf: Buffer) -> Vec<(i64, i64, i64, String)> {
    let l = lua::lua().expect("lua");
    let f: mlua::Function = l
        .load(
            r#"
            return function(buf)
              local out = {}
              local marks = vim.api.nvim_buf_get_extmarks(buf, -1, 0, -1, { details = true })
              for _, m in ipairs(marks) do
                local d = m[4]
                if d.hl_group then
                  out[#out + 1] = { m[2], m[3], d.end_col or m[3], d.hl_group }
                end
              end
              table.sort(out, function(a, b)
                if a[1] ~= b[1] then return a[1] < b[1] end
                if a[2] ~= b[2] then return a[2] < b[2] end
                return a[3] < b[3]
              end)
              return out
            end
            "#,
        )
        .call(())
        .expect("load helper");
    let rows: Vec<mlua::Table> = f.call(buf.id()).expect("get_extmarks");
    rows.into_iter()
        .map(|t| {
            (t.get::<i64>(1).unwrap(), t.get::<i64>(2).unwrap(), t.get::<i64>(3).unwrap(), t.get::<String>(4).unwrap())
        })
        .collect()
}

/// The highlight group covering byte `col` of `row`, if any.
pub fn hl_at(buf: Buffer, row: i64, col: i64) -> Option<String> {
    highlights(buf).into_iter().find(|(r, s, e, _)| *r == row && *s <= col && col < *e).map(|h| h.3)
}

/// Feed keys as if typed and process them immediately (`nvim_feedkeys` with
/// mode `"x"`), with `<...>` notation translated.
pub fn feed(keys: &str) {
    let l = lua::lua().expect("lua");
    let f: mlua::Function = l
        .load(
            r#"
            return function(keys)
              local k = vim.api.nvim_replace_termcodes(keys, true, false, true)
              vim.api.nvim_feedkeys(k, "x", false)
            end
            "#,
        )
        .call(())
        .expect("load helper");
    f.call::<()>(keys.to_string()).expect("feedkeys");
}

/// The current window.
pub fn current_win() -> Window {
    Window(lua::call_api("nvim_get_current_win", ()).expect("current win"))
}

/// Number of windows in the current tabpage.
pub fn win_count() -> usize {
    let wins: Vec<i64> = lua::call_api("nvim_tabpage_list_wins", (0,)).expect("list wins");
    wins.len()
}

/// Let pending `vim.schedule` callbacks and timers run for up to `ms`
/// milliseconds, returning early once `done` holds.
pub fn wait(ms: u64, done: impl Fn() -> bool + 'static) -> bool {
    let l = lua::lua().expect("lua");
    let pred = l.create_function(move |_, ()| Ok(done())).expect("pred");
    let wait: mlua::Function = lua::vim().expect("vim").get("wait").expect("vim.wait");
    wait.call::<bool>((ms as i64, pred, 5i64)).expect("vim.wait")
}

/// A `Surface` of `w` x `h` cells (rounded border, so the text area is exactly
/// `w` x `h`) showing `build`, plus its buffer.
pub fn surf<W: wp_ui::prelude::Widget + 'static>(
    w: u32,
    h: u32,
    build: impl Fn() -> W + 'static,
) -> (wp_ui::prelude::Surface, Buffer) {
    use wp_ui::prelude::*;
    let s = Surface::float(PopupOptions { size: Size::cells(w, h), ..Default::default() })
        .expect("surface")
        .show(build)
        .expect("show");
    let buf = s.popup().buffer();
    (s, buf)
}

/// Evaluate a Lua chunk (`return ...`) and convert the result.
pub fn lua_eval<T: mlua::FromLua>(code: &str) -> T {
    lua::lua().expect("lua").load(code).eval::<T>().expect("lua eval")
}

/// Run a Lua chunk for its side effects.
pub fn lua_run(code: &str) {
    lua::lua().expect("lua").load(code).exec().expect("lua exec");
}

/// All windows of the current tabpage that are floating, in id order.
pub fn floats() -> Vec<Window> {
    let wins: Vec<i64> = lua::call_api("nvim_tabpage_list_wins", (0,)).expect("list wins");
    wins.into_iter()
        .map(Window)
        .filter(|w| w.config().map(|c| !c.relative.is_empty()).unwrap_or(false))
        .collect()
}

/// Render a widget tree on a throw-away `w` x `h` surface and return the
/// trimmed buffer lines.
pub fn render<W: wp_ui::prelude::Widget + 'static>(w: u32, h: u32, build: impl Fn() -> W + 'static) -> Vec<String> {
    let (_s, buf) = surf(w, h, build);
    trimmed(buf)
}

/// Shorthand for building an expected `Vec<String>`.
pub fn v(lines: &[&str]) -> Vec<String> {
    lines.iter().map(|s| s.to_string()).collect()
}

/// Run an Ex command through `nvim_command` (what `wp_ui::lua::cmd` is meant
/// to do; see the BUG note in widgets.rs).
pub fn ex(command: &str) {
    lua::call_api::<()>("nvim_command", (command.to_string(),)).expect("nvim_command");
}

/// Resize the editor and fire `VimResized` (headless nvim does not always).
pub fn resize(cols: u32, lines: u32) {
    ex(&format!("set columns={cols} lines={lines}"));
    ex("silent! doautocmd VimResized");
}
