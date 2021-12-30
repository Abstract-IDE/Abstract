<br/>
<p align="center">
  <a href="https://github.com/shaeinst/roshnivim">
    <img src="https://raw.githubusercontent.com/shaeinst/media/main/images/github-repositories/roshnivim/roshnivim_logo_transparent.png" alt="Logo" width="500" height="320">
  </a>
</p>

<br>

<div align="center" >
  <a href="https://github.com/shaeinst/roshnivim#screenshots">Screenshots</a>
  <a href="https://github.com/shaeinst/roshnivim/issues">Request Feature</a>

  ![Contributors](https://img.shields.io/github/contributors/shaeinst/roshnivim?color=dark-green) ![Issues](https://img.shields.io/github/issues/shaeinst/roshnivim) ![License](https://img.shields.io/github/license/shaeinst/roshnivim)
</div>



## Table Of Contents

* [About](#about)
* [Getting Started](#getting-started)
  * [Prerequisites](#prerequisites)
  * [Installation](#installation)
* [Usage](#usage)
  * [Stracture](#stracture-of-roshnivim)
* [Mappings](#mappings)
* [License](#license)
* [Screenshots](#screenshots)
* [To-do](#to-do)


## About
<div align="center"
  roshnivim ->  roshni + vim -> light + vim (roshni means light in hindi/urdu) <br>
  logo-style -> rosh(n)i(vim)  <br>
</div>
<br>
roshnivim, can be called neovim's distro, is a predefined configs so that you don't need 1000hr to setup neovim as an IDE. <br><br><br>

![screenshot_welcome](https://raw.githubusercontent.com/shaeinst/media/main/images/github-repositories/roshnivim/roshnivim-welcom.png)

## Features
- ```Project based config loading``` ( you can define configs in ```.__nvim__.lua``` file in the root of your working project so that you don't have to change config everytime you work on new/seperate project )
- ```Your own custom configs and Mappings``` (if you don't like roshnivim's default config/mapping, you can change/override it on [override_defalut.lua](https://github.com/shaeinst/roshnivim/blob/main/lua/customs/override_defalut.lua) OR ```~/.__nvim__.lua``` file )
- ```Separate config file for each plugins``` each plugin has their own config file which is defined in lua/plugins directory
- ```Easily Disable plugin``` roshnivim's using [packer](https://github.com/wbthomason/packer.nvim) as plugin manager. conmmenting out ```config``` option from plugin options in [packer config file](https://github.com/shaeinst/roshnivim/blob/main/lua/plugins/packer_nvim.lua) will disable that plugin
- ```Easy Installation``` install roshnivim with single command


## Getting Started
roshnivim can be installed by just runing a script.

#### Prerequisites

  * neovim >= 6.0

#### Installation

single command to install roshnivim
```bash
python <(curl -s https://raw.githubusercontent.com/shaeinst/roshnivim/main/setup.py)
```
or if you want to install it by cloning
```bash
git clone https://github.com/shaeinst/roshnivim
cd roshnivim
python setup.py
```
pass ```--delete 1``` as an argument if you don't want to keep ```.git```, ```README.md```, ```LICENSE``` and ```setup.py``` file.
Example:
```bash
python <(curl -s https://raw.githubusercontent.com/shaeinst/roshnivim/main/setup.py) --delete 1
```

NOTE1:
it could take some time depending on you connection (it's going to install plugins and some LSs).<br>
So, be patient and follow the output throw by setup.py script<br><br>
NOTE2:
only some LSs are going to be installed. for more, install with [LspInstall](https://github.com/williamboman/nvim-lsp-installer) <br>
for example: to install C/C++'s LS ``` :LspInstall clangd ```
<br><br>



## Usage
every one has their own favourite configs and keybindings. roshnivim try its best to provide likable configs and mapping.
<br>
so in case you don't like to use mapping or configs by roshnivim, you can change it in [override_defalut.lua](https://github.com/shaeinst/roshnivim/blob/main/lua/customs/override_defalut.lua) file. <br>

### Project Stracture
```
├── extra/
│   └── snippets/                  / custom defined snippets
│   ...
├── init.lua                       / load/source configs | heart of roshnivim
├── lua/
│   ├── configs.lua                / configs that's don't depends on plugins
│   ├── mappings.lua               / mappings that don't depends on plugins
│   ├── customs/
│   │   ├── project_env.lua        / load configs from .__nvim__.lua file defined in any project you're working on
│   │   ├── override_defalut.lua   / configs to override defined config
│   │   └── roshniline.lua         / i am working on it. btw it's a status line
│   └── plugins/                   / dir containing configs for plugins. each plugin has it's own config and can be locaed through init.lua file
│       └── packer_nvim.lua        / manage plugins
│       ...
├── plugin/                        / auto-created by plugin manager
└── setup.py                       / python-script to install/update roshnivim
```
### Mappings
``` ; M ``` to show mappings (it will show mapping in telescope) <br>
```;```     is a ```leader key```.   you can change it in [init.lua](https://github.com/shaeinst/roshnivim/blob/main/init.lua) file
<br>

## License
Distributed under the MIT License.
<br><br>

## Screenshots
   ![screenshot_lua](https://raw.githubusercontent.com/shaeinst/media/main/images/github-repositories/roshnivim/buff_and_nerdtree.png)
   ![screenshot_pythonlsp](https://raw.githubusercontent.com/shaeinst/media/main/images/github-repositories/roshnivim/python_lsp.png)
   ![screenshot_running_c](https://raw.githubusercontent.com/shaeinst/media/main/images/github-repositories/roshnivim/running_c.png)
   ![screenshot_telescode](https://raw.githubusercontent.com/shaeinst/media/main/images/github-repositories/roshnivim/telescope_as_fuzzy_finder.png)
   ![screenshot_lua](https://raw.githubusercontent.com/shaeinst/media/main/images/github-repositories/roshnivim/packer.png)
   ![screenshot_codeaction](https://raw.githubusercontent.com/shaeinst/media/main/images/github-repositories/roshnivim/codeaction_in_flutter_app.png)



<br>

## To-Do
- write decent documentation


## Known Bugs
- shows false line number on status line (sometime)


## Thanks to
- [Neovim](https://github.com/neovim/) -- for awesome EDITOR
- [shaankhan](https://readme.shaankhan.dev/) -- for readme
- [Neovim-Subreddit](https://www.reddit.com/r/neovim/) -- for awesome supporting community
- [LunarVim](https://github.com/LunarVim/LunarVim) -- for some reference
- Plugin Authors -- without you, neovim is incomplete
- and YOU
<br>
