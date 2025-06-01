from __future__ import annotations

import enum
import dataclasses


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


# https://stackoverflow.com/a/24105344
class TypesEnumMeta(enum.EnumMeta):
    def __call__(cls: GenEds, value, *args, **kw):
        if isinstance(value, str):
            for member in cls:  # pylint: disable=E1133
                if member.value.Name.lower() == value.lower():
                    value = member
                    break
                elif (
                    member.value.ShortName
                    and member.value.ShortName.lower() == value.lower()
                ):
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
    WELLNESS_FOR_LIFE = GenEdStructure(
        Name="Wellness for Life",
        Reqd=1,
        ReqdIsCredit=False,
        Type=GenEdType.Core,
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
        ShortName="Natural World",
        Url="/general-education/understanding-the-natural-world/",
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


if __name__ == "__main__":
    print(GenEds("Principles of Nature"))
