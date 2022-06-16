
# importing required module
# -------------------------------
import subprocess
import argparse
from pathlib import Path
from datetime import datetime
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
SCRIPT_PATH = Path(__file__).parent.absolute()

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

    if Path(f"{SCRIPT_PATH}/setup.py").is_file() and \
       Path(f"{SCRIPT_PATH}/.__roshnivim__").is_file():
        return False
    if roshnivim_git():
        return False

    return True

# -------------------------------


# -------------------------------
def compile_nvim():
    print( "\npress CTRL+C when you see something like: \"packer.compile: Complete\"\n")
    packer_compile_cmd = [ "nvim", "--headless", "-c", "autocmd User PackerComplete quitall", "-c", "PackerSync" ]
    subprocess.run(packer_compile_cmd)

# -------------------------------


# -------------------------------
def remove_no_require():
    subprocess.run(["rm", "-rf", ".git*", "LICENSE", "README.md", "setup.py", ".__*"],
                   cwd=NVIM_CONF_PATH)
    print("REMOVED: .git ,LICENSE ,README.md ,setup.py ,.__roshnivim__")
# -------------------------------


# -------------------------------
def main():

    print("--------------------------------")
    print("installing...this may take some time.")
    print("--------------------------------\n")

    # create required directories
    create_require_dir(require_dir)

    # backup config if backup argument is 1
    if backup == 1:
        backup_nvim()

    if update == 1 and roshnivim_git():
        subprocess.run(["git", "pull"], cwd=NVIM_CONF_PATH)

    else:
        if need_to_clone_roshnivim():
            clone_repro(CONFIG, "https://github.com/shaeinst/roshnivim", "nvim")

        else:
            # prevent copying or removing if setup.up is running from ~/.config/nvim/
            if SCRIPT_PATH != NVIM_CONF_PATH:
                # remove ~/.config/nvim/ to prevent depth parent copy(eg: ~/.config/nvim/nvim)
                subprocess.run(["rm", "-rf", NVIM_CONF_PATH])

                print("copying config...")
                subprocess.run(["cp", "-rf", SCRIPT_PATH, NVIM_CONF_PATH])

    # compile configs
    try:
        compile_nvim()
    except KeyboardInterrupt:
        print("\n\n")

    if delete == 1:
        print("cleaning config...")
        remove_no_require()

    # cloning lazy-builder tool (https://github.com/shaeinst/lazy-builder)
    lazy_builder_path = f"{CUSTOM_TOOLS_DIR}/lazy-builder"
    if not Path(lazy_builder_path).exists():
        try:
            print("installing additional...")
            repository = "https://github.com/shaeinst/lazy-builder"
            clone_repro(CUSTOM_TOOLS_DIR, repository, 'lazy-builder')
        except KeyboardInterrupt:
            print("additional tools didn't install")

# -------------------------------

if __name__ == '__main__':
    main()

