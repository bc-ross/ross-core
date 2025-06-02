import pandas as pd
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

MIN_REQD_CREDITS = 128


class Program:
    def __init__(self, name: str):
        self.df = pd.read_pickle(
            pathlib.Path("current_programs").joinpath(pathvalidate.sanitize_filename(name).replace(" ", "_") + ".pkl")
        )

    def get_courses(self) -> pd.DataFrame:
        return self.df.copy()

    def validate_plan(self, course_df):
        return None  # TODO


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
        self.gened_eles = pd.read_pickle(pathlib.Path("current_programs").joinpath("General_Education.pkl"))

    def validate(self):
        return all(prog.validate_plan(self.df) for prog in self.programs) and self.gened_validate()

    def gened_validate(self):
        if self.df.map(lambda x: x.credit if not pd.isna(x) else 0).sum().sum() < MIN_REQD_CREDITS:
            return False

        gened_dict = {}
        gened_df = self.df[self.df.map(lambda x: isinstance(x, GenEdStub) or isinstance(x, GenEdCourse))]
        degree_df = self.df[self.df.map(lambda x: isinstance(x, DegreeCourse) or isinstance(x, ElectiveCourse))]
        # return degree_df  # HACK
        for item in gened_df.stack().dropna():
            gened_dict[item.info.name] = (item.credit if item.info.value.ReqdIsCredit else 1) + gened_dict.get(
                item.info.name, 0
            )
        for gened in GenEds:
            if gened.value.Reqd > gened_dict.get(gened.name, 0):
                gened_codes = set(obj.code for obj in self.gened_eles[gened.name].dropna())
                for item in (
                    degree_df[degree_df.map(lambda obj: obj.code in gened_codes, na_action="ignore")].stack().dropna()
                ):
                    gened_dict[gened.name] = (item.credit if gened.value.ReqdIsCredit else 1) + gened_dict.get(
                        gened.name, 0
                    )
                if gened.value.Reqd > gened_dict.get(gened.name, 0):
                    return False  # TODO: add logging

        return True  # TODO: is all checks done? Foundations etc.?
