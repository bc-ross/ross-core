import os
import subprocess

EXAMPLE_RS = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "resources", "example_program.rs"))


def parse_course_code(code):
    code = code.strip()
    if "-" not in code:
        raise ValueError(f"Invalid course code format: {code}")
    stem, num = code.split("-", 1)
    try:
        num = int(num)
    except ValueError:
        num = f'"{num}"'
    return f'CC!("{stem.upper()}", {num})'


def main():
    print("=== Program Definition REPL ===")
    name = input("Program name: ").strip()
    semesters = []
    sem_num = 1
    while True:
        line = input(f"Semester {sem_num} course codes (comma-separated, blank to finish): ").strip()
        if not line:
            break
        codes = [parse_course_code(code) for code in line.split(",") if code.strip()]
        semesters.append(f"            vec![{', '.join(codes)}],")
        sem_num += 1

    stems_line = input("Associated stems (comma-separated): ").strip()
    stems = [f'"{s.strip().upper()}".to_string()' for s in stems_line.split(",") if s.strip()]

    rust_code = (
        """
#![allow(unused_imports)]

use crate::CC;
use crate::schedule::{CourseCode, Elective::*, Program};

pub fn prog() -> Program {
    Program {
    """
        + f"""
            name: "{name}".to_string(),
            semesters: vec![
    {chr(10).join(semesters)}
            ],
            assoc_stems: vec![{", ".join(stems)}],
    """
        + "electives: vec![],\n}}"
    )
    with open(EXAMPLE_RS, "w", encoding="utf-8") as f:
        f.write(rust_code)
    print(f"Program written to {EXAMPLE_RS}")
    subprocess.run(["rustfmt", str(EXAMPLE_RS)], check=True)


if __name__ == "__main__":
    main()
