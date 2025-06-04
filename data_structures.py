import dataclasses
import enum

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


class ClassBase:
    pass


class CourseBase(ClassBase):
    pass


class StubBase(ClassBase):
    pass


class DegreeBase(ClassBase):
    pass


class ElectiveBase(ClassBase):
    pass


class GenEdBase(ClassBase):
    pass


@dataclasses.dataclass
class DegreeCourse(CourseBase, DegreeBase):
    # kind: CourseType = dataclasses.field(init=False, default=CourseType.DEGREE) # FIXME add this logic, maybe?
    name: str
    code: str
    credit: int
    url: str

    def __str__(self):
        return self.code


@dataclasses.dataclass
class GenEdStub(StubBase, GenEdBase):
    name: str
    info: GenEds
    credit: int

    def __str__(self):
        return "GENED: " + self.name


@dataclasses.dataclass
class GenEdCourse(CourseBase, GenEdBase):
    name: str
    code: str
    info: GenEds
    credit: int
    url: str

    def __str__(self):
        return "GENED: " + self.code + f" ({self.info.name})"


@dataclasses.dataclass
class ElectiveStub(StubBase, ElectiveBase):
    name: str
    credit: int

    def __str__(self):
        return "ELE: " + self.name


@dataclasses.dataclass
class ElectiveCourse(CourseBase, ElectiveBase):
    name: str
    code: str
    stub: ElectiveStub
    credit: int
    url: str

    def __str__(self):
        return "ELE: " + self.code + f" ({self.stub.name})"
