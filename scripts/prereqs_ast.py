import ast
import pathlib
import re
import subprocess

PATH = pathlib.Path(__file__).parent.parent.joinpath("resources", "course_reqs.rs")

PREAMBLE = """
use crate::{GR, CC};
use crate::prereqs::{Grade, GradeLetter, GradeQualifier, CourseReq::{self, *}};
use crate::schedule::CourseCode;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! { pub static ref PREREQS_MAP: HashMap<CourseCode, CourseReq> = HashMap::from([
"""

POSTAMBLE = "]);}"


def rustify(node, default_stem, in_tuple=False, co_prefix=False):
    # Helper to wrap with PreCourse/CoCourse or their Grade variants
    def wrap_course(stem, code, grade=None):
        cc = f'CC!("{stem}", {code})'
        if grade is not None:
            gr = f"GR!({grade})"
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
            # Grade macro
            return ", ".join(str(ast.unparse(arg)).replace('"', "") for arg in args)
    elif isinstance(node, ast.Tuple):
        elts = node.elts
        if len(elts) == 3:
            # (STEM, CODE, GRADE)
            stem = rustify(elts[0], default_stem, in_tuple=True)
            code = rustify(elts[1], default_stem, in_tuple=True)
            grade = rustify(elts[2], default_stem, in_tuple=True)
            return wrap_course(stem, code, grade)
        elif len(elts) == 2:
            # (STEM, CODE) or (CODE, GRADE)
            if isinstance(elts[0], ast.Constant) and isinstance(elts[1], ast.Constant):
                if isinstance(elts[1].value, str) and re.fullmatch(r"[A-DF][+-]?", elts[1].value):
                    code = rustify(elts[0], default_stem, in_tuple=True)
                    grade = rustify(elts[1], default_stem, in_tuple=True)
                    return wrap_course(default_stem, code, grade)
                else:
                    stem = rustify(elts[0], default_stem, in_tuple=True)
                    code = rustify(elts[1], default_stem, in_tuple=True)
                    return wrap_course(stem, code)
            elif isinstance(elts[1], ast.Call) and getattr(elts[1].func, "id", None) == "GR":
                code = rustify(elts[0], default_stem, in_tuple=True)
                grade = rustify(elts[1], default_stem, in_tuple=True)
                return wrap_course(default_stem, code, grade)
            else:
                stem = rustify(elts[0], default_stem, in_tuple=True)
                code = rustify(elts[1], default_stem, in_tuple=True)
                return wrap_course(stem, code)
        elif len(elts) == 1:
            # (CODE,) or (STEM,)
            return rustify(elts[0], default_stem, in_tuple)
        else:
            raise ValueError(f"Unsupported tuple length: {len(elts)} in {ast.dump(node)}")
    elif isinstance(node, ast.Constant):
        if isinstance(node.value, int):
            if in_tuple:
                return str(node.value)
            else:
                return wrap_course(default_stem, node.value)
        elif isinstance(node.value, str):
            # If it looks like a grade, just return the string (parent will wrap in GR!)
            if re.fullmatch(r"[A-DF][+-]?", node.value):
                return node.value
            else:
                return f'"{node.value}"'
    elif isinstance(node, ast.Name):
        # Instructor, None, etc.
        return node.id
    raise ValueError(f"Unsupported AST node: {ast.dump(node)}")


def parse_req(req, default_stem):
    expr = ast.parse(preprocess_grades(req), mode="eval").body
    return rustify(expr, default_stem)


def preprocess_grades(expr: str) -> str:
    # Add spaces to catch grades at the start/end
    expr = f" {expr} "
    expr = re.sub(r"(?<=[\s\(\[,])([A-DF][+-]?)(?=[\s\)\],])", r'"\1"', expr)
    return expr.strip()


# Example usage:
def main():
    if PATH.exists():
        resp = input(f"File {PATH} exists. Overwrite? [Y/n] ").strip().lower()
        if resp != "y":
            print("Aborting.")
            return
    with open(PATH, "w") as f:
        f.write(PREAMBLE)
        print("Rust CourseReq REPL. Type 'stem STEM' to set stem, 'exit' to quit.")
        stem = "CS"
        while True:
            inp = input(f"[stem={stem}] code> ").strip()
            if inp.lower() == "exit":
                break
            if inp.lower().startswith("stem "):
                stem = inp.split()[1].upper()
                continue
            if not inp.isdigit():
                print("Enter a course code (e.g., 1020) or 'stem STEM'.")
                continue
            code = inp
            req = input(f"[stem={stem}] req for {code}> ").strip()
            if not req:
                print("No requirement entered, skipping.")
                continue
            parsed_req = parse_req(req, stem)
            print("Added course!")
            f.write(f'(\n    CC!("{stem}", {code}),\n    {parsed_req},\n),\n')
        f.write(POSTAMBLE)
    subprocess.run(["rustfmt", str(PATH)], check=True)


if __name__ == "__main__":
    main()
