//! Seek: a search hub for files, file contents, buffers and buffer lines,
//! built on ab-nui (`wp_ui`).
//!
//! One window, one mode at a time:
//!
//! | mode    | searches                            | prefix | key     |
//! | ------- | ----------------------------------- | ------ | ------- |
//! | files   | project files (fuzzy)               |        | `<C-f>` |
//! | grep    | file contents (literal or regex)    | `>`    | `<C-g>` |
//! | buffers | open buffers (fuzzy)                | `#`    | `<C-b>` |
//! | lines   | lines of the current buffer (fuzzy) | `/`    | `<C-l>` |
//!
//! Searching runs on background threads (ripgrep's `.gitignore`-aware walker,
//! fzf-style scoring in parallel), so large projects never block Neovim. The
//! preview is a real buffer with syntax highlighting.
//!
//! - `:AbstractSeek [mode] [query]` (again while open: closes it, or with
//!   `on_reopen = "keep"` leaves it; another mode/query switches to it)
//! - `:AbstractSeekToggle [mode] [query]`, `require("seek").toggle({ .. })`
//! - `require("seek").open({ mode = "grep", query = "TODO", cwd = "~/src" })`
//! - `require("seek").grep_word()` — grep the visual selection or the word
//!   under the cursor
//! - `require("seek").setup({ .. })` — see `config.rs` for every option.

mod actions;
mod config;
mod engine;
mod entry;
mod fuzzy;
mod nvim;
mod preview;
mod session;

use std::cell::RefCell;
use std::mem::ManuallyDrop;
use std::path::PathBuf;
use std::rc::Rc;

use mlua::{Table, Value};
use wp_ui::nvim::{self as ui, CommandOpts};
use wp_ui::view::{Level, NotifyOptions, notify};
use wp_ui::{Error, Result, lua};

pub use config::Config;
pub use entry::Mode;
use session::{OpenOpts, Session};

thread_local! {
    /// The open seek window.
    ///
    /// `ManuallyDrop`: a seek window still open when Neovim exits must not be
    /// torn down from a thread-exit destructor — that would call into Lua
    /// after Neovim has shut it down (and crash on quit).
    static SEEK: ManuallyDrop<RefCell<Option<Rc<Session>>>> = const { ManuallyDrop::new(RefCell::new(None)) };
}

// ============================================================================
// Public API
// ============================================================================

/// Register `:AbstractSeek` and `require("seek")`. ab-nui must already be
/// initialised (`wp_ui::init`). Options already in `vim.g.abstract_seek_opts`
/// are kept.
pub fn setup() {
    setup_with_opts(Value::Nil);
}

/// Like [`setup`], replacing the options with `opts` (a Lua table) unless it is `nil`.
pub fn setup_with_opts(opts: Value) {
    report(try_setup(opts));
}

fn try_setup(opts: Value) -> Result<()> {
    if !opts.is_nil() {
        Config::store(opts)?;
    }
    nvim::call::<()>("highlights", ())?;
    // Colorschemes clear highlight links; put ours back.
    let group = ui::augroup("AbstractSeek")?;
    ui::autocmd(&["ColorScheme"], None, Some(group), |_| {
        let _ = nvim::call::<()>("highlights", ());
    })?;
    let opts = |desc: &str| CommandOpts {
        nargs: Some("*"),
        desc: Some(desc.into()),
        complete: Some("customlist,v:lua.require'seek'.complete".into()),
        ..Default::default()
    };
    ui::user_command("AbstractSeek", &opts("Seek: [files|grep|buffers|lines] [query]"), |args| {
        let (mode, query) = parse_args(&args.fargs);
        report(open(mode, query, None));
    })?;
    ui::user_command("AbstractSeekToggle", &opts("Seek, open or close: [mode] [query]"), |args| {
        let (mode, query) = parse_args(&args.fargs);
        report(toggle(mode, query, None));
    })?;
    export_api()
}

fn open_session() -> Option<Rc<Session>> {
    SEEK.with(|s| s.borrow().clone()).filter(|s| s.is_visible())
}

/// Open seek. When it is already open:
/// - asked for another mode or a query: switch the open window to it;
/// - asked for another directory: start afresh there;
/// - otherwise (opening it again): close it (`on_reopen = "toggle"`, the
///   default) or leave it as it is (`on_reopen = "keep"`).
pub fn open(mode: Option<Mode>, query: Option<String>, cwd: Option<PathBuf>) -> Result<()> {
    if let Some(session) = open_session()
        // A directory worked out inside seek's own prompt (`%:p:h` there
        // is just the cwd) means "where seek is", not a new place.
        && cwd.as_deref().is_none_or(|dir| session.has_root(dir) || session.has_focus())
    {
        if mode.is_some_and(|m| m != session.mode()) || query.is_some() {
            if let Some(m) = mode {
                session.set_mode(m);
            }
            if let Some(q) = query {
                session.set_query(&q)?;
            }
            return session.focus();
        }
        return match Config::load()?.on_reopen {
            config::Reopen::Toggle => {
                close();
                Ok(())
            },
            config::Reopen::Keep => session.focus(),
        };
    }
    close();
    let session = Session::open(Config::load()?, OpenOpts { mode, query, cwd })?;
    SEEK.with(|s| *s.borrow_mut() = Some(session));
    Ok(())
}

/// Close seek if it is open, open it (like [`open`]) if not.
pub fn toggle(mode: Option<Mode>, query: Option<String>, cwd: Option<PathBuf>) -> Result<()> {
    if open_session().is_some() {
        close();
        Ok(())
    } else {
        open(mode, query, cwd)
    }
}

/// Close seek, if open.
pub fn close() {
    if let Some(session) = SEEK.with(|s| s.borrow_mut().take()) {
        session.shutdown();
    }
}

/// Grep for the visual selection, or the word under the cursor.
pub fn grep_word() -> Result<()> {
    let word: String = nvim::call("selection_or_word", ())?;
    open(Some(Mode::Grep), Some(word), None)
}

/// Show an error where there is no caller to return it to.
pub(crate) fn report(res: Result<()>) {
    if let Err(e) = res {
        let opts = NotifyOptions { title: Some("Seek".into()), level: Level::Error, ..Default::default() };
        if notify(e.to_string(), opts).is_err() {
            eprintln!("Seek: {e}");
        }
    }
}

/// `[mode] [query...]` command arguments.
fn parse_args(args: &[String]) -> (Option<Mode>, Option<String>) {
    let (mode, rest) = match args.first().and_then(|a| Mode::parse(a)) {
        Some(m) => (Some(m), &args[1..]),
        None => (None, args),
    };
    (mode, (!rest.is_empty()).then(|| rest.join(" ")))
}

/// `require("seek")`.
fn export_api() -> Result<()> {
    let l = lua::lua()?;
    let api = l.create_table()?;

    api.set(
        "open",
        l.create_function(|_, opts: Option<Table>| {
            report(lua_opts(opts).and_then(|(m, q, d)| open(m, q, d)));
            Ok(())
        })?,
    )?;
    api.set(
        "toggle",
        l.create_function(|_, opts: Option<Table>| {
            report(lua_opts(opts).and_then(|(m, q, d)| toggle(m, q, d)));
            Ok(())
        })?,
    )?;
    api.set(
        "close",
        l.create_function(|_, ()| {
            close();
            Ok(())
        })?,
    )?;
    api.set(
        "grep_word",
        l.create_function(|_, ()| {
            report(grep_word());
            Ok(())
        })?,
    )?;
    api.set(
        "setup",
        l.create_function(|_, opts: Value| {
            setup_with_opts(opts);
            Ok(())
        })?,
    )?;
    // Command-line completion: mode names for the first argument.
    api.set(
        "complete",
        l.create_function(|_, (lead, line): (String, String)| {
            let words = line.split_whitespace().count();
            let first_arg = words <= 1 || (words == 2 && !line.ends_with(' '));
            let names: Vec<String> = if first_arg {
                Mode::ALL.iter().map(|m| m.name().to_string()).filter(|n| n.starts_with(&lead)).collect()
            } else {
                Vec::new()
            };
            Ok(names)
        })?,
    )?;

    let loaded: Table = l.globals().get::<Table>("package")?.get("loaded")?;
    loaded.set("seek", api)?;
    Ok(())
}

/// `{ mode = .., query = .., cwd = .. }` from Lua.
fn lua_opts(opts: Option<Table>) -> Result<(Option<Mode>, Option<String>, Option<PathBuf>)> {
    let Some(opts) = opts else { return Ok((None, None, None)) };
    let mode = match opts.get::<Option<String>>("mode")? {
        Some(name) => Some(Mode::parse(&name).ok_or_else(|| Error::invalid(format!("unknown mode \"{name}\"")))?),
        None => None,
    };
    let query = opts.get::<Option<String>>("query")?;
    let cwd = opts.get::<Option<String>>("cwd")?.filter(|d| !d.is_empty()).map(|d| {
        let expanded: String = lua::call_fn("expand", (d.clone(),)).unwrap_or(d);
        PathBuf::from(expanded)
    });
    Ok((mode, query, cwd))
}
