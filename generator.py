import pandas as pd
from lxml import etree
import pathlib
import pathvalidate
from data_structures import (
    ProgramStub,
    ProgramKind,
    DegreeCourse,
    GenEdCourse,
    GenEds,
    GenEdStub,
    ElectiveCourse,
)
from xml_structures import Course, CourseKind

MIN_REQD_CREDITS = 128


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


class Program:
    def __init__(self, name: str):
        file_path = pathlib.Path("scraped_programs").joinpath(
            pathvalidate.sanitize_filename(name).replace(" ", "_") + ".xml"
        )
        with open(file_path, "r", encoding="utf-8") as file:
            self.df = parse_and_convert_xml(file.read(), "semester")

    def get_courses(self) -> pd.DataFrame:
        return self.df.copy()

    def validate_plan(self, course_df):
        # Placeholder for validation logic
        return None


def filter_by_kind(df: pd.DataFrame, *kinds: CourseKind) -> pd.DataFrame:
    """Filters the DataFrame by the specified course kinds."""
    semesters_list = []
    for col in df.columns.get_level_values(0).unique():
        mask = df[(col, "kind")].isin(kinds)
        semester_cols = df.loc[:, col]
        semesters_list.append(semester_cols.where(mask))
    return pd.concat(semesters_list, axis=1)


def filter_to_list(df: pd.DataFrame, *kinds: CourseKind) -> pd.DataFrame:
    """Filters the DataFrame by the specified course kinds."""
    semesters_list = []
    for col in df.columns.get_level_values(0).unique():
        mask = df[(col, "kind")].isin(kinds)
        semester_cols = df.loc[:, col]
        semesters_list.append(semester_cols.where(mask))
    return pd.concat(semesters_list, ignore_index=True).dropna(how="all")


class CourseSequence:
    def __init__(self, program_names: list[str] | None = None):
        self.programs = []
        dfs_list = []

        if program_names:
            for name in program_names:
                program = Program(name)
                self.programs.append(program)
                dfs_list.append(program.get_courses())

        self.df = pd.concat(dfs_list, ignore_index=True) if dfs_list else pd.DataFrame()

        gened_file_path = pathlib.Path("scraped_programs").joinpath("General_Education.xml")
        with open(gened_file_path, "r", encoding="utf-8") as file:
            self.gened_eles = parse_and_convert_xml(file.read(), "gened")

    def validate(self):
        return all(prog.validate_plan(self.df) for prog in self.programs) and self.gened_validate()

    def gened_validate(self):
        if int(self.df.loc[:, (slice(None), "credit")].sum().sum()) < MIN_REQD_CREDITS:
            return False

        gened_dict = {}
        gened_df = filter_to_list(self.df, CourseKind.GENED_STUB, CourseKind.GENED)
        degree_df = filter_to_list(self.df, CourseKind.DEGREE, CourseKind.ELECTIVE)
        for course_row in gened_df.itertuples(index=False):
            gened_dict[course_row.info] = (
                course_row.credit if GenEds[course_row.info].value.ReqdIsCredit else 1
            ) + gened_dict.get(course_row.info, 0)
        for gened in GenEds:
            if gened.value.Reqd > gened_dict.get(gened.name, 0):
                for item in (
                    degree_df[degree_df["code"].isin(self.gened_eles[(gened.name, "code")])]
                    .dropna(how="all")
                    .itertuples(index=False)
                ):
                    gened_dict[gened.name] = (item.credit if gened.value.ReqdIsCredit else 1) + gened_dict.get(
                        gened.name, 0
                    )
                if gened.value.Reqd > gened_dict.get(gened.name, 0):
                    return False  # TODO: add logging

        return True  # TODO: is all checks done? Foundations etc.?
