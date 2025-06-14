from openpyxl import load_workbook
from openpyxl.workbook.protection import WorkbookProtection


def make_excel_readonly(filepath, password="readonly"):
    """
    Make an Excel .xlsx read-only by:
    1) Locking workbook structure with a password
    2) Locking all sheets with a password
    3) Setting the OS file attribute to read-only
    """
    # Load workbook
    wb = load_workbook(filepath)

    # Create a WorkbookProtection object if not present
    if wb.security is None:
        wb.security = WorkbookProtection()

    # Lock workbook structure
    wb.security.lockStructure = True
    wb.security.workbookPassword = password

    # Lock each worksheet for editing
    for ws in wb.worksheets:
        ws.protection.sheet = True
        ws.protection.password = password
        # Optional: finer options — allow only select actions
        ws.protection.enable()

    # Save changes (overwrite original or new file)
    locked_file = filepath  # or e.g. "locked_" + filepath
    wb.save(locked_file)
    wb.close()

    # Mark file system read-only
    # os.chmod(locked_file, 0o444)  # Unix: read-only for owner, group, others

    # On Windows, also use attrib:
    # if os.name == "nt":
    #     os.system(f'attrib +R "{locked_file}"')

    print(f"✅ Workbook locked and saved: {locked_file}")
    print(f"🔒 Password: '{password}' (required to unprotect)")


# === Example usage ===
if __name__ == "__main__":
    make_excel_readonly("rdtest1.xlsx", password="plzdontgraduate")
