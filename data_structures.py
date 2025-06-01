import enum
import dataclasses
from gened_data import GenEds


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


@dataclasses.dataclass
class GenEdCourse:
    name: str
    credit: str


@dataclasses.dataclass
class ElectiveCourse:
    name: str
    credit: str
