import io
import pathlib
import xml.etree.ElementTree as ET
import zipfile

import mutablezip


def embed_file_as_invisible_ole(base_xlsx, file_to_embed, object_name="HiddenObject"):
    # Normalize name
    embed_name = pathlib.Path(file_to_embed).name
    embedded_path = f"xl/embeddings/{embed_name}"

    with mutablezip.MutableZipFile(base_xlsx, "a", compression=zipfile.ZIP_DEFLATED) as zf:
        # 1. Write file to embeddings/
        with open(file_to_embed, "rb") as f:
            zf.writestr(embedded_path, f.read())

        # 2. Update [Content_Types].xml
        ct_tree = ET.parse(io.BytesIO(zf.read("[Content_Types].xml")))
        ct_root = ct_tree.getroot()
        ns_ct = "http://schemas.openxmlformats.org/package/2006/content-types"
        ET.register_namespace("", ns_ct)

        mime_type = "application/octet-stream"
        if not any(el.attrib.get("PartName") == "/" + embedded_path for el in ct_root.findall(f"{{{ns_ct}}}Override")):
            ET.SubElement(ct_root, f"{{{ns_ct}}}Override", {"PartName": "/" + embedded_path, "ContentType": mime_type})

        out_ct = io.BytesIO()
        ct_tree.write(out_ct, xml_declaration=True, encoding="utf-8")
        zf.writestr("[Content_Types].xml", out_ct.getvalue())

        # 3. Update xl/_rels/workbook.xml.rels
        rels_path = "xl/_rels/workbook.xml.rels"
        if rels_path in zf.namelist():
            rels_tree = ET.parse(io.BytesIO(zf.read(rels_path)))
        else:
            rels_tree = ET.ElementTree(
                ET.Element("Relationships", xmlns="http://schemas.openxmlformats.org/package/2006/relationships")
            )

        rels_root = rels_tree.getroot()
        next_rid = 1 + max(
            (
                int(rel.attrib["Id"][3:])
                for rel in rels_root.findall("Relationship")
                if rel.attrib["Id"].startswith("rId")
            ),
            default=0,
        )
        ET.SubElement(
            rels_root,
            "Relationship",
            {
                "Id": f"rId{next_rid}",
                "Type": "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject",
                "Target": f"embeddings/{embed_name}",
            },
        )
        rels_out = io.BytesIO()
        rels_tree.write(rels_out, xml_declaration=True, encoding="utf-8")
        zf.writestr(rels_path, rels_out.getvalue())

        # 4. Update xl/workbook.xml (refer to object but not draw it)
        wb_path = "xl/workbook.xml"
        wb_tree = ET.parse(io.BytesIO(zf.read(wb_path)))
        wb_root = wb_tree.getroot()
        ns_wb = {"": "http://schemas.openxmlformats.org/spreadsheetml/2006/main"}
        ET.register_namespace("", ns_wb[""])

        # Add dummy oleObjects section if it doesn't exist
        ole_tag = ET.Element("oleObjects")
        ole_object = ET.SubElement(
            ole_tag,
            "oleObject",
            {
                "progId": "Package",
                "r:id": f"rId{next_rid}",
                "name": object_name,
                "dvAspect": "DVASPECT_CONTENT",
                "objectId": "_123456",
            },
        )
        wb_root.append(ole_tag)

        wb_out = io.BytesIO()
        wb_tree.write(wb_out, xml_declaration=True, encoding="utf-8")
        zf.writestr(wb_path, wb_out.getvalue())

    print(f"✅ Successfully embedded file {file_to_embed} invisibly in {base_xlsx}")


import pandas as pd

pd.DataFrame({"A": [1, 2, 3], "B": [4, 5, 6]}).to_excel("test1.xlsx", index=False)
# embed_file_as_invisible_ole("test1.xlsx", "test.xml")

import shutil
import subprocess

shutil.copyfile("test1.xlsx", "test1.zip")
subprocess.run(["explorer", "test1.xlsx"])
subprocess.run(["explorer", "test1.zip"])
