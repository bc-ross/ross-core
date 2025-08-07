use crate::prereqs::CourseReq;
use crate::schedule::CourseCode;
use std::collections::HashMap;
use varisat::{CnfFormula, ExtendFormula, Lit, Var, solver::Solver};

#[derive(Debug, Clone)]
pub struct SatSolution {
    pub additional_courses: HashMap<usize, Vec<CourseCode>>, // semester -> courses to add
    pub total_courses: Vec<Vec<CourseCode>>,                 // complete schedule including prereqs
}

/// SAT-based prerequisite solver
pub struct PrereqSatSolver {
    /// Maps (course, semester) -> variable indicating course is taken in that semester
    course_semester_vars: HashMap<(CourseCode, usize), Var>,
    /// Maps course -> variable indicating course is taken at any point
    course_taken_vars: HashMap<CourseCode, Var>,
    /// All variables used
    all_vars: Vec<Var>,
    /// The CNF formula
    formula: CnfFormula,
    /// Number of semesters
    num_semesters: usize,
}

impl PrereqSatSolver {
    pub fn new(num_semesters: usize) -> Self {
        Self {
            course_semester_vars: HashMap::new(),
            course_taken_vars: HashMap::new(),
            all_vars: Vec::new(),
            formula: CnfFormula::new(),
            num_semesters,
        }
    }

    fn new_var(&mut self) -> Var {
        let var = Var::from_index(self.all_vars.len());
        self.all_vars.push(var);
        var
    }

    fn get_course_semester_var(&mut self, course: &CourseCode, semester: usize) -> Var {
        if let Some(&var) = self.course_semester_vars.get(&(course.clone(), semester)) {
            var
        } else {
            let var = self.new_var();
            self.course_semester_vars
                .insert((course.clone(), semester), var);
            var
        }
    }

    fn get_course_taken_var(&mut self, course: &CourseCode) -> Var {
        if let Some(&var) = self.course_taken_vars.get(course) {
            var
        } else {
            let var = self.new_var();
            self.course_taken_vars.insert(course.clone(), var);
            var
        }
    }

    /// Add constraint that if a course is taken in any semester, the course_taken variable is true
    fn add_course_taken_constraints(&mut self) {
        let course_semester_vars = self.course_semester_vars.clone();

        for ((course, _), &semester_var) in &course_semester_vars {
            let taken_var = self.get_course_taken_var(course);
            // If course is taken in semester, then it's taken overall: semester_var -> taken_var
            self.formula.add_clause(&[
                !Lit::from_var(semester_var, false),
                Lit::from_var(taken_var, false),
            ]);
        }

        // If course is taken overall, it must be taken in at least one semester
        let course_taken_vars = self.course_taken_vars.clone();
        for (course, &taken_var) in &course_taken_vars {
            let mut clause = vec![!Lit::from_var(taken_var, false)];
            for sem in 0..self.num_semesters {
                if let Some(&sem_var) = self.course_semester_vars.get(&(course.clone(), sem)) {
                    clause.push(Lit::from_var(sem_var, false));
                }
            }
            if clause.len() > 1 {
                self.formula.add_clause(&clause);
            }
        }
    }

    /// Add constraint that a course can only be taken in one semester
    fn add_uniqueness_constraints(&mut self) {
        let mut course_semesters: HashMap<CourseCode, Vec<(usize, Var)>> = HashMap::new();

        for ((course, semester), &var) in &self.course_semester_vars {
            course_semesters
                .entry(course.clone())
                .or_default()
                .push((*semester, var));
        }

        for (_, semesters) in course_semesters {
            // At most one semester can be true for each course
            for i in 0..semesters.len() {
                for j in i + 1..semesters.len() {
                    let (_, var1) = semesters[i];
                    let (_, var2) = semesters[j];
                    // ¬var1 ∨ ¬var2 (at most one can be true)
                    self.formula
                        .add_clause(&[!Lit::from_var(var1, false), !Lit::from_var(var2, false)]);
                }
            }
        }
    }

    /// Recursively encode a CourseReq, returning a variable that is true iff the req is satisfied by semester sem_idx
    fn encode_course_req(&mut self, req: &CourseReq, sem_idx: usize) -> Var {
        match req {
            CourseReq::And(reqs) => {
                let this_var = self.new_var();
                let sub_vars: Vec<Var> = reqs
                    .iter()
                    .map(|sub| self.encode_course_req(sub, sem_idx))
                    .collect();

                // this_var <=> (sub1 & sub2 & ...)
                for &v in &sub_vars {
                    self.formula
                        .add_clause(&[!Lit::from_var(this_var, false), Lit::from_var(v, false)]);
                }
                let mut clause = sub_vars
                    .iter()
                    .map(|&v| !Lit::from_var(v, false))
                    .collect::<Vec<_>>();
                clause.push(Lit::from_var(this_var, false));
                self.formula.add_clause(&clause);
                this_var
            }
            CourseReq::Or(reqs) => {
                let this_var = self.new_var();
                let sub_vars: Vec<Var> = reqs
                    .iter()
                    .map(|sub| self.encode_course_req(sub, sem_idx))
                    .collect();

                // this_var <=> (sub1 | sub2 | ...)
                for &v in &sub_vars {
                    self.formula
                        .add_clause(&[Lit::from_var(this_var, false), !Lit::from_var(v, false)]);
                }
                let mut clause = sub_vars
                    .iter()
                    .map(|&v| Lit::from_var(v, false))
                    .collect::<Vec<_>>();
                clause.push(!Lit::from_var(this_var, false));
                self.formula.add_clause(&clause);
                this_var
            }
            CourseReq::PreCourse(code) | CourseReq::PreCourseGrade(code, _) => {
                // Course must be taken in a previous semester (0..sem_idx)
                let this_var = self.new_var();
                let mut clause = vec![!Lit::from_var(this_var, false)];
                for prev_sem in 0..sem_idx {
                    let course_var = self.get_course_semester_var(code, prev_sem);
                    clause.push(Lit::from_var(course_var, false));
                }
                if clause.len() > 1 {
                    self.formula.add_clause(&clause);
                } else {
                    // No previous semesters, so prereq cannot be satisfied
                    self.formula.add_clause(&[!Lit::from_var(this_var, false)]);
                }
                this_var
            }
            CourseReq::CoCourse(code) | CourseReq::CoCourseGrade(code, _) => {
                // Course must be taken in this semester or earlier (0..=sem_idx)
                let this_var = self.new_var();
                let mut clause = vec![!Lit::from_var(this_var, false)];
                for co_sem in 0..=sem_idx {
                    let course_var = self.get_course_semester_var(code, co_sem);
                    clause.push(Lit::from_var(course_var, false));
                }
                self.formula.add_clause(&clause);
                this_var
            }
            CourseReq::Program(_) | CourseReq::Instructor | CourseReq::None => {
                // Always satisfied
                let var = self.new_var();
                self.formula.add_clause(&[Lit::from_var(var, false)]);
                var
            }
        }
    }

    /// Add constraints for the existing schedule (courses that are already planned)
    pub fn add_existing_schedule(&mut self, schedule: &[Vec<CourseCode>]) {
        for (sem_idx, semester) in schedule.iter().enumerate() {
            for course in semester {
                let var = self.get_course_semester_var(course, sem_idx);
                // Force this course to be taken in this semester
                self.formula.add_clause(&[Lit::from_var(var, false)]);
            }
        }
    }

    /// Add prerequisite constraints for all courses in the schedule
    pub fn add_prereq_constraints(
        &mut self,
        schedule: &[Vec<CourseCode>],
        prereqs: &HashMap<CourseCode, CourseReq>,
    ) {
        for (sem_idx, semester) in schedule.iter().enumerate() {
            for course in semester {
                if let Some(req) = prereqs.get(course) {
                    // If course is taken in this semester, its prereqs must be satisfied
                    let course_var = self.get_course_semester_var(course, sem_idx);
                    let prereq_var = self.encode_course_req(req, sem_idx);
                    // course_var -> prereq_var
                    self.formula.add_clause(&[
                        !Lit::from_var(course_var, false),
                        Lit::from_var(prereq_var, false),
                    ]);
                }
            }
        }
    }

    /// Solve the SAT problem and return a solution
    pub fn solve(&mut self, original_schedule: &[Vec<CourseCode>]) -> Option<SatSolution> {
        self.add_course_taken_constraints();
        self.add_uniqueness_constraints();

        let mut solver = Solver::new();
        solver.add_formula(&self.formula);

        if solver.solve().unwrap() {
            let model = solver.model().unwrap();
            let mut total_courses = vec![Vec::new(); self.num_semesters];

            // Extract which courses are taken in which semesters
            for ((course, semester), &var) in &self.course_semester_vars {
                let idx = var.index();
                if idx < model.len() && model[idx].is_positive() {
                    total_courses[*semester].push(course.clone());
                }
            }

            // Calculate additional courses (courses not in original schedule)
            let mut additional_courses = HashMap::new();
            for (sem_idx, total_sem_courses) in total_courses.iter().enumerate() {
                let original_sem_courses = original_schedule
                    .get(sem_idx)
                    .map(|s| s.as_slice())
                    .unwrap_or(&[]);
                let additional: Vec<CourseCode> = total_sem_courses
                    .iter()
                    .filter(|course| !original_sem_courses.contains(course))
                    .cloned()
                    .collect();

                if !additional.is_empty() {
                    additional_courses.insert(sem_idx, additional);
                }
            }

            Some(SatSolution {
                additional_courses,
                total_courses,
            })
        } else {
            None
        }
    }
}

/// Public interface function to solve prerequisites for a schedule
pub fn solve_prereqs(
    schedule: Vec<Vec<CourseCode>>,
    prereqs: &HashMap<CourseCode, CourseReq>,
) -> Option<SatSolution> {
    let num_semesters = schedule.len();
    let mut solver = PrereqSatSolver::new(num_semesters);

    solver.add_existing_schedule(&schedule);
    solver.add_prereq_constraints(&schedule, prereqs);

    solver.solve(&schedule)
}

/// SAT-based equivalent of Schedule::ensure_prereqs that uses the SAT solver
pub fn ensure_prereqs_sat(
    schedule: Vec<Vec<CourseCode>>,
    prereqs: &HashMap<CourseCode, CourseReq>,
) -> Vec<Vec<Vec<CourseCode>>> {
    // Find courses with unmet prerequisites
    let mut courses_needing_prereqs = Vec::new();
    for (sem_idx, semester) in schedule.iter().enumerate() {
        for course in semester {
            if let Some(req) = prereqs.get(course) {
                if !is_prereq_satisfied(req, &schedule, sem_idx) {
                    courses_needing_prereqs.push((course.clone(), req.clone(), sem_idx));
                }
            }
        }
    }

    if courses_needing_prereqs.is_empty() {
        return vec![schedule];
    }

    // Build a comprehensive prerequisite map including all potential prereq courses
    let mut expanded_prereqs = prereqs.clone();
    for (_, req, _) in &courses_needing_prereqs {
        add_all_course_prereqs(req, prereqs, &mut expanded_prereqs);
    }

    // Use SAT solver to find solutions
    if let Some(solution) = solve_prereqs(schedule.clone(), &expanded_prereqs) {
        vec![solution.total_courses]
    } else {
        vec![]
    }
}

/// Helper function to check if a prerequisite is satisfied
fn is_prereq_satisfied(req: &CourseReq, schedule: &[Vec<CourseCode>], sem_idx: usize) -> bool {
    match req {
        CourseReq::And(reqs) => reqs
            .iter()
            .all(|r| is_prereq_satisfied(r, schedule, sem_idx)),
        CourseReq::Or(reqs) => reqs
            .iter()
            .any(|r| is_prereq_satisfied(r, schedule, sem_idx)),
        CourseReq::PreCourse(code) | CourseReq::PreCourseGrade(code, _) => {
            schedule.iter().take(sem_idx).flatten().any(|c| c == code)
        }
        CourseReq::CoCourse(code) | CourseReq::CoCourseGrade(code, _) => schedule
            .iter()
            .take(sem_idx + 1)
            .flatten()
            .any(|c| c == code),
        CourseReq::Program(_) | CourseReq::Instructor | CourseReq::None => true,
    }
}

/// Helper function to recursively add all courses mentioned in prerequisites
fn add_all_course_prereqs(
    req: &CourseReq,
    original_prereqs: &HashMap<CourseCode, CourseReq>,
    expanded_prereqs: &mut HashMap<CourseCode, CourseReq>,
) {
    match req {
        CourseReq::And(reqs) | CourseReq::Or(reqs) => {
            for sub_req in reqs {
                add_all_course_prereqs(sub_req, original_prereqs, expanded_prereqs);
            }
        }
        CourseReq::PreCourse(code)
        | CourseReq::CoCourse(code)
        | CourseReq::PreCourseGrade(code, _)
        | CourseReq::CoCourseGrade(code, _) => {
            if !expanded_prereqs.contains_key(code) {
                if let Some(course_req) = original_prereqs.get(code) {
                    expanded_prereqs.insert(code.clone(), course_req.clone());
                    add_all_course_prereqs(course_req, original_prereqs, expanded_prereqs);
                } else {
                    expanded_prereqs.insert(code.clone(), CourseReq::None);
                }
            }
        }
        _ => {}
    }
}

pub fn test_prereq_sat() {
    // Example: C requires (A OR B)
    let a = CourseCode {
        stem: "MATH".to_string(),
        code: 1010.into(),
    };
    let b = CourseCode {
        stem: "PHYS".to_string(),
        code: 2010.into(),
    };
    let c = CourseCode {
        stem: "CHEM".to_string(),
        code: 1210.into(),
    };
    let d = CourseCode {
        stem: "MATH".to_string(),
        code: 1020.into(),
    };

    // Create a simple test schedule: semester 0 has course D, semester 1 has course C
    let schedule = vec![
        vec![d.clone()],
        vec![c.clone()], // This requires (A OR B) as prerequisite
    ];

    // Set up prerequisites: C requires (A OR B)
    let mut prereqs = HashMap::new();
    prereqs.insert(
        c.clone(),
        CourseReq::Or(vec![
            CourseReq::PreCourse(a.clone()),
            CourseReq::PreCourse(b.clone()),
        ]),
    );

    println!("Testing SAT solver with schedule:");
    for (i, sem) in schedule.iter().enumerate() {
        println!("  Semester {}: {:?}", i, sem);
    }
    println!("Prerequisites: C requires (A OR B)");

    match solve_prereqs(schedule.clone(), &prereqs) {
        Some(solution) => {
            println!("SAT solution found!");
            println!("Additional courses needed:");
            for (semester, courses) in &solution.additional_courses {
                println!("  Semester {}: {:?}", semester, courses);
            }
            println!("Complete schedule:");
            for (semester, courses) in solution.total_courses.iter().enumerate() {
                println!("  Semester {}: {:?}", semester, courses);
            }
        }
        None => {
            println!("No solution found - prerequisites cannot be satisfied!");
        }
    }
}
