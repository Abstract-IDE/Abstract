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
* [Documentaton](#documentaton)
* [Project Stracture](#project-stracture)
* [License](#license)
* [Screenshots](#screenshots)
* [To-do](#to-do)


## About

Abstract, The NeoVim configuration to achieve the power of Modern IDE

![2022-07-25_20-56](https://user-images.githubusercontent.com/41078534/180812465-8e65807c-a25e-4bc6-8abd-38d32538259e.png)



## Documentaton
more about Abstract and installation can be found on [DOCS](https://abstract-ide.github.io/site/docs/getting-started/installation/abstract)



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

- add installer(setup.py) support for Windows and Mac os



## Known Bugs

- there is no known bugs yet. Please open the issue if you find one.

