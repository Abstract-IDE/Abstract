import subprocess
from shutil import which
from pathlib import Path
from datetime import datetime

try:
    from pynvim import attach
except ImportError:
    # install pip if it's not installed
    try:
        import pip
    except ModuleNotFoundError:
        subprocess.run(["python3", "-m", "ensurepip", "--default-pip"])
    subprocess.run(["pip", "install", "pynvim"])
    from pynvim import attach


nvim = attach(
    "child", argv=[which("env") or "/usr/bin/env", "nvim", "--embed", "--headless"]
)
NVIM_DATA_PATH = nvim.funcs.stdpath("data")
NVIM_CONF_PATH = nvim.funcs.stdpath("config")
NVIM_CACHE_PATH = nvim.funcs.stdpath("cache")
NVIM_STATE_PATH = nvim.funcs.stdpath("state")
SCRIPT_PATH = Path(__file__).parent.absolute()


def backup_nvim():
    def mv(path, date):
        if Path(path).exists():
            subprocess.run(["mv", path, f"{path}-old_{date}"])
            print(f"{path} -> {path}-old_{date}")

    current_date = datetime.today().strftime("%Y-%m-%d_%T")
    print("backing up your configs")
    mv(NVIM_CONF_PATH, current_date)
    mv(NVIM_DATA_PATH, current_date)
    mv(NVIM_CACHE_PATH, current_date)
    mv(NVIM_STATE_PATH, current_date)
    print("\n")


def compile_nvim():
    print("\nsetting up config and installing plugins...")
    print(
        "Please wait; the installation of the plugin will take some time depending on your network.\n"
    )
    init_cmd = [
        "nvim",
        "--headless",
        "-c",
        "autocmd User LazyLoad quitall",
    ]
    subprocess.run(init_cmd)


def remove_no_required():
    subprocess.run(["rm", "-rf", ".git*", "LICENSE", "README.md"], cwd=NVIM_CONF_PATH)


def main():
    print("-------------------------------------")
    print("installing...this may take some time.")
    print("-------------------------------------\n")

    backup_nvim()
    subprocess.run(
        [
            "git",
            "clone",
            "https://github.com/Abstract-IDE/Abstract-Bridge",
            NVIM_CONF_PATH,
        ],
    )
    try:
        compile_nvim()
        remove_no_required()
    except KeyboardInterrupt:
        print("\n\n")
        print("!!!installation terminated!!!!")

    msg = """\n\n
    !!!WARNING!!!
    Our automated setup process aims to streamline the installation experience.
    In the event of errors during installation, it is advisable to investigate potential
    network-related issues. To conduct a more detailed analysis, launch nvim from your
    terminal. If nvim reports errors, it indicates an unsuccessful installation.
    """
    print(msg)


if __name__ == "__main__":
    main()
