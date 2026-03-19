use nvim_oxi::api::{
    self,
    opts::OptionOpts,
    types::{WindowConfig, WindowRelativeTo, WindowStyle, WindowTitle, WindowTitlePosition},
};

use crate::{config::Config, state::State};

// -----------------------------------------------------------------------------
// Floating Window Management
// -----------------------------------------------------------------------------

/// Build a `WindowConfig` for the floating terminal window.
fn build_win_config(state: &State, config: &Config) -> nvim_oxi::Result<WindowConfig> {
    let cols = api::get_option_value::<i64>("columns", &OptionOpts::default())?;
    let lines = api::get_option_value::<i64>("lines", &OptionOpts::default())?;

    let w = ((cols as f64) * config.width).floor().max(1.0) as u32;
    let h = ((lines as f64) * config.height).floor().max(1.0) as u32;
    let r = (((lines as u32).saturating_sub(h) as f64) * config.offset_row).floor() as f64;
    let c = (((cols as u32).saturating_sub(w) as f64) * config.offset_col).floor() as f64;

    let title_text = format!(" {} {}/{} ", config.title, state.current + 1, state.len());

    let win_config = WindowConfig::builder()
        .relative(WindowRelativeTo::Editor)
        .width(w)
        .height(h)
        .row(r)
        .col(c)
        .style(WindowStyle::Minimal)
        .border(config.border())
        .title(WindowTitle::SimpleString(title_text.into()))
        .title_pos(config.title_position())
        .focusable(true)
        .zindex(50)
        .build();

    Ok(win_config)
}

/// Open a floating window showing `buf`, storing the handle in `state.win`.
pub fn open_floating(state: &mut State, buf: &nvim_oxi::api::Buffer) -> nvim_oxi::Result<()> {
    let config = state.config.clone();
    let win_config = build_win_config(state, &config)?;
    let win = api::open_win(buf, true, &win_config)?;
    state.win = Some(win);

    // crate::autocmds::guard_buf(state, buf.clone())?;

    Ok(())
}

/// Update the title of the existing floating window to reflect the current index.
pub fn update_title(state: &State) -> nvim_oxi::Result<()> {
    if let Some(win) = &state.win {
        if win.is_valid() {
            let title_text = format!(" {} {}/{} ", state.config.title, state.current + 1, state.len());

            let title_pos = state.config.title_position();

            // Build a minimal config with only the title fields to update.
            let mut update = WindowConfig::default();
            update.title = Some(WindowTitle::SimpleString(title_text.into()));
            update.title_pos = Some(title_pos);

            // Use `set_config` on the window handle directly (not the "current" window).
            let mut win = win.clone();
            win.set_config(&update)?;
        }
    }
    Ok(())
}

/// Safely resolve the title position, falling back to Right.
impl Config {
    pub(crate) fn title_position_enum(&self) -> WindowTitlePosition {
        match self.title_pos.as_str() {
            "left" => WindowTitlePosition::Left,
            "center" => WindowTitlePosition::Center,
            _ => WindowTitlePosition::Right,
        }
    }
}
