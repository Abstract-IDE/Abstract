//! The thin `nvim` wrappers: buffers, windows, extmarks, autocmds, keymaps and
//! user commands.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use wp_ui::nvim::{self, CommandOpts, KeymapOpts};
use wp_ui::prelude::*;

use crate::support::*;

fn counter() -> Rc<Cell<u32>> {
    Rc::new(Cell::new(0))
}

// ---- Buffer ----------------------------------------------------------------------

#[nvim_oxi::test]
fn buffer_scratch_set_and_get_lines() {
    setup();
    let buf = Buffer::scratch().unwrap();
    assert!(buf.is_valid());
    buf.set_all(["one", "two", "three"]).unwrap();
    assert_eq!(buf.line_count().unwrap(), 3);
    assert_eq!(buf.get_lines(0, -1).unwrap(), v(&["one", "two", "three"]));
    assert_eq!(buf.get_lines(1, 2).unwrap(), v(&["two"]));
    assert_eq!(buf.first_line().unwrap(), "one");
    buf.set_lines(1, 2, ["TWO", "2b"]).unwrap();
    assert_eq!(lines(buf), v(&["one", "TWO", "2b", "three"]));
    buf.set_lines(0, 0, ["top"]).unwrap();
    assert_eq!(lines(buf)[0], "top");
    // Scratch buffers are unlisted, unnamed and have no file type.
    assert_eq!(buf.name().unwrap(), "");
    assert_eq!(buf.get_option::<String>("buftype").unwrap(), "nofile");
}

#[nvim_oxi::test]
fn buffer_set_text_replaces_ranges() {
    setup();
    let buf = Buffer::scratch().unwrap();
    buf.set_all(["hello world", "second"]).unwrap();
    buf.set_text(0, 6, 0, 11, ["there"]).unwrap();
    assert_eq!(lines(buf), v(&["hello there", "second"]));
    buf.set_text(0, 5, 1, 3, ["", "X"]).unwrap();
    assert_eq!(lines(buf), v(&["hello", "Xond"]));
}

#[nvim_oxi::test]
fn buffer_lock_still_allows_wrapper_writes_and_restores_flags() {
    setup();
    let buf = Buffer::scratch().unwrap();
    buf.set_all(["a"]).unwrap();
    buf.lock().unwrap();
    assert!(!buf.get_option::<bool>("modifiable").unwrap());
    assert!(buf.get_option::<bool>("readonly").unwrap());
    buf.set_all(["b", "c"]).unwrap();
    buf.set_text(0, 0, 0, 1, ["B"]).unwrap();
    assert_eq!(lines(buf), v(&["B", "c"]));
    assert!(!buf.get_option::<bool>("modifiable").unwrap());
    assert!(buf.get_option::<bool>("readonly").unwrap());
}

#[nvim_oxi::test]
fn buffer_options_name_and_delete() {
    setup();
    let buf = Buffer::scratch().unwrap();
    buf.set_option("filetype", "abtest").unwrap();
    assert_eq!(buf.get_option::<String>("filetype").unwrap(), "abtest");
    buf.set_name("ab-nui-test-buffer").unwrap();
    assert!(buf.name().unwrap().ends_with("ab-nui-test-buffer"));
    assert_eq!(Buffer(buf.id()), buf);
    buf.delete().unwrap();
    assert!(!buf.is_valid());
}

// ---- Window ----------------------------------------------------------------------

#[nvim_oxi::test]
fn window_open_float_config_and_close() {
    setup();
    let buf = Buffer::scratch().unwrap();
    let main = current_win();
    let rect = wp_ui::geometry::Rect { row: 2.0, col: 3.0, width: 12, height: 4 };
    let mut cfg = FloatConfig::new(rect);
    cfg.enter = false;
    cfg.zindex = 70;
    let win = Window::open_float(buf, &cfg).unwrap();
    assert_eq!(current_win(), main, "enter=false keeps focus");
    let c = win.config().unwrap();
    assert_eq!((c.relative.as_str(), c.anchor.as_str()), ("editor", "NW"));
    assert_eq!((c.width, c.height, c.row, c.col, c.zindex), (12, 4, 2.0, 3.0, Some(70)));
    assert_eq!(win.buffer().unwrap(), buf);
    assert_eq!((win.width().unwrap(), win.height().unwrap()), (12, 4));

    cfg.rect = wp_ui::geometry::Rect { row: 5.0, col: 6.0, width: 20, height: 2 };
    win.set_config(&cfg).unwrap();
    let c = win.config().unwrap();
    assert_eq!((c.width, c.height, c.row, c.col), (20, 2, 5.0, 6.0));

    assert_eq!(win_count(), 2);
    win.close().unwrap();
    assert!(!win.is_valid());
    assert_eq!(win_count(), 1);
}

#[nvim_oxi::test]
fn window_enter_focus_cursor_and_options() {
    setup();
    let buf = Buffer::scratch().unwrap();
    buf.set_all(["abc", "def", "ghi"]).unwrap();
    let main = current_win();
    let win =
        Window::open_float(buf, &FloatConfig::new(wp_ui::geometry::Rect { row: 1.0, col: 1.0, width: 10, height: 3 }))
            .unwrap();
    assert_eq!(current_win(), win, "enter defaults to true");
    win.set_cursor(2, 1).unwrap();
    assert_eq!(win.cursor().unwrap(), (2, 1));
    win.set_option("wrap", false).unwrap();
    assert!(!win.get_option::<bool>("wrap").unwrap());
    win.set_option("cursorline", true).unwrap();
    assert!(win.get_option::<bool>("cursorline").unwrap());
    main.focus().unwrap();
    assert_eq!(current_win(), main);
    win.focus().unwrap();
    assert_eq!(current_win(), win);
    win.apply_default_highlight().unwrap();
    assert!(win.get_option::<String>("winhighlight").unwrap().contains("FloatBorder:AbNuiBorder"));
}

#[nvim_oxi::test]
fn window_open_split_is_not_a_float() {
    setup();
    let buf = Buffer::scratch().unwrap();
    let main = current_win();
    let win = Window::open_split(buf, &SplitConfig::new(SplitDir::Below)).unwrap();
    assert_eq!(win_count(), 2);
    assert_eq!(win.config().unwrap().relative, "");
    assert_eq!(current_win(), win);
    assert!(win.height().unwrap() > 0);
    let mut cfg = SplitConfig::new(SplitDir::Left);
    cfg.size = Some(15);
    cfg.enter = false;
    let left = Window::open_split(buf, &cfg).unwrap();
    assert_eq!(left.width().unwrap(), 15);
    assert_eq!(current_win(), win);
    left.close().unwrap();
    win.close().unwrap();
    assert_eq!(current_win(), main);
}

// ---- Namespace / extmarks -----------------------------------------------------------

fn marked_buffer() -> (Buffer, Namespace) {
    let buf = Buffer::scratch().unwrap();
    buf.set_all(["hello world", "second line", "third"]).unwrap();
    (buf, Namespace::create("ab_nui_test").unwrap())
}

#[nvim_oxi::test]
fn namespace_highlight_and_clear() {
    setup();
    let (buf, ns) = marked_buffer();
    ns.highlight(buf, 0, 0, 5, "ErrorMsg").unwrap();
    ns.highlight(buf, 1, 7, 11, "Title").unwrap();
    assert_eq!(highlights(buf), vec![(0, 0, 5, "ErrorMsg".into()), (1, 7, 11, "Title".into())]);
    assert_eq!(hl_at(buf, 0, 4).as_deref(), Some("ErrorMsg"));
    assert_eq!(hl_at(buf, 0, 5), None);
    ns.clear(buf).unwrap();
    assert!(highlights(buf).is_empty());
}

#[nvim_oxi::test]
fn namespace_get_and_delete_extmarks() {
    setup();
    let (buf, ns) = marked_buffer();
    let a = ns.set_extmark(buf, 0, 2, &ExtmarkOpts::default()).unwrap();
    let b = ns.set_extmark(buf, 2, 1, &ExtmarkOpts::default()).unwrap();
    assert_ne!(a, b);
    let all = ns.get_extmarks(buf, (0, 0), (-1, -1)).unwrap();
    assert_eq!(all.iter().map(|m| (m.row, m.col)).collect::<Vec<_>>(), vec![(0, 2), (2, 1)]);
    assert_eq!(all[0].id, a);
    let first_two_rows = ns.get_extmarks(buf, (0, 0), (1, -1)).unwrap();
    assert_eq!(first_two_rows.len(), 1);
    ns.del_extmark(buf, a).unwrap();
    let left = ns.get_extmarks(buf, (0, 0), (-1, -1)).unwrap();
    assert_eq!(left.len(), 1);
    assert_eq!(left[0].id, b);
    // Re-using an id moves the mark instead of adding one.
    let again = ns.set_extmark(buf, 1, 0, &ExtmarkOpts { id: Some(b), ..Default::default() }).unwrap();
    assert_eq!(again, b);
    let left = ns.get_extmarks(buf, (0, 0), (-1, -1)).unwrap();
    assert_eq!(left.iter().map(|m| (m.row, m.col)).collect::<Vec<_>>(), vec![(1, 0)]);
    ns.clear(buf).unwrap();
    assert!(ns.get_extmarks(buf, (0, 0), (-1, -1)).unwrap().is_empty());
}

#[nvim_oxi::test]
fn extmark_virt_text_and_virt_lines() {
    setup();
    let (buf, ns) = marked_buffer();
    ns.set_extmark(
        buf,
        0,
        0,
        &ExtmarkOpts {
            virt_text: Some(Line::from_spans([Span::raw(" note"), Span::hl(" warn", "WarningMsg")])),
            virt_text_pos: Some(VirtTextPos::Eol),
            ..Default::default()
        },
    )
    .unwrap();
    ns.set_extmark(
        buf,
        1,
        0,
        &ExtmarkOpts {
            virt_lines: Some(vec![Line::raw("v1"), Line::from_spans([Span::hl("v2", "Title")])]),
            virt_lines_above: true,
            ..Default::default()
        },
    )
    .unwrap();
    let code = format!(
        "local m = vim.api.nvim_buf_get_extmarks({}, {}, 0, -1, {{ details = true }}); return vim.json.encode({{ m[1][4].virt_text, m[1][4].virt_text_pos, m[2][4].virt_lines, m[2][4].virt_lines_above }})",
        buf.id(),
        ns.id()
    );
    let json: String = lua_eval(&code);
    assert_eq!(json, r#"[[[" note"],[" warn","WarningMsg"]],"eol",[[["v1"]],[["v2","Title"]]],true]"#);
    // Buffer text is untouched by decorations.
    assert_eq!(lines(buf), v(&["hello world", "second line", "third"]));
}

#[nvim_oxi::test]
fn extmark_sign_text_conceal_and_line_hl() {
    setup();
    let (buf, ns) = marked_buffer();
    ns.set_extmark(
        buf,
        0,
        0,
        &ExtmarkOpts {
            sign_text: Some(">>".into()),
            sign_hl_group: Some("ErrorMsg".into()),
            line_hl_group: Some("Visual".into()),
            ..Default::default()
        },
    )
    .unwrap();
    ns.set_extmark(
        buf,
        1,
        0,
        &ExtmarkOpts { end_row: Some(1), end_col: Some(6), conceal: Some("*".into()), ..Default::default() },
    )
    .unwrap();
    let sign: String = lua_eval(&format!(
        "return vim.api.nvim_buf_get_extmarks({}, {}, 0, -1, {{ details = true }})[1][4].sign_text",
        buf.id(),
        ns.id()
    ));
    assert_eq!(sign.trim(), ">>");
    let line_hl: String = lua_eval(&format!(
        "return vim.api.nvim_buf_get_extmarks({}, {}, 0, -1, {{ details = true }})[1][4].line_hl_group",
        buf.id(),
        ns.id()
    ));
    assert_eq!(line_hl, "Visual");
    let conceal: String = lua_eval(&format!(
        "return vim.api.nvim_buf_get_extmarks({}, {}, 0, -1, {{ details = true }})[2][4].conceal",
        buf.id(),
        ns.id()
    ));
    assert_eq!(conceal, "*");
}

#[nvim_oxi::test]
fn render_writes_lines_and_byte_range_highlights() {
    setup();
    let buf = Buffer::scratch().unwrap();
    let ns = Namespace::create("ab_nui_render_test").unwrap();
    let ls = [Line::from_spans([Span::raw("é "), Span::hl("日本", "Title"), Span::raw("!")]), Line::raw("plain")];
    nvim::render(buf, ns, &ls).unwrap();
    assert_eq!(lines(buf), v(&["é 日本!", "plain"]));
    // é is 2 bytes + space = 3; 日本 is 6 bytes.
    assert_eq!(highlights(buf), vec![(0, 3, 9, "Title".into())]);
    // Re-rendering replaces previous highlights.
    nvim::render(buf, ns, &[Line::raw("x")]).unwrap();
    assert_eq!(lines(buf), v(&["x"]));
    assert!(highlights(buf).is_empty());
}

// ---- autocmds ----------------------------------------------------------------------

#[nvim_oxi::test]
fn autocmd_fires_with_args_and_can_be_deleted() {
    setup();
    let seen: Rc<RefCell<Vec<(String, String)>>> = Rc::new(RefCell::new(Vec::new()));
    let s2 = seen.clone();
    let id = nvim::autocmd(&["User"], Some("AbNuiTest"), None, move |a| {
        s2.borrow_mut().push((a.event.clone(), a.matched.clone()));
    })
    .unwrap();
    ex("doautocmd User AbNuiTest");
    ex("doautocmd User AbNuiOther");
    assert_eq!(*seen.borrow(), vec![("User".to_string(), "AbNuiTest".to_string())]);
    nvim::del_autocmd(id).unwrap();
    ex("doautocmd User AbNuiTest");
    assert_eq!(seen.borrow().len(), 1);
}

#[nvim_oxi::test]
fn autocmd_once_fires_a_single_time() {
    setup();
    let n = counter();
    let n2 = n.clone();
    nvim::autocmd_once(&["User"], Some("AbNuiOnce"), None, move |_| n2.set(n2.get() + 1)).unwrap();
    ex("doautocmd User AbNuiOnce");
    ex("doautocmd User AbNuiOnce");
    assert_eq!(n.get(), 1);
}

#[nvim_oxi::test]
fn autocmd_multiple_events_share_one_callback() {
    setup();
    let events = Rc::new(RefCell::new(Vec::<String>::new()));
    let e2 = events.clone();
    nvim::autocmd(&["User"], Some("AbNuiA"), None, {
        let e2 = e2.clone();
        move |a| e2.borrow_mut().push(format!("{}:{}", a.event, a.matched))
    })
    .unwrap();
    nvim::autocmd(&["User", "BufNew"], Some("*"), None, move |a| {
        if a.event == "User" {
            e2.borrow_mut().push("star".into());
        }
    })
    .unwrap();
    ex("doautocmd User AbNuiA");
    assert_eq!(*events.borrow(), v(&["User:AbNuiA", "star"]));
}

#[nvim_oxi::test]
fn buf_autocmd_only_fires_for_its_buffer() {
    setup();
    let (a, b) = (Buffer::scratch().unwrap(), Buffer::scratch().unwrap());
    let n = counter();
    let n2 = n.clone();
    nvim::buf_autocmd(&["User"], a, None, move || n2.set(n2.get() + 1)).unwrap();
    lua_run(&format!("vim.api.nvim_exec_autocmds('User', {{ buffer = {}, }})", a.id()));
    assert_eq!(n.get(), 1);
    lua_run(&format!("vim.api.nvim_exec_autocmds('User', {{ buffer = {}, }})", b.id()));
    assert_eq!(n.get(), 1);
}

#[nvim_oxi::test]
fn augroup_deletion_removes_its_autocmds_and_recreate_clears() {
    setup();
    let n = counter();
    let g = nvim::augroup("AbNuiTestGroup").unwrap();
    let n2 = n.clone();
    nvim::autocmd(&["User"], Some("AbNuiG"), Some(g), move |_| n2.set(n2.get() + 1)).unwrap();
    ex("doautocmd User AbNuiG");
    assert_eq!(n.get(), 1);
    // Re-creating the group clears what was in it.
    let g2 = nvim::augroup("AbNuiTestGroup").unwrap();
    ex("doautocmd User AbNuiG");
    assert_eq!(n.get(), 1);
    let n3 = n.clone();
    nvim::autocmd(&["User"], Some("AbNuiG"), Some(g2), move |_| n3.set(n3.get() + 10)).unwrap();
    ex("doautocmd User AbNuiG");
    assert_eq!(n.get(), 11);
    nvim::del_augroup(g2).unwrap();
    ex("doautocmd User AbNuiG");
    assert_eq!(n.get(), 11);
}

#[nvim_oxi::test]
fn unique_augroups_are_independent() {
    setup();
    let a = nvim::unique_augroup("AbNuiUniq").unwrap();
    let b = nvim::unique_augroup("AbNuiUniq").unwrap();
    assert_ne!(a, b);
    let (na, nb) = (counter(), counter());
    let (na2, nb2) = (na.clone(), nb.clone());
    nvim::autocmd(&["User"], Some("AbNuiU"), Some(a), move |_| na2.set(na2.get() + 1)).unwrap();
    nvim::autocmd(&["User"], Some("AbNuiU"), Some(b), move |_| nb2.set(nb2.get() + 1)).unwrap();
    nvim::del_augroup(a).unwrap();
    ex("doautocmd User AbNuiU");
    assert_eq!((na.get(), nb.get()), (0, 1));
}

// ---- keymaps ----------------------------------------------------------------------

#[nvim_oxi::test]
fn buf_keymap_fires_and_del_keymap_removes() {
    setup();
    let buf = Buffer::scratch().unwrap();
    lua_run(&format!("vim.api.nvim_set_current_buf({})", buf.id()));
    let n = counter();
    let n2 = n.clone();
    nvim::buf_keymap(buf, "n", "<F6>", KeymapOpts::default(), move || n2.set(n2.get() + 1)).unwrap();
    feed("<F6><F6>");
    assert_eq!(n.get(), 2);
    nvim::del_keymap(Some(buf), "n", "<F6>").unwrap();
    feed("<F6>");
    assert_eq!(n.get(), 2);
}

#[nvim_oxi::test]
fn buf_keymap_is_buffer_local_and_mode_specific() {
    setup();
    let a = Buffer::scratch().unwrap();
    let b = Buffer::scratch().unwrap();
    let n = counter();
    let n2 = n.clone();
    nvim::buf_keymap(a, "n", "<F7>", KeymapOpts::default(), move || n2.set(n2.get() + 1)).unwrap();
    lua_run(&format!("vim.api.nvim_set_current_buf({})", b.id()));
    feed("<F7>");
    assert_eq!(n.get(), 0);
    lua_run(&format!("vim.api.nvim_set_current_buf({})", a.id()));
    feed("<F7>");
    assert_eq!(n.get(), 1);
    // Insert mode has no such mapping: the key is typed as <F7> (no callback).
    feed("i<F7><Esc>");
    assert_eq!(n.get(), 1);
}

#[nvim_oxi::test]
fn buf_keymap_multiple_modes_via_separate_calls() {
    setup();
    let buf = Buffer::scratch().unwrap();
    lua_run(&format!("vim.api.nvim_set_current_buf({})", buf.id()));
    let n = counter();
    for mode in ["n", "i"] {
        let n2 = n.clone();
        nvim::buf_keymap(buf, mode, "<F8>", KeymapOpts::default(), move || n2.set(n2.get() + 1)).unwrap();
    }
    feed("<F8>");
    feed("i<F8><Esc>");
    assert_eq!(n.get(), 2);
}

// ---- user commands ----------------------------------------------------------------

#[nvim_oxi::test]
fn user_command_receives_args_fargs_and_bang() {
    setup();
    /// What one invocation received: `(args, fargs, bang)`.
    type Call = (String, Vec<String>, bool);
    let got: Rc<RefCell<Vec<Call>>> = Rc::new(RefCell::new(Vec::new()));
    let g2 = got.clone();
    nvim::user_command(
        "AbNuiCmd",
        &CommandOpts { nargs: Some("*"), bang: true, desc: Some("test".into()), ..Default::default() },
        move |a| g2.borrow_mut().push((a.args, a.fargs, a.bang)),
    )
    .unwrap();
    ex("AbNuiCmd one two");
    ex("AbNuiCmd! x");
    ex("AbNuiCmd");
    assert_eq!(
        *got.borrow(),
        vec![
            ("one two".to_string(), v(&["one", "two"]), false),
            ("x".to_string(), v(&["x"]), true),
            (String::new(), vec![], false),
        ]
    );
}

#[nvim_oxi::test]
fn user_command_range_and_deletion() {
    setup();
    let ranges = Rc::new(RefCell::new(Vec::<(i64, i64)>::new()));
    let r2 = ranges.clone();
    nvim::user_command("AbNuiRange", &CommandOpts { range: true, ..Default::default() }, move |a| {
        r2.borrow_mut().push((a.line1, a.line2));
    })
    .unwrap();
    let buf = Buffer::scratch().unwrap();
    buf.set_all(["a", "b", "c", "d"]).unwrap();
    lua_run(&format!("vim.api.nvim_set_current_buf({})", buf.id()));
    ex("2,3AbNuiRange");
    assert_eq!(*ranges.borrow(), vec![(2, 3)]);
    nvim::del_user_command("AbNuiRange").unwrap();
    let r: bool = lua_eval("return pcall(vim.cmd, 'AbNuiRange')");
    assert!(!r, "the command no longer exists");
    assert_eq!(ranges.borrow().len(), 1);
    assert!(nvim::del_user_command("AbNuiRange").is_err());
}

#[nvim_oxi::test]
fn user_command_nargs_is_enforced() {
    setup();
    let n = counter();
    let n2 = n.clone();
    nvim::user_command("AbNuiNoArgs", &CommandOpts::default(), move |_| n2.set(n2.get() + 1)).unwrap();
    ex("AbNuiNoArgs");
    assert_eq!(n.get(), 1);
    let ok: bool = lua_eval("return pcall(vim.cmd, 'AbNuiNoArgs extra')");
    assert!(!ok, "a command without nargs rejects arguments");
    assert_eq!(n.get(), 1);
}

#[nvim_oxi::test]
fn user_command_numeric_nargs() {
    setup();
    // Neovim only accepts 0/1 as numbers; the string form must still work.
    let got = Rc::new(RefCell::new(Vec::<String>::new()));
    let g = got.clone();
    nvim::user_command("AbNuiOneArg", &CommandOpts { nargs: Some("1"), ..Default::default() }, move |a| {
        g.borrow_mut().push(a.args)
    })
    .unwrap();
    ex("AbNuiOneArg hello world");
    assert_eq!(*got.borrow(), vec!["hello world".to_string()]);
    let ok: bool = lua_eval("return pcall(vim.cmd, 'AbNuiOneArg')");
    assert!(!ok, "nargs=1 requires an argument");

    let n = counter();
    let n2 = n.clone();
    nvim::user_command("AbNuiZeroArgs", &CommandOpts { nargs: Some("0"), ..Default::default() }, move |_| {
        n2.set(n2.get() + 1)
    })
    .unwrap();
    ex("AbNuiZeroArgs");
    assert_eq!(n.get(), 1);
}
