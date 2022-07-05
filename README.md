<br/>

<p align="center"> 
 <img src="https://user-images.githubusercontent.com/41078534/175897440-adaa1da8-08d9-4f6a-9d80-f687b4e296e6.svg" height="400 widht="400" >
</p>


<div align="center" >
  <a href="https://github.com/Abstract-IDE/Abstract#screenshots">Screenshots</a>
  <a href="https://github.com/Abstract-IDE/Abstract/issues">Request Feature</a>

  ![Contributors](https://img.shields.io/github/contributors/Abstract-IDE/Abstract?color=dark-green) ![Issues](https://img.shields.io/github/issues/Abstract-IDE/Abstract) ![License](https://img.shields.io/github/license/Abstract-IDE/Abstract)
</div>





## Table Of Contents

* [About](#about)
* [Features](#features)
* [Getting Started](#getting-started)
  * [Prerequisites](#prerequisites)
  * [Installation](#installation)
* [Usage](#usage)
  * [Stracture](#stracture-of-abstract)
* [Mappings](#mappings)
* [License](#license)
* [Screenshots](#screenshots)
* [To-do](#to-do)


## About

Abstract, The NeoVim configuration to achieve the power of Modern IDE

![screenshot_welcome](https://raw.githubusercontent.com/shaeinst/media/main/images/github-repositories/abstract/abstract-welcom.png)



## Features

- ```Project based config loading``` ( you can define configs in ```.__nvim__.lua``` file in the root of your working project so that you don't have to change config everytime you work on new/seperate project )
- ```Your own custom configs and Mappings``` (if you don't like Abstract's default config/mapping, you can change/override it on [override_defalut.lua](https://github.com/Abstract-IDE/Abstract/blob/main/lua/customs/override_defalut.lua) OR ```~/.__nvim__.lua``` file )
- ```Separate config file for each plugins``` each plugin has their own config file which is defined in lua/plugins directory
- ```Easily Disable plugin``` Abstract's using [packer](https://github.com/wbthomason/packer.nvim) as plugin manager. conmmenting out ```config``` option from plugin options in [packer config file](https://github.com/Abstract-IDE/Abstract/blob/main/lua/plugins/packer_nvim.lua) will disable that plugin
- ```Easy Installation``` install Abstract with single command



## Getting Started

Abstract can be installed by just runing a script.


#### Prerequisites

  * neovim v0.7


#### Installation

single command to install Abstract
```bash
python <(curl -s https://raw.githubusercontent.com/Abstract-IDE/Abstract/main/setup.py)
```
or if you want to install it by cloning
```bash
git clone https://github.com/Abstract-IDE/Abstract
cd Abstract
python setup.py
```
pass ```--delete 1``` as an argument if you don't want to keep ```.git```, ```README.md```, ```LICENSE``` and ```setup.py``` file.
Example:
```bash
python <(curl -s https://raw.githubusercontent.com/Abstract-IDE/Abstract/main/setup.py) --delete 1
```

NOTE1:
it could take some time depending on you connection (it's going to install plugins and some LSs).<br>
So, be patient and follow the output throw by setup.py script<br><br>
NOTE2:
only some LSs are going to be installed. for more, install with [LspInstall](https://github.com/williamboman/nvim-lsp-installer) <br>
for example: to install C/C++'s LS ``` :LspInstall clangd ```
<br><br>



## Usage

every one has their own favourite configs and keybindings. Abstract try its best to provide likable configs and mapping.
<br>
so in case you don't like to use mapping or configs by Abstract, you can change it in [override_defalut.lua](https://github.com/Abstract-IDE/Abstract/blob/main/lua/customs/override_defalut.lua) file. <br>


### Project Stracture

```
├── extra/
│   └── snippets/                  / custom defined snippets
│   ...
├── init.lua                       / load/source configs | heart of Abstract
├── lua/
│   ├── autocmd.lua                / auto command configs 
│   ├── configs.lua                / configs that's don't depends on plugins
│   ├── mappings.lua               / mappings that don't depends on plugins
│   ├── packer_nvim.lua            / manage plugins
│   ├── customs/
│   │   ├── override_defalut.lua   / configs to override defined config
│   │   └── abstractline.lua         / i am working on it. btw it's a status line
│   └── plugins/                   / dir containing configs for plugins. each plugin has it's own config and can be locaed through init.lua file
│   ...
├── plugin/                        / auto-created by plugin manager
└── setup.py                       / python-script to install/update Abstract
```


### Mappings

you can change ```Leader key``` in [init.lua](https://github.com/Abstract-IDE/Abstract/blob/main/init.lua) file

| Keys                | Functions                                                              |
| --------------------|:---------------------------------------------------------------------- |
| ```;```             | leader key                                                             |
| ```tm```            | show all mappings (it will show mapping in telescope)                  |
| ```<C-p>```         | Find files from current file's project                                 |
| ```<C-f>```         | show all files from current working directory                          |
| ```tt```            | Launch Telescope without any argument                                  |
| ```<Leader>q```     | close buffer                                                           |
| ```<C-s>```         | save file                                                              |
| ```<C-h>```         | scroll window horizontally (left)                                      |
| ```<C-l>```         | scroll window horizontally (right)                                     |
| ```??```            | clear Search Results                                                   |
| ```<M-q>```         | (M=Alt) close all other windows but leave current buffers    open.     |
| ```K```             | move selected line(s) up                                               |
| ```J```             | move selected line(s) down                                             |



## License

Distributed under the MIT License.
<br><br>



## Screenshots
   ![screenshot_buff_and_nerdtree](https://user-images.githubusercontent.com/41078534/177386049-93fc7a75-2f23-4d53-92a3-9ce8999283bf.png)
   ![screenshot_python_lsp](https://user-images.githubusercontent.com/41078534/177386239-f77ea88e-a934-4979-8806-017f39225e9d.png)
   ![screenshot_running_c](https://user-images.githubusercontent.com/41078534/177386287-53ea37a3-6349-40f6-b0f3-fcb5942fdb8f.png)
   ![screenshot_telescope_as_fuzzy_finder](https://user-images.githubusercontent.com/41078534/177386330-4863acdb-9c66-4f68-9b6a-96f379becf05.png)
   ![screenshot_packer](https://user-images.githubusercontent.com/41078534/177386379-955c7ddf-8750-4497-8e94-1d4fde28d6da.png)
   ![screenshot_codeaction_in_flutter_app](https://user-images.githubusercontent.com/41078534/177386497-bb984c56-9bf2-40e5-a9df-88f87767feb0.png)
<br>



## To-Do

- add installer(setup.py) support for Windows OS
- write a decent documentation



## Known Bugs

- there is no known bugs yet. Please open the issue if you find one.



## Thanks to

- [Neovim](https://github.com/neovim/) -- for awesome EDITOR
- [shaankhan](https://readme.shaankhan.dev/) -- for readme
- [Neovim-Subreddit](https://www.reddit.com/r/neovim/) -- for awesome supporting community
- [LunarVim](https://github.com/LunarVim/LunarVim) -- for some reference
- [Manytools.org](https://manytools.org/hacker-tools/ascii-banner) -- for ascii-banner
- Plugin Authors -- without you, neovim is incomplete
- and YOU
<br>

