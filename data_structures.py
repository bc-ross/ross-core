import enum
import dataclasses
from gened_data import GenEds


class ProgramKind(enum.Enum):
    Bachelor = "Bachelor"
    Minor = "Minor"

    def __repr__(self):
        return f"{self.__class__.__name__}.{self.name}"


# @dataclasses.data
class ProgramStub:
    def __init__(self, name, kind, url, is_spec=False, degree=None):
        self.name = name
        self.kind = kind
        self.url = url
        self.is_spec = is_spec

    def __repr__(self):
        return f"ProgramStub(name={repr(self.name)}, kind={repr(self.kind)}, url={repr(self.url)}, is_spec={repr(self.is_spec)})"


class DegreeCourse:
    def __init__(self, name, code, credit, url):
        self.name = name
        self.code = code
        self.credit = credit
        self.url = url

    def __repr__(self):
        return f"DegreeCourse(name={repr(self.name)}, code={repr(self.code)}, credit={repr(self.credit)}, url={repr(self.url)})"


class GenEdCourse:
    def __init__(self, name, credit):
        self.name = name
        self.credit = credit

    def __repr__(self):
        return f"GenEdCourse(name={repr(self.name)}, credit={repr(self.credit)})"


class ElectiveCourse:
    def __init__(self, name, credit):
        self.name = name
        self.credit = credit

    def __repr__(self):
        return f"ElectiveCourse(name={repr(self.name)}, credit={repr(self.credit)})"
