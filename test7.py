import glob
import pathlib

import pandas as pd
from lxml import etree
from openpyxl.workbook.protection import WorkbookProtection

from xml_structures import CourseKind


def parse_and_convert_xml(xml_string, root_tag):
    """Parses XML and converts it into a MultiIndex DataFrame."""
    root = etree.fromstring(xml_string)
    semester_data = {}

    for semester_elem in root.findall(root_tag):
        sem_name = semester_elem.attrib["name"]
        courses = []

        for course_elem in semester_elem.findall("course"):
            course_data = {field.tag: field.text for field in course_elem}
            course_data["kind"] = CourseKind(course_data["kind"])
            course_data["credit"] = int(course_data["credit"])
            courses.append(course_data)

        semester_data[sem_name] = courses

    # Align semester data to have consistent row structures
    max_rows = max(len(courses) for courses in semester_data.values())
    for sem in semester_data:
        courses = semester_data[sem]
        while len(courses) < max_rows:
            courses.append({})
        semester_data[sem] = courses

    # Create MultiIndex DataFrame
    dfs = []
    for sem, records in semester_data.items():
        df = pd.DataFrame(records)
        df.columns = pd.MultiIndex.from_product([[sem], df.columns])
        dfs.append(df)

    return pd.concat(dfs, axis=1)


def trim_titles(s: str) -> str:
    """Trim titles to 31 characters for Excel compatibility."""
    if len(s) > 31:
        return s[:31]
    return s


def main(filename, password):
    # Get the list of XML files
    xml_files = glob.glob("scraped_programs/*.xml")

    # Initialize a dictionary to store DataFrames
    dataframes = {}

    # Process each XML file
    for file in xml_files:
        file = pathlib.Path(file)
        if file.stem == "General_Education":
            root_tag = "gened"
        else:
            root_tag = "semester"
        with open(file, "r", encoding="utf-8") as f:
            xml_content = f.read()
        df = parse_and_convert_xml(xml_content, root_tag=root_tag)
        dataframes[trim_titles(file.stem)] = df

    # Create an Excel writer
    with pd.ExcelWriter(filename) as writer:
        # Write the main DataFrame to the "Schedule" sheet
        main_df = pd.concat(dataframes.values(), axis=1)
        main_df.to_excel(writer, sheet_name="Schedule")

        # Write each individual DataFrame to its own sheet
        for file, df in dataframes.items():
            sheet_name = file.split("/")[-1].replace(".xml", "")
            df.to_excel(writer, sheet_name=sheet_name)
        # Hide each individual sheet except the main "Schedule" sheet
        workbook = writer.book

        # Create a WorkbookProtection object if not present
        if workbook.security is None:
            workbook.security = WorkbookProtection()

        # Lock workbook structure
        workbook.security.lockStructure = True
        workbook.security.workbookPassword = password

        # # Save changes (overwrite original or new file)
        # locked_file = filepath  # or e.g. "locked_" + filepath
        # wb.save(locked_file)
        # wb.close()

        for sheet_name in writer.sheets:
            worksheet = writer.sheets[sheet_name]
            worksheet.protection.sheet = True
            worksheet.protection.password = password
            # Optional: finer options — allow only select actions
            worksheet.protection.enable()
            if sheet_name != "Schedule":
                worksheet.sheet_state = "hidden"


if __name__ == "__main__":
    main("test_hidden2.xlsx", "plzdontgraduate")
