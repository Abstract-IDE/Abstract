import subprocess

def javascript_run(file_name):
    cmd = f"node {file_name}"
    cmd = "".join(cmd).split()
    subprocess.run(cmd)

