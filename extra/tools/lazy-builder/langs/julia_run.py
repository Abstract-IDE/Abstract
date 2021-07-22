import subprocess

def julia_run(file_name):
    cmd = f"julia {file_name}"
    cmd = "".join(cmd).split()
    subprocess.run(cmd)

