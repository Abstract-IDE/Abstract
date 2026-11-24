//! Regression tests guarding against field/mask misalignments between the
//! option structs and the layout expected by Neovim.

use std::cell::RefCell;
use std::rc::Rc;

use nvim_oxi::api::{self, Buffer, opts::*, types::*};

fn counter() -> (Rc<RefCell<u32>>, Rc<RefCell<u32>>) {
    let c = Rc::new(RefCell::new(0));
    (Rc::clone(&c), c)
}

// 1.
#[nvim_oxi::test]
fn autocmd_buffer_local_fires_only_for_its_buffer() {
    let buf_a = api::create_buf(true, false).unwrap();
    let buf_b = api::create_buf(true, false).unwrap();

    let (inner, outer) = counter();
    let opts = CreateAutocmdOpts::builder()
        .buffer(buf_a.clone())
        .callback(move |_args| {
            *inner.borrow_mut() += 1;
            Ok::<_, nvim_oxi::Error>(false)
        })
        .build();
    api::create_autocmd(["User"], &opts).unwrap();

    let exec_b = ExecAutocmdsOpts::builder().buffer(buf_b).build();
    api::exec_autocmds(["User"], &exec_b).unwrap();
    assert_eq!(0, *outer.borrow(), "must not fire for another buffer");

    let exec_a = ExecAutocmdsOpts::builder().buffer(buf_a).build();
    api::exec_autocmds(["User"], &exec_a).unwrap();
    assert_eq!(1, *outer.borrow(), "must fire for its own buffer");
}

// 2.
#[nvim_oxi::test]
fn autocmd_all_opts_read_back() {
    let group = api::create_augroup("RegrGroup", &Default::default()).unwrap();

    let opts = CreateAutocmdOpts::builder()
        .once(true)
        .desc("regression desc")
        .group("RegrGroup")
        .patterns(["*.regr"])
        .nested(true)
        .command("echo 'x'")
        .build();
    let id = api::create_autocmd(["BufNew"], &opts).unwrap();

    let get = GetAutocmdsOpts::builder().group("RegrGroup").build();
    let infos = api::get_autocmds(&get).unwrap().collect::<Vec<_>>();
    assert_eq!(1, infos.len(), "{infos:?}");
    let info = &infos[0];
    assert_eq!(Some(id), info.id);
    assert!(info.once);
    assert_eq!(Some("regression desc"), info.desc.as_deref());
    assert_eq!(Some(group), info.group);
    assert_eq!(Some("RegrGroup"), info.group_name.as_deref());
    assert_eq!("*.regr", info.pattern);
    assert_eq!("BufNew", info.event);
    assert_eq!("echo 'x'", info.command);
    assert!(!info.buflocal);
}

// 3.
#[nvim_oxi::test]
fn get_autocmds_by_buffer_and_id() {
    let buf_a = api::create_buf(true, false).unwrap();
    let buf_b = api::create_buf(true, false).unwrap();

    let mk = |buf: &Buffer, cmd: &str| {
        let opts = CreateAutocmdOpts::builder()
            .buffer(buf.clone())
            .command(cmd)
            .build();
        api::create_autocmd(["User"], &opts).unwrap()
    };
    let id_a = mk(&buf_a, "echo 'a'");
    let id_b = mk(&buf_b, "echo 'b'");

    let get = GetAutocmdsOpts::builder().buffer(buf_a.clone()).build();
    let infos = api::get_autocmds(&get).unwrap().collect::<Vec<_>>();
    assert_eq!(1, infos.len(), "{infos:?}");
    assert_eq!(Some(id_a), infos[0].id);
    assert_eq!(Some(buf_a.clone()), infos[0].buffer);

    let get = GetAutocmdsOpts::builder().id(id_b).build();
    let infos = api::get_autocmds(&get).unwrap().collect::<Vec<_>>();
    assert_eq!(1, infos.len(), "{infos:?}");
    assert_eq!(Some(id_b), infos[0].id);
    assert_eq!("echo 'b'", infos[0].command);
}

// 4.
#[nvim_oxi::test]
fn clear_autocmds_by_buffer() {
    let buf_a = api::create_buf(true, false).unwrap();
    let buf_b = api::create_buf(true, false).unwrap();

    for buf in [&buf_a, &buf_b] {
        let opts = CreateAutocmdOpts::builder()
            .buffer(buf.clone())
            .command("echo 'x'")
            .build();
        api::create_autocmd(["User"], &opts).unwrap();
    }

    let clear = ClearAutocmdsOpts::builder().buffer(buf_a.clone()).build();
    api::clear_autocmds(&clear).unwrap();

    let n = |buf: &Buffer| {
        let get = GetAutocmdsOpts::builder().buffer(buf.clone()).build();
        api::get_autocmds(&get).unwrap().count()
    };
    assert_eq!(0, n(&buf_a));
    assert_eq!(1, n(&buf_b));
}

// 5.
#[nvim_oxi::test]
fn set_get_hl_many_attributes() {
    let opts = SetHighlightOpts::builder()
        .bold(true)
        .italic(true)
        .underline(true)
        .strikethrough(true)
        .reverse(true)
        .foreground("#ff0000")
        .background("#00ff00")
        .build();
    api::set_hl(0, "RegrHl", &opts).unwrap();

    let get = GetHighlightOpts::builder().name("RegrHl").build();
    let GetHlInfos::Single(hl) = api::get_hl(0, &get).unwrap() else {
        panic!("expected a single highlight");
    };
    assert_eq!(Some(true), hl.bold);
    assert_eq!(Some(true), hl.italic);
    assert_eq!(Some(true), hl.underline);
    assert_eq!(Some(true), hl.strikethrough);
    assert_eq!(Some(true), hl.reverse);
    assert_eq!(Some(0xff0000), hl.foreground);
    assert_eq!(Some(0x00ff00), hl.background);
    assert_ne!(Some(true), hl.undercurl);
    assert_ne!(Some(true), hl.standout);
    assert_ne!(Some(true), hl.nocombine);
}

#[nvim_oxi::test]
fn set_get_hl_link() {
    let opts = SetHighlightOpts::builder().link("Comment").build();
    api::set_hl(0, "RegrLink", &opts).unwrap();

    let get = GetHighlightOpts::builder().name("RegrLink").build();
    let GetHlInfos::Single(hl) = api::get_hl(0, &get).unwrap() else {
        panic!("expected a single highlight");
    };
    assert_eq!(Some("Comment"), hl.link.as_deref());
}

#[nvim_oxi::test]
#[cfg(feature = "neovim-0-12")]
fn set_hl_update_merges() {
    api::set_hl(0, "RegrUpdate", &SetHighlightOpts::builder().bold(true).build())
        .unwrap();
    let opts = SetHighlightOpts::builder().italic(true).update(true).build();
    api::set_hl(0, "RegrUpdate", &opts).unwrap();

    let get = GetHighlightOpts::builder().name("RegrUpdate").build();
    let GetHlInfos::Single(hl) = api::get_hl(0, &get).unwrap() else {
        panic!("expected a single highlight");
    };
    assert_eq!(Some(true), hl.bold);
    assert_eq!(Some(true), hl.italic);
}

// 6.
#[nvim_oxi::test]
fn get_extmarks_filter_by_type() {
    let mut buf = Buffer::current();
    buf.set_lines(.., true, ["hello world"]).unwrap();
    let ns = api::create_namespace("RegrExtmarkType");

    let hl = SetExtmarkOpts::builder().end_col(3).hl_group("Comment").build();
    let hl_id = buf.set_extmark(ns, 0, 0, &hl).unwrap();

    let vt = SetExtmarkOpts::builder().virt_text([("vt", "Comment")]).build();
    let vt_id = buf.set_extmark(ns, 0, 5, &vt).unwrap();

    let ids = |ty: &str| {
        let opts = GetExtmarksOpts::builder().ty(ty).build();
        buf.get_extmarks(
            ns,
            ExtmarkPosition::ByTuple((0, 0)),
            ExtmarkPosition::ByTuple((usize::MAX, 0)),
            &opts,
        )
        .unwrap()
        .map(|(id, ..)| id)
        .collect::<Vec<_>>()
    };

    assert_eq!(vec![hl_id], ids("highlight"));
    assert_eq!(vec![vt_id], ids("virt_text"));
}

// 7.
#[nvim_oxi::test]
fn list_tabpages_and_chans() {
    assert_eq!(1, api::list_tabpages().count());
    api::command("tabnew").unwrap();
    assert_eq!(2, api::list_tabpages().count());

    let chans = api::list_chans().collect::<Vec<_>>();
    if let Some(first) = chans.first() {
        let infos = api::get_chan_info(first.id);
        assert!(infos.is_ok(), "{infos:?}");
        assert_eq!(first.id, infos.unwrap().id);
    }
}

// 8.
#[nvim_oxi::test]
fn put_inserts_lines() {
    let buf = Buffer::current();
    api::put(
        ["first", "second"].into_iter(),
        RegisterType::Linewise,
        true,
        false,
    )
    .unwrap();
    let lines =
        buf.get_lines(.., true).unwrap().map(|l| l.to_string()).collect::<Vec<_>>();
    assert!(lines.contains(&"first".to_owned()), "{lines:?}");
    assert!(lines.contains(&"second".to_owned()), "{lines:?}");
}

#[nvim_oxi::test]
fn paste_single_call_inserts_text() {
    let buf = Buffer::current();
    let ok = api::paste("pasted text", false, PastePhase::SingleCall).unwrap();
    assert!(ok);
    let lines =
        buf.get_lines(.., true).unwrap().map(|l| l.to_string()).collect::<Vec<_>>();
    assert!(lines.iter().any(|l| l.contains("pasted text")), "{lines:?}");
}

// 9.
#[nvim_oxi::test]
fn open_term_force_crlf() {
    let buf = api::create_buf(false, true).unwrap();
    let opts = OpenTermOpts::builder().force_crlf(true).build();
    let chan = api::open_term(&buf, &opts).unwrap();
    assert_ne!(0, chan);
}

#[nvim_oxi::test]
fn open_term_on_input() {
    let buf = api::create_buf(false, true).unwrap();
    let opts = OpenTermOpts::builder()
        .on_input(|_args| Ok::<_, nvim_oxi::Error>(()))
        .build();
    let chan = api::open_term(&buf, &opts).unwrap();
    assert_ne!(0, chan);
}

// 10.
#[nvim_oxi::test]
fn user_command_opts_read_back() {
    let opts = CreateCommandOpts::builder()
        .register(true)
        .bang(true)
        .bar(true)
        .nargs(CommandNArgs::Any)
        .desc("regression command")
        .build();
    api::create_user_command("RegrCmd", ":", &opts).unwrap();

    let opts = CreateCommandOpts::builder().nargs(CommandNArgs::One).build();
    api::create_user_command("RegrPlain", ":", &opts).unwrap();

    let cmds = api::get_commands(&Default::default())
        .unwrap()
        .collect::<Vec<_>>();
    let cmd = cmds.iter().find(|c| c.name == "RegrCmd").unwrap();
    assert!(cmd.register);
    assert!(cmd.bang);
    assert!(cmd.bar);
    assert_eq!(CommandNArgs::Any, cmd.nargs);

    let plain = cmds.iter().find(|c| c.name == "RegrPlain").unwrap();
    assert!(!plain.register);
    assert!(!plain.bang);
    assert!(!plain.bar);
    assert_eq!(CommandNArgs::One, plain.nargs);
}

#[nvim_oxi::test]
fn get_commands_with_lua_complete() {
    api::command(
        "lua vim.api.nvim_create_user_command('LuaCompl', function() end, \
         { nargs = '*', complete = function() return {} end })",
    )
    .unwrap();

    let opts = CreateCommandOpts::builder()
        .nargs(CommandNArgs::Any)
        .complete(CommandComplete::File)
        .build();
    api::create_user_command("NamedCompl", ":", &opts).unwrap();

    let cmds = api::get_commands(&Default::default())
        .expect("get_commands must succeed")
        .collect::<Vec<_>>();

    let lua = cmds.iter().find(|c| c.name == "LuaCompl").unwrap();
    assert_eq!(None, lua.complete);

    let named = cmds.iter().find(|c| c.name == "NamedCompl").unwrap();
    assert_eq!(Some("file"), named.complete.as_deref());
}

// 11.
#[nvim_oxi::test]
#[cfg(feature = "neovim-0-12")]
fn echo_opts_variants() {
    let none = None::<&str>;
    api::echo([("hello", none)], true, &EchoOpts::builder().build()).unwrap();
    api::echo(
        [("hello", none)],
        true,
        &EchoOpts::builder().verbose(false).err(false).build(),
    )
    .unwrap();

    api::echo(
        [("regr-err-message", none)],
        true,
        &EchoOpts::builder().err(true).build(),
    )
    .unwrap();
    let out = api::exec2(":messages", &ExecOpts::builder().output(true).build())
        .unwrap()
        .map(|s| s.to_string())
        .unwrap_or_default();
    assert!(out.contains("regr-err-message"), "{out:?}");
}
