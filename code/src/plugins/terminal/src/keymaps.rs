use nvim_oxi::{
    Dictionary, Function,
    api::{
        self,
        opts::SetKeymapOpts,
        types::{LogLevel, Mode},
    },
};
use wl_utils::panic::SafeFunctionExt;

use crate::config::Config;

// -----------------------------------------------------------------------------
// Keymap Registration
// -----------------------------------------------------------------------------

/// Register the toggle keymap if configured.
pub fn register(config: &Config) {
    let key = match config.keymap.as_ref().and_then(|k| k.toggle.as_deref()) {
        Some(k) if !k.is_empty() => k.to_string(),
        _ => return,
    };

    let toggle_fn = Function::<(), ()>::from_safe_fn(|()| {
        // If we're in terminal mode, escape to normal first.
        if let Ok(mode) = api::get_mode()
            && mode.mode == "t"
        {
            let keys = api::replace_termcodes("<C-\\><C-n>", true, false, true);
            let n_mode = nvim_oxi::String::from("n");
            api::feedkeys(&keys, &n_mode, true);
        }

        if let Err(e) = crate::terminal::toggle() {
            let _ = api::notify(&format!("AbstractTerminal Error: {}", e), LogLevel::Error, &Dictionary::new());
        }
        Ok(())
    });

    let map_opts = SetKeymapOpts::builder().silent(true).desc("Toggle floating terminal").callback(toggle_fn).build();

    let _ = api::set_keymap(Mode::Normal, &key, "", &map_opts);
    let _ = api::set_keymap(Mode::Insert, &key, "", &map_opts);
    let _ = api::set_keymap(Mode::Terminal, &key, "", &map_opts);
}
