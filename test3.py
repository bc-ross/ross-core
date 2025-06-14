import zipfile

import mutablezip

with mutablezip.MutableZipFile("my.xlsx", "a", compression=zipfile.ZIP_DEFLATED) as z:
    z.writestr("myCustomBin/myfile.zip", open("myfile.zip", "rb").read())

    # Update [Content_Types].xml
    ct = z.read("[Content_Types].xml").decode()
    if "myCustomBin" not in ct:
        ct = ct.replace(
            "</Types>", '<Override PartName="/myCustomBin/myfile.zip" ContentType="application/zip"/></Types>'
        )
        z.writestr("[Content_Types].xml", ct)
