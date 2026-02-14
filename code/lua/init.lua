local lib_path = os.getenv("HOME") .. "/codeDNA/work/opensource/neovim/Abstract/code/target/debug/?.so"
-- local lib_path = os.getenv("HOME") .. "/codeDNA/work/opensource/neovim/Abstract/code/target/release/?.so"
package.cpath = package.cpath .. ";" .. lib_path


require("libabstract")
