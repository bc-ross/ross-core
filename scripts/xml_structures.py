from __future__ import annotations

import dataclasses
import enum

from gened_data import GenEds


class CourseKind(enum.Enum):
    DEGREE = "Degree"
    GENED = "GenEd"
    GENED_STUB = "GenEdStub"
    ELECTIVE = "Elective"
    ELECTIVE_STUB = "ElectiveStub"

    def __str__(self):
        return self.value


@dataclasses.dataclass
class Course:
    kind: CourseKind
    name: str
    credit: int
    code: str | None = None
    url: str | None = None
    info: GenEds | None = None
