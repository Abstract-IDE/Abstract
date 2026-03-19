use nvim_oxi::{
    Dictionary, Function, Object,
    api::{
        self,
        opts::CreateCommandOpts,
        types::{CommandArgs, CommandNArgs, LogLevel},
    },
};

use crate::{config::Config, keymaps, state, terminal};

// -----------------------------------------------------------------------------
// Setup & Initialization
// -----------------------------------------------------------------------------

/// Setup with default options.
pub fn setup() {
    setup_with_opts(Object::nil());
    build_api();
}

/// Setup with a user-provided options table.
pub fn setup_with_opts(opts: Object) {
    // Store opts in vim.g so Lua-side code can also read them if needed.
    let _ = api::set_var("abstract_terminal_opts", opts);

    // Load the typed config.
    let config = Config::load();

    // Register keymaps before initialising state (keymaps reference terminal::toggle).
    keymaps::register(&config);

    // Initialise the global state.
    state::init_state(config);

    // Register the `:AbstractTerminal` command.
    let cmd_opts = CreateCommandOpts::builder()
        .nargs(CommandNArgs::One)
        .desc("AbstractTerminal plugin: use subcommands (e.g. toggle)")
        .build();

    let _ = api::create_user_command(
        "AbstractTerminal",
        |args: CommandArgs| -> std::result::Result<(), nvim_oxi::Error> {
            let cmd = args.args.unwrap_or_default();

            let res = match cmd.as_str() {
                "toggle" => terminal::toggle(),
                "new" => terminal::new_terminal(),
                "next" => terminal::cycle(1, true),
                "prev" => terminal::cycle(-1, true),
                "close" => terminal::close(),
                _ => {
                    let _ = api::notify(
                        &format!("AbstractTerminal: unknown command \"{}\"", cmd),
                        LogLevel::Error,
                        &Dictionary::new(),
                    );
                    Ok(())
                },
            };

            if let Err(e) = res {
                let _ = api::notify(&format!("AbstractTerminal Error: {}", e), LogLevel::Error, &Dictionary::new());
            }

            Ok(())
        },
        &cmd_opts,
    );
}

/// Build the dictionary of Lua-callable functions exposed by the plugin.
pub fn build_api() -> Dictionary {
    let mut dict = Dictionary::new();

    dict.insert("setup", Object::from(Function::<Object, ()>::from_fn(setup_with_opts)));
    dict.insert(
        "toggle",
        Object::from(Function::<(), ()>::from_fn(|()| {
            let _ = terminal::toggle();
        })),
    );
    dict.insert(
        "new",
        Object::from(Function::<(), ()>::from_fn(|()| {
            let _ = terminal::new_terminal();
        })),
    );
    dict.insert(
        "next",
        Object::from(Function::<(), ()>::from_fn(|()| {
            let _ = terminal::cycle(1, true);
        })),
    );
    dict.insert(
        "prev",
        Object::from(Function::<(), ()>::from_fn(|()| {
            let _ = terminal::cycle(-1, true);
        })),
    );
    dict.insert(
        "close",
        Object::from(Function::<(), ()>::from_fn(|()| {
            let _ = terminal::close();
        })),
    );

    dict
}
