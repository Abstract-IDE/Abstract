//! Adapters that install ab-nui's [`Menu`] and [`Input`] as Neovim's
//! `vim.ui.select` / `vim.ui.input` implementations, so every plugin that uses
//! the standard prompts gets ab-nui UIs.
//!
//! The `vim.ui` contract: the completion callback is invoked **exactly once** —
//! with the choice on confirm, or with `nil` on cancel.

use mlua::{Function, Table, Value};

use crate::error::Result;
use crate::geometry::{Dim, Position, Size};
use crate::lua;
use crate::nvim::single_line;
use crate::widget::input::{Input, InputOptions};
use crate::widget::menu::{Menu, MenuItem, MenuOptions};

/// Install ab-nui as `vim.ui.select`.
pub fn register_select() -> Result<()> {
    let l = lua::lua()?;
    // `opts` may be nil (`vim.ui.select(items, nil, cb)`).
    let func: Function = l.create_function(|_, (items, opts, on_choice): (Table, Option<Table>, Function)| {
        select_impl(&items, opts.as_ref(), on_choice).map_err(mlua::Error::external)
    })?;
    lua::vim()?.get::<Table>("ui")?.set("select", func)?;
    Ok(())
}

/// Install ab-nui as `vim.ui.input`.
pub fn register_input() -> Result<()> {
    let l = lua::lua()?;
    let func: Function = l.create_function(|_, (opts, on_confirm): (Value, Value)| {
        // `opts` may be nil, or left out altogether (`vim.ui.input(cb)`).
        let (opts, on_confirm) = match (opts, on_confirm) {
            (Value::Function(f), Value::Nil) => (None, f),
            (opts, Value::Function(f)) => (opts.as_table().cloned(), f),
            _ => return Err(mlua::Error::runtime("vim.ui.input: on_confirm must be a function")),
        };
        input_impl(opts.as_ref(), on_confirm).map_err(mlua::Error::external)
    })?;
    lua::vim()?.get::<Table>("ui")?.set("input", func)?;
    Ok(())
}

/// Install both adapters.
pub fn register_all() -> Result<()> {
    register_select()?;
    register_input()
}

/// A prompt as a border title: trimmed, without the trailing `:`, on one line.
fn title_from_prompt(prompt: &str) -> String {
    format!(" {} ", single_line(prompt.trim().trim_end_matches(':')))
}

/// Call the user's completion callback. Its errors are not ours to swallow:
/// they are reported through `vim.notify` at ERROR level (the callback runs
/// from a keymap or a scheduled tick, where there is no caller to rethrow to).
fn answer(what: &str, callback: &Function, args: impl mlua::IntoLuaMulti) {
    if let Err(err) = callback.call::<()>(args) {
        let msg = format!("{what} callback failed: {err}");
        let notify = || -> Result<()> {
            let level: i64 = lua::vim()?.get::<Table>("log")?.get::<Table>("levels")?.get("ERROR")?;
            // Looked up per call: plugins replace `vim.notify` at any time.
            lua::vim()?.get::<Function>("notify")?.call::<()>((msg.as_str(), level))?;
            Ok(())
        };
        let echo = || -> Result<()> {
            let l = lua::lua()?;
            let chunks = l.create_sequence_from([l.create_sequence_from([msg.as_str(), "ErrorMsg"])?])?;
            lua::call_api::<()>("nvim_echo", (chunks, true, l.create_table()?))
        };
        if notify().is_err() {
            let _ = echo();
        }
    }
}

fn select_impl(items: &Table, opts: Option<&Table>, on_choice: Function) -> Result<()> {
    // Label each item with opts.format_item (falling back to tostring).
    let format_item: Option<Function> = opts.and_then(|o| o.get("format_item").ok());
    let tostring: Function = lua::lua()?.globals().get("tostring")?;

    let mut entries: Vec<MenuItem<usize>> = Vec::new();
    let mut originals: Vec<Value> = Vec::new();
    let mut max_width: u16 = 0;
    for (i, item) in items.clone().sequence_values::<Value>().enumerate() {
        let item = item?;
        let label: String = match &format_item {
            Some(f) => f.call(item.clone())?,
            None => tostring.call(item.clone())?,
        };
        let label = single_line(&label).into_owned();
        max_width = max_width.max(crate::text::display_width(&label));
        entries.push(MenuItem::new(label, i));
        originals.push(item);
    }

    let title: Option<String> = opts.and_then(|o| o.get::<String>("prompt").ok());
    let height = (entries.len() as u32).clamp(1, 20);
    let width = (max_width as u32 + 4).clamp(20, 80);
    let menu = Menu::new(
        entries,
        MenuOptions {
            title: title.as_deref().map(title_from_prompt),
            size: Size { width: Dim::Cells(width), height: Dim::Cells(height) },
            position: Position::Center,
            ..Default::default()
        },
    )?;

    // vim.ui contract: on_choice fires exactly once (Menu::open guarantees it).
    menu.open(move |choice: Option<usize>| match choice {
        Some(i) => {
            let item = originals.get(i).cloned().unwrap_or(Value::Nil);
            answer("vim.ui.select: on_choice", &on_choice, (item, (i + 1) as i64));
        },
        None => answer("vim.ui.select: on_choice", &on_choice, (Value::Nil, Value::Nil)),
    })
}

fn input_impl(opts: Option<&Table>, on_confirm: Function) -> Result<()> {
    let title: Option<String> = opts.and_then(|o| o.get::<String>("prompt").ok());
    let initial: String = opts.and_then(|o| o.get::<String>("default").ok()).unwrap_or_default();

    let input = Input::new(InputOptions {
        title: Some(title.as_deref().map(title_from_prompt).unwrap_or_else(|| " Input ".to_string())),
        initial,
        ..Default::default()
    })?;

    // vim.ui contract: on_confirm fires exactly once (Input::open guarantees it).
    input.open(move |text: Option<String>| match text {
        Some(text) => answer("vim.ui.input: on_confirm", &on_confirm, text),
        None => answer("vim.ui.input: on_confirm", &on_confirm, Value::Nil),
    })
}
