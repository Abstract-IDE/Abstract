# LAZY-BUILDER

Lazy-Builder is a python script for buinding or running any programming language('s file) in vim by mapping lazy-builder script in vimrc(for vim), init.vim(for neo-vim) file.

### logic
logic of lazy-build is simple. it just analyse the extension of file which you want to build/run.If programming language is Interpreted, it ran the file as interpreted language OR if programming language is Compiled, it ran the/build file as compliled language
 
## Uses


```
usage: python build.py [-h] [-r] [-b] [-br] [-o] program_name

positional arguments:
  program_name  File/Program that you want to RUN/BUILD (eg. hello.c, calculate.py, auto.sh)

optional arguments:
  -h, --help    show this help message and exit
  -r    0 or 1, 1 means RUN and 0(default) means don't RUN the program
  -b    0 or 1, 1 means BUILD and 0(default) means don't BUILD the program
  -br   0 or 1, 1 means BUILD and RUN and 0 means don't BUILD and RUN the program
  -o    location for output of compiled language (eg. /home/user/folder1/)

```

## Using with vim/neo-vim
you can use lazy-build with vim by mapping as shown in below example:

[ NOTE: first install  [vim-floaterm](https://github.com/voldikss/vim-floaterm) plugin
                    OR
if you don't want to install vim-floaterm plugin, go with [this](https://github.com/shaeinst/lazy-builder/tree/main) branch and follow [this](https://github.com/shaeinst/lazy-builder/tree/main#using-with-vimneo-vim) example of mapping (which use builtin split option)
] 
```
" Run program file when leader+r is pressed
nnoremap <Leader>r :w <CR><bar> :FloatermNew
    \ python /home/user/folder/lazy-builder/build.py -o /home/user/.cache/build_files/ -r 1 % <CR><CR>

" build the program file when leader+b is pressed
nnoremap <Leader>b :w <CR><bar> :FloatermNew
    \ time
    \ python /home/user/folder/lazy-builder/build.py -o /home/user/.cache/build_files/ -b 1 % <CR><CR>
    
" build and then run the program when leader+o is pressed
nnoremap <Leader>o :w <CR><bar> :FloatermNew
    \ time
    \ python /home/user/folder/lazy-builder/build.py -o /home/user/.cache/build_files/ -br 1 % <CR><CR>
```
replace the following location to the location where you downloaded/cloned lazy-builder repositories
```
/home/user/folder/lazy-builder/build.py
```
replace this location to your desired location where you want to put the output/build/compiled files. 
```
/home/user/.cache/build_files/
```
or if you want the compiled file to stay in it's parent directory(from where it's compiled) then remove "-o /home/user/.cache/build_files/" from binding



# currently supported languages
```
[ Compiled Languages ]
c/c++
D
go
qbasic
rust
V

[ Interpreted Languages ]
bash
javascript
julia
perl
python
R
sql
zsh

[ more comming soon...]
```
# to-do
add more language support

## Screenshots
### running python program
![running python file using lazy-builder script](https://github.com/shaeinst/lazy-builder/blob/LazyBuild-floaterm/Screenshots/2020-12-30_19-46.png)

### compiling c++ program
![running python file using lazy-builder script](https://github.com/shaeinst/lazy-builder/blob/LazyBuild-floaterm/Screenshots/2020-12-30_19-46_1.png)

### running c++ program
![running python file using lazy-builder script](https://github.com/shaeinst/lazy-builder/blob/LazyBuild-floaterm/Screenshots/2020-12-30_19-47.png)

### compiling and then running c++ program
![running python file using lazy-builder script](https://github.com/shaeinst/lazy-builder/blob/LazyBuild-floaterm/Screenshots/2020-12-30_19-47_1.png)

### running bash program
![running python file using lazy-builder script](https://github.com/shaeinst/lazy-builder/blob/LazyBuild-floaterm/Screenshots/2020-12-30_19-49.png)




