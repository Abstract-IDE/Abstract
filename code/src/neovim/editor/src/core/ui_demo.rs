//! `wp_ui` demo commands — a living gallery of everything the UI library can
//! do, and the manual integration test for it. `:AbstractGallery` is the tour.

use std::cell::RefCell;

use wp_ui::nvim::{CommandOpts, user_command};
use wp_ui::prelude::*;
use wp_ui::view::widgets::progress::SPINNER_DOTS;

thread_local! {
    static COUNTER: RefCell<Option<Surface>> = const { RefCell::new(None) };
    static GALLERY: RefCell<Option<Surface>> = const { RefCell::new(None) };
    static SPINNER: RefCell<Option<(Surface, SpinnerState)>> = const { RefCell::new(None) };
    static SPLIT: RefCell<Option<Surface>> = const { RefCell::new(None) };
    static PICKER: RefCell<Option<Layout>> = const { RefCell::new(None) };
}

/// Register the `:Abstract*` demo commands. Requires `wp_ui::init` to have
/// been called.
pub(crate) fn setup() -> wp_ui::Result<()> {
    wp_ui::theme::setup()?;
    // ab-nui becomes the `vim.ui.select` / `vim.ui.input` implementation.
    wp_ui::vim_ui::register_all()?;

    let opts = CommandOpts { desc: Some("ab-nui demo".into()), ..Default::default() };

    user_command("AbstractHello", &opts, |_| {
        let _ = hello();
    })?;
    user_command("AbstractCounter", &opts, |_| {
        let _ = counter();
    })?;
    user_command("AbstractMenu", &opts, |_| {
        let _ = menu();
    })?;
    user_command("AbstractInput", &opts, |_| {
        let _ = input();
    })?;
    user_command("AbstractSelect", &opts, |_| {
        let _ = wp_ui::lua::cmd(
            "lua vim.ui.select({'alpha','beta','gamma'}, {prompt='Pick one'}, \
             function(item) if item then vim.notify('picked: '..item) end end)",
        );
    })?;
    user_command("AbstractSpinner", &opts, |_| {
        let _ = spinner();
    })?;
    user_command("AbstractSplit", &opts, |_| {
        let _ = split();
    })?;
    user_command("AbstractNotify", &opts, |_| {
        let _ = notify_demo();
    })?;
    user_command("AbstractGallery", &opts, |_| {
        let _ = gallery();
    })?;
    user_command("AbstractPicker", &opts, |_| {
        let _ = picker();
    })?;
    Ok(())
}

/// Static popup (the README's hello world).
fn hello() -> wp_ui::Result<()> {
    let popup = Popup::new(PopupOptions {
        title: Some(" Abstract UI ".into()),
        size: Size::cells(40, 9),
        position: Position::Center,
        ..Default::default()
    })?;
    popup.open()?;
    popup.set_content(&[Line::empty(), Line::raw("        hello world — press q")])?;
    popup.on_close_key("q")?;
    Ok(())
}

/// Reactive counter surface; the command toggles it (state survives hiding).
fn counter() -> wp_ui::Result<()> {
    COUNTER.with(|slot| {
        let mut slot = slot.borrow_mut();
        if let Some(surface) = slot.as_ref() {
            return surface.toggle();
        }
        let count = Signal::new(0i64);
        let surface = Surface::float(PopupOptions {
            title: Some(" Counter ".into()),
            size: Size::cells(46, 9),
            ..Default::default()
        })?
        .show(move || {
            let dec = count.clone();
            let inc = count.clone();
            col![
                Expanded::new(Center::new(
                    text!["count: ", (count.get().to_string(), "Number")]
                )),
                Padding::symmetric(2, 1, row![
                    Button::new("-1").on_press(move || dec.update(|n| *n -= 1)),
                    Spacer::new(),
                    KeyHints::new(vec![("Tab", "focus"), ("CR", "press"), ("Esc", "hide")]),
                    Spacer::new(),
                    Button::new("+1").on_press(move || inc.update(|n| *n += 1)),
                ]),
            ]
        })?;
        *slot = Some(surface);
        Ok(())
    })
}

/// A long menu — exercises cursor sync + scroll-follow.
fn menu() -> wp_ui::Result<()> {
    let items: Vec<MenuItem<usize>> =
        (1..=50).map(|i| MenuItem::new(format!("Item number {i}"), i)).collect();
    Menu::new(items, MenuOptions { title: Some(" Long Menu ".into()), ..Default::default() })?
        .open(|i| {
            let _ = notify(format!("chose item {i}"), NotifyOptions::default());
        })
}

/// Input with live on_change echo.
fn input() -> wp_ui::Result<()> {
    Input::new(InputOptions { title: Some(" Rename ".into()), ..Default::default() })?
        .on_change(|text| {
            let _ = wp_ui::lua::cmd(&format!("echo 'typing: {}'", text.replace('\'', "''")));
        })
        .open_with(
            |text| {
                let _ = notify(format!("submitted: {text}"), NotifyOptions::default());
            },
            || {
                let _ = notify("cancelled", NotifyOptions { level: Level::Warn, ..Default::default() });
            },
        )
}

/// Timer-driven spinner; closing (running the command again) stops the timer.
fn spinner() -> wp_ui::Result<()> {
    SPINNER.with(|slot| {
        let mut slot = slot.borrow_mut();
        if slot.take().is_some() {
            return Ok(()); // dropped: surface freed, timer closed
        }
        let state = SpinnerState::start(80)?;
        let progress = Signal::new(0.0f32);
        {
            // Drive the progress bar off the same ticks.
            let frame = state.frame();
            let progress = progress.clone();
            let _keep = effect(move || {
                let f = frame.get();
                progress.set_silent(((f % 100) as f32) / 100.0);
            });
        }
        let spin = state.clone();
        let surface = Surface::float(PopupOptions {
            title: Some(" Working… ".into()),
            size: Size::cells(40, 5),
            ..Default::default()
        })?
        .show(move || {
            let frame = spin.frame().get();
            col![
                Padding::symmetric(2, 1, row![
                    Spinner::new(&spin).frames(SPINNER_DOTS),
                    SizedBox::w(1),
                    Text::new("run :AbstractSpinner again to stop"),
                ]),
                Padding::symmetric(2, 0, ProgressBar::new(((frame % 100) as f32) / 100.0)
                    .label(format!("{}%", frame % 100))),
            ]
        })?;
        *slot = Some((surface, state));
        Ok(())
    })
}

/// A split-hosted surface + raw extmark decorations (virt_text/virt_lines).
fn split() -> wp_ui::Result<()> {
    SPLIT.with(|slot| {
        let mut slot = slot.borrow_mut();
        if let Some(surface) = slot.as_ref() {
            return surface.toggle();
        }
        let surface = Surface::split(SplitConfig {
            size: Some(40),
            ..SplitConfig::new(SplitDir::Right)
        })?
        .show(|| {
            col![
                Text::new("A split-hosted surface").fg("Title"),
                Divider::new(),
                Text::new("Below: extmark decorations added to this buffer.").wrap(Wrap::Word),
            ]
        })?;

        // Raw extmark decorations on the surface's buffer.
        let buf = surface.popup().buffer();
        let ns = Namespace::create("abstract_demo_marks")?;
        ns.set_extmark(
            buf,
            0,
            0,
            &ExtmarkOpts {
                virt_text: Some(Line::from_spans([Span::hl("● virt_text eol", "DiagnosticInfo")])),
                virt_text_pos: Some(VirtTextPos::Eol),
                ..Default::default()
            },
        )?;
        ns.set_extmark(
            buf,
            1,
            0,
            &ExtmarkOpts {
                virt_lines: Some(vec![
                    Line::from_spans([Span::hl("  ── a virtual line ──", "Comment")]),
                ]),
                ..Default::default()
            },
        )?;
        *slot = Some(surface);
        Ok(())
    })
}

/// Toast notifications, one per level.
fn notify_demo() -> wp_ui::Result<()> {
    notify("plain info toast", NotifyOptions::default())?;
    notify("build finished", NotifyOptions { level: Level::Success, ..Default::default() })?;
    notify(
        "watch out — this one wraps because it is a fairly long warning message",
        NotifyOptions { level: Level::Warn, ..Default::default() },
    )?;
    notify(
        "something failed (sticky — dismissed only via handle)",
        NotifyOptions { level: Level::Error, timeout_ms: Some(8000), ..Default::default() },
    )?;
    Ok(())
}

/// The kitchen sink: tabs switching between form, data, and misc pages.
fn gallery() -> wp_ui::Result<()> {
    GALLERY.with(|slot| {
        let mut slot = slot.borrow_mut();
        if let Some(surface) = slot.as_ref() {
            return surface.toggle();
        }

        // All state created ONCE, captured by the rebuilt-per-frame tree.
        let tab = Signal::new(0usize);
        let field = FieldState::new("");
        let area = AreaState::new("multi-line\ntext area");
        let checked = Signal::new(true);
        let on = Signal::new(false);
        let radio = Signal::new(0usize);
        let select = SelectState::new();
        let list_state = ListState::new();
        let table_state = ListState::new();
        let tree_state = TreeState::new();
        let scroll = Signal::new(0u16);

        let surface = Surface::float(PopupOptions {
            title: Some(" Gallery ".into()),
            size: Size::ratio(0.7, 0.7),
            ..Default::default()
        })?
        .show(move || {
            let page: Element = match tab.get() {
                0 => Box::new(col![
                    Padding::all(1, col![
                        row![Text::new("name:  ").fg("Comment"), Expanded::new(TextField::new(&field).placeholder("type here…"))],
                        SizedBox::h(1),
                        row![
                            Checkbox::new("checkbox", checked.clone()),
                            SizedBox::w(3),
                            Toggle::new("toggle", on.clone()),
                        ],
                        SizedBox::h(1),
                        RadioGroup::new(vec!["one".into(), "two".into(), "three".into()], radio.clone()).horizontal(),
                        SizedBox::h(1),
                        row![Text::new("select: ").fg("Comment"), Select::new(vec!["red".into(), "green".into(), "blue".into()], &select)],
                        SizedBox::h(1),
                        SizedBox::new(40, 4).child(TextArea::new(&area)),
                    ]),
                ]),
                1 => Box::new(Padding::all(1, row![
                    Expanded::new(
                        List::new((1..=40).map(|i| format!("row {i}")).collect::<Vec<_>>(), &list_state)
                            .multi_select()
                    ),
                    Divider::vertical(),
                    Expanded::new(Table::new(
                        vec![
                            TableColumn::new("name").width(ColWidth::Flex(2)),
                            TableColumn::new("size").width(ColWidth::Fixed(8)).align(TextAlign::Right),
                            TableColumn::new("kind").width(ColWidth::Flex(1)),
                        ],
                        (1..=30)
                            .map(|i| vec![
                                Line::raw(format!("file_{i}.rs")),
                                Line::raw(format!("{} K", i * 3)),
                                Line::from_spans([Span::hl("rust", "Function")]),
                            ])
                            .collect(),
                        &table_state,
                    )),
                ])),
                _ => Box::new(Padding::all(1, row![
                    Expanded::new(Tree::new(
                        vec![
                            TreeNode::new("src".to_string(), vec![
                                TreeNode::new("view".to_string(), vec![
                                    TreeNode::leaf("widget.rs".to_string()),
                                    TreeNode::leaf("canvas.rs".to_string()),
                                ]),
                                TreeNode::leaf("lib.rs".to_string()),
                            ]),
                            TreeNode::leaf("Cargo.toml".to_string()),
                        ],
                        &tree_state,
                    )),
                    Divider::vertical(),
                    Expanded::new(ScrollView::new(scroll.clone(), col![
                        Text::new("A ScrollView over wrapped text. ".repeat(12)).wrap(Wrap::Word),
                        Divider::new(),
                        Text::new("日本語 CJK width test ▶ aligned").fg("Title"),
                        Text::new("Scroll with j/k when focused.").fg("Comment"),
                    ])),
                ])),
            };

            col![
                TabBar::new(vec!["Form".into(), "Data".into(), "Misc".into()], tab.clone()),
                Divider::new(),
                Expanded::new(Stack::new(vec![page])),
                Divider::new(),
                KeyHints::new(vec![("Tab", "focus"), ("h/l", "tabs"), ("Esc", "hide")]),
            ]
        })?;
        *slot = Some(surface);
        Ok(())
    })
}

/// Telescope-shape: query field + filtered list + live preview, three
/// surfaces arranged by a `Layout`, sharing signals.
fn picker() -> wp_ui::Result<()> {
    PICKER.with(|slot| {
        let mut slot = slot.borrow_mut();
        if let Some(layout) = slot.as_ref() {
            return layout.toggle();
        }

        let items: Vec<String> = (1..=100).map(|i| format!("crates/module_{i:03}.rs")).collect();
        let query = FieldState::new("");
        let list_state = ListState::new();

        let field_pane = {
            let query = query.clone();
            Surface::float(PopupOptions {
                title: Some(" Find ".into()),
                enter: true,
                ..Default::default()
            })?
            .show(move || TextField::new(&query).placeholder("filter…"))?
        };
        field_pane.hide(); // Layout::open shows everything in place

        let list_pane = {
            let items = items.clone();
            let query = query.clone();
            let state = list_state.clone();
            Surface::float(PopupOptions { enter: false, ..Default::default() })?.show(move || {
                let q = query.value.get().to_lowercase();
                List::new(items.clone(), &state).filter(move |item: &String| item.to_lowercase().contains(&q))
            })?
        };
        list_pane.hide();

        let preview_pane = {
            let items = items.clone();
            let state = list_state.clone();
            Surface::float(PopupOptions {
                title: Some(" Preview ".into()),
                enter: false,
                ..Default::default()
            })?
            .show(move || {
                let i = state.selected.get().min(items.len().saturating_sub(1));
                col![
                    text![("path: ", "Comment"), (items[i].clone(), "Title")],
                    Divider::new(),
                    Text::new(format!("pretend contents of {} — the preview pane re-renders \
                                       whenever the selection signal changes.", items[i]))
                        .wrap(Wrap::Word),
                ]
            })?
        };
        preview_pane.hide();

        let layout = Layout::new(
            Size::ratio(0.8, 0.8),
            Position::Center,
            Pane::col(vec![
                Pane::surface(field_pane, Dim::Cells(3)),
                Pane::row(vec![
                    Pane::surface(list_pane, Dim::Ratio(0.4)),
                    Pane::surface(preview_pane, Dim::Ratio(0.6)),
                ]),
            ]),
        );
        layout.open()?;
        *slot = Some(layout);
        Ok(())
    })
}
