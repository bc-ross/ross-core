import enum


class ProgramKind(enum.Enum):
    Bachelor = "Bachelor"
    Minor = "Minor"

    def __repr__(self):
        return f"{self.__class__.__name__}.{self.name}"


class ProgramStub:
    def __init__(self, name, kind, url, is_spec=False, degree=None):
        self.name = name
        self.kind = kind
        self.url = url
        self.is_spec = is_spec

    def __repr__(self):
        return f"ProgramStub(name={repr(self.name)}, kind={repr(self.kind)}, url={repr(self.url)}, is_spec={repr(self.is_spec)})"
