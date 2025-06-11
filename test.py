import glob
import io
import mimetypes
import pathlib
import xml.etree.ElementTree as ET
import zipfile

import mutablezip


def embed_file_structure(base_xlsx, files_to_embed):
    with mutablezip.MutableZipFile(base_xlsx, "a", compression=zipfile.ZIP_DEFLATED) as zf:
        # Load [Content_Types].xml
        tree = ET.parse(io.BytesIO(zf.read("[Content_Types].xml")))
        root = tree.getroot()
        ns = {"ct": "http://schemas.openxmlformats.org/package/2006/content-types"}
        ET.register_namespace("", ns["ct"])

        for idx, file_path in enumerate(files_to_embed):
            file_name = pathlib.Path(file_path).name
            embedded_path = f"xl/embeddings/{file_name}"

            # Embed file in xl/embeddings/
            with open(file_path, "rb") as f:
                zf.writestr(embedded_path, f.read())

            # Add correct content type
            mime_type, _ = mimetypes.guess_type(file_name)
            content_type = mime_type or "application/octet-stream"

            # Avoid duplicate <Override>
            override_exists = any(
                el.attrib.get("PartName") == "/" + embedded_path for el in root.findall("ct:Override", ns)
            )

            if not override_exists:
                ET.SubElement(
                    root,
                    "{http://schemas.openxmlformats.org/package/2006/content-types}Override",
                    {"PartName": "/" + embedded_path, "ContentType": content_type},
                )

            # Ensure xl/_rels/workbook.xml.rels exists
            rels_path = "xl/_rels/workbook.xml.rels"
            if rels_path in zf.namelist():
                rels_tree = ET.parse(io.BytesIO(zf.read(rels_path)))
            else:
                # Create new relationships file if missing
                rels_root = ET.Element(
                    "Relationships", xmlns="http://schemas.openxmlformats.org/package/2006/relationships"
                )
                rels_tree = ET.ElementTree(rels_root)

            rels_root = rels_tree.getroot()
            # Avoid duplicate relationship
            already_linked = any(
                rel.attrib.get("Target") == f"/embeddings/{file_name}" for rel in rels_root.findall("Relationship")
            )
            if not already_linked:
                ET.SubElement(
                    rels_root,
                    "Relationship",
                    {
                        "Id": f"rId_embed_{idx}",
                        "Type": "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package",
                        "Target": f"/embeddings/{file_name}",
                    },
                )
            # Write back the relationships
            rels_out = io.BytesIO()
            rels_tree.write(rels_out, xml_declaration=True, encoding="utf-8")
            zf.writestr(rels_path, rels_out.getvalue())

        # Save updated [Content_Types].xml
        outf = io.BytesIO()
        tree.write(outf, xml_declaration=True, encoding="utf-8")
        zf.writestr("[Content_Types].xml", outf.getvalue())

    print(f"✅ Success: Files embedded invisibly in {base_xlsx}")


embed_file_structure("clean multi.xlsx", glob.glob("scraped_programs/*.xml"))
