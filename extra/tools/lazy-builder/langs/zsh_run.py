import subprocess

def zsh_run(file_name):
    cmd = f"zsh {file_name}"
    cmd = "".join(cmd).split()
    subprocess.run(cmd)
