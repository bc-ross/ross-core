import io
import pathlib
import xml.etree.ElementTree as ET
import zipfile

import mutablezip


def embed_file_hidden_excel(base_xlsx, files_to_embed):
    with mutablezip.MutableZipFile(base_xlsx, "a", compression=zipfile.ZIP_DEFLATED) as zf:
        # Parse [Content_Types].xml
        tree = ET.parse(io.BytesIO(zf.read("[Content_Types].xml")))
        root = tree.getroot()
        ns = {"ct": "http://schemas.openxmlformats.org/package/2006/content-types"}
        ET.register_namespace("", ns["ct"])

        for idx, file_path in enumerate(files_to_embed, 1):
            file_name = pathlib.Path(file_path).name
            custom_part = f"customXml/item{idx}.bin"
            rels_part = f"customXml/_rels/item{idx}.xml.rels"
            xml_part = f"customXml/item{idx}.xml"

            # Write the raw file into customXml/itemX.bin
            with open(file_path, "rb") as f:
                zf.writestr(custom_part, f.read())
            zf.writestr(xml_part, b"<root/>")  # Placeholder

            # Write relationship file pointing to the .bin
            rels = ET.Element("Relationships", xmlns="http://schemas.openxmlformats.org/package/2006/relationships")
            ET.SubElement(
                rels,
                "Relationship",
                {
                    "Id": "rId1",
                    "Type": "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml",
                    "Target": f"item{idx}.bin",
                },
            )
            rels_tree = ET.ElementTree(rels)
            rels_out = io.BytesIO()
            rels_tree.write(rels_out, xml_declaration=True, encoding="utf-8")
            zf.writestr(rels_part, rels_out.getvalue())

            # Add <Override> for the parts
            for part_name, content_type in [
                (f"/{custom_part}", "application/octet-stream"),
                (f"/{xml_part}", "application/xml"),
                (f"/{rels_part}", "application/vnd.openxmlformats-package.relationships+xml"),
            ]:
                if not any(el.attrib.get("PartName") == part_name for el in root.findall("ct:Override", ns)):
                    ET.SubElement(
                        root,
                        "{http://schemas.openxmlformats.org/package/2006/content-types}Override",
                        {"PartName": part_name, "ContentType": content_type},
                    )

        # Save updated [Content_Types].xml
        outf = io.BytesIO()
        tree.write(outf, xml_declaration=True, encoding="utf-8")
        zf.writestr("[Content_Types].xml", outf.getvalue())

    print(f"✅ Embedded {len(files_to_embed)} file(s) invisibly in {base_xlsx}")


embed_file_hidden_excel("clean1.xlsx", ["test.xml"])
