use std::sync::{LazyLock, Mutex};

use nvim_oxi::{self, mlua};
use strum::IntoStaticStr;

use crate::plugins::spec::extract_section_raw;

const KEYMAPS: &str = include_str!("keymaps.lua");

#[allow(unused, clippy::enum_variant_names)]
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum Key {
    Builtin,
    //
    AbstractTerminal,
    AbstractWindow,
    CodeRunner,
    Dap,
    Fff,
    GitGraph,
    GotoPreview,
    Grapple,
    Hop,
    Hovercraft,
    Kulala,
    LspConfig,
    Markview,
    NeoTree,
    Oil,
    SessionManager,
    Snacks,
    SnacksBufdelete,
    SnacksGh,
    SnacksGitBrowse,
    SnacksLazygit,
    SnacksPicker,
    Trouble,
    WhichKey,
}

// Global static keymap
pub static MAPPING: LazyLock<Mapping> = LazyLock::new(Mapping::new);
pub static LOADED_PLUGINS: LazyLock<Mutex<Vec<Key>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[derive(Debug, Copy, Clone)]
pub struct Mapping;

impl Mapping {
    pub fn new() -> Self {
        let _ = mlua::lua().load(KEYMAPS).exec().inspect_err(|e| {
            eprintln!("{e}");
        });

        Self
    }

    /// Fire-and-forget signal.
    pub fn signal(self, plugin: Key) {
        if let Ok(mut loaded) = LOADED_PLUGINS.lock() {
            loaded.push(plugin);
        } else {
            nvim_oxi::api::err_writeln("MapLoader: LOADED_PLUGINS mutex poisoned; signal ignored");
        }
    }

    /// Registers all collected keymaps with which-key.
    pub fn register_all(&self) -> nvim_oxi::Result<()> {
        let loaded = LOADED_PLUGINS
            .lock()
            .map_err(|_| mlua::Error::RuntimeError("MapLoader: LOADED_PLUGINS mutex poisoned".to_string()))?;

        let lua = mlua::lua();

        for plugin in loaded.iter().copied() {
            let keymap = self.get_map(plugin);
            let code = format!("require('which-key').add({})", keymap);
            lua.load(&code).exec()?;
        }

        Ok(())
    }
}

#[allow(unused)]
impl Mapping {
    pub fn set_map(&self, key: Key) -> nvim_oxi::Result<()> {
        let which_key = format!("require'which-key'.add({})", self.get_map(key));
        mlua::lua().load(which_key).exec().inspect_err(|e| {
            eprintln!("{e}");
        })?;

        Ok(())
    }

    pub fn set_map_str(&self, key: Key) -> &'static str {
        Box::leak(format!("require'which-key'.add({})", self.get_map(key)).into_boxed_str())
    }

    pub fn get_map(&self, key: Key) -> &'static str {
        let section: &'static str = key.into();
        extract_section_raw(KEYMAPS, section)
    }
}
