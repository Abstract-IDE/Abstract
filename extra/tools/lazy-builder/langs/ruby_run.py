import subprocess

def ruby_run(file_name):
    cmd = f"ruby {file_name}"
    cmd = "".join(cmd).split()
    subprocess.run(cmd)

