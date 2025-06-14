import glob
import io
import pathlib
import xml.etree.ElementTree as ET
import zipfile

import mutablezip


def embed_file_structure(base_xlsx, files_to_embed):
    with mutablezip.MutableZipFile(base_xlsx, "a", compression=zipfile.ZIP_DEFLATED) as zf:
        tree = ET.parse(io.BytesIO(zf.read("[Content_Types].xml")))
        root = tree.getroot()
        ns = {"ct": "http://schemas.openxmlformats.org/package/2006/content-types"}
        ET.register_namespace("", ns["ct"])

        for embedded_path in files_to_embed:
            embedded_path = pathlib.Path(embedded_path).as_posix().lstrip("/")
            if embedded_path in zf.namelist():
                zf.read(embedded_path)  # Allows MutableZip to overwrite it later somehow
            with open(embedded_path, "rb") as f:
                zf.writestr(embedded_path, f.read())

            # Avoid duplicate entries
            existing = any(el.attrib.get("PartName") == "/" + embedded_path for el in root.findall("ct:Override", ns))

            if not existing:
                ET.SubElement(
                    root,
                    r"{http://schemas.openxmlformats.org/package/2006/content-types}Override",
                    {"PartName": "/" + embedded_path, "ContentType": "application/xml"},
                )
        outf = io.BytesIO()
        tree.write(outf, xml_declaration=True, encoding="utf-8")
        zf.writestr("[Content_Types].xml", outf.getvalue())

    print(f"✅ Success: File embedded in {base_xlsx}")


embed_file_structure("rdtest1.xlsx", glob.glob("scraped_programs/*.xml"))
