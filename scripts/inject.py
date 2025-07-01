import glob
import os
import pathlib
import zipfile

LOW_YEAR = 2024


def inject(data_dir, exec_path):
    try:
        zipfile.ZipFile(exec_path, "r").close()
    except zipfile.BadZipFile:
        pass
    else:
        return  # Don't reinject!

    with zipfile.ZipFile(
        pathlib.Path(data_dir).joinpath(f"{LOW_YEAR}-{LOW_YEAR + 1}/temp.zip"), "w", compression=zipfile.ZIP_STORED
    ) as zf:  # ZIP_DEFLATED, compresslevel=9) as zf:
        for i in glob.glob(pathlib.Path(data_dir).joinpath(f"{LOW_YEAR}-{LOW_YEAR + 1}/*.xml").as_posix()):
            pth = pathlib.Path(i)
            zf.write(i, pathlib.Path(*pth.parts[:-3], *pth.parts[-2:]))
    with (
        open(exec_path, "ab") as exe,
        open(pathlib.Path(data_dir).joinpath(f"{LOW_YEAR}-{LOW_YEAR + 1}/temp.zip"), "rb") as zipobj,
    ):
        # exe.seek(0, os.SEEK_END)
        exe.write(zipobj.read())
    os.remove(pathlib.Path(data_dir).joinpath(f"{LOW_YEAR}-{LOW_YEAR + 1}/temp.zip"))


if __name__ == "__main__":
    # print(list(os.environ.keys()))
    print()
    exec_path = pathlib.Path(
        os.environ["CARGO_MAKE_CRATE_CUSTOM_TRIPLE_TARGET_DIRECTORY"],
        os.environ["CARGO_MAKE_OUTPUT_DIR"],
        os.environ["CARGO_MAKE_PROJECT_NAME"],
    )
    if os.environ["CARGO_MAKE_RUST_TARGET_OS"] == "windows":
        exec_path = exec_path.with_name(exec_path.name + ".exe")  # Suffix?
    inject(pathlib.Path(os.environ["CARGO_MAKE_WORKSPACE_WORKING_DIRECTORY"], "scraped_programs"), exec_path)
