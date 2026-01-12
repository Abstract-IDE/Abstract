local lib_path = os.getenv("HOME") .. "/codeDNA/dev/Projects/neovim/rust-Abstract-rewrite/code/target/debug/?.so"
-- local lib_path = os.getenv("HOME") .. "/codeDNA/dev/Projects/neovim/rust-Abstract-rewrite/code/target/release/?.so"
package.cpath = package.cpath .. ";" .. lib_path


require("libabstract")
