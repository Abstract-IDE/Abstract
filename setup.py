# importing required module
# -------------------------------
import subprocess
import argparse
from pathlib import Path
from datetime import datetime
import fileinput
# -------------------------------


# argument handeling
# -------------------------------
parser = argparse.ArgumentParser()
parser.add_argument(
    "--delete",
    "-d",
    type=int,
    default=0,
    help="0 or 1, 0 means don't delete(Default) and 1 means delete the no \
            required file/directory (.git, LICENSE, README.md)",
)
parser.add_argument(
    "--backup",
    "-b",
    type=int,
    default=1,
    help="0 or 1, 0 means don't backup and 1 means backup(Default) the \
            config before any configuration",
)
parser.add_argument(
    "--update",
    "-u",
    type=int,
    default=0,
    help="0 or 1, 1 means update and 0 means don't update(Default) the \
            Roshnivim",
)

args = parser.parse_args()
delete = args.delete
backup = args.backup
update = args.update
# -------------------------------

# directory locations
# -------------------------------
HOME = Path.home()
CACHE = f"{HOME}/.cache"
CONFIG = f"{HOME}/.config"
CACHE_BUILD_PATH = f"{CACHE}/build_files"
NVIM_CONF_PATH = f"{CONFIG}/nvim"
NVIM_PLUG_PATH = f"{HOME}/.local/share/nvim"
CUSTOM_TOOLS_DIR = f"{NVIM_PLUG_PATH}/custom_tools"

# -------------------------------

# directories we must have
# -------------------------------
require_dir = [
    f"{CONFIG}",
    f"{CACHE}/nvim/swap",
    f"{CACHE}/nvim/shada",
    f"{CACHE}/nvim/backedUP",
    f"{CACHE}/nvim/undos",
    CUSTOM_TOOLS_DIR,
    CACHE_BUILD_PATH,
]

# -------------------------------


# -------------------------------
def create_require_dir(dirs):
    print("creating required directories...")
    for dir in dirs:
        if not Path(dir).exists():
            Path(dir).mkdir(parents=True)
            print(" ",dir)

# -------------------------------


# -------------------------------
def clone_repro(path, repository, name):
    subprocess.run(['git', 'clone', repository, name], cwd=path)

# -------------------------------


# -------------------------------
def backup_nvim():
    current_date = datetime.today().strftime("%Y-%m-%d_%T")
    if Path(NVIM_CONF_PATH).exists():
        subprocess.run(['cp', '-rf', 'nvim', f'nvim-old_{current_date}'], cwd=CONFIG)
        print(f"your old config: {NVIM_CONF_PATH}_{current_date}\n")

# -------------------------------


# -------------------------------
def roshnivim_git():
    """check if roshnivim exist as a git project"""
    if Path(NVIM_CONF_PATH).exists():
        if Path(f"{NVIM_CONF_PATH}/.__roshnivim__").is_file():
                if Path(f"{NVIM_CONF_PATH}/.git").exists():
                    return True
    return False

# -------------------------------


# -------------------------------
def need_to_clone_roshnivim():
    """check if we need to clone the roshnivim repro,
    if setup.py run without cloning roshnivim
    """

    script_path = Path(__file__).parent.absolute()
    if Path(f"{script_path}/setup.py").is_file() and \
       Path(f"{script_path}/.__roshnivim__").is_file():
        return False
    if roshnivim_git():
        return False

    return True

# -------------------------------


# -------------------------------
def replace_text(filename, old_text, new_text):
    with fileinput.FileInput(filename, inplace=True) as file:
        for line in file:
            print(line.replace(old_text, new_text), end='')

# -------------------------------


# -------------------------------
def compile_nvim():
    packer_compile_cmd = ["nvim", "--headless", "-c", "PackerSync"]
    subprocess.run(packer_compile_cmd, cwd=NVIM_CONF_PATH)

# -------------------------------


# -------------------------------
def remove_no_require():
    subprocess.run(["rm", "-rf", ".git*", "LICENSE", "README.md", "setup.py", ".__*"],
                   cwd=NVIM_CONF_PATH)
    print("Removed: .git")
    print("Removed: LICENSE")
    print("Removed: README.md")
    print("Removed: setup.py")
    print("Removed: .__roshnivim__")
# -------------------------------


# -------------------------------
def disable_config(comment, text_colorscheme, text_impatient, text_filetype):
    # Make Comment
    if comment:
        replace_text(f"{NVIM_CONF_PATH}/lua/configs.lua", text_colorscheme, f"--{text_colorscheme}")
        replace_text(f"{NVIM_CONF_PATH}/init.lua", text_impatient, f"--{text_impatient}")
        replace_text(f"{NVIM_CONF_PATH}/init.lua", text_filetype, f"--{text_filetype}")
    #UnComment
    else:
        replace_text(f"{NVIM_CONF_PATH}/lua/configs.lua", f"--{text_colorscheme}", text_colorscheme)
        replace_text(f"{NVIM_CONF_PATH}/init.lua", f"--{text_impatient}", text_impatient)
        replace_text(f"{NVIM_CONF_PATH}/init.lua", f"--{text_filetype}", text_filetype)

# -------------------------------


# -------------------------------
def install_roshnivim():

    text_colorscheme = "cmd('colorscheme rvcs'"
    text_impatient = "require('plugins/impatient_nvim')"
    text_filetype = "require('plugins/filetype_nvim')"
    # going to comment some line of code to prevent getting any error.
    disable_config(True, text_colorscheme, text_impatient, text_filetype)

    # run nvim command to install plugins
    print("\ninstalling PLUGINS...")
    packer_install_cmd = [ "nvim", "--headless", "-c", "autocmd User PackerComplete quitall", "-c", "PackerSync" ]
    try:
        subprocess.run(packer_install_cmd, cwd=NVIM_CONF_PATH)
    except KeyboardInterrupt:
        # uncomment the line of code we commented before installing the plugin
        disable_config(False, text_colorscheme, text_impatient, text_filetype)
        print("\n")

    # uncomment the line of code we commented before installing the plugin
    disable_config(False, text_colorscheme, text_impatient, text_filetype)

    # recompile configs
    print("\n\nwow! roshnivim is installed")
    try:
        print(
            "\nwhen you see something like: 'packer.compile: Complete', press CTRL+C\n"
        )
        compile_nvim()
    except KeyboardInterrupt:
        print("\n")

# -------------------------------


# -------------------------------
def main():

    # create required directories
    create_require_dir(require_dir)
    # backup config if backup argument is 1
    if backup == 1:
        backup_nvim()

    if update == 1 and roshnivim_git():
        subprocess.run(["git", "pull"], cwd=NVIM_CONF_PATH)
        try:
            print(
                "\nwhen you see something like: 'packer.compile: Complete', press CTRL+C\n"
            )
            compile_nvim()
        except KeyboardInterrupt:
            print("\n")

    else:
        if need_to_clone_roshnivim():
            clone_repro(CONFIG, "https://github.com/shaeinst/roshnivim", "nvim")

        if not roshnivim_git:
            print("copying config...")
            subprocess.run(["cp", "-rfv", "../*n*vim*", f"{CONFIG}/nvim"])

        install_roshnivim()

    if delete == 1:
        print("cleaning config...")
        remove_no_require()

    # cloning lazy-builder tool (https://github.com/shaeinst/lazy-builder)
    lazy_builder_path = f"{CUSTOM_TOOLS_DIR}/lazy-builder"
    if not Path(lazy_builder_path).exists():
        print("installing additional...")
        repository = "https://github.com/shaeinst/lazy-builder"
        clone_repro(CUSTOM_TOOLS_DIR, repository, 'lazy-builder')

# -------------------------------


main()
