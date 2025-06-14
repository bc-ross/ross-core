import io
import pathlib
import xml.etree.ElementTree as ET
import zipfile

import mutablezip

NS = {
    "main": "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
    "rel": "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
    "xdr": "http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing",
    "a": "http://schemas.openxmlformats.org/drawingml/2006/main",
}
for prefix, uri in NS.items():
    ET.register_namespace(prefix if prefix != "main" else "", uri)


def embed_ole_hidden(base_xlsx, file_path, sheet="xl/worksheets/sheet1.xml"):
    file_name = pathlib.Path(file_path).name
    embed_target = f"xl/embeddings/{file_name}"
    drawing_path = "xl/drawings/drawing1.xml"
    drawing_rels_path = "xl/drawings/_rels/drawing1.xml.rels"
    sheet_rels_path = sheet.replace("worksheets/", "worksheets/_rels/") + ".rels"

    with mutablezip.MutableZipFile(base_xlsx, "a", compression=zipfile.ZIP_DEFLATED) as zf:
        # 1. Embed file
        with open(file_path, "rb") as f:
            zf.writestr(embed_target, f.read())

        # 2. Update [Content_Types].xml
        ct_tree = ET.parse(io.BytesIO(zf.read("[Content_Types].xml")))
        ct_root = ct_tree.getroot()

        def add_override(part, ctype):
            if not any(el.attrib.get("PartName") == "/" + part for el in ct_root):
                ET.SubElement(ct_root, f"{{{NS['main']}}}Override", {"PartName": "/" + part, "ContentType": ctype})

        add_override(embed_target, "application/vnd.openxmlformats-officedocument.oleObject")
        add_override(drawing_path, "application/vnd.openxmlformats-officedocument.drawing+xml")
        add_override(drawing_rels_path, "application/vnd.openxmlformats-package.relationships+xml")

        out = io.BytesIO()
        ct_tree.write(out, encoding="utf-8", xml_declaration=True)
        zf.writestr("[Content_Types].xml", out.getvalue())

        # 3. Write drawing1.xml
        wsDr = ET.Element(f"{{{NS['xdr']}}}wsDr")
        twoCellAnchor = ET.SubElement(wsDr, f"{{{NS['xdr']}}}twoCellAnchor", editAs="oneCell")

        ET.SubElement(twoCellAnchor, f"{{{NS['xdr']}}}from").extend(
            [
                ET.Element(f"{{{NS['xdr']}}}col", text="0"),
                ET.Element(f"{{{NS['xdr']}}}colOff", text="0"),
                ET.Element(f"{{{NS['xdr']}}}row", text="0"),
                ET.Element(f"{{{NS['xdr']}}}rowOff", text="0"),
            ]
        )
        ET.SubElement(twoCellAnchor, f"{{{NS['xdr']}}}to").extend(
            [
                ET.Element(f"{{{NS['xdr']}}}col", text="2"),
                ET.Element(f"{{{NS['xdr']}}}colOff", text="0"),
                ET.Element(f"{{{NS['xdr']}}}row", text="4"),
                ET.Element(f"{{{NS['xdr']}}}rowOff", text="0"),
            ]
        )

        oleObject = ET.SubElement(
            twoCellAnchor,
            f"{{{NS['xdr']}}}oleObject",
            {"progId": "Package", "dvAspect": "DVASPECT_CONTENT", "objectId": "_13579", f"{{{NS['rel']}}}id": "rId1"},
        )
        ET.SubElement(oleObject, f"{{{NS['xdr']}}}objectPr", {"drawAspect": "content", "oleUpdate": "Embed"})

        sheet_drawing = io.BytesIO()
        ET.ElementTree(wsDr).write(sheet_drawing, encoding="utf-8", xml_declaration=True)
        zf.writestr(drawing_path, sheet_drawing.getvalue())

        # 4. Write drawing1.xml.rels
        rels = ET.Element("Relationships", xmlns=NS["rel"])
        ET.SubElement(
            rels,
            "Relationship",
            {
                "Id": "rId1",
                "Type": "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject",
                "Target": f"../embeddings/{file_name}",
            },
        )
        rels_out = io.BytesIO()
        ET.ElementTree(rels).write(rels_out, encoding="utf-8", xml_declaration=True)
        zf.writestr(drawing_rels_path, rels_out.getvalue())

        # 5. Link sheet1.xml to drawing1.xml
        sheet_tree = ET.parse(io.BytesIO(zf.read(sheet)))
        sheet_root = sheet_tree.getroot()

        if not sheet_root.find(f"{{{NS['main']}}}drawing"):
            ET.SubElement(sheet_root, f"{{{NS['main']}}}drawing", {f"{{{NS['rel']}}}id": "rId1000"})

        sheet_out = io.BytesIO()
        sheet_tree.write(sheet_out, encoding="utf-8", xml_declaration=True)
        zf.writestr(sheet, sheet_out.getvalue())

        # 6. Add relationship from sheet1.xml to drawing1.xml
        if sheet_rels_path in zf.namelist():
            sheet_rels_tree = ET.parse(io.BytesIO(zf.read(sheet_rels_path)))
        else:
            sheet_rels_tree = ET.ElementTree(ET.Element("Relationships", xmlns=NS["rel"]))

        sheet_rels_root = sheet_rels_tree.getroot()
        if not any(r.attrib.get("Target") == "../drawings/drawing1.xml" for r in sheet_rels_root):
            ET.SubElement(
                sheet_rels_root,
                "Relationship",
                {
                    "Id": "rId1000",
                    "Type": "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing",
                    "Target": "../drawings/drawing1.xml",
                },
            )

        out_rels = io.BytesIO()
        sheet_rels_tree.write(out_rels, encoding="utf-8", xml_declaration=True)
        zf.writestr(sheet_rels_path, out_rels.getvalue())

    print("✅ Embedded file as invisible, persistent OLE without Excel errors.")


import pandas as pd

pd.DataFrame({"A": [1, 2, 3], "B": [4, 5, 6]}).to_excel("test1.xlsx", index=False)
embed_ole_hidden("test1.xlsx", "test.xml")

import shutil
import subprocess

shutil.copyfile("test1.xlsx", "test1.zip")
subprocess.run(["explorer", "test1.xlsx"])
subprocess.run(["explorer", "test1.zip"])
