use mlua::{Table, Value};
use wp_ui::nvim::{self, CommandOpts};
use wp_ui::{Error, Result, lua};

use crate::config::Config;
use crate::state;
use crate::terminal::{self, report};

/// A `:AbstractTerminal` action, as exposed to Lua.
type Action = fn() -> Result<()>;

// -----------------------------------------------------------------------------
// Setup & Initialization
// -----------------------------------------------------------------------------

/// Setup with default options. ab-nui must already be initialised
/// (`wp_ui::init`).
pub fn setup() {
    setup_with_opts(Value::Nil);
}

/// Setup with a user-provided options table (or `nil`). Calling it again
/// applies the new options to terminals opened afterwards.
pub fn setup_with_opts(opts: Value) {
    report(try_setup(opts));
}

fn try_setup(opts: Value) -> Result<()> {
    // Kept in vim.g so Lua-side code can read the options too.
    Config::store(opts)?;
    let config = Config::load()?;
    if let Some(key) = &config.toggle_key {
        map_toggle(key)?;
    }
    state::install(config);

    nvim::user_command(
        "AbstractTerminal",
        &CommandOpts {
            nargs: Some("1"),
            desc: Some("AbstractTerminal: toggle | new | next | prev | close".into()),
            ..Default::default()
        },
        |args| report(run(args.args.trim())),
    )?;

    export_api()
}

/// `:AbstractTerminal <cmd>`.
fn run(cmd: &str) -> Result<()> {
    match cmd {
        "toggle" => terminal::toggle(),
        "new" => terminal::new_terminal(),
        "next" => terminal::cycle(1, true),
        "prev" => terminal::cycle(-1, true),
        "close" => terminal::close(),
        other => Err(Error::invalid(format!("unknown command \"{other}\""))),
    }
}

/// Bind the configured toggle key in normal, insert and terminal mode.
fn map_toggle(key: &str) -> Result<()> {
    let l = lua::lua()?;
    let modes = l.create_sequence_from(["n", "i", "t"])?;
    let callback = l.create_function(|_, ()| {
        report(terminal::toggle());
        Ok(())
    })?;
    let opts = l.create_table()?;
    opts.set("silent", true)?;
    opts.set("desc", "Toggle floating terminal")?;
    lua::vim_fn("keymap.set")?.call::<()>((modes, key, callback, opts))?;
    Ok(())
}

/// Make the plugin available as `require("terminal")`.
fn export_api() -> Result<()> {
    let l = lua::lua()?;
    let api = l.create_table()?;

    api.set(
        "setup",
        l.create_function(|_, opts: Value| {
            setup_with_opts(opts);
            Ok(())
        })?,
    )?;

    let actions: [(&str, Action); 5] = [
        ("toggle", terminal::toggle),
        ("new", terminal::new_terminal),
        ("next", || terminal::cycle(1, true)),
        ("prev", || terminal::cycle(-1, true)),
        ("close", terminal::close),
    ];
    for (name, action) in actions {
        api.set(
            name,
            l.create_function(move |_, ()| {
                report(action());
                Ok(())
            })?,
        )?;
    }

    let loaded: Table = l.globals().get::<Table>("package")?.get("loaded")?;
    loaded.set("terminal", api)?;
    Ok(())
}
