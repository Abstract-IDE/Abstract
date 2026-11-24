# Local fork of nvim-oxi

This directory is a vendored copy of [noib3/nvim-oxi](https://github.com/noib3/nvim-oxi),
patched so it works with current Neovim releases. The workspace consumes it as a
path dependency (see `[workspace.dependencies]` in `code/Cargo.toml`).

| | |
|---|---|
| Upstream base | `main` @ `7ad27a7d3c8a72c34e7b9914eb77fb0644ff3f65` (2026-05-17, four commits past the `v0.6.0` tag) |
| Verified against | Neovim **0.12.6** |
| Feature to enable | `neovim-0-12` |

## Why it exists

nvim-oxi passes `#[repr(C)]` mirrors of Neovim's option structs ("keysets")
straight into Neovim's C functions, and computes each field's presence bit from
the complete set of field names. When Neovim adds, renames or reorders a field,
every binding that uses that struct silently breaks until the mirror is updated.

Neovim 0.12.2 did exactly that to the autocommand API
([nvim-oxi#309](https://github.com/noib3/nvim-oxi/issues/309),
[#306](https://github.com/noib3/nvim-oxi/issues/306)): `create_autocmd` fails with
`Required: 'command' or 'callback'`. Upstream has not shipped a fix.

## Which Neovim versions each feature targets

| Feature | Neovim | Notes |
|---|---|---|
| `neovim-0-11` | 0.11.x | unchanged from upstream, apart from the fixes marked "all versions" below |
| `neovim-0-12` | **0.12.2 – 0.12.6** | 0.12.0 and 0.12.1 are **not** supported: 0.12.2 changed keyset layouts within the 0.12 series, and one struct definition cannot match both |
| `neovim-nightly` | — | unchanged from upstream; nightly has since moved on, so expect breakage |

The keyset definitions are identical across 0.12.2 … 0.12.6 (checked by diffing
`src/nvim/api/keysets_defs.h` between the tags).

## What was changed

Everything below was found by comparing every options struct and every
`extern "C"` declaration against the Neovim 0.12.6 sources, then confirmed by
the test suite.

### Options structs (`crates/api/src/opts`)

| Struct | Problem | Versions |
|---|---|---|
| `CreateAutocmdOpts`, `ClearAutocmdsOpts`, `ExecAutocmdsOpts`, `GetAutocmdsOpts` | 0.12.2 added a `buf` key next to the deprecated `buffer`. Both fields now exist; the builder's `buffer()` writes `buf`. | 0.12 |
| `GetAutocmdsOpts` | the `id` key was missing; added (`.id(..)`). | all |
| `SetHighlightOpts` | new layout: attributes sorted alphabetically, `blink`/`conceal`/`dim`/`overline`/`update` added, `cterm` is a dictionary, `global_link` renamed `link_global`. | 0.12 |
| `DecorationProviderOpts` | `on_range` was missing; added, with `OnRangeArgs`. | 0.12 |
| `EchoOpts` | `_truncate` (private) and `source` were missing. | 0.12 |
| `OpenTermOpts` | `force_crlf` was missing; added. | all |
| `CreateCommandOpts` | field was named `register_`; the key is `register`. | all |
| `GetExtmarksOpts` | field was named `ty`; the key is `type` (builder method is still `ty`). | all |

The last two happened to produce the right bits by luck; the derive macro now
hashes raw identifiers (`r#type`) under their real name so the mirrors can be
written faithfully.

### FFI declarations (`crates/api/src/ffi`)

These had the wrong parameter list on every supported Neovim version, so the
calls passed arguments in the wrong registers:

| Function | Was missing |
|---|---|
| `nvim_get_chan_info` | leading `channel_id`, `arena` |
| `nvim_list_chans`, `nvim_list_tabpages` | `arena` |
| `nvim_load_context` | `err` |
| `nvim_paste` | leading `channel_id` |
| `nvim_put` | `arena` |
| `object_to_hl_id` | returned `Integer`; C returns `int` |

### Types (`crates/api/src/types`)

- `CommandInfos::complete`: on 0.12, a command whose completion is a Lua
  function reports that function instead of a name, which made
  `get_commands()` panic as soon as any such command existed (Neovim ships
  some). It is now `None` in that case.
- `HighlightInfos`: added the attributes Neovim actually returns (`blink`,
  `conceal`, `dim`, `overline`, `nocombine`, `underdashed`, `underdotted`,
  `underdouble`, `link`, `ctermfg`, `ctermbg`). The old `underdash`,
  `underdot`, `underlineline` never matched a real key and are deprecated.

### Tests

`tests/src/api/regression.rs` adds one test per fix above; each asserts an
observable effect in Neovim rather than just a successful return.

## Running the tests

From this directory, with the Neovim you want to verify on `$PATH`:

```sh
cargo test -p tests --no-default-features --features neovim-0-12
```

Upstream at the base commit: 92 passed, 9 failed on Neovim 0.12.6.
This fork: 117 passed, 0 failed.

## Removed from the upstream tree

`.github/`, `flake.nix`, `flake.lock`, `.envrc` (CI and dev-shell files that do
nothing here). Everything else is kept as-is so upstream changes can be merged
with a plain diff against the base commit above.

## Dropping the fork

Once upstream releases a version that passes its suite on the Neovim in use,
point `nvim-oxi` in `code/Cargo.toml` back at crates.io, remove
`src/libs/nvim-oxi` and the `exclude` entry, and re-check that `mlua` in
`[workspace.dependencies]` matches the version that release depends on.
