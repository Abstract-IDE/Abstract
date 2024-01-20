
# importing required module
# -------------------------------
import subprocess
from shutil import which
import argparse
from pathlib import Path
from datetime import datetime

# import pynvim and install it if it's not already installed
try:
    from pynvim import attach
except ImportError:
    # install pip if it's not installed
    try:
        import pip
    except ModuleNotFoundError:
        command = ["python3", "-m", "ensurepip", "--default-pip"]
        subprocess.run(command)
    command = ['pip', 'install', 'pynvim']
    subprocess.run(command)
    from pynvim import attach
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
            Abstract",
)

args = parser.parse_args()
delete = args.delete
backup = args.backup
update = args.update
# -------------------------------


# directory locations
# -------------------------------
nvim = attach('child', argv=[which("env"), "nvim", "--embed", "--headless"])

HOME = Path.home()
NVIM_DATA_DIR = nvim.funcs.stdpath('data')
NVIM_CONF_PATH = nvim.funcs.stdpath('config')

CONFIG = NVIM_CONF_PATH.split("nvim")[0]
CACHE = f"{NVIM_DATA_DIR}/.cache"
CUSTOM_TOOLS_DIR = f"{NVIM_DATA_DIR}/custom_tools"
CACHE_BUILD_PATH = f"{HOME}/.cache/build_files"
SCRIPT_PATH = Path(__file__).parent.absolute()

LAZY_ROOT_PATH = NVIM_DATA_DIR+"/lazy"
LAZY_DIR = LAZY_ROOT_PATH+"/lazy.nvim"

# -------------------------------


# directories we must have
# -------------------------------
require_dir = [
    f"{CONFIG}",
    f"{CACHE}/swap",
    f"{CACHE}/view",
    f"{CACHE}/shada",
    f"{CACHE}/backedUP",
    f"{CACHE}/undos",
    CUSTOM_TOOLS_DIR,
    CACHE_BUILD_PATH,
]
# -------------------------------


# -------------------------------
def create_require_dir(dirs):
    need_to_inform = False
    once = True

    for dir in dirs:
        if not Path(dir).exists():
            need_to_inform = True
            if need_to_inform and once:
                print("creating required directories...")
                once = False
            Path(dir).mkdir(parents=True)
            print(" ", dir)
# -------------------------------


# -------------------------------
def clone_repro(path, repository, name):
    subprocess.run(['git', 'clone', repository, name], cwd=path)
# -------------------------------


# -------------------------------
def update_abstract():
    subprocess.run(["git", "pull"], cwd=NVIM_CONF_PATH)
# -------------------------------


# -------------------------------
def backup_nvim():
    current_date = datetime.today().strftime("%Y-%m-%d_%T")
    if Path(NVIM_CONF_PATH).exists():
        subprocess.run(['cp', '-rf', 'nvim', f'nvim-old_{current_date}'], cwd=CONFIG)
        print(f"\nyour old config: {NVIM_CONF_PATH}_{current_date}\n")
# -------------------------------


# -------------------------------
def clean():
    if Path(LAZY_ROOT_PATH).exists():
        subprocess.run(["rm", "-rf", LAZY_ROOT_PATH])
        print("\nREMOVED: ", LAZY_ROOT_PATH)
# -------------------------------


# -------------------------------
def abstract_git():
    """check if abstract exist as a git project"""

    if Path(NVIM_CONF_PATH).exists():
        if Path(f"{NVIM_CONF_PATH}/.__abstract__").is_file():
            if Path(f"{NVIM_CONF_PATH}/.git").exists():
                return True
    return False
# -------------------------------


# -------------------------------
def need_to_clone_abstract():
    """check if we need to clone the abstract repro,
    if setup.py run without cloning abstract
    """

    if Path(f"{SCRIPT_PATH}/setup.py").is_file() and \
       Path(f"{SCRIPT_PATH}/.__abstract__").is_file():
        return False

    if abstract_git():
        return False

    return True
# -------------------------------


# -------------------------------
def compile_nvim():
    print("\ncompiling config and plugins...")
    print("\nIf it's taking long time then press CTRL+C and run setup.py file again \n (2-5 minutes would be enough) \n")
    lazy_compile_cmd = ["nvim", "--headless", "-c", "autocmd User LazySync quitall", "-c", "Lazy sync"]
    subprocess.run(lazy_compile_cmd)
# -------------------------------


# -------------------------------
def setup_lazy():
    print("\nsetting up lazy plugin manager...")

    if not Path(LAZY_ROOT_PATH).exists():
        Path(LAZY_ROOT_PATH).mkdir(parents=True)

    if not Path(LAZY_DIR).exists():
        print(LAZY_DIR)
        repository = "https://github.com/folke/lazy.nvim"
        subprocess.run(["git", "clone", "--depth", "1", repository], cwd=LAZY_ROOT_PATH)
# -------------------------------


# -------------------------------
def remove_no_require():
    subprocess.run(["rm", "-rf", ".git*", "LICENSE", "README.md", "setup.py", ".__*"], cwd=NVIM_CONF_PATH)
    print("\nREMOVED: .git ,LICENSE ,README.md ,setup.py ,.__abstract__")
# -------------------------------


# -------------------------------
def main():

    print("--------------------------------")
    print("installing...this may take some time.")
    print("--------------------------------\n")

    # backup config if backup argument is 1
    if backup == 1:
        backup_nvim()

    # create required directories
    create_require_dir(require_dir)

    if update == 1 and abstract_git():
        update_abstract()

    else:
        if need_to_clone_abstract():
            print("\n")
            clone_repro(CONFIG, "https://github.com/Abstract-IDE/Abstract", "nvim")

        else:
            # prevent copying or removing if setup.up is running from ~/.config/nvim/
            if str(SCRIPT_PATH) != str(NVIM_CONF_PATH):
                if Path(f"{SCRIPT_PATH}/setup.py").is_file() and Path(f"{SCRIPT_PATH}/.__abstract__").is_file():
                    # remove ~/.config/nvim/ to prevent depth parent copy(eg: ~/.config/nvim/nvim)
                    subprocess.run(["rm", "-rf", NVIM_CONF_PATH])
                    print("\ncopying config...")
                    subprocess.run(["cp", "-rf", SCRIPT_PATH, NVIM_CONF_PATH])

                else:
                    update_abstract()

            else:
                update_abstract()

    # compile configs
    try:
        clean()
        setup_lazy()
        compile_nvim()
    except KeyboardInterrupt:
        print("\n\n")
        print("!!!someting went wrong!!!!\n please re-run setup.py file")

    if delete == 1:
        print("\ncleaning config...")
        remove_no_require()

    # cloning lazy-builder tool (https://github.com/shaeinst/lazy-builder)
    lazy_builder_path = f"{CUSTOM_TOOLS_DIR}/lazy-builder"
    if not Path(lazy_builder_path).exists():
        try:
            print("\ninstalling additional...")
            repository = "https://github.com/Abstract-IDE/lazy-builder"
            clone_repro(CUSTOM_TOOLS_DIR, repository, 'lazy-builder')
        except KeyboardInterrupt:
            print("additional tools didn't install\n")
    else:
        subprocess.run(["git", "pull"], cwd=lazy_builder_path)

    msg = """\n\n
    !!!WARNING!!!
    we try our best to auto setup as much as possible.
    you may get some errors during installation (maybe it's your network problem?)
    so, open nvim (from your terminal ) and if nvim throws any errors,
    it means that installaton wasn't sucessfull.
    In that case, please re-run setup.py or
    python <(curl -s https://raw.githubusercontent.com/Abstract-IDE/Abstract/main/setup.py)
    """
    print(msg)


# -------------------------------


if __name__ == '__main__':
    main()

