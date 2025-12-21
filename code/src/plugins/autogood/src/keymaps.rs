use nvim_oxi::{
    Function,
    Result, //
    api::{
        self,
        opts::SetKeymapOpts,
        set_keymap,
        types::Mode, //
    },
};

pub struct Mapping;

impl Mapping {
    pub fn init() -> Result<()> {
        Self::smart_dd()?;
        Self::visually_codeblock_shift()?;
        Self::move_selected_upndown()?;
        Self::go_back_normal_in_terminal()?;
        Self::ctrl_backspace_delete()?;
        Self::smart_visual_paste()?;
        Self::smart_save_in_insert_mode()?;
        Self::scroll_from_center()?;
        Self::search_within_visual()?;

        Ok(())
    }

    pub fn smart_dd() -> Result<()> {
        let callback = Function::from_fn(move |_| -> Result<()> {
            let line = api::get_current_line()?;

            if line.trim().is_empty() {
                api::command("normal! \"_dd")?;
            } else {
                api::command("normal! dd")?;
            }

            Ok(())
        });

        let opts = SetKeymapOpts::builder().noremap(true).silent(true).callback(callback).build();

        api::set_keymap(Mode::Normal, "dd", "", &opts)?;

        Ok(())
    }

    // easier moving of code blocks
    fn visually_codeblock_shift() -> Result<()> {
        set_keymap(Mode::Visual, "<", "<gv", &default_key_opts())?;
        set_keymap(Mode::Visual, ">", ">gv", &default_key_opts())?;
        Ok(())
    }

    // move selected line(s) up/down
    fn move_selected_upndown() -> Result<()> {
        set_keymap(Mode::Visual, "J", ":m '>+1<CR>gv=gv", &default_key_opts())?;
        set_keymap(Mode::Visual, "K", ":m '<-2<CR>gv=gv", &default_key_opts())?;
        Ok(())
    }

    // escape terminal to normal mode
    fn go_back_normal_in_terminal() -> Result<()> {
        set_keymap(Mode::Terminal, "<Esc>", "<c-\\><c-n>", &default_key_opts())?;
        Ok(())
    }

    // Ctrl+Backspace delete (insert + cmdline)
    fn ctrl_backspace_delete() -> Result<()> {
        let map_opts = SetKeymapOpts::builder().noremap(true).build();
        set_keymap(Mode::CmdLine, "<C-BS>", "<C-w>", &map_opts)?;
        set_keymap(Mode::Insert, "<C-BS>", "<C-w>", &map_opts)?;
        Ok(())
    }

    // paste without overwriting yank
    fn smart_visual_paste() -> Result<()> {
        set_keymap(Mode::Visual, "p", r#"<Cmd>silent! normal! "_dP<CR>"#, &default_key_opts())?;
        Ok(())
    }

    // smart save (insert + normal)
    fn smart_save_in_insert_mode() -> Result<()> {
        let rhs = "<ESC>ma<ESC>:update <CR>`a";
        set_keymap(Mode::Insert, "<C-s>", rhs, &default_key_opts())?;
        set_keymap(Mode::Normal, "<C-s>", rhs, &default_key_opts())?;
        Ok(())
    }

    // scroll from center
    fn scroll_from_center() -> Result<()> {
        set_keymap(Mode::Normal, "<C-d>", "<C-d>zz", &default_key_opts())?;
        set_keymap(Mode::Normal, "<C-u>", "<C-u>zz", &default_key_opts())?;
        Ok(())
    }

    // search within visual selection
    fn search_within_visual() -> Result<()> {
        let opts = SetKeymapOpts::builder().noremap(true).build();
        set_keymap(Mode::Visual, "/", "<Esc>/\\%V", &opts)?;
        Ok(())
    }
}

fn default_key_opts() -> SetKeymapOpts {
    SetKeymapOpts::builder().noremap(true).silent(true).build()
}
