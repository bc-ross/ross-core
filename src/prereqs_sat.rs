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
    pub fn solve(&mut self) -> Option<SatSolution> {
        self.add_course_taken_constraints();
        self.add_uniqueness_constraints();

        let mut solver = Solver::new();
        solver.add_formula(&self.formula);

        if solver.solve().unwrap() {
            let model = solver.model().unwrap();
            let mut additional_courses = HashMap::new();
            let mut total_courses = vec![Vec::new(); self.num_semesters];

            // Extract which courses are taken in which semesters
            for ((course, semester), &var) in &self.course_semester_vars {
                let idx = var.index();
                if idx < model.len() && model[idx].is_positive() {
                    total_courses[*semester].push(course.clone());
                }
            }

            // Calculate additional courses (courses not in original schedule)
            // This would need the original schedule passed in to compute the difference
            // For now, we'll return all courses as additional
            for (sem_idx, courses) in total_courses.iter().enumerate() {
                if !courses.is_empty() {
                    additional_courses.insert(sem_idx, courses.clone());
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

    solver.solve()
}

pub fn test_prereq_sat() {
    // Example: (A OR B) AND C
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

    // Create a simple test schedule: semester 0 has course D, semester 1 has nothing yet
    let schedule = vec![
        vec![d.clone()],
        vec![c.clone()], // This requires (A OR B)
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

    match solve_prereqs(schedule, &prereqs) {
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
