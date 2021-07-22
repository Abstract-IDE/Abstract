import subprocess


def qbasic_exec():
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


def qbasic_build():
    cmd = f"/home/lazy/codeDNA/confiFILES/programmingLANGUAGE/qb64/qb64 -x {file_name_with_location} -o {file_name_no_extc}"
    if output_location != "None":
        cmd = f"/home/lazy/codeDNA/confiFILES/programmingLANGUAGE/qb64/qb64 -x {file_name_with_location} -o {output_location}{file_name_no_extc}"
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


def qbasic_buildexec():
    qbasic_build()
    qbasic_exec()


def qbasic_run(program_name, conditions, output_loc="None"):
    build, run, build_run = conditions
    global file_name, file_name_no_extc, output_location, file_name_with_location, file_name_no_extc_with_loc
    file_name = program_name.split("/")[-1]
    file_name_no_extc = file_name.split(".")[0]     # getting rid of extension
    file_name_no_extc_with_loc = program_name.split(".")[0]
    file_name_with_location = program_name
    output_location = str(output_loc)

    if run == 1 and build == 0:
        qbasic_exec()
    if run == 1 and build == 1:
        qbasic_buildexec()
    if build == 1 and run == 0:
        qbasic_build()
    if build_run == 1:
        qbasic_buildexec()

