from __future__ import annotations

import enum
import dataclasses

GENERIC_ELECTIVE_NAMES = [
    "General Elective (or second major class)",
    "Elective",
    "Electives",
    "Electives/Minor",
    "Electives or Foundation",
    "Electives (depending on internship)",
]


class DefaultGenEdCodes(enum.Enum):
    ENGLISH_COMPOSITION = ["ENGL-1000", "ENGL-1010", "ENGL-1030"]
    INTRO_THEOLOGY = ["THEO-1100"]
    WELLNESS_FOR_LIFE = ["EXSC-1115", "NURS-3200"]
    NATURAL_PHILOSOPHY = ["PHIL-2100", "PHIL-2310"]
    FOREIGN_LANGUAGE = ["LATN-1000", "LATN-1020"]


class GenEdType(enum.Enum):
    Foundation = "Foundation"
    Core = "Core"  # Treating Fitness as part of the Core
    SkillsPerspectives = "Skills and Perspectives"


@dataclasses.dataclass
class GenEdStructure:
    Name: str
    Reqd: int
    ReqdIsCredit: bool
    Type: GenEdType
    ShortName: str | None = None
    Url: str | None = None


def ident_gened(member: GenEdStructure, value: str) -> GenEdStructure:
    if member.value.Name.lower() == value.lower():
        return True
    elif member.value.ShortName and member.value.ShortName.lower() == value.lower():
        return True

    if value.endswith(" Foundation"):
        return ident_gened(member, value[:-11])
    return False


# https://stackoverflow.com/a/24105344
class TypesEnumMeta(enum.EnumMeta):
    def __call__(cls: GenEds, value, *args, **kw):
        if isinstance(value, str):
            for member in cls:  # pylint: disable=E1133
                if ident_gened(member, value):
                    value = member
                    break
        return super().__call__(value, *args, **kw)


class GenEds(enum.Enum, metaclass=TypesEnumMeta):
    WRITTEN_COMMUNICATION = GenEdStructure(
        Name="Written Communication",
        Reqd=2,
        ReqdIsCredit=False,
        Type=GenEdType.SkillsPerspectives,
        Url="/general-education/written-communication/",
    )
    INTRO_THEOLOGY = GenEdStructure(
        Name="Introduction to Theology",
        Reqd=1,
        ReqdIsCredit=False,
        Type=GenEdType.Core,
    )
    ENGLISH_COMPOSITION = GenEdStructure(
        Name="English Composition",
        Reqd=1,
        ReqdIsCredit=False,
        Type=GenEdType.Core,
    )
    WELLNESS_FOR_LIFE = GenEdStructure(
        Name="Wellness for Life",
        Reqd=1,
        ReqdIsCredit=False,
        Type=GenEdType.Core,
    )
    EXERCISE_FITNESS = GenEdStructure(
        Name="Exercise Fitness Course",
        Reqd=1,
        ReqdIsCredit=False,
        Type=GenEdType.Core,
        ShortName="EXSC Fitness Course",
    )
    NATURAL_PHILOSOPHY = GenEdStructure(
        Name="Principles of Nature",
        Reqd=1,
        ReqdIsCredit=False,
        Type=GenEdType.Core,
    )
    FOREIGN_LANGUAGE = GenEdStructure(
        Name="Foreign Language",
        Reqd=8,
        ReqdIsCredit=True,
        Type=GenEdType.Core,
    )
    AESTHETIC_EXPERIENCE = GenEdStructure(
        Name="Aesthetic Experience",
        Reqd=6,
        ReqdIsCredit=True,
        Type=GenEdType.Foundation,
        Url="/general-education/aesthetic-experience/",
        ShortName="Aesthetic",
    )
    FAITH = GenEdStructure(
        Name="Faith",
        Reqd=6,
        ReqdIsCredit=True,
        Type=GenEdType.Foundation,
        Url="/general-education/faith/",
    )
    GLOBAL_PERSPECTIVE = GenEdStructure(
        Name="Global Perspective",
        Reqd=1,
        ReqdIsCredit=False,
        Type=GenEdType.SkillsPerspectives,
        Url="/general-education/global-perspective/",
    )
    HISTORICAL_INQUIRY = GenEdStructure(
        Name="Historical Inquiry",
        Reqd=6,
        ReqdIsCredit=True,
        Type=GenEdType.Foundation,
        Url="/general-education/historical-inquiry/",
        ShortName="Historical",
    )
    MATHEMATICAL_REASONING = GenEdStructure(
        Name="Mathematical Reasoning",
        Reqd=3,
        ReqdIsCredit=True,
        Type=GenEdType.Foundation,
        Url="/general-education/mathematical-reasoning/",
    )
    ORAL_COMMUNICATION = GenEdStructure(
        Name="Oral Communication",
        Reqd=1,
        ReqdIsCredit=False,
        Type=GenEdType.SkillsPerspectives,
        Url="/general-education/oral-communication/",
    )
    PERSON_AND_COMMUNITY = GenEdStructure(
        Name="Person and Community in the Contemporary World",
        Reqd=3,
        ReqdIsCredit=True,
        Type=GenEdType.Foundation,
        ShortName="Person and Community",
        Url="/general-education/person-community-contemporary-world/",
    )
    PHILOSOPHICAL_INQUIRY = GenEdStructure(
        Name="Philosophical Inquiry",
        Reqd=6,
        ReqdIsCredit=True,
        Type=GenEdType.Foundation,
        Url="/general-education/philosophical-inquiry/",
    )
    SCIENTIFIC_METHOD = GenEdStructure(
        Name="Scientific Method",
        Reqd=1,
        ReqdIsCredit=False,
        Type=GenEdType.SkillsPerspectives,
        Url="/general-education/scientific-method/",
    )
    UNDERSTANDING_NATURAL_WORLD = GenEdStructure(  # NOTE: TWO DISCIPLINES
        Name="Understanding the Natural World",
        Reqd=7,
        ReqdIsCredit=True,
        Type=GenEdType.Foundation,
        ShortName="Natural World",  # FIXME Foundation(with lab) breaks
        Url="/general-education/understanding-natural-world/",
    )
    VISUAL_COMMUNICATION = GenEdStructure(
        Name="Visual Communication",
        Reqd=1,
        ReqdIsCredit=False,
        Type=GenEdType.SkillsPerspectives,
        Url="/general-education/visual-communication/",
    )
    WESTERN_PERSPECTIVE = GenEdStructure(
        Name="Western Perspective",
        Reqd=1,
        ReqdIsCredit=False,
        Type=GenEdType.SkillsPerspectives,
        Url="/general-education/western-perspective/",
    )

    def __str__(self):
        return self.name


if __name__ == "__main__":
    print(GenEds("Principles of Nature"))
