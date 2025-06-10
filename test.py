import io
import os
import pathlib
import shutil
import xml.etree.ElementTree as ET
import zipfile

import mutablezip


def embed_file_in_xlsx(base_xlsx, file_to_embed, embedded_path, output_xlsx):
    temp_dir = "temp_xlsx"

    # Clean up any previous temp dir
    if os.path.exists(temp_dir):
        shutil.rmtree(temp_dir)
    os.makedirs(temp_dir)

    # Step 1: Unzip base_xlsx into temp_dir
    with mutablezip.MutableZipFile(base_xlsx, "r") as zin:
        zin.extractall(temp_dir)

    # Step 2: Copy the file into the desired path inside the temp folder
    full_embed_path = os.path.join(temp_dir, embedded_path.lstrip("/"))
    os.makedirs(os.path.dirname(full_embed_path), exist_ok=True)
    shutil.copyfile(file_to_embed, full_embed_path)

    # Step 3: Patch [Content_Types].xml to register the new file
    content_types_path = os.path.join(temp_dir, "[Content_Types].xml")
    tree = ET.parse(content_types_path)
    root = tree.getroot()
    ns = {"ct": "http://schemas.openxmlformats.org/package/2006/content-types"}
    ET.register_namespace("", ns["ct"])

    # Avoid duplicate entries
    existing = any(el.attrib.get("PartName") == embedded_path for el in root.findall("ct:Override", ns))

    if not existing:
        ET.SubElement(
            root,
            r"{http://schemas.openxmlformats.org/package/2006/content-types}Override",
            {"PartName": embedded_path, "ContentType": "text/string"},
        )
        tree.write(content_types_path, xml_declaration=True, encoding="utf-8")

    # Step 4: Zip everything back into output_xlsx
    with mutablezip.MutableZipFile(output_xlsx, "w") as zout:  # , compression=zipfile.ZIP_DEFLATED
        for root_dir, _, files in os.walk(temp_dir):
            for file in files:
                full_path = os.path.join(root_dir, file)
                rel_path = os.path.relpath(full_path, temp_dir)
                zout.write(full_path, rel_path)

    # Step 5: Cleanup
    shutil.rmtree(temp_dir)

    print(f"✅ Success: File embedded in {output_xlsx}")


def embed_file_in_xlsx(base_xlsx, file_to_embed, embedded_path, output_xlsx):
    with mutablezip.MutableZipFile(base_xlsx, "a", compression=zipfile.ZIP_DEFLATED) as zf:
        # # Step 1: Unzip base_xlsx into temp_dir
        # with mutablezip.MutableZipFile(base_xlsx, "r") as zin:
        #     zin.extractall(temp_dir)

        # Step 2: Copy the file into the desired path inside the temp folder
        # full_embed_path = os.path.join(temp_dir, embedded_path.lstrip("/"))
        # os.makedirs(os.path.dirname(full_embed_path), exist_ok=True)
        # shutil.copyfile(file_to_embed, full_embed_path)

        # Write the file directly into the zip at the specified path
        zf.write(file_to_embed, embedded_path.lstrip("/"))

        # Step 3: Patch [Content_Types].xml to register the new file
        # content_types_path = os.path.join(temp_dir, "[Content_Types].xml")
        tree = ET.parse(io.BytesIO(zf.read("[Content_Types].xml")))
        root = tree.getroot()
        ns = {"ct": "http://schemas.openxmlformats.org/package/2006/content-types"}
        ET.register_namespace("", ns["ct"])

        # Avoid duplicate entries
        existing = any(el.attrib.get("PartName") == embedded_path for el in root.findall("ct:Override", ns))

        if not existing:
            ET.SubElement(
                root,
                r"{http://schemas.openxmlformats.org/package/2006/content-types}Override",
                {"PartName": embedded_path, "ContentType": "application/xml"},
            )
            # zf.removeFile("[Content_Types].xml")
            outf = io.BytesIO()
            tree.write(outf, xml_declaration=True, encoding="utf-8")
            zf.writestr("[Content_Types].xml", outf.getvalue())

    # # Step 4: Zip everything back into output_xlsx
    # with mutablezip.MutableZipFile(output_xlsx, "w") as zout:  # , compression=zipfile.ZIP_DEFLATED
    #     for root_dir, _, files in os.walk(temp_dir):
    #         for file in files:
    #             full_path = os.path.join(root_dir, file)
    #             rel_path = os.path.relpath(full_path, temp_dir)
    #             zout.write(full_path, rel_path)

    # Step 5: Cleanup
    # shutil.rmtree(temp_dir)

    print(f"✅ Success: File embedded in {base_xlsx}")


def embed_file_structure(excel_file, relpaths, basepath=None):
    basepath = pathlib.Path(basepath) if basepath else pathlib.Path()
    with mutablezip.MutableZipFile(excel_file, "a", compression=zipfile.ZIP_DEFLATED) as zf:
        with zf.open("[Content_Types].xml", "r") as contents:
            tree = ET.parse(contents)
        root = tree.getroot()
        ns = {"ct": "http://schemas.openxmlformats.org/package/2006/content-types"}
        ET.register_namespace("", ns["ct"])

        for relpath in relpaths:
            full_path = basepath / pathlib.Path(relpath)
            zf.write(full_path, relpath)

            existing = any(el.attrib.get("PartName") == relpath for el in root.findall("ct:Override", ns))
            if not existing:
                ET.SubElement(
                    root,
                    r"{http://schemas.openxmlformats.org/package/2006/content-types}Override",
                    {"PartName": relpath, "ContentType": "application/xml"},
                )

        zf.removeFile("[Content_Types].xml")
        with zf.open("[Content_Types].xml", "w") as contents:
            tree.write(contents, xml_declaration=True, encoding="utf-8")


def embed_file_structure(excel_file, relpaths, basepath=None):
    basepath = pathlib.Path(basepath) if basepath else pathlib.Path()
    with mutablezip.MutableZipFile(excel_file, "a", compression=zipfile.ZIP_DEFLATED) as zf:
        # # Step 1: Unzip base_xlsx into temp_dir
        # with mutablezip.MutableZipFile(base_xlsx, "r") as zin:
        #     zin.extractall(temp_dir)

        # Step 2: Copy the file into the desired path inside the temp folder
        # full_embed_path = os.path.join(temp_dir, embedded_path.lstrip("/"))
        # os.makedirs(os.path.dirname(full_embed_path), exist_ok=True)
        # shutil.copyfile(file_to_embed, full_embed_path)

        # Step 3: Patch [Content_Types].xml to register the new file
        # content_types_path = os.path.join(temp_dir, "[Content_Types].xml")
        tree = ET.parse(io.BytesIO(zf.read("[Content_Types].xml")))
        root = tree.getroot()
        ns = {"ct": "http://schemas.openxmlformats.org/package/2006/content-types"}
        ET.register_namespace("", ns["ct"])

        for relpath in relpaths:
            full_path = basepath / pathlib.Path(relpath)
            # Write the file directly into the zip at the specified path
            zf.write(full_path, relpath.lstrip("/"))

            # Avoid duplicate entries
            existing = any(el.attrib.get("PartName") == relpath for el in root.findall("ct:Override", ns))

            if not existing:
                ET.SubElement(
                    root,
                    r"{http://schemas.openxmlformats.org/package/2006/content-types}Override",
                    {"PartName": relpath, "ContentType": "application/xml"},
                )
                # zf.removeFile("[Content_Types].xml")

        outf = io.BytesIO()
        tree.write(outf, xml_declaration=True, encoding="utf-8")
        zf.writestr("[Content_Types].xml", outf.getvalue())

    # # Step 4: Zip everything back into output_xlsx
    # with mutablezip.MutableZipFile(output_xlsx, "w") as zout:  # , compression=zipfile.ZIP_DEFLATED
    #     for root_dir, _, files in os.walk(temp_dir):
    #         for file in files:
    #             full_path = os.path.join(root_dir, file)
    #             rel_path = os.path.relpath(full_path, temp_dir)
    #             zout.write(full_path, rel_path)

    # Step 5: Cleanup
    # shutil.rmtree(temp_dir)

    print(f"✅ Success: File embedded in {excel_file}")


# === USAGE ===

embed_file_in_xlsx(
    base_xlsx="clean_base.xlsx",  # Must exist
    file_to_embed="scraped_programs/Art_(BA).xml",  # Must exist
    embedded_path="/scraped_programs/Art_(BA).xml",  # Where it will be stored inside the .xlsx
    output_xlsx="patched.xlsx",  # Output file
)
