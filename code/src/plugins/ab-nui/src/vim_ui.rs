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
use crate::widget::input::{Input, InputOptions};
use crate::widget::menu::{Menu, MenuItem, MenuOptions};

/// Install ab-nui as `vim.ui.select`.
pub fn register_select() -> Result<()> {
    let l = lua::lua()?;
    let func: Function = l.create_function(|_, (items, opts, on_choice): (Table, Table, Function)| {
        select_impl(&items, &opts, on_choice).map_err(mlua::Error::external)
    })?;
    lua::vim()?.get::<Table>("ui")?.set("select", func)?;
    Ok(())
}

/// Install ab-nui as `vim.ui.input`.
pub fn register_input() -> Result<()> {
    let l = lua::lua()?;
    let func: Function = l.create_function(|_, (opts, on_confirm): (Value, Function)| {
        let opts = match opts {
            Value::Table(t) => Some(t),
            _ => None,
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

fn select_impl(items: &Table, opts: &Table, on_choice: Function) -> Result<()> {
    // Label each item with opts.format_item (falling back to tostring).
    let format_item: Option<Function> = opts.get("format_item").ok();
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
        max_width = max_width.max(crate::text::display_width(&label));
        entries.push(MenuItem::new(label, i));
        originals.push(item);
    }

    let title: Option<String> = opts.get("prompt").ok();
    let height = (entries.len() as u32).clamp(1, 20);
    let width = (max_width as u32 + 4).clamp(20, 80);
    let menu = Menu::new(
        entries,
        MenuOptions {
            title: title.map(|t| format!(" {} ", t.trim().trim_end_matches(':'))),
            size: Size { width: Dim::Cells(width), height: Dim::Cells(height) },
            position: Position::Center,
            ..Default::default()
        },
    )?;

    // vim.ui contract: on_choice fires exactly once (Menu::open_with already
    // guarantees choose/cancel are mutually exclusive and single-shot).
    let choice = on_choice.clone();
    menu.open_with(
        move |i: usize| {
            let item = originals.get(i).cloned().unwrap_or(Value::Nil);
            let _ = choice.call::<()>((item, (i + 1) as i64));
        },
        move || {
            let _ = on_choice.call::<()>((Value::Nil, Value::Nil));
        },
    )
}

fn input_impl(opts: Option<&Table>, on_confirm: Function) -> Result<()> {
    let title: Option<String> = opts.and_then(|o| o.get::<String>("prompt").ok());
    let initial: String = opts.and_then(|o| o.get::<String>("default").ok()).unwrap_or_default();

    let input = Input::new(InputOptions {
        title: Some(
            title
                .map(|t| format!(" {} ", t.trim().trim_end_matches(':')))
                .unwrap_or_else(|| " Input ".to_string()),
        ),
        initial,
        ..Default::default()
    })?;

    let confirm = on_confirm.clone();
    input.open_with(
        move |text: String| {
            let _ = confirm.call::<()>(text);
        },
        move || {
            let _ = on_confirm.call::<()>(Value::Nil);
        },
    )
}
