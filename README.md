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
* [#Warning](#warning)


## About
<div align="center"
  roshnivim ->  roshni + vim -> light + vim (roshni means light in hindi/urdu) <br>
  logo-style -> rosh(n)i(vim)  <br>
</div>
<br>

roshnivim, can be called neovim's distro, is a predefined configs so that you don't need 1000hr to setup neovim as an IDE. <br><br><br>


## Getting Started

roshnivim can be installed by just runing a script.

#### Prerequisites

  * neovim >= 5.0

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

<br>
NOTE1:<br>
it could take some time depending on you connection (it's going to install plugins and some LSs).<br>
So, be patient and follow the output throw by setup.sh script<br><br>
NOTE2:<br>
only some LSs are going to be installed. for more, install with [LspInstall](https://github.com/williamboman/nvim-lsp-installer)
<br>
for example: to install C/C++'s LS ```:LspInstall clangd```
<br><br>


## Usage
every one has their own favourite config and keybinding. roshnivim try its best to provide likable configs and mapping.
<br>
so in case you don't like to use mapping or configs by roshnivim, you can change it in [override_defalut.lua](https://github.com/shaeinst/roshnivim/blob/main/lua/customs/override_defalut.lua) file. <br>

### Project Stracture
```
├── extra/
│   └── snippets/                  / custom defined snippets
├── init.lua                       / use to install plugins, load configs and leader key is defined here
├── lua/
│   ├── configs.lua                / configs that's don't depends on plugins
│   ├── mappings.lua               / mappings that don't depends on plugins
│   ├── customs/
│   │   ├── override_defalut.lua   / configs to override defined config
│   │   └── roshniline.lua         / i am working on it. btw it's a status line
│   └── plugins/                   / dir containing configs for plugins. each plugin has it's own config and can be locaed through init.lua file
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

<details close><summary>expand</summary>

   ![screenshot_lua](https://raw.githubusercontent.com/shaeinst/media/main/images/github-repositories/roshnivim/init.lua.png)
   ![screenshot_running_c](https://raw.githubusercontent.com/shaeinst/media/main/images/github-repositories/roshnivim/running_c.png)
   ![screenshot_telescode](https://raw.githubusercontent.com/shaeinst/media/main/images/github-repositories/roshnivim/telescope_as_fuzzy_finder.png)
   ![screenshot_codeaction](https://raw.githubusercontent.com/shaeinst/media/main/images/github-repositories/roshnivim/codeaction_in_flutter_app.png)
   ![screenshot_pythonlsp](https://raw.githubusercontent.com/shaeinst/media/main/images/github-repositories/roshnivim/python_lsp.png)

</details>



<br>

## To-Do
- load config from .__nvim__.lua file if it's defined in project
- write Document --(one day, for sure)

## Thanks to
- [shaankhan](https://readme.shaankhan.dev/) -- for readme
- ..more will be added later

<br>

## Warning!
this is not final. there is no specific rule made to follow to change in roshnivim. i will do this all later. <br>
But don't worry. if anything big is going to change, it's mapping or colors

## Known Bugs
- shows false line number status

