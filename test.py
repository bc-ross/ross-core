import io
import pathlib
import xml.etree.ElementTree as ET
import zipfile

import mutablezip


def embed_hidden_ole(base_xlsx, file_path, sheet="xl/worksheets/sheet1.xml"):
    file_name = pathlib.Path(file_path).name
    embed_path = f"xl/embeddings/{file_name}"

    with mutablezip.MutableZipFile(base_xlsx, "a", compression=zipfile.ZIP_DEFLATED) as zf:
        # 1. Embed file
        with open(file_path, "rb") as f:
            zf.writestr(embed_path, f.read())

        # 2. Update [Content_Types].xml
        ct_tree = ET.parse(io.BytesIO(zf.read("[Content_Types].xml")))
        ct_root = ct_tree.getroot()
        ns_ct = "http://schemas.openxmlformats.org/package/2006/content-types"
        ET.register_namespace("", ns_ct)
        if not any(e.attrib.get("PartName") == "/" + embed_path for e in ct_root.findall(f"{{{ns_ct}}}Override")):
            ET.SubElement(
                ct_root,
                f"{{{ns_ct}}}Override",
                {
                    "PartName": "/" + embed_path,
                    "ContentType": "application/vnd.openxmlformats-officedocument.oleObject",
                },
            )
        out = io.BytesIO()
        ct_tree.write(out, encoding="utf-8", xml_declaration=True)
        zf.writestr("[Content_Types].xml", out.getvalue())

        # 3. Update sheet1.xml.rels to link to the embedded file
        rels_path = sheet.replace("worksheets/", "worksheets/_rels/") + ".rels"
        if rels_path in zf.namelist():
            rels_tree = ET.parse(io.BytesIO(zf.read(rels_path)))
        else:
            rels_tree = ET.ElementTree(
                ET.Element("Relationships", xmlns="http://schemas.openxmlformats.org/package/2006/relationships")
            )
        rels_root = rels_tree.getroot()

        next_rid = 1 + max(
            [int(r.attrib["Id"][3:]) for r in rels_root.findall("Relationship") if r.attrib["Id"].startswith("rId")]
            or [0]
        )
        rel_id = f"rId{next_rid}"
        ET.SubElement(
            rels_root,
            "Relationship",
            {
                "Id": rel_id,
                "Type": "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject",
                "Target": f"../embeddings/{file_name}",
            },
        )
        rels_out = io.BytesIO()
        rels_tree.write(rels_out, encoding="utf-8", xml_declaration=True)
        zf.writestr(rels_path, rels_out.getvalue())

        # 4. Update sheet1.xml to contain an invisible oleObject
        sheet_tree = ET.parse(io.BytesIO(zf.read(sheet)))
        sheet_root = sheet_tree.getroot()
        ns_main = "http://schemas.openxmlformats.org/spreadsheetml/2006/main"
        ns_r = "http://schemas.openxmlformats.org/officeDocument/2006/relationships"
        ET.register_namespace("", ns_main)
        ET.register_namespace("r", ns_r)

        ole_objects = sheet_root.find(f".//{{{ns_main}}}oleObjects")
        if ole_objects is None:
            ole_objects = ET.Element(f"{{{ns_main}}}oleObjects")
            sheet_root.append(ole_objects)

        ET.SubElement(
            ole_objects,
            f"{{{ns_main}}}oleObject",
            {
                "progId": "Package",
                f"{{{ns_r}}}id": rel_id,
                "dvAspect": "DVASPECT_CONTENT",
                "objectId": "_123456",
                "link": "false",
                "oleUpdate": "Embed",
            },
        )

        sheet_out = io.BytesIO()
        sheet_tree.write(sheet_out, encoding="utf-8", xml_declaration=True)
        zf.writestr(sheet, sheet_out.getvalue())

    print(f"✅ Embedded {file_name} invisibly in {base_xlsx}")


import pandas as pd

pd.DataFrame({"A": [1, 2, 3], "B": [4, 5, 6]}).to_excel("test1.xlsx", index=False)
embed_hidden_ole("test1.xlsx", "test.xml")

import shutil
import subprocess

shutil.copyfile("test1.xlsx", "test1.zip")
subprocess.run(["explorer", "test1.xlsx"])
subprocess.run(["explorer", "test1.zip"])
