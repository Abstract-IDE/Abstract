## NOT worked
# import subprocess

# def sql_run(file_name):
#     cmd = f"mysql < {file_name} --table "
#     cmd = "".join(cmd).split()
#     subprocess.run(cmd)

import os

def sql_run(file_name):
    cmd = f"mysql < {file_name} --table "
    os.system(cmd)

