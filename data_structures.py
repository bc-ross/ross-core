import enum
import dataclasses
from gened_data import GenEds, GENERIC_ELECTIVE_NAMES, DefaultGenEdCodes


class ProgramKind(enum.Enum):
    Bachelor = "Bachelor"
    Minor = "Minor"

    def __repr__(self):
        return f"{self.__class__.__name__}.{self.name}"


@dataclasses.dataclass
class ProgramStub:
    name: str
    kind: str
    url: str
    is_spec: bool = False


@dataclasses.dataclass
class DegreeCourse:
    name: str
    code: str
    credit: int
    url: str

    def __str__(self):
        return self.code


@dataclasses.dataclass
class GenEdStub:
    name: str
    info: GenEds
    credit: int

    def __str__(self):
        return "GENED: " + self.name


@dataclasses.dataclass
class GenEdCourse:
    name: str
    code: str
    info: GenEds
    credit: int
    url: str

    def __str__(self):
        return "GENED: " + self.code + f" ({self.name})"


@dataclasses.dataclass
class ElectiveStub:
    name: str
    credit: int

    def __str__(self):
        return "ELE: " + self.name


@dataclasses.dataclass
class ElectiveCourse:
    name: str
    code: str
    stub: ElectiveStub
    credit: int
    url: str

    def __str__(self):
        return "ELE: " + self.code + f" ({self.name})"
