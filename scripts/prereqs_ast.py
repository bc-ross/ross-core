import ast
import itertools
import pathlib
import re
import subprocess

PATH = pathlib.Path(__file__).parent.parent.joinpath("resources", "course_reqs")

BASE_FILENAME = "mod.rs"

BASE_PREAMBLE = """
use crate::prereqs::CourseReq;
use crate::schedule::CourseCode;
use std::collections::HashMap;
use std::iter::empty;

"""

BASE_MIDAMBLE = "\npub fn prereqs() -> HashMap<CourseCode, CourseReq> { empty()"

BASE_POSTAMBLE = ".collect()}"

PREAMBLE = """
#![allow(unused_imports)]

use crate::prereqs::{
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};

pub fn prereqs() -> Vec<(CourseCode, CourseReq)> { vec![
"""

POSTAMBLE = "]}"


def strip_quotes(s):
    if isinstance(s, str) and s.startswith('"') and s.endswith('"'):
        return s[1:-1]
    return s


def rustify(node, default_stem, in_tuple=False, co_prefix=False):
    # Helper to wrap with PreCourse/CoCourse or their Grade variants
    def wrap_course(stem, code, grade=None, co_prefix=False):
        cc = f'CC!("{stem}", {code})'
        if grade is not None:
            gr = grade if grade.startswith("GR!") else f"GR!({grade})"
            return f"{'CoCourseGrade' if co_prefix else 'PreCourseGrade'}({cc}, {gr})"
        else:
            return f"{'CoCourse' if co_prefix else 'PreCourse'}({cc})"

    if isinstance(node, ast.Call):
        func = node.func.id
        args = node.args
        if func == "And" or func == "Or":
            sub = [rustify(arg, default_stem) for arg in args]
            return f"{func}(vec![{', '.join(sub)}])"
        if func == "Co":
            # Co(...) just sets co_prefix for the inner logic
            return rustify(ast.Tuple(elts=args, ctx=ast.Load()), default_stem, co_prefix=True)
        if func == "Prog":
            return f'Program("{rustify(args[0], default_stem)}")'
        if func == "GR":
            return f"GR!({', '.join(rustify(arg, default_stem) for arg in args)})"
    elif isinstance(node, ast.Tuple):
        elts = node.elts
        if len(elts) == 3:
            # (STEM, CODE, GRADE)
            stem = strip_quotes(rustify(elts[0], default_stem, in_tuple=True))
            code = rustify(elts[1], default_stem, in_tuple=True)
            grade = rustify(elts[2], default_stem, in_tuple=True)
            return wrap_course(stem, code, grade, co_prefix)
        elif len(elts) == 2:
            # (STEM, CODE) or (CODE, GRADE)
            if isinstance(elts[0], ast.Constant) and isinstance(elts[1], ast.Constant):
                # (CODE, GRADE) if second is a grade string
                if isinstance(elts[1].value, str) and re.fullmatch(r"[A-DF][+-]?", elts[1].value):
                    code = rustify(elts[0], default_stem, in_tuple=True)
                    grade = rustify(elts[1], default_stem, in_tuple=True)
                    return wrap_course(default_stem, code, grade, co_prefix)
                else:
                    # (STEM, CODE) where code can be string or int
                    stem = strip_quotes(rustify(elts[0], default_stem, in_tuple=True))
                    code = rustify(elts[1], default_stem, in_tuple=True)
                    return wrap_course(stem, code, None, co_prefix)
            elif isinstance(elts[1], ast.Call) and getattr(elts[1].func, "id", None) == "GR":
                code = rustify(elts[0], default_stem, in_tuple=True)
                grade = rustify(elts[1], default_stem, in_tuple=True)
                return wrap_course(default_stem, code, grade, co_prefix)
            else:
                stem = strip_quotes(rustify(elts[0], default_stem, in_tuple=True))
                code = rustify(elts[1], default_stem, in_tuple=True)
                return wrap_course(stem, code, None, co_prefix)
        elif len(elts) == 1:
            # (CODE,)
            code = rustify(elts[0], default_stem, in_tuple=True)
            return wrap_course(default_stem, code, None, co_prefix)
        else:
            raise ValueError(f"Unsupported tuple length: {len(elts)} in {ast.dump(node)}")
    elif isinstance(node, ast.Constant):
        if isinstance(node.value, int):
            if in_tuple:
                return str(node.value)
            else:
                return wrap_course(default_stem, node.value, None, co_prefix=co_prefix)
        elif isinstance(node.value, str):
            # If it looks like a grade, just return the string (parent will wrap in GR!)
            if re.fullmatch(r"[A-DF][+-]?", node.value):
                return node.value
            # If it looks like a course code (alphanumeric, length > 2), emit as string
            elif re.fullmatch(r"[A-Za-z0-9]{3,}", node.value):
                return f'"{node.value}"'
            else:
                return f'"{node.value}"'
    elif isinstance(node, ast.Name):
        if node.id in {"Instructor", "None"}:
            return node.id
        if in_tuple:
            return f'"{node.id}"'
        return wrap_course(default_stem, f'"{node.id}"', None, co_prefix=co_prefix)
    raise ValueError(f"Unsupported AST node: {ast.dump(node)}")


def parse_req(req, default_stem):
    expr = ast.parse(preprocess_grades(req), mode="eval").body
    return rustify(expr, default_stem)


def preprocess_grades(expr: str) -> str:
    # Add spaces to catch grades at the start/end
    expr = f" {expr} "
    expr = re.sub(r"(?<=[\s\(\[,])([A-DF][+-]?)(?=[\s\)\],])", r'"\1"', expr)
    return expr.strip()


class StopMeError(Exception):
    pass


# Example usage:
def main():
    # if PATH.exists():
    #     resp = input(f"File {PATH} exists. Overwrite? [Y/n] ").strip().lower()
    #     if resp != "y":
    #         print("Aborting.")
    #         return
    with open(PATH.joinpath(BASE_FILENAME), "w") as base_f:
        base_f.write(BASE_PREAMBLE)
        print("Rust CourseReq REPL. Type 'stem STEM' to set stem, 'exit' to quit.")
        try:
            stem = input("Stem? ")
            while True:
                with open(PATH.joinpath("TEMP.rs").with_stem(f"stem_{stem.lower()}"), "w") as f:
                    f.write(PREAMBLE)

                    try:
                        while True:
                            inp = input(f"[stem={stem}] code> ").strip()
                            if inp.lower() == "exit":
                                raise StopMeError
                            if inp.lower().startswith("stem "):
                                stem = inp.split()[1].upper()
                                break
                            # if not inp.isdigit():
                            #     print("Enter a course code (e.g., 1020) or 'stem STEM'.")
                            #     continue
                            code = inp
                            while True:
                                req = input(f"[stem={stem}] req for {code}> ").strip()
                                if not req:
                                    print("No requirement entered, skipping.")
                                    break
                                if stem:
                                    try:
                                        parsed_req = parse_req(req, stem)
                                    except SyntaxError as e:
                                        print(f"Error parsing requirement: {e}")
                                        continue
                                    print("Added course!")
                                    f.write(f'(\n    CC!("{stem}", {code}),\n    {parsed_req},\n),\n')
                                    break
                    finally:
                        f.write(POSTAMBLE)
        except StopMeError:
            pass
        finally:
            for i in PATH.glob("stem_*.rs"):
                base_f.write(f"mod stem_{i.stem[5:].lower()};\n")
            base_f.write(BASE_MIDAMBLE)
            for i in PATH.glob("stem_*.rs"):
                base_f.write(f".chain(stem_{i.stem[5:].lower()}::prereqs().into_iter())\n")
            base_f.write(BASE_POSTAMBLE)
    for i in itertools.chain(PATH.glob("stem_*.rs"), [PATH.joinpath(BASE_FILENAME)]):
        subprocess.run(["rustfmt", str(i)], check=True)


if __name__ == "__main__":
    main()
