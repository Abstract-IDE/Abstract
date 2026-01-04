use nvim_oxi::{
    Result,
    api::{
        self, create_autocmd,
        opts::{CreateAugroupOpts, CreateAutocmdOpts},
    },
};

use wl_utils::neovim::types::events::Events;

pub struct AutoCmds {}

impl AutoCmds {
    pub fn new(name: &str) -> Result<Self> {
        let opts = CreateAugroupOpts::builder().clear(true).build();
        let group_id = api::create_augroup(name, &opts)?;

        Self::highlight_on_yank(group_id, Some(150))?;
        Self::open_file_last_position(group_id)?;
        Self::remove_whitespace_on_save(group_id)?;
        Self::clear_last_used_search(group_id)?;
        Self::dont_suspend_with_cz(group_id)?;

        Ok(Self {})
    }

    // WARN: it was supposed to called by callback not the command. this function can be improved
    fn highlight_on_yank(group_id: u32, timeout_ms: Option<u16>) -> Result<()> {
        let timeout = timeout_ms.unwrap_or(150);
        let cmd = format!("lua vim.hl.on_yank({{higroup = 'Search', timeout = {}, on_visual = true}})", timeout);

        let opts = CreateAutocmdOpts::builder()
            .group(group_id)
            .desc("highlight text on yank")
            .patterns(["*"])
            .command(cmd)
            .build();
        create_autocmd([Events::TextYankPost.as_ref()], &opts)?;

        Ok(())
    }

    fn open_file_last_position(group_id: u32) -> Result<()> {
        let opts = CreateAutocmdOpts::builder()
            .group(group_id)
            .desc("jump to the last position when reopening a file")
            .patterns(["*"])
            .command(r#"if line("'\"") > 0 && line("'\"") <= line("$") | exe "normal! g`\"" | endif"#)
            .build();

        create_autocmd([Events::BufWinEnter.as_ref()], &opts)?;

        Ok(())
    }

    fn remove_whitespace_on_save(group_id: u32) -> Result<()> {
        let opts = CreateAutocmdOpts::builder()
            .desc("remove whitespaces on save")
            .group(group_id)
            .patterns(["*"])
            .command("%s/\\s\\+$//e")
            .build();

        create_autocmd([Events::BufWritePre.as_ref()], &opts)?;

        Ok(())
    }

    fn _no_autocomment_newline(group_id: u32) -> Result<()> {
        let opts = CreateAutocmdOpts::builder()
            .desc("don't auto comment new line")
            .group(group_id)
            .patterns(["*"])
            .command("setlocal formatoptions-=c formatoptions-=r formatoptions-=o")
            .build();

        create_autocmd([Events::BufEnter.as_ref(), Events::FileType.as_ref()], &opts)?;

        Ok(())
    }

    fn clear_last_used_search(group_id: u32) -> Result<()> {
        let opts = CreateAutocmdOpts::builder()
            .desc("clear the last used search pattern")
            .group(group_id)
            .patterns(["*"])
            .command("let @/ = ''")
            .build();

        create_autocmd([Events::BufWinEnter.as_ref()], &opts)?;

        Ok(())
    }

    fn dont_suspend_with_cz(group_id: u32) -> Result<()> {
        let opts = CreateAutocmdOpts::builder()
            .desc("map ctrl+z to nothing so that it doesn't suspend terminal")
            .group(group_id)
            .patterns(["*"])
            .command("nnoremap <c-z> <nop>")
            .build();

        create_autocmd([Events::BufEnter.as_ref()], &opts)?;

        Ok(())
    }
}
