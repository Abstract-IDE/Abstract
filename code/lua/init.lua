local lib_path = os.getenv("HOME") .. "/codeDNA/dev/Projects/neovim/rust-Abstract-rewrite/code/target/debug/?.so"
-- local lib_path = os.getenv("HOME") .. "/Downloads/abspm-neovim-plugin-manager-3/abspm/target/debug/?.so"
package.cpath = package.cpath .. ";" .. lib_path

----------------------------------------------------

require("libabstract")
-- require("libabspm.so")

-- print("--------------------")
-- print(abstract)
-- print("--------------------")
