# ab-nui

A small, composable **UI library for Neovim, written in Rust** and imported as
`wp_ui`. It is the foundation other Abstract plugins use to build their
interfaces — from one-line popups to full Flutter-style, reactive widget trees.

It talks to Neovim purely through the `vim.*` API via **mlua** — it does **not**
depend on `nvim-oxi`. (The workspace pins mlua to the exact version/feature set
`nvim-oxi` requires — `0.11`, `luajit`, no `send` — so Cargo unifies them into
one crate instance and we reuse the host's live `Lua`. Both this crate and the
root `Cargo.toml` reference the same `[workspace.dependencies]` entry, so the
pin can't drift.)

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
- [Testing](#testing)
- [Setup](#setup)

---

## Design principles

1. **No nvim-oxi.** Everything goes through Neovim's `vim` table via mlua. The
   host hands us its `Lua` once (`wp_ui::init`) and we keep it in a thread-local.
2. **Single-threaded.** Neovim's Lua runs on one thread, so the reactive graph
   uses `Rc`/`RefCell` and all callbacks are `Fn` — no locks, no `Send` bounds.
3. **Reactive by default.** UI is described as a function of state. Reads
   subscribe; writes repaint (once per callback, however many signals it
   writes). You never manually redraw.
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
                            │ compiles a tree → Canvas → Vec<Row> (diffed)
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
   │  memo/     │                   │               │                   │
   └────────────┴───────────────────┴───────────────┴──────────────────┘
            ┌─────────────────────────────────────────────────────────┐
   low      │  nvim — Buffer · Window · Namespace · keymap · autocmd    │
   level    ├─────────────────────────────────────────────────────────┤
            │  lua  — the single `vim.*` bridge (the only mlua surface) │
            └─────────────────────────────────────────────────────────┘
```

| Module     | Purpose                                                          |
| ---------- | --------------------------------------------------------------- |
| `lua`      | the one place that touches mlua; cached `vim.api`/`vim.fn` handles, `cmd`, and the embedded Lua driver |
| `reactive` | `Signal`, `effect`, `memo`, `batch`, `untrack`, `unowned`, `store!` — state management |
| `text`     | `Span` / `Line` — styled content; `wrap_line`, `display_width`  |
| `geometry` | `Dim`, `Size`, `Rect`, `Position` — float layout math           |
| `nvim`     | `Buffer`, `Window` (floats + splits), extmarks (highlights, virt text/lines, signs), keymaps, autocmds, user commands, timers/`schedule`, the diffing `Renderer` |
| `theme`    | highlight groups (linked to standard groups by default)         |
| `widget`   | `Popup`, `Menu`, `Input` — imperative components                |
| `view`     | the declarative widget tree: `Widget`, `Surface`, 30+ widgets, macros, `notify` toasts, multi-window `Layout` |
| `vim_ui`   | adapters installing `Menu`/`Input` as `vim.ui.select`/`vim.ui.input` |

---

## The render pipeline (how a frame happens)

For the view layer, one "frame" is one run of the reactive effect that drives a
`Surface`. End to end:

```
            build()                    paint                  to_rows()
 Signals ──────────────▶ Widget ───────────────────▶ Canvas ───────────────▶ Vec<Row>
   ▲       (tree built     tree    (children measured   (cell    (merge cells   { text, marks }
   │        fresh,                  during paint;        grid)    by hl group)        │
   │        reads tracked)          register Focusables)                              ▼
   │                                                                    Popup::render_rows()
   │                                                                                 │
   │                                                                      nvim::Renderer: diff vs
   │                                                                      previous frame, changed
   │                                                                      runs of rows only
   │                                                                                 │
   │                                                                      ONE call into the Lua
   │                                                                      driver (set_lines +
   │                                                                      extmarks for those rows)
   │                                                                                 │
   └──────── signal.set() re-runs this effect ◀──── key routed to focused widget's handler
```

1. The effect calls `build()`, which constructs a fresh widget tree. Any
   `Signal`/`Memo` read during `build()` (or inside a widget's `paint`) is
   recorded as a dependency of this effect.
2. `paint(cx, area, canvas)` draws onto a cell `Canvas` and registers
   interactive regions (`Focusable`s) on `cx`. The surface does not call
   `root.measure(..)` first; widgets that need their children's sizes measure
   them during `paint`. After painting, the focus index is corrected (it
   follows the focused widget's key if the tree changed shape, and is clamped
   if the interactive set shrank). If it changed, the tree is painted once more
   in the same frame, so the highlight is never one frame late.
3. `Canvas::to_rows()` flattens the grid to `Vec<nvim::Row>` (`Row { text,
   marks }`; adjacent cells with the same highlight merge into one `Mark`).
   `Popup::render_rows()` hands them to the popup's `nvim::Renderer`, which
   compares them with the previous frame and sends only the changed runs of
   consecutive rows to Neovim, in one call into the Lua-side driver. Rows
   whose text is unchanged but whose highlights changed are re-highlighted
   without resending the text; an identical frame sends nothing.
4. A key press hits the surface's buffer-local key table → the runtime routes
   it to the focused widget's handler → the handler mutates one or more
   `Signal`s → the effect re-runs → back to step 1. The handler runs inside a
   `batch`, so its signal writes are queued and the effect runs once after the
   handler returns: one repaint per key press however many signals it writes.
   **No explicit redraw call exists.**

---

## Internals

### 1. The Lua bridge (`lua`)

The only module that imports mlua. The host calls `wp_ui::init(&lua)` once; the
handle is stored in a thread-local and cloned (cheaply — it's reference-counted)
whenever needed.

Crossing from Rust into Lua is the expensive part of every operation, so the
bridge keeps crossings few:

- **Cached handles.** `vim.api.*` and `vim.fn.*` functions are resolved once and
  kept; so are functions reachable by a dotted `vim` path
  (`lua::vim_fn("keymap.set")`).
- **An embedded Lua driver** (the `DRIVER` constant, compiled once) with
  `render`, `map_keys` and `timer_start`/`timer_stop`/`timer_close`. Rust hands
  it data and it makes the API calls on the Lua side, so painting a frame or
  installing a surface's key table is one crossing rather than one per extmark
  or per key.
- **`ManuallyDrop` thread-locals.** The `Lua` handle and the bridge are never
  dropped at thread exit, so nothing calls into Lua while Neovim is shutting
  down.

```rust
thread_local! {
    static LUA: ManuallyDrop<RefCell<Option<Lua>>> = const { ManuallyDrop::new(RefCell::new(None)) };
    static BRIDGE: ManuallyDrop<RefCell<Option<Rc<Bridge>>>> = const { ManuallyDrop::new(RefCell::new(None)) };
}
```

Everything else in the crate calls helpers here instead of mlua directly:

```rust
lua::call_api::<i64>("nvim_create_buf", (false, true))?;  // vim.api.* (cached)
lua::call_fn::<String>("expand", ("%:p",))?;              // vim.fn.*  (cached)
lua::vim_fn("keymap.set")?;                                // any vim.<dotted.path> function
lua::cmd("startinsert")?;                                  // nvim_exec2
lua::get_option::<i64>("columns")?;                        // nvim_get_option_value
```

`lua::cmd` calls `nvim_exec2` rather than `vim.cmd`: on current Neovim `vim.cmd`
is a callable table, not a function, so fetching it as a function fails.

This isolation is what keeps the rest of the crate nvim-oxi-free and makes it
trivial to see (and audit) every Neovim call.

### 2. Reactivity (`reactive`)

A tiny synchronous reactive graph, modeled on Solid/Leptos. Three primitives:
`Signal` (mutable reactive value), `effect` (auto-tracked side effect), `memo`
(cached derived value), plus `batch`/`untrack`/`unowned` and `store!` for
global values.

**The graph.** There are no integer-keyed tables of edges. A `Signal` owns its
subscriber list (behind an `Rc`), and an effect is a node that holds the lists
of the signals it read during its last run:

```rust
struct EffectNode {
    id: Id,
    kind: Kind,                              // Effect | Memo
    f: RefCell<Option<NodeFn>>,              // taken out while it runs
    sources: RefCell<Vec<Rc<Subscribers>>>,  // signals/memos read in the last run
    subs: Option<Rc<Subscribers>>,           // a memo's own readers
    children: RefCell<Vec<Id>>,              // effects/memos created in the last run
    state: Cell<State>,                      // clean / check / dirty
    // … queued, running, retry, disposed flags
}
```

A thread-local `Runtime` keeps a registry of every live effect (that is what
keeps an effect alive until it is disposed), the current observer and owner,
one queue of effects ordered by creation, and the batch depth.

**Automatic dependency tracking.**

- `get()`/`with()` call `track`: if an effect is currently running, it records
  the edge both ways (the signal's subscriber list gets a weak reference to the
  effect, the effect's `sources` gets the list). `get_untracked()` and
  `with_untracked()` read without subscribing, and `untrack(|| ..)` runs a
  closure with tracking switched off.
- `set()`/`update()` mutate the value, then mark subscribers: effects are
  queued, memos are marked dirty (and their readers "check"). A write never
  runs subscribers recursively: the queue is drained when the outermost write
  (or `batch`) finishes. Outside a batch, `set` has re-run its subscribers by
  the time it returns.

**Running an effect** detaches its old dependencies first, so each run rebuilds
an exact, minimal dependency set (no stale edges). The closure is taken out of
the node while it runs, so no `RefCell` borrow is held across user code and an
effect may dispose itself.

**Consistency and ordering.**

- Memos are pull-based: a write marks the memos downstream of it dirty, and
  reading a memo recomputes it first if needed. So reading a memo anywhere — in
  an effect, inside a `batch`, or untracked — always gives a value computed from
  the current signals, and no memo or effect ever sees a mix of old and new
  inputs (no glitches).
- Queued effects run in creation order, oldest first, so a parent runs before
  the effects it created. An effect reached only through memos whose values
  came out unchanged is skipped.
- An effect that writes a signal it also reads is re-run after it finishes,
  until it settles. An effect that runs more than 10 000 times in one flush is
  treated as a runaway: it alone is disposed (with one error notification);
  everything else keeps working.
- `batch(|| ..)` holds propagation back: effects that depend on any signal
  written inside run once, after the outermost `batch` returns. Every Neovim
  callback the library registers (keymaps, autocmds, timers, `schedule`/`defer`,
  user commands) already runs in a batch, so a handler that writes several
  signals causes one repaint. You only need `batch` when writing signals from
  somewhere else.
- `Signal::update`/`with` run their closure without propagating anything until
  the value's borrow is released, so the closure may write other signals whose
  subscribers read this one. Reading or writing *the same* signal inside its
  own `update`/`with` panics with a clear message.
- A new effect's first run happens immediately, even inside a batch; anything it
  writes is propagated once it finishes.
- A panic inside an effect doesn't kill it: the closure is restored and runs
  again on the next change (a panicking *first* run disposes the effect).

**Extras.** `Signal::set_if_changed(v)` skips the write (and returns `false`)
when `v` equals the current value; `Signal::with_untracked(f)` and
`Memo::get_untracked()` are the non-subscribing reads.

**`memo(f)`** caches a derived value (`T: Clone + PartialEq`). It is lazy after
its first computation, recomputes only when an input changed, and notifies its
readers only when the new value differs. Its computation is disposed when the
last clone of the `Memo` is dropped.

**Lifetimes.** Signals are reference counted: when the last clone goes away, the
value and its bookkeeping are freed. The runtime itself lives in a
`ManuallyDrop` thread-local and is left alone at thread exit.

**Ownership & disposal.** `effect()` returns an `EffectHandle`. Dropping it does
*nothing* (so `effect(..)` stays fire-and-forget); calling `handle.dispose()`
(or `dispose_effect(id)`) removes the closure and detaches it from every
signal. Effects and memos created while another effect runs are *owned* by it:
they are disposed when it re-runs or is disposed, so nested effects don't leak.
`unowned(|| ..)` opts out — whatever is created inside lives until its own
handle ends it. Popups, surfaces and toasts create their render effects this
way, so a window opened from inside an effect belongs to its handle, not to
that effect (see [§8](#8-surface-lifecycle)).

**Shared and global state.** A `Signal` is already a clone-shareable handle —
pass clones to everything that needs the value. `store!` declares a
lazily-initialized, process-global one:

```rust
wp_ui::store!(pub static THEME: u8 = 0);
THEME::get();              // reactive read (widgets reading it re-render)
THEME::update(|t| *t ^= 1);
let sig = THEME::signal(); // hand the Signal to a widget
```

`store!` also has `with`, `get_untracked`, `set_if_changed`, and works for
non-`Clone` types (only `get`/`get_untracked` need `Clone`).

### 3. Content & canvas (`text`, `view::canvas`)

`text` is the styled-content model used everywhere:

```rust
Span { text: String, hl: Option<String> }   // a run of text + highlight group
Line { spans: Vec<Span> }                    // one rendered line
```

The **core** (`Popup`) turns `&[Line]` into `nvim::Row`s (the line's text plus
one `Mark` per highlighted run; marks are byte offsets, matching the text) and
renders them through the diffing `Renderer` (see §5): `Popup::set_content(&[Line])`
flattens the lines to rows and only the rows that differ from the last frame are
sent to Neovim.

The **view** layer paints onto a `Canvas` instead — a `width × height` grid of
`Cell { ch, style }`. Highlight groups are interned per canvas: a cell holds a
small id into the canvas's style table, so cells are `Copy` and "same highlight
as the previous cell" is an integer comparison. `Canvas::reset(w, h)` clears the
grid and resizes it while reusing its allocations (a `Surface` keeps one canvas
for its whole life). After painting, `Canvas::to_rows()` walks each row and
merges adjacent cells that share a highlight into a single `Mark`; this is what
the view layer renders from. `Canvas::to_lines()` still exists and produces
the equivalent `Vec<Line>` (adjacent same-highlight cells merged into one
`Span`). Either way the widget tree and the humble popup end in the same
renderer.

### 4. Geometry (`geometry`)

Float placement math. `Dim` is `Cells(u32)` or `Ratio(f64)`; `Size` pairs two
`Dim`s; `Position` is `Center`, `At { row, col }` (absolute cells) or
`Ratio { row, col }` (a fraction of the free space: `1.0` hugs the bottom/right
edge, and it stays put relative to the editor on resize).

A popup's `Size` is its **content** size; the border is added around it.
`Rect::resolve_framed(size, pos, border.frame())` places the whole box (content
plus border) inside `usable_area()` — the editor minus the command line and a
visible tabline — so a `Size::ratio(1.0, 1.0)` popup fills the screen without
being clipped and `Center` is truly centred. `Rect::resolve(size, pos)` is the
border-less form. `split_h`/`split_v` carve a rect into panes (for multi-window
layouts).

> Note: `geometry::Size` (float sizing, `Dim`-based) is distinct from
> `view::Size` (a measured `w × h` in cells used during widget layout).

### 5. Neovim wrappers (`nvim`)

Thin, safe handles over the API — all built on `lua::call_api`:

- **`Buffer(i64)`** — `scratch`, `set_lines`/`set_all`/`get_lines`,
  `set_option`, `lock`, `delete`. Writes go through `with_modifiable`, which
  temporarily clears `modifiable`/`readonly` and restores them (so locked UI
  buffers can be repainted without the `W10` warning).
- **`Window(i64)`** — `open_float(buf, &FloatConfig)`, `set_config`,
  `set_option`, `apply_default_highlight`, cursor, `close`, `current()`.
  `FloatConfig` carries border/title/zindex/etc and builds the `nvim_open_win`
  table. `set_config` with `title: None` clears an existing title, and
  re-configuring a cursor-relative float keeps it where it is.
- **`Namespace(i64)`** — `create`, `clear`, `highlight(row, start, end, group)`
  via extmarks.
- **events** — `buf_keymap(buf, mode, lhs, opts, Fn)` (a Rust callback as a Lua
  function via `vim.keymap.set`), `buf_keymap_table(buf, mode, keys, dispatch)`
  (many keys bound to one Rust dispatcher in a single trip into Lua; each key
  is an `(lhs, code)` pair and `dispatch(code)` is called), `augroup`,
  `buf_autocmd`, `autocmd`, `user_command`. Every callback registered here runs
  inside a `batch`. `KeymapOpts` is `{ silent, nowait }` (callbacks are never
  remapped, so there is no `noremap`); `CommandOpts` with `complete` set and no
  `nargs` takes `nargs = "*"` (Neovim rejects completion without arguments).
- **`Renderer`, `Row`, `Mark`** — a `Row` is one buffer line (`text` plus
  `marks`, byte ranges with a highlight group). A `Renderer` is tied to one
  buffer + namespace and remembers the last frame it painted; `render(rows)`
  sends only the changed runs of rows, in one call into the Lua driver, and
  falls back to a full repaint when it does not know the buffer's contents
  (first frame, row count changed, `invalidate()` called, or the buffer's
  `changedtick` no longer matches because someone else edited it).
  `Row::from_line(&Line)` converts styled lines; a line break inside a line is
  shown as `↵` (a buffer line can't contain one — it used to blank the whole
  frame).
- **`render(buf, ns, &[Line])`** — the stateless form: replaces the whole buffer
  and every extmark of the namespace. Anything that repaints the same buffer
  repeatedly should keep a `Renderer` instead.
- **`Timer`** (`start`/`once`/`interval`), `schedule`, `defer` — callbacks run
  on the main loop inside a `batch`. A `Timer` never invokes its callback after
  `stop`/`close`/drop, not even for a tick that was already queued.

### 6. Core components (`widget`)

Imperative components for quick UIs:

- **`Popup`** — the workhorse: a scratch buffer shown in a float (or a split,
  via `PopupOptions.host`), with:
  - `set_content(&[Line])` (static) or `render_reactive(|| Vec<Line>)` (an effect
    that repaints when its signals change — owned by the popup, disposed on close);
    both go through the popup's diffing `Renderer`,
  - `render_rows(Vec<Row>)` — the same, for pre-flattened rows (what
    `Canvas::to_rows()` produces; used by `Surface`),
  - `invalidate()` — call after writing to the popup's buffer by other means
    (e.g. through `buffer()`), so the next paint starts from scratch instead of
    diffing against content that is no longer there,
  - `namespace()` — the highlight namespace the popup paints with,
  - `on_key(mode, lhs, Fn)` for buffer-local Rust keymaps,
  - `hide()` (window only — content, effects and keymaps survive, `open()`
    brings it back) vs `close()` (full teardown: effects disposed, per-instance
    augroup deleted, window closed, buffer deleted; `open()` after it is an
    error),
  - `closer()` — a reusable close handle for callbacks,
  - `set_position`/`set_size` — move/resize a live popup; `set_title(..)` /
    `clear_title()` (an empty title also clears),
  - `Popup::with_buffer(buf, opts)` — host a buffer you already have (a
    terminal, a file) instead of a scratch one; the popup manages only the
    window and never touches or deletes the buffer,
  - `inner_size()` — its content size in cells (used by `Surface`).
  - Each popup owns a unique autocmd group; `WinClosed` keeps `is_open()`
    truthful after an external `:q`, and `VimResized` re-anchors it.
- **`Menu`** — selectable list over `Popup`. The cursor row *is* the selection
  (any motion — `G`, counts, search, mouse — moves the `▌` marker too).
  `open(|choice: Option<T>| ..)` calls back **exactly once**: `Some` on `<CR>`,
  `None` on cancel — `<Esc>`, `q`, `<CR>` on an empty list, the window being
  closed any other way (`:q`, `<C-w>c`, another plugin), or focus leaving it.
  Opening from insert mode leaves insert mode first.
- **`Input`** — single-line prompt over `Popup`, value exposed as
  `Signal<String>`, with an `on_change` builder. `open(|text: Option<String>| ..)`
  has the same exactly-once contract as `Menu`.
- **`vim_ui::register_all()`** installs both as `vim.ui.select` /
  `vim.ui.input` (`opts` may be `nil`; errors raised by the caller's callback
  are reported with `vim.notify` instead of vanishing).

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

Layout containers (`Column`/`Row`) `measure` their children while painting,
distribute space (fixed children keep their size but never more than what is
left; `flex() > 0` children — `Spacer`, `Expanded` — share the leftover, with
rounding remainders handed out so every cell is used), then `paint` each child
into its sub-`Area`. (`Surface` itself only calls `paint` on the root.)

Two rules every widget follows (and a custom widget should too):

- **`measure` respects its constraints** — it returns `c.clamp(..)`, never more
  than `max_w × max_h`.
- **Painting is clipped to the widget's `Area`.** The `Canvas` carries a clip
  rectangle: layouts paint each child inside `canvas.with_clip(area, ..)`, and
  text goes through `put_str_clipped`/`put_line_clipped`, so a long label can't
  overwrite a border or a neighbour.

The widget set:

| Kind        | Widgets |
| ----------- | ------- |
| layout      | `Column` `Row` `Stack` `Positioned` `Center` `Align` `Padding` `SizedBox` `Spacer` `Expanded` `Fraction` `Divider` (h/v) `Container` (Single/Rounded/Double/Thick/Custom borders, title + footer, per-side padding) `Frame` (header/body/footer in 1–2 columns, divider through the body only, scrollbar thumb on the divider) `ScrollView` `Scrollbar` |
| content     | `Text` (multi-span, `wrap`, `align`, `ellipsis`, `max_lines`) `KeyHints` `ProgressBar` `Spinner` (timer-driven `SpinnerState`) |
| interactive | `Button` `Checkbox` `Toggle` `RadioGroup` `Select` (dropdown via overlay) `TabBar` `List` (custom rows, filter, multi-select, scrollbar, full-width selection bar) `Table` (Fixed/Fraction/Flex columns) `Tree` (collapsible, custom rendering) `TextField` (full cursor editing, mask, on_change/on_submit) `TextArea` |
| wrappers    | `Keyed` — stable focus keys + per-widget custom key handlers for any child |
| real buffers | `Embed` (an `EmbeddedWindow` laid over its area) · `BufferInput` (a one-line input edited with Vim itself) · `CodeView` (syntax-highlighted, read-only text with a highlighted line and matches) |

A boxed widget (`Element`) is itself a `Widget`, so a child picked at runtime
can go anywhere a widget is expected.

**Mouse wheel and touchpad.** A widget makes its area scrollable with
`cx.on_scroll(canvas.screen_map(), area, handler)` while painting; `List`,
`Table` and `Tree` move their selection, `ScrollView` its content. A surface
routes each wheel step to whatever is under the pointer: the innermost
scrollable widget, or one of its embedded windows (scrolled natively); outside
it, Neovim's own behaviour — nothing at all when the surface is modal.
`Surface::scroll_at(row, col, rows)` does the same from code. A click can't
leave the cursor in a part of a surface that isn't focusable (the frame, a
code view): it moves on to the surface's input.

**Lists.** The selected row is a full-width bar. Text styled with
`AbNuiMatch` (matched characters) keeps the bar's background and takes the
complementary text colour, bold (`theme::inverse(group)`); other styled text (an icon) keeps its colour on it
(`theme::mix(fg_group, bg_group)`). Both build combined groups on demand, kept
in sync with the colorscheme.
`.gutter(false)` drops the `▌`/`✓` column. `text::truncate_line_start(line,
width)` cuts from the left, keeping the end (a file name) visible;
`Text::ellipsis_start()` does it for a `Text`.

**Real buffers in the tree.** Some UI is best served by a real buffer: text
edited with every Vim motion, or a file shown with Neovim's own syntax
highlighting. An `EmbeddedWindow` is a real window the surface keeps exactly
over its `Embed` widget's area — placed after each paint, moved on resize,
hidden when the widget isn't painted or the surface is hidden, closed with the
surface (`on_open` runs each time it opens; `on_close` reports closes from
elsewhere, e.g. `:q` in it). Built on it:

- `BufferInput` — `value()` is a `Signal<String>` that follows every edit (a
  multi-line paste is joined), `placeholder(..)`, `start_insert(bool)`,
  `on_key(&["i", "n"], lhs, f)`, and `on_key_or(modes, lhs, f)` where `f`
  returns whether it took the key — `false` lets the key do what it normally
  does (it runs under Neovim's text lock: decide, schedule changes). `text()`
  reads the buffer, so it is current even mid-edit. Completion menus are off
  in it. (`nvim::buf_keymap_or` is the same for any buffer.)
- `CodeView` — `line_numbers(bool)` (default on), `show_media(path)` (see
  below), `set_content(&lines, filetype)` (treesitter when a parser
  exists, regex syntax otherwise), `highlight(Some(line), &[(start, end)])`
  (`AbNuiCodeLine`/`AbNuiCodeMatch`, line brought into view), `scroll_by(n)`.
  Not focusable, but the wheel/touchpad scrolls it.

**Media** (`media`, used by `CodeView::show_media`): images, video frames and
PDF pages in terminals with the kitty graphics protocol's unicode
placeholders (kitty, Ghostty; not inside tmux or a nested Neovim) and
`termguicolors` — no Neovim plugin involved. Images are decoded
in Rust (png, jpg, gif, webp, bmp, tiff, ico) on a background thread, scaled
to fit with their aspect kept (using the terminal's cell size in pixels), sent
as PNG through `nvim_ui_send`, and shown with Unicode placeholders — cells in
the buffer — so they move, clip and scroll with the window. A video frame
uses `ffmpeg`, a PDF page `pdftoppm` or `mutool`, when installed.

`icons::file(name)` gives a file's icon and highlight, in the file type's own
colour: from mini.icons when it is set up, else nvim-web-devicons (cached;
`None` without either). An `Embed` takes all the space it is given unless
given a fixed `.height(rows)` (a `BufferInput` is one row).

Widget state that must survive rebuilds lives in clone-shareable bundles
(`ListState`, `FieldState`, `AreaState`, `TreeState`, `SelectState`) created
once, outside the build closure.

`ScrollView` repaints when its offset signal changes (it used to read it
untracked).

Beyond single-window trees: `view::notify` shows stacked auto-dismissing toast
floats, and `view::Layout`/`Pane` arrange several `Surface`s (telescope-style
multi-pane UIs) that share state through common `Signal`s.

The canvas is unicode-width-aware: double-width glyphs (CJK, emoji) occupy two
cells (leader + continuation) so columns stay aligned with what Neovim renders.

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

**Key routing (`Surface`).** One buffer-local key table is installed **once**
through `buf_keymap_table`: `<Tab>`/`<S-Tab>`, the named keys (`<CR>`, `<BS>`,
arrows, …), `<Space>`, every printable ASCII char and `<C-a>`..`<C-z>` (except
`<C-c>`) are bound to a single Rust dispatcher (one callback and one trip into
Lua, instead of ~135 separate keymap callbacks). The dispatcher decodes the key
and first offers it to the *currently focused* widget (looked up in a
`RefCell<Vec<Focusable>>` refreshed every frame); if the widget does not consume
it, `<Tab>`/`<S-Tab>` move focus and `<Esc>` hides. Because all keys funnel
through the focused widget, the same `j` means "move down" to a `List` and
"type j" to a `TextField` — no modal Neovim insert mode involved. The dispatcher
runs in a `batch`, so a handler's signal writes cause one repaint. `<C-h>`
arrives as `Backspace` (many terminals send `^H` for it); `<C-i>`/`<C-m>`/`<C-[>`
arrive as `Tab`/`Enter`/`Esc`, since terminals send the same codes.

The key table is not installed at all for a surface whose window the cursor can
never be in — `focusable: false` and `enter: false`, e.g. a toast.

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
  installs the key table. `show(build)` mounts then opens (calling it a second
  time is an error); `reopen`/`hide`/`toggle` just move the window, and return a
  "surface is closed" error after `close()`.
- `Drop for SurfaceInner` (and `close()`) calls `free()`: dispose the effect,
  close the window, delete the buffer — releasing the captured state. A
  `BufWipeout` autocmd is a safety net if the buffer is wiped externally; if it
  is only *unloaded* (`:bdelete`), `reopen()` reinstalls the keys and repaints.
- The render effect is `unowned`, so a surface shown from inside an effect
  lives as long as its handle, not as long as that effect's current run.
- Only visible surfaces react to resizes, and only when their own size changed.
- A surface's view is pinned: if its window is scrolled (mouse wheel, touchpad,
  `zl`, `<C-e>`), it snaps back to its first line and column
  (`Window::reset_view`), so the UI never shows shifted or half-painted.
- **Modal** (`.modal(true)`): while shown, a backdrop (`AbNuiBackdrop`, dimmed
  with true colour) covers the rest of the editor and catches the mouse, so
  clicks and scrolls behind do nothing; and if the cursor lands in a window
  behind (`<C-w>w`, a plugin), it is put back — into the first focusable
  embedded window, else the surface. Floats stacked above the surface (a menu
  it opens) and the command-line window are allowed. `focus()` moves the
  cursor into the surface the same way.
- **Closing from inside the tree.** Capturing the `Surface` in its own `build`
  or key handlers creates a reference cycle (it would never be freed). Use
  `surface.closer()` instead: a `Fn()` holding a weak reference that closes the
  surface if it is still alive.

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
- **Global** — a `store!` value. Intentionally process-global; any
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
.open(|choice| match choice {
    Some(value) => { /* chosen */ }
    None => { /* cancelled — always reported, exactly once */ }
})?;

Input::new(InputOptions { title: Some(" Rename ".into()), ..Default::default() })?
    .open(|text: Option<String>| { /* Some(submitted) or None (cancelled) */ })?;
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
Signal::new(v) -> Signal<T>           // .get() .with(f) .get_untracked() .with_untracked(f) .set(v) .set_if_changed(v) .update(f)
effect(|| { .. }) -> EffectHandle     // .dispose();  dispose_effect(id)
memo(|| expr) -> Memo<T>              // .get() .get_untracked() .with(f)
batch(|| { .. }) / untrack(|| expr)   // group writes into one repaint / read without subscribing
unowned(|| ..)                         // effects created inside outlive the running effect
store!(pub static NAME: T = init);    // NAME::get/get_untracked/with/set/set_if_changed/update/signal

// nvim (wp_ui::nvim; Renderer/Row/Mark/buf_keymap_table are not in the prelude)
Renderer::new(buf, ns) -> .render(Vec<Row>)? .invalidate()    Row { text, marks: Vec<Mark> }   Row::from_line(&line)
Mark { start, end, hl }   nvim::render(buf, ns, &[Line])?     // stateless full repaint
buf_keymap_table(buf, "n", [(lhs, code)], |code| ..)?         // many keys, one dispatcher
Timer::start / ::once / ::interval                            // never fires after stop/close/drop
// canvas
Canvas::new(w, h) / .reset(w, h) / .to_rows() / .to_lines()   // wp_ui::view::canvas::Canvas

// text
Span::raw(s) / Span::hl(s, group);  Line::raw(s) / Line::from_spans([..]) / Line::empty()

// geometry
Size::cells(w, h) / Size::ratio(w, h);  Position::Center / ::At { row, col } / ::Ratio { row, col }

// core components
Popup::new(opts)? -> .open()? .set_content(&[..])? .render_rows(rows)? .invalidate() .namespace()
                     .render_reactive(||..) .on_key(..)? .set_title(..)? .clear_title()? .hide() .close()
Popup::with_buffer(buf, opts)?                                // float over an existing buffer
Menu::new(items, opts)?.open(|choice: Option<T>| ..)?;  Input::new(opts)?.open(|text: Option<String>| ..)?

// view: layout
col![..] / row![..] / stack![..]   Column / Row (.main(..).cross(..))   Stack / Positioned
Center / Align(alignment, child)   Padding::all/symmetric/only/left/top/right/bottom
SizedBox::new(w,h) / ::w(n) / ::h(n)   Spacer::new()   Expanded::new(c)   Fraction::w(0.5, c)
Divider::new() / ::vertical()   ScrollView::new(offset_signal, c)   Scrollbar::new(total, view, off)
Container::new(c).border(BoxStyle::Double).border_hl(..).title(..).title_pos(..).footer(..).padding(..)
// view: content
Text::new(s).fg(g) / ::from_line(line).span(s, g).wrap(Wrap::Word).align(..).ellipsis().max_lines(n)
text!["plain ", ("styled", "Title")]   KeyHints::new(vec![("q", "quit")])
ProgressBar::new(0.4).label("40%")   Spinner::new(&SpinnerState::start(80)?)
// view: interactive (state bundles survive rebuilds — create them once)
Button::new(s).on_press(||..)   Checkbox::new(s, bool_signal)   Toggle::new(s, bool_signal)
RadioGroup::new(opts, sel_signal).horizontal()   Select::new(opts, &SelectState::new())
TabBar::new(labels, active_signal)   List::new(items, &ListState::new()).render(..).filter(..).multi_select()
Table::new(cols, rows, &state)   Tree::new(nodes, &TreeState::new()).render(..).on_activate(..)
TextField::new(&FieldState::new("")).placeholder(..).mask('*').on_submit(..)   TextArea::new(&AreaState::new(""))
Keyed::new(w).focus_key("id").on_key(Key::Char('x'), ||..)
// view: runtime & beyond
Surface::float(opts)?.show(build)?   // .hide() .reopen() .toggle() .close() .closer() .modal(b) .focus() .initial_focus(i) .focus_wrap(b) .on_key(..)
Frame::new(vec![FrameColumn::new(header, body).footer(f), ..]).split(0.6).scrollbar(total, offset_signal)
EmbeddedWindow::new(EmbedOptions { focusable, enter, zindex })?  // .widget() .on_open(f) .on_close(f) .buffer() .window() .focus()
BufferInput::new()?.placeholder(s).start_insert(b)  // .value() .text() .set_text(s) .on_key(&["i","n"], lhs, f) .widget()
CodeView::new()?  // .set_content(&lines, ft) .highlight(Some(line), &[(s, e)]) .scroll_by(n) .widget()
icons::file(name) -> Option<(icon, hl)>
Surface::split(SplitConfig::new(SplitDir::Right))?.show(build)?
notify("msg", NotifyOptions { level: Level::Warn, ..Default::default() })? -> NotifyHandle
Layout::new(size, pos, Pane::col(vec![..])).open()?   // multi-window arrangements
wp_ui::vim_ui::register_all()?                          // become vim.ui.select / vim.ui.input
```

---

## Testing

- **Unit tests**: `cargo test -p ab-nui`.
- **Integration tests** run inside a real headless Neovim: `cargo test -p ab-nui-tests`.
  The crate lives in `nvim-tests/` and uses nvim-oxi's test harness
  (`#[nvim_oxi::test]`, one Neovim per test) purely as a dev tool; the library
  itself still does not depend on nvim-oxi. `nvim-tests/src/support.rs` has the
  shared helpers: `setup()` (init the library and install the theme), readers for
  what Neovim shows (`lines`, `trimmed`, `highlights`, `hl_at`, `floats`),
  `feed(keys)` to type keys, `wait(ms, pred)` to let scheduled callbacks and
  timers run, and `surf`/`render` to mount a widget tree on a `Surface` of a
  given size.
- **Benchmarks** are opt-in and run only when `AB_NUI_BENCH_OUT` names a file;
  they append wall time and per-iteration Neovim API call counts to it:

  ```sh
  AB_NUI_BENCH_OUT=/tmp/ab-nui-bench.txt cargo test -p ab-nui-tests --release bench
  ```

Measured against the previous implementation (release build, Neovim 0.12.6):

| Scenario                                      | Before    | After     | Note |
| --------------------------------------------- | --------- | --------- | ---- |
| key press that changes 3 signals              | 68.8 µs   | 15.2 µs   | 3 tree rebuilds → 1 |
| show + dismiss a toast                        | 1226 µs   | 172 µs    | 134 keymaps → 0 |
| create + show + close a surface               | 1258 µs   | 525 µs    | |
| scroll a 500-item list by one row             | 154.7 µs  | 113.5 µs  | every visible row changes, so diffing helps least here |

---

## Setup

Call `wp_ui::init` once during plugin setup with the host's `Lua` handle, then
install the default highlight groups:

```rust
wp_ui::init(&nvim_oxi::mlua::lua());
wp_ui::theme::setup().ok();
```

After that, build UI from anywhere on the Lua thread. The tests in `nvim-tests/src/`
(for example `widgets.rs`, `interactive.rs`, `lifecycle.rs`) are working examples of
every widget.
