import ast
import pathlib
import subprocess

PATH = pathlib.Path(__file__).parent.parent.joinpath("resources", "course_reqs.rs")

PREAMBLE = """
use crate::CC;
use crate::prereqs::CourseReq::{self, *};
use crate::schedule::CourseCode;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref PREREQS_MAP: HashMap<CourseCode, CourseReq> = HashMap::from([
"""

POSTAMBLE = """
    ]);
}
"""

def rustify(node, default_stem, in_tuple=False):
    if isinstance(node, ast.Call):
        func = node.func.id
        args = [rustify(arg, default_stem) for arg in node.args]
        if func == "And" or func == "Or":
            return f'{func}(vec![{", ".join(args)}])'
        if func == "Co":
            if len(node.args) == 2:
                stem = rustify(node.args[0], default_stem, in_tuple=True)
                num = rustify(node.args[1], default_stem, in_tuple=True)
                return f'CoCourse(CC!("{stem}", {num}))'
            else:
                num = rustify(node.args[0], default_stem, in_tuple=True)
                return f'CoCourse(CC!("{default_stem}", {num}))'
        if func == "Prog":
            return f'Program("{args[0]}")'
    elif isinstance(node, ast.Tuple):
        # (STEM, num)
        stem = rustify(node.elts[0], default_stem, in_tuple=True)
        num = rustify(node.elts[1], default_stem, in_tuple=True)
        return f'PreCourse(CC!("{stem}", {num}))'
    elif isinstance(node, ast.Constant):
        if isinstance(node.value, int):
            if in_tuple:
                return str(node.value)
            else:
                return f'PreCourse(CC!("{default_stem}", {node.value}))'
        else:
            return str(node.value)
    elif isinstance(node, ast.Name):
        # Instructor, None, etc.
        return node.id
    else:
        raise ValueError(f"Unsupported AST node: {ast.dump(node)}")

def parse_req(req, default_stem):
    expr = ast.parse(req, mode='eval').body
    return rustify(expr, default_stem)

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