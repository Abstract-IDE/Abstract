-- --------------------------------------------------------------------
--   PluginName: vim-rooter
--   Github:     github.com/airblade/vim-rooter
-- --------------------------------------------------------------------

-- HELP NOTE -->
-- Using root-finding functionality in other scripts The public function FindRootDirectory()
-- returns the absolute path to the root directory as a string,
-- if a root directory is found, or an empty string otherwise.



-- --------------------------------------------------------------------
--       CONFIGS
-- --------------------------------------------------------------------
-- How to identify a root directory
-- Set g:rooter_patterns to a list of identifiers. They are checked breadth-first as
-- Rooter walks up the directory tree and the first match is used.

-- To specify the root has a certain directory or file (which may be a glob), just give the name:
vim.cmd([[ let g:rooter_patterns = ['.git', 'Makefile', '*.sln', 'build/env.sh'] ]])
-- To specify the root is a certain directory, prefix it with =.
-- let g:rooter_patterns = ['=src']
-- To specify the root has a certain directory as an ancestor (useful for excluding directories), prefix it with ^:
-- let g:rooter_patterns = ['^fixtures']
-- To specify the root has a certain directory as its direct ancestor / parent (useful when you put working projects in a common directory), prefix it with >:
-- let g:rooter_patterns = ['>Latex']
-- To exclude a pattern, prefix it with !.
-- let g:rooter_patterns = ['!.git/worktrees', '!=extras', '!^fixtures', '!build/env.sh']

-- By default vim-rooter uses :cd to change directory. To use :lcd or :tcd instead:
-- let g:rooter_cd_cmd = 'lcd'

-- To stop Rooter echoing the project directory:
vim.g.rooter_silent_chdir = 1

-- By default Rooter doesn't resolve symbolic links in the file or directory which triggers it. To do so:
-- let g:rooter_resolve_links = 1



--                   --Which buffers trigger Rooter--
-- By default all files and directories trigger Rooter. Configure a comma separated list of
-- patterns to specify which files trigger. Include / to trigger on directories. For example:
-- Everything (default)          -->     let g:rooter_targets = '/,*'
-- All files                     -->     let g:rooter_targets = '*'
-- YAML files                    -->     let g:rooter_targets = '*.yml,*.yaml'
-- Directories and YAML files    -->     let g:rooter_targets = '/,*.yml,*.yaml'
-- let g:rooter_targets = '$HOME/.config/nvim/configs, ~/codeDNA/dev, $HOME/codeDNA/LearnTeach_Code'

--                           --Non-project files--
-- Don't change directory (default).                     --> let g:rooter_change_directory_for_non_project_files = ''
-- Change to file's directory (similar to autochdir).    --> let g:rooter_change_directory_for_non_project_files = 'current'
-- Change to home directory.                             --> let g:rooter_change_directory_for_non_project_files = 'home'

-- --------------------------------------------------------------------
--       end of CONFIGS
-- --------------------------------------------------------------------





-- --------------------------------------------------------------------
--       MAPPINGS
-- --------------------------------------------------------------------

-- --------------------------------------------------------------------
--       enf of MAPPINGS
-- --------------------------------------------------------------------

