import enum, dataclasses

from gened_data import GenEds, GenEdStructure


class CourseKind(enum.Enum):
    DEGREE = "Degree"
    GENED = "GenEd"
    GENED_STUB = "GenEdStub"
    ELECTIVE = "Elective"
    ELECTIVE_STUB = "ElectiveStub"


@dataclasses.dataclass
class Course:
    kind: CourseKind
    name: str
    credit: int
    code: str | None = None
    url: str | None = None
    info: GenEds | None = None
