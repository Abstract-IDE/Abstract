//! Micro-benchmarks, off unless `AB_NUI_BENCH_OUT` names a file to append the
//! results to:
//!
//! ```sh
//! AB_NUI_BENCH_OUT=/tmp/ab-nui-bench.txt cargo test -p ab-nui-tests --release bench
//! ```
//!
//! Besides wall time they count the Neovim API calls that repainting makes,
//! which is the number that matters and does not depend on the machine.

use std::fmt::Write as _;
use std::time::Instant;

use wp_ui::prelude::*;

use crate::support::*;

fn out_path() -> Option<String> {
    std::env::var("AB_NUI_BENCH_OUT").ok().filter(|p| !p.is_empty())
}

/// Wrap the buffer-painting API functions with counters. Must run before
/// [`setup`], so that anything caching `vim.api.*` picks up the wrappers.
fn install_counters() {
    nvim_oxi::mlua::lua()
        .load(
            r#"
            _G.ab_bench = { nvim_buf_set_lines = 0, nvim_buf_set_extmark = 0, nvim_buf_clear_namespace = 0,
                            nvim_get_option_value = 0, nvim_set_option_value = 0, nvim_buf_set_keymap = 0 }
            for name in pairs(_G.ab_bench) do
              local real = vim.api[name]
              vim.api[name] = function(...)
                _G.ab_bench[name] = _G.ab_bench[name] + 1
                return real(...)
              end
            end
            -- vim.keymap.set goes through nvim_buf_set_keymap / nvim_set_keymap.
            "#,
        )
        .exec()
        .expect("install counters");
}

fn counters() -> Vec<(String, i64)> {
    let table: mlua::Table = nvim_oxi::mlua::lua().globals().get("ab_bench").expect("counters");
    let mut all: Vec<(String, i64)> = table.pairs::<String, i64>().map(|p| p.expect("pair")).collect();
    all.sort();
    all
}

fn reset_counters() {
    nvim_oxi::mlua::lua()
        .load("for k in pairs(_G.ab_bench) do _G.ab_bench[k] = 0 end")
        .exec()
        .expect("reset");
}

fn report(name: &str, iterations: u32, elapsed: std::time::Duration) {
    let Some(path) = out_path() else { return };
    let mut line = format!(
        "{name}: {iterations} iterations in {:.2} ms ({:.1} us each);",
        elapsed.as_secs_f64() * 1e3,
        elapsed.as_secs_f64() * 1e6 / iterations as f64
    );
    for (api, count) in counters() {
        let _ = write!(line, " {}={:.1}/iter", api.trim_start_matches("nvim_"), count as f64 / iterations as f64);
    }
    line.push('\n');
    use std::io::Write as _;
    let mut file = std::fs::OpenOptions::new().create(true).append(true).open(path).expect("open bench output");
    file.write_all(line.as_bytes()).expect("write bench output");
}

/// Move a list selection down a 60x30 surface, one key at a time.
#[nvim_oxi::test]
fn bench_list_navigation() {
    if out_path().is_none() {
        return;
    }
    install_counters();
    setup();
    let state = ListState::new();
    let st = state.clone();
    let surface = Surface::float(PopupOptions { size: Size::cells(60, 30), ..Default::default() })
        .unwrap()
        .show(move || {
            let items: Vec<String> = (0..500).map(|i| format!("item number {i}")).collect();
            col![
                Text::new("A list of things").fg("Title"),
                Divider::new(),
                Expanded::new(List::new(items, &st)),
                KeyHints::new(vec![("j/k", "move"), ("Esc", "hide")]),
            ]
        })
        .unwrap();

    let iterations = 400;
    reset_counters();
    let start = Instant::now();
    for _ in 0..iterations {
        feed("j");
    }
    let elapsed = start.elapsed();
    assert_eq!(state.selected.get_untracked(), iterations as usize);
    report("list_navigation", iterations, elapsed);
    surface.close();
}

/// Repaint a surface where three pieces of state change per key press.
#[nvim_oxi::test]
fn bench_three_signals_per_key() {
    if out_path().is_none() {
        return;
    }
    install_counters();
    setup();
    let (a, b, c) = (Signal::new(0u32), Signal::new(0u32), Signal::new(0u32));
    let builds = Signal::new(0u32);
    let (ra, rb, rc, nb) = (a.clone(), b.clone(), c.clone(), builds.clone());
    let surface = Surface::float(PopupOptions { size: Size::cells(40, 10), ..Default::default() })
        .unwrap()
        .show(move || {
            nb.set_silent(nb.get_untracked() + 1);
            let (wa, wb, wc) = (ra.clone(), rb.clone(), rc.clone());
            col![
                Text::new(format!("a = {}", ra.get())),
                Text::new(format!("b = {}", rb.get())),
                Text::new(format!("c = {}", rc.get())),
                Button::new("bump").on_press(move || {
                    wa.update(|n| *n += 1);
                    wb.update(|n| *n += 2);
                    wc.update(|n| *n += 3);
                }),
            ]
        })
        .unwrap();

    let iterations = 400;
    reset_counters();
    let before = builds.get_untracked();
    let start = Instant::now();
    for _ in 0..iterations {
        feed("<CR>");
    }
    let elapsed = start.elapsed();
    assert_eq!(c.get_untracked(), 3 * iterations);
    report("three_signals_per_key", iterations, elapsed);
    if let Some(path) = out_path() {
        use std::io::Write as _;
        let rebuilds = (builds.get_untracked() - before) as f64 / iterations as f64;
        let mut file = std::fs::OpenOptions::new().append(true).open(path).unwrap();
        writeln!(file, "three_signals_per_key: {rebuilds:.1} tree rebuilds per key press").unwrap();
    }
    surface.close();
}

/// Create, show and free surfaces (dominated by installing the key table).
#[nvim_oxi::test]
fn bench_surface_open_close() {
    if out_path().is_none() {
        return;
    }
    install_counters();
    setup();
    let iterations = 100;
    reset_counters();
    let start = Instant::now();
    for i in 0..iterations {
        let surface = Surface::float(PopupOptions { size: Size::cells(30, 5), ..Default::default() })
            .unwrap()
            .show(move || col![Text::new(format!("surface {i}")), Button::new("ok")])
            .unwrap();
        surface.close();
    }
    report("surface_open_close", iterations, start.elapsed());
}

/// Show and dismiss toasts (non-focusable surfaces).
#[nvim_oxi::test]
fn bench_notify() {
    if out_path().is_none() {
        return;
    }
    install_counters();
    setup();
    let iterations = 100;
    reset_counters();
    let start = Instant::now();
    for i in 0..iterations {
        let handle =
            notify(format!("toast number {i}"), NotifyOptions { timeout_ms: None, ..Default::default() }).unwrap();
        handle.dismiss();
    }
    report("notify", iterations, start.elapsed());
}
