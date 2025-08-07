# schedule_optimizer.py
from ortools.sat.python import cp_model

# ---------------------------
# Example data -- REPLACE with your data
# ---------------------------
# Each course dict must include:
# id: unique string id
# name: human-readable name
# credits: integer
# required: True if a program course that must be taken exactly once
# geneds: list of gened ids this course satisfies (may be empty)
# elective_group: string group id if the course is an elective option (else None)
# prereqs: list of course ids that must be taken prior to this course

COURSES = [
    {
        "id": "MATH101",
        "name": "Calc I",
        "credits": 4,
        "required": True,
        "geneds": [],
        "elective_group": None,
        "prereqs": [],
    },
    {
        "id": "MATH102",
        "name": "Calc II",
        "credits": 4,
        "required": True,
        "geneds": [],
        "elective_group": None,
        "prereqs": ["MATH101"],
    },
    {
        "id": "CS101",
        "name": "Intro CS",
        "credits": 3,
        "required": True,
        "geneds": [],
        "elective_group": None,
        "prereqs": [],
    },
    {
        "id": "CS201",
        "name": "Data Structures",
        "credits": 3,
        "required": True,
        "geneds": [],
        "elective_group": None,
        "prereqs": ["CS101"],
    },
    {
        "id": "ENG001",
        "name": "Comp I",
        "credits": 3,
        "required": False,
        "geneds": ["WRI"],
        "elective_group": None,
        "prereqs": [],
    },
    {
        "id": "PHIL01",
        "name": "Intro Ethics",
        "credits": 3,
        "required": False,
        "geneds": ["HUM"],
        "elective_group": None,
        "prereqs": [],
    },
    {
        "id": "BIO01",
        "name": "Biology I",
        "credits": 4,
        "required": False,
        "geneds": ["SCI"],
        "elective_group": None,
        "prereqs": [],
    },
    {
        "id": "ART01",
        "name": "Art Hist",
        "credits": 3,
        "required": False,
        "geneds": ["ART"],
        "elective_group": None,
        "prereqs": [],
    },
    # Elective options (choose a number from group "ELEC_A")
    {
        "id": "ELEC_A1",
        "name": "Elective A1",
        "credits": 2,
        "required": False,
        "geneds": [],
        "elective_group": "ELEC_A",
        "prereqs": [],
    },
    {
        "id": "ELEC_A2",
        "name": "Elective A2",
        "credits": 3,
        "required": False,
        "geneds": [],
        "elective_group": "ELEC_A",
        "prereqs": [],
    },
    # An advanced course that requires CS201
    {
        "id": "CS301",
        "name": "Algorithms",
        "credits": 3,
        "required": False,
        "geneds": [],
        "elective_group": None,
        "prereqs": ["CS201"],
    },
]

# gened requirement mapping: gened_id -> number of distinct gened courses required
# e.g. "WRI":1 means you must take at least one course that lists geneds containing "WRI"
GENED_REQUIREMENTS = {
    "WRI": 1,
    "HUM": 1,
    "SCI": 1,
    # add whatever geneds your program requires...
}

# elective group requirements: group_id -> number required from that group
ELECTIVE_REQUIREMENTS = {
    "ELEC_A": 1,  # must pick 1 from this elective group
}

NUM_SEMESTERS = 8
MAX_CREDITS_PER_SEM = 18

# ---------------------------
# Build indices
# ---------------------------
course_ids = [c["id"] for c in COURSES]
course_index = {cid: i for i, cid in enumerate(course_ids)}
num_courses = len(COURSES)
semesters = list(range(NUM_SEMESTERS))

# Precompute lists for geneds and electives
gened_to_course_indices = {}
for g in GENED_REQUIREMENTS:
    gened_to_course_indices[g] = [course_index[c["id"]] for c in COURSES if g in c.get("geneds", [])]

elective_group_to_course_indices = {}
for g in ELECTIVE_REQUIREMENTS:
    elective_group_to_course_indices[g] = [course_index[c["id"]] for c in COURSES if c.get("elective_group") == g]

# ---------------------------
# Model & Variables
# ---------------------------
model = cp_model.CpModel()

# Binary variables course_in_sem[(i,s)] == 1 if course i is scheduled in semester s
course_in_sem = {}
for i in range(num_courses):
    for s in semesters:
        course_in_sem[(i, s)] = model.NewBoolVar(f"c_{i}_s_{s}")

# Optional: a binary var course_taken[i] to indicate course is taken in some semester
course_taken = []
for i in range(num_courses):
    v = model.NewBoolVar(f"taken_{i}")
    course_taken.append(v)
    # link: taken == OR_s(course_in_sem[i,s])  -> use sum >=1 and sum <= many with big-M boolean logic
    model.Add(sum(course_in_sem[(i, s)] for s in semesters) == 1).OnlyEnforceIf(v)  # if taken then exactly one semester
    model.Add(sum(course_in_sem[(i, s)] for s in semesters) == 0).OnlyEnforceIf(v.Not())  # if not taken then none

# For required program courses, enforce taken == True
for i, c in enumerate(COURSES):
    if c.get("required", False):
        model.Add(course_taken[i] == 1)

# Otherwise courses can be taken or not taken (depending on gened/elective/prereqs)
# But to avoid both-link complexity we already linked course_taken <-> course_in_sem above.

# ---------------------------
# Semester credit limit constraint
# ---------------------------
for s in semesters:
    model.Add(sum(COURSES[i]["credits"] * course_in_sem[(i, s)] for i in range(num_courses)) <= MAX_CREDITS_PER_SEM)

# ---------------------------
# Prerequisite ordering constraints
# For each course c with prereq p:
#   For every semester s: course_in_sem[c,s] <= sum_{t < s} course_in_sem[p,t]
# This ensures if c is placed in semester s, each prereq p must be placed in an earlier semester.
# Also this has the side-effect: if c is scheduled, prereqs must be scheduled (since RHS >0 => prereq is scheduled).
# ---------------------------
for i, c in enumerate(COURSES):
    for pre in c.get("prereqs", []):
        if pre not in course_index:
            raise ValueError(f"Prereq {pre} for course {c['id']} not found in COURSES list.")
        p_idx = course_index[pre]
        for s in semesters:
            if s == 0:
                # can't schedule a course with a prereq in semester 0
                model.Add(course_in_sem[(i, s)] == 0)
            else:
                model.Add(course_in_sem[(i, s)] <= sum(course_in_sem[(p_idx, t)] for t in range(s)))

# ---------------------------
# Gen-ed requirements
# For each gened G requiring k courses: sum_{all courses that satisfy G, all semesters} >= k
# ---------------------------
for g, k in GENED_REQUIREMENTS.items():
    indices = gened_to_course_indices.get(g, [])
    if not indices:
        raise ValueError(f"No courses available that satisfy gened {g}. Problem infeasible.")
    model.Add(sum(course_in_sem[(i, s)] for i in indices for s in semesters) >= k)

# ---------------------------
# Elective group requirements
# For each group E requiring k courses: sum_{all group courses, all semesters} >= k
# ---------------------------
for group, k in ELECTIVE_REQUIREMENTS.items():
    indices = elective_group_to_course_indices.get(group, [])
    if not indices:
        raise ValueError(f"No courses available in elective group {group}. Problem infeasible.")
    model.Add(sum(course_in_sem[(i, s)] for i in indices for s in semesters) >= k)

# ---------------------------
# All prerequisites selected (implicit in prereq ordering constraints):
# If a course is taken, its prereqs must be scheduled in earlier semesters.
# (Handled above)
# ---------------------------

# ---------------------------
# OPTIONAL: a constraint to prevent duplicate scheduling (we already enforced exact 0 or 1 via course_taken linking)
# But we can also explicitly ensure at-most-once:
# ---------------------------
for i in range(num_courses):
    model.Add(sum(course_in_sem[(i, s)] for s in semesters) <= 1)

# ---------------------------
# Objective: minimize total credit count
# (If all required + prereqs are fixed, this will prefer lower-credit choices among geneds/electives.)
# ---------------------------
model.Minimize(sum(COURSES[i]["credits"] * course_in_sem[(i, s)] for i in range(num_courses) for s in semesters))

# ---------------------------
# Solve
# ---------------------------
solver = cp_model.CpSolver()
solver.parameters.max_time_in_seconds = 30  # adjust as needed
solver.parameters.num_search_workers = 8

status = solver.Solve(model)

if status == cp_model.OPTIMAL or status == cp_model.FEASIBLE:
    total_credits = 0
    schedule = {s: [] for s in semesters}
    for s in semesters:
        sem_credits = 0
        for i in range(num_courses):
            if solver.Value(course_in_sem[(i, s)]) == 1:
                schedule[s].append((COURSES[i]["id"], COURSES[i]["name"], COURSES[i]["credits"]))
                sem_credits += COURSES[i]["credits"]
                total_credits += COURSES[i]["credits"]
        print(f"Semester {s + 1}: credits={sem_credits}")
        for cid, name, cr in schedule[s]:
            print(f"  - {cid}: {name} ({cr})")
        print()
    print("Total credits scheduled:", total_credits)
    print("Objective lower bound (best found):", solver.ObjectiveValue())
else:
    print("No feasible solution found. Status:", status)
