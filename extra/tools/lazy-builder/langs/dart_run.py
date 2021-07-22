import subprocess

def dart_run(file_name):
    cmd = f"dart {file_name}"
    cmd = "".join(cmd).split()
    subprocess.run(cmd)

