import subprocess

def R_run(file_name):
    cmd = f"Rscript {file_name}"
    cmd = "".join(cmd).split()
    subprocess.run(cmd)

