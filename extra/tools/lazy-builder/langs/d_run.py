import subprocess


def d_exec():
    cmd = f"{file_name_no_extc_with_loc}"
    if output_location != "None":
        cmd = f"{output_location}{file_name_no_extc}"
    cmd = "".join(cmd).split()

    try:

        print("████████████████ RUNNING PROGRAM ████████████████")
        subprocess.run(cmd)
        print("\n\n")
    except FileNotFoundError:
        print(f"compiled file, {file_name_no_extc_with_loc}, \033[91mnot found")


def d_build():
    cmd = f"dmd {file_name_with_location} -of={file_name_no_extc}"
    if output_location != "None":
        cmd = f"dmd {file_name_with_location} -of={output_location}{file_name_no_extc}"
    cmd = "".join(cmd).split()

    status = subprocess.run(cmd)

    if  status.returncode == 0:
        print("__________________________________________")
        print("✔\tCompilation Successful")
        print("__________________________________________")
    if status.returncode != 0:
        print("__________________________________________")
        print("❌\tCompilation Failed")
        print("__________________________________________")
        exit(1)


def d_buildexec():
    d_build()
    d_exec()


def d_run(program_name, conditions, output_loc="None"):
    build, run, build_run = conditions
    global file_name, file_name_no_extc, output_location, file_name_with_location, file_name_no_extc_with_loc
    file_name = program_name.split("/")[-1]
    file_name_no_extc = file_name.split(".")[0]     # getting rid of extension
    file_name_no_extc_with_loc = program_name.split(".")[0]
    file_name_with_location = program_name
    output_location = str(output_loc)

    if run == 1 and build == 0:
        d_exec()
    if run == 1 and build == 1:
        d_buildexec()
    if build == 1 and run == 0:
        d_build()
    if build_run == 1:
        d_buildexec()

