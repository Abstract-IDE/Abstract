# ab-nui

A small, composable **UI library for Neovim, written in Rust** and imported as
`wp_ui`. It is the foundation other Abstract plugins use to build their
interfaces — from one-line popups to full Flutter-style, reactive widget trees.

It talks to Neovim purely through the `vim.*` API via **mlua** — it does **not**
depend on `nvim-oxi`. (mlua here is pinned to the same version/feature set that
`nvim-oxi` uses, so Cargo unifies them into one crate instance and we reuse the
host's live `Lua`.)

---

## Table of contents

- [Design principles](#design-principles)
- [Architecture](#architecture)
- [The render pipeline](#the-render-pipeline-how-a-frame-happens)
- [Internals](#internals)
  - [1. The Lua bridge (`lua`)](#1-the-lua-bridge-lua)
  - [2. Reactivity (`reactive`)](#2-reactivity-reactive)
  - [3. Content & canvas (`text`, `view::canvas`)](#3-content--canvas-text-viewcanvas)
  - [4. Geometry (`geometry`)](#4-geometry-geometry)
  - [5. Neovim wrappers (`nvim`)](#5-neovim-wrappers-nvim)
  - [6. Core components (`widget`)](#6-core-components-widget)
  - [7. The view layer (`view`)](#7-the-view-layer-view)
  - [8. Surface lifecycle](#8-surface-lifecycle)
  - [9. Theme (`theme`)](#9-theme-theme)
- [State management](#state-management)
- [Examples](#examples)
- [API quick reference](#api-quick-reference)
- [Setup](#setup)

---

## Design principles

1. **No nvim-oxi.** Everything goes through Neovim's `vim` table via mlua. The
   host hands us its `Lua` once (`wp_ui::init`) and we keep it in a thread-local.
2. **Single-threaded.** Neovim's Lua runs on one thread, so the reactive graph
   uses `Rc`/`RefCell` and all callbacks are `Fn` — no locks, no `Send` bounds.
3. **Reactive by default.** UI is described as a function of state. Reads
   subscribe; writes repaint. You never manually redraw.
4. **Everything is a widget.** In the view layer, text, layout, spacing,
   buttons, lists — all implement one `Widget` trait and compose freely.
5. **Lifetime = handle.** A live UI is owned by a handle. Keep it → it persists
   (even hidden). Drop it → it is fully freed. Nothing leaks for the life of the
   process unless you intentionally make it global.

---

## Architecture

Two ways to build UI sit on a shared core. Pick the altitude you need:

```
            ┌─────────────────────────────────────────────────────────┐
            │  view  — declarative widget tree (Flutter-style)         │
   high     │  Surface · Widget · Column/Row/Center/... · Button/...   │
   level    │          · col!/row! macros                              │
            └───────────────┬─────────────────────────────────────────┘
                            │ compiles a tree → Canvas → Vec<Line>
            ┌───────────────┴─────────────────────────────────────────┐
            │  widget — imperative components                          │
   mid      │  Popup (float + buffer + reactive render + keymaps)      │
   level    │  Menu · Input                                            │
            └───────────────┬─────────────────────────────────────────┘
                            │
   ┌────────────┬───────────┴───────┬───────────────┬──────────────────┐
   │  reactive  │  text             │  geometry     │  theme            │
   │  Signal/   │  Span/Line        │  Size/Rect/   │  highlight groups │
   │  effect/   │  (styled content) │  Position     │                   │
   │  memo/Store│                   │               │                   │
   └────────────┴───────────────────┴───────────────┴──────────────────┘
            ┌─────────────────────────────────────────────────────────┐
   low      │  nvim — Buffer · Window · Namespace · keymap · autocmd    │
   level    ├─────────────────────────────────────────────────────────┤
            │  lua  — the single `vim.*` bridge (the only mlua surface) │
            └─────────────────────────────────────────────────────────┘
```

| Module     | Purpose                                                          |
| ---------- | --------------------------------------------------------------- |
| `lua`      | the one place that touches mlua; `vim.api`/`vim.fn`/`vim.cmd`   |
| `reactive` | `Signal`, `effect`, `memo`, `Store`, `store!` — state management |
| `text`     | `Span` / `Line` — styled content                                |
| `geometry` | `Dim`, `Size`, `Rect`, `Position` — float layout math           |
| `nvim`     | `Buffer`, `Window`, `Namespace`, keymaps, autocmds, `render`    |
| `theme`    | highlight groups (linked to standard groups by default)         |
| `widget`   | `Popup`, `Menu`, `Input` — imperative components                |
| `view`     | the declarative widget tree: `Widget`, `Surface`, widgets, macros |

---

## The render pipeline (how a frame happens)

For the view layer, one "frame" is one run of the reactive effect that drives a
`Surface`. End to end:

```
            build()                 measure + paint                to_lines()
 Signals ──────────────▶ Widget ───────────────────▶ Canvas ───────────────▶ Vec<Line>
   ▲       (tree built     tree    (lay out children    (cell    (merge cells   + spans
   │        fresh,                  within Area;          grid)    by hl group)
   │        reads tracked)          register Focusables)                │
   │                                                                     ▼
   │                                                         Popup.set_content()
   │                                                                     │
   │                                                       nvim_buf_set_lines +
   │                                                       extmark highlights
   │                                                                     │
   └──────────────── signal.set() re-runs this effect ◀──── key routed to focused
                                                              widget's handler
```

1. The effect calls `build()`, which constructs a fresh widget tree. Any
   `Signal`/`Store` read during `build()` (or inside a widget's `paint`) is
   recorded as a dependency of this effect.
2. `measure(constraints)` computes desired sizes; `paint(cx, area, canvas)` draws
   onto a cell `Canvas` and registers interactive regions (`Focusable`s) on `cx`.
3. The `Canvas` is flattened to `Vec<Line>` (adjacent cells with the same
   highlight merge into one `Span`) and pushed to the buffer.
4. A key press hits a buffer-local map → the runtime routes it to the focused
   widget's handler → the handler mutates a `Signal` → the effect re-runs →
   back to step 1. **No explicit redraw call exists.**

---

## Internals

### 1. The Lua bridge (`lua`)

The only module that imports mlua. The host calls `wp_ui::init(&lua)` once; the
handle is stored in a thread-local and cloned (cheaply — it's reference-counted)
whenever needed.

```rust
thread_local! { static LUA: RefCell<Option<Lua>> = const { RefCell::new(None) }; }
```

Everything else in the crate calls helpers here instead of mlua directly:

```rust
lua::call_api::<i64>("nvim_create_buf", (false, true))?;  // vim.api.*
lua::call_fn::<String>("expand", ("%:p",))?;              // vim.fn.*
lua::cmd("startinsert")?;                                  // vim.cmd(...)
lua::get_option::<i64>("columns")?;                        // nvim_get_option_value
```

This isolation is what keeps the rest of the crate nvim-oxi-free and makes it
trivial to see (and audit) every Neovim call.

### 2. Reactivity (`reactive`)

A tiny synchronous reactive graph, modeled on Solid/Leptos. Three primitives:
`Signal` (mutable reactive value), `effect` (auto-tracked side effect), `memo`
(cached derived value), plus `Store`/`store!` for shared/global values.

**The graph.** A thread-local `Runtime` holds everything, keyed by integer ids:

```rust
struct Runtime {
    next: Id,                                  // id allocator
    observer: Option<Id>,                      // effect currently running
    effects: HashMap<Id, Rc<RefCell<dyn FnMut()>>>,
    sources: HashMap<Id, HashSet<Id>>,         // effect -> signals it read
    subscribers: HashMap<Id, HashSet<Id>>,     // signal -> effects watching it
    running: HashSet<Id>,                       // re-entrancy guard
}
```

**Automatic dependency tracking.** A `Signal` is `{ id, Rc<RefCell<T>> }`.

- `get()` calls `track(id)`: if an effect is currently running (`observer` is
  set), it records the edge both ways (`subscribers[signal] += observer`,
  `sources[observer] += signal`).
- `set()`/`update()` mutate the value, then `notify(id)`: every subscribed effect
  is re-run.

**Running an effect** detaches its old dependencies first, so each run rebuilds
an exact, minimal dependency set (no stale edges):

```rust
fn run_effect(id) {
    // 1. (graph locked) skip if already running; drop old `sources[id]` edges;
    //    take the closure; set observer = id.
    // 2. (graph UNLOCKED) call the closure — its get()s re-subscribe via track().
    // 3. (graph locked) restore previous observer.
}
```

The borrow of the runtime is always released **before** user code runs, and a
`running` set prevents a write→run→write cycle from recursing into the same
effect. That's what makes `signal.set()` safe to call from inside an effect or a
key handler.

**`memo(f)`** is an effect that writes its result into a private signal, so it
recomputes only when its inputs change and notifies its own readers.

**Disposal.** `effect()` returns an `EffectHandle`. Dropping it does *nothing*
(so `effect(..)` stays fire-and-forget); calling `handle.dispose()` (or
`dispose_effect(id)`) removes the closure and detaches it from every signal.
This is how a closed `Surface` stops reacting — see [§8](#8-surface-lifecycle).

**`Store` & `store!`.** `Store<T>` is a clone-shareable wrapper over a `Signal`
— the recommended type for shared state. `store!` declares a lazily-initialized,
process-global one:

```rust
wp_ui::store!(pub static THEME: u8 = 0);
THEME::get();              // reactive read (widgets reading it re-render)
THEME::update(|t| *t ^= 1);
let sig = THEME::signal(); // hand the Signal to a widget
```

### 3. Content & canvas (`text`, `view::canvas`)

`text` is the styled-content model used everywhere:

```rust
Span { text: String, hl: Option<String> }   // a run of text + highlight group
Line { spans: Vec<Span> }                    // one rendered line
```

The **core** (`Popup`) renders `&[Line]` directly: it writes the plain text with
`nvim_buf_set_lines`, then lays one extmark per styled span (highlight columns
are byte offsets, matching the text). See `nvim::render`.

The **view** layer paints onto a `Canvas` instead — a `width × height` grid of
`Cell { ch, hl }`. After painting, `Canvas::to_lines()` walks each row and merges
adjacent cells that share a highlight into a single `Span`, producing the exact
same `Vec<Line>` the core renders. So the fancy widget tree and the humble popup
bottom out in the identical rendering path.

### 4. Geometry (`geometry`)

Float placement math. `Dim` is `Cells(u32)` or `Ratio(f64)`; `Size` pairs two
`Dim`s; `Position` is `Center` or `At { row, col }`. `Rect::resolve(size, pos)`
turns those into concrete editor cells against the live editor size, and
`split_h`/`split_v` carve a rect into panes (for multi-window layouts).

> Note: `geometry::Size` (float sizing, `Dim`-based) is distinct from
> `view::Size` (a measured `w × h` in cells used during widget layout).

### 5. Neovim wrappers (`nvim`)

Thin, safe handles over the API — all built on `lua::call_api`:

- **`Buffer(i64)`** — `scratch`, `set_lines`/`set_all`/`get_lines`,
  `set_option`, `lock`, `delete`. Writes go through `with_modifiable`, which
  temporarily clears `modifiable`/`readonly` and restores them (so locked UI
  buffers can be repainted without the `W10` warning).
- **`Window(i64)`** — `open_float(buf, &FloatConfig)`, `set_config`,
  `set_option`, `apply_default_highlight`, cursor, `close`. `FloatConfig` carries
  border/title/zindex/etc and builds the `nvim_open_win` table.
- **`Namespace(i64)`** — `create`, `clear`, `highlight(row, start, end, group)`
  via extmarks.
- **events** — `buf_keymap(buf, mode, lhs, opts, Fn)` (a Rust callback as a Lua
  function via `vim.keymap.set`), `augroup`, `buf_autocmd`.
- **`render(buf, ns, &[Line])`** — the core line+highlight renderer.

### 6. Core components (`widget`)

Imperative components for quick UIs:

- **`Popup`** — the workhorse: a scratch buffer shown in a float, with:
  - `set_content(&[Line])` (static) or `render_reactive(|| Vec<Line>)` (an effect
    that repaints when its signals change),
  - `on_key(mode, lhs, Fn)` for buffer-local Rust keymaps,
  - `closer()` — a reusable close handle for callbacks,
  - `inner_size()` — its content size in cells (used by `Surface`).
- **`Menu`** — selectable list over `Popup` + a `Signal<usize>` selection.
- **`Input`** — single-line prompt over `Popup`, value exposed as `Signal<String>`.

### 7. The view layer (`view`)

The declarative, "everything is a widget" layer.

**The trait.** A two-phase layout, plus interaction registration:

```rust
pub trait Widget {
    fn measure(&self, c: Constraints) -> Size;          // desired size, no effects
    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas); // draw + register
    fn flex(&self) -> u16 { 0 }                          // flex weight on parent's axis
}
pub type Element = Box<dyn Widget>;
```

Layout containers (`Column`/`Row`) `measure` their children, distribute space
(fixed children keep their size; `flex() > 0` children — `Spacer`, `Expanded` —
share the leftover), then `paint` each child into its sub-`Area`. `Center`,
`Align`, `Padding`, `SizedBox`, `Divider`, and `Container` (bordered/titled box)
round out layout; `Text` is the leaf; `Button`, `ListView`, `TextField` are
interactive.

**Interactions are part of the tree.** During `paint`, an interactive widget:

```rust
let focused = cx.will_focus();          // am I the focused one? (style accordingly)
cx.register(area, Rc::new(move |key| {  // returns true if it consumed the key
    match key { Key::Enter => { on_press(); true } _ => false }
}));
```

Focus ids are assigned by **registration order**, which is stable across rebuilds
(the tree is rebuilt identically each frame), so focus stays put without manual
keys. The current focus index is a `Signal<usize>` — so moving focus repaints.

**Key routing (`Surface`).** Buffer-local maps are installed **once** for
`<Tab>`/`<S-Tab>` (focus next/prev), the named keys (`<CR>`, `<BS>`, arrows, …),
and every printable ASCII char. Each map calls a dispatcher that looks up the
*currently focused* widget (from a `RefCell<Vec<Focusable>>` refreshed every
frame) and hands it the `Key`. Because all keys funnel through the focused
widget, the same `j` means "move down" to a `ListView` and "type j" to a
`TextField` — no modal Neovim insert mode involved.

### 8. Surface lifecycle

A `Surface` is an owned RAII handle (`Rc<SurfaceInner>`) that **owns the UI's
lifetime**:

| Action                       | Window | State (signals) | Render effect |
| ---------------------------- | ------ | --------------- | ------------- |
| `hide()` / `toggle()`        | closed | **kept**        | kept (still reacts) |
| `reopen()` / `toggle()`      | shown  | intact          | intact        |
| `close()` or **drop handle** | closed | **freed**       | disposed      |

- The buffer is created with `bufhidden = "hide"`, so closing the window keeps
  the buffer (and thus the rendered state); the buffer is wiped explicitly on
  free.
- `mount` runs once: it starts the render effect (storing the `EffectHandle`) and
  installs the key maps. `show(build)` mounts then opens; `reopen`/`hide`/
  `toggle` just move the window.
- `Drop for SurfaceInner` (and `close()`) calls `free()`: dispose the effect,
  close the window, delete the buffer — releasing the captured state. A
  `BufWipeout` autocmd is a safety net if the buffer is wiped externally.

The upshot: **persist while hidden, freed when the handle goes away.** Keeping
the handle in your plugin's state is normal app-state management, not a leak —
you can free it whenever you like.

### 9. Theme (`theme`)

A handful of highlight groups (`AbNuiNormal`, `AbNuiBorder`, `AbNuiTitle`,
`AbNuiSelection`, `AbNuiField`, `AbNuiFocus`, …) linked to standard Neovim groups
by default, so widgets look right in any colorscheme. `theme::setup()` installs
them (with `default = true`, so user overrides win); `theme::set_hl`/`link`
restyle.

---

## State management

Two kinds of state, both reactive — pick by lifetime:

- **Scoped (per-UI)** — a `Signal` captured by a widget tree, owned by its
  `Surface`. Lives while the surface lives; freed when it's dropped. Use for a
  window's own state (a counter, a query string, a selection).
- **Global** — a `Store` / `store!` value. Intentionally process-global; any
  widget that reads it re-renders when it changes. Use for app-wide state
  (theme, mode, shared selection) you manage yourself.

```rust
// scoped: lifetime tied to the Surface
let count = Signal::new(0);

// global: lifetime tied to the process (you control it)
wp_ui::store!(static MODE: u8 = 0);
```

---

## Examples

### Hello world (core `Popup`)

```rust
use wp_ui::prelude::*;

let popup = Popup::new(PopupOptions {
    title: Some(" Abstract UI ".into()),
    size: Size::cells(40, 9),
    position: Position::Center,
    ..Default::default()
})?;
popup.open()?;
popup.set_content(&[Line::empty(), Line::raw("        hello world")])?;
popup.on_close_key("q")?;
```

### Reactive counter (view + macros)

```rust
use wp_ui::prelude::*;

let count = Signal::new(0i64);
let surface = Surface::float(PopupOptions {
    title: Some(" Counter ".into()),
    size: Size::cells(44, 9),
    ..Default::default()
})?
.show(move || {
    let dec = count.clone();
    let inc = count.clone();
    col![
        Expanded::new(Center::new(
            Text::new(format!("count: {}", count.get())).fg("Title")
        )),
        Padding::symmetric(2, 1, row![
            Button::new("-1").on_press(move || dec.update(|n| *n -= 1)),
            Spacer::new(),
            Text::new("Tab focus · CR press · Esc hide").fg("Comment"),
            Spacer::new(),
            Button::new("+1").on_press(move || inc.update(|n| *n += 1)),
        ]),
    ]
})?;
// keep `surface` alive; `surface.toggle()` to hide/show; drop it to free.
```

### Menu & input (core components)

```rust
Menu::new(
    vec![MenuItem::new("Open", "open"), MenuItem::new("Quit", "quit")],
    MenuOptions { title: Some(" Actions ".into()), ..Default::default() },
)?
.open(|value| { /* chosen */ })?;

Input::new(InputOptions { title: Some(" Rename ".into()), ..Default::default() })?
    .open(|text| { /* submitted */ })?;
```

### Global store driving a widget

```rust
wp_ui::store!(static TICK: i64 = 0);

// a window that reacts to TICK, even though it never sets it:
let view = Surface::float(opts)?
    .show(|| Center::new(Text::new(format!("tick: {}", TICK::get())).fg("Title")))?;

// elsewhere — this repaints the window above:
TICK::update(|n| *n += 1);
```

### A custom widget

Everything is a `Widget`; your own is no different:

```rust
use wp_ui::view::{Area, Constraints, Cx, Size, Widget};
use wp_ui::view::canvas::Canvas;

struct Dot { filled: bool }

impl Widget for Dot {
    fn measure(&self, c: Constraints) -> Size { Size::new(1.min(c.max_w), 1) }
    fn paint(&self, _cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let ch = if self.filled { '●' } else { '○' };
        canvas.set(area.x, area.y, ch, None);
    }
}

// use it like any built-in: col![ Dot { filled: true }, Text::new("ready") ]
```

---

## API quick reference

```rust
// reactive
Signal::new(v) -> Signal<T>           // .get() .with(f) .get_untracked() .set(v) .update(f)
effect(|| { .. }) -> EffectHandle     // .dispose();  dispose_effect(id)
memo(|| expr) -> Memo<T>              // .get() .with(f)
Store::new(v) -> Store<T>             // same surface as Signal + .signal()
store!(pub static NAME: T = init);    // NAME::get/set/update/signal

// text
Span::raw(s) / Span::hl(s, group);  Line::raw(s) / Line::from_spans([..]) / Line::empty()

// geometry
Size::cells(w, h) / Size::ratio(w, h);  Position::Center / Position::At { row, col }

// core components
Popup::new(opts)? -> .open()? .set_content(&[..])? .render_reactive(||..) .on_key(..)? .close()
Menu::new(items, opts)?.open(on_choose)?;  Input::new(opts)?.open(on_submit)?

// view: layout
col![..] / row![..]   Column / Row (.main(..).cross(..))   Center / Align(alignment, child)
Padding::all(n, c) / ::symmetric(h, v, c)   SizedBox::new(w,h) / ::w(n) / ::h(n)
Spacer::new()   Expanded::new(c)   Divider::new()   Container::new(c).border(..).title(..).padding(..)
// view: content & interactive
Text::new(s).fg(group)   Button::new(s).on_press(||..)
ListView::new(items, sel_signal).on_select(|i|..)   TextField::new(value_signal).placeholder(..)
// view: runtime
Surface::float(opts)?.show(build)? -> Surface   // .hide() .reopen() .toggle()? .close() .is_visible()
```

---

## Setup

Call `wp_ui::init` once during plugin setup with the host's `Lua` handle, then
install the default highlight groups:

```rust
wp_ui::init(&nvim_oxi::mlua::lua());
wp_ui::theme::setup().ok();
```

After that, build UI from anywhere on the Lua thread. See
`code/src/neovim/editor/src/core/ui_demo.rs` for working `:AbstractHello`,
`:AbstractCounter`, and `:AbstractGlobal` commands.
