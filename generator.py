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


def xml_to_multiindex_df(xml_string):
    root = etree.fromstring(xml_string)

    semester_data = {}  # key: semester, value: list of dicts (courses)

    for semester_elem in root.findall("semester"):
        sem_name = semester_elem.attrib["name"]
        courses = []

        for course_elem in semester_elem.findall("course"):
            course_data = {}
            for field in course_elem:
                course_data[field.tag] = field.text
            course_data["kind"] = CourseKind(course_data["kind"])
            course_data["credit"] = int(course_data["credit"])
            courses.append(course_data)

        semester_data[sem_name] = courses

    # Align courses per semester into a consistent row structure
    max_rows = max(len(courses) for courses in semester_data.values())

    # Pad shorter lists with empty dicts so all semesters have the same row count
    for sem in semester_data:
        courses = semester_data[sem]
        while len(courses) < max_rows:
            courses.append({})
        semester_data[sem] = courses

    # Create DataFrame per semester, then concatenate along columns
    dfs = []
    for sem, records in semester_data.items():
        df = pd.DataFrame(records)
        df.columns = pd.MultiIndex.from_product([[sem], df.columns])
        dfs.append(df)

    final_df = pd.concat(dfs, axis=1)
    return final_df


def xml_to_gened_df(xml_string):
    root = etree.fromstring(xml_string)

    semester_data = {}  # key: semester, value: list of dicts (courses)

    for semester_elem in root.findall("gened"):
        sem_name = semester_elem.attrib["name"]
        courses = []

        for course_elem in semester_elem.findall("course"):
            course_data = {}
            for field in course_elem:
                course_data[field.tag] = field.text
            course_data["kind"] = CourseKind(course_data["kind"])
            course_data["credit"] = int(course_data["credit"])
            courses.append(course_data)

        semester_data[sem_name] = courses

    # Align courses per semester into a consistent row structure
    max_rows = max(len(courses) for courses in semester_data.values())

    # Pad shorter lists with empty dicts so all semesters have the same row count
    for sem in semester_data:
        courses = semester_data[sem]
        while len(courses) < max_rows:
            courses.append({})
        semester_data[sem] = courses

    # Create DataFrame per semester, then concatenate along columns
    dfs = []
    for sem, records in semester_data.items():
        df = pd.DataFrame(records)
        df.columns = pd.MultiIndex.from_product([[sem], df.columns])
        dfs.append(df)

    final_df = pd.concat(dfs, axis=1)
    return final_df


class Program:
    def __init__(self, name: str):
        with open(  # FIXME should be current_programs
            pathlib.Path("scraped_programs").joinpath(pathvalidate.sanitize_filename(name).replace(" ", "_") + ".xml"),
            "r",
            encoding="utf-8",
        ) as file:
            self.df = xml_to_multiindex_df(file.read())

    def get_courses(self) -> pd.DataFrame:
        return self.df.copy()

    def validate_plan(self, course_df):
        return None  # TODO


def filter_by_kind(df: pd.DataFrame, *kinds: CourseKind) -> pd.DataFrame:
    """Filters the DataFrame by the specified course kind."""
    semesters_list = []
    for col in df.columns.get_level_values(0).unique():
        mask = df[(col, "kind")].isin(kinds)
        semester_cols = df.loc[:, col]
        semesters_list.append(semester_cols.where(mask))
    return pd.concat(semesters_list, axis=1)


def filter_to_list(df: pd.DataFrame, *kinds: CourseKind) -> pd.DataFrame:
    """Filters the DataFrame by the specified course kind."""
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
            for i in program_names:
                prog = Program(i)
                self.programs.append(prog)
                dfs_list.append(prog.get_courses())
        self.df = pd.concat(dfs_list, ignore_index=True) if dfs_list else pd.DataFrame()
        with open(
            pathlib.Path("scraped_programs").joinpath("General_Education.xml"),  # FIXME should be current_programs
            "r",
            encoding="utf-8",
        ) as file:
            self.gened_eles = xml_to_gened_df(file.read())

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
