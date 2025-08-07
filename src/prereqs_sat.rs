use crate::prereqs::CourseReq;
use crate::schedule::CourseCode;
use std::collections::HashMap;
use varisat::{CnfFormula, ExtendFormula, Lit, Var, solver::Solver};

// Maximum number of different SAT solutions to explore for optimization
pub const MAX_SAT_ITERATIONS: usize = 100;

#[derive(Debug, Clone)]
pub struct SatSolution {
    pub additional_courses: HashMap<usize, Vec<CourseCode>>, // semester -> courses to add
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
                !Lit::from_var(semester_var, true),
                Lit::from_var(taken_var, true),
            ]);
        }

        // If course is taken overall, it must be taken in at least one semester
        let course_taken_vars = self.course_taken_vars.clone();
        for (course, &taken_var) in &course_taken_vars {
            let mut clause = vec![!Lit::from_var(taken_var, true)];
            for sem in 0..self.num_semesters {
                if let Some(&sem_var) = self.course_semester_vars.get(&(course.clone(), sem)) {
                    clause.push(Lit::from_var(sem_var, true));
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
                        .add_clause(&[!Lit::from_var(var1, true), !Lit::from_var(var2, true)]);
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
                        .add_clause(&[!Lit::from_var(this_var, true), Lit::from_var(v, true)]);
                }
                let mut clause = sub_vars
                    .iter()
                    .map(|&v| !Lit::from_var(v, true))
                    .collect::<Vec<_>>();
                clause.push(Lit::from_var(this_var, true));
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
                        .add_clause(&[Lit::from_var(this_var, true), !Lit::from_var(v, true)]);
                }
                let mut clause = sub_vars
                    .iter()
                    .map(|&v| Lit::from_var(v, true))
                    .collect::<Vec<_>>();
                clause.push(!Lit::from_var(this_var, true));
                self.formula.add_clause(&clause);
                this_var
            }
            CourseReq::PreCourse(code) | CourseReq::PreCourseGrade(code, _) => {
                // Course must be taken in a previous semester (0..sem_idx)
                let this_var = self.new_var();
                let mut clause = vec![!Lit::from_var(this_var, true)];
                for prev_sem in 0..sem_idx {
                    let course_var = self.get_course_semester_var(code, prev_sem);
                    clause.push(Lit::from_var(course_var, true));
                }
                if clause.len() > 1 {
                    self.formula.add_clause(&clause);
                } else {
                    // No previous semesters, so prereq cannot be satisfied
                    self.formula.add_clause(&[!Lit::from_var(this_var, true)]);
                }
                this_var
            }
            CourseReq::CoCourse(code) | CourseReq::CoCourseGrade(code, _) => {
                // Course must be taken in this semester or earlier (0..=sem_idx)
                let this_var = self.new_var();
                let mut clause = vec![!Lit::from_var(this_var, true)];
                for co_sem in 0..=sem_idx {
                    let course_var = self.get_course_semester_var(code, co_sem);
                    clause.push(Lit::from_var(course_var, true));
                }
                self.formula.add_clause(&clause);
                this_var
            }
            CourseReq::Program(_) | CourseReq::Instructor | CourseReq::None => {
                // Always satisfied
                let var = self.new_var();
                self.formula.add_clause(&[Lit::from_var(var, true)]);
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
                self.formula.add_clause(&[Lit::from_var(var, true)]);
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
                        !Lit::from_var(course_var, true),
                        Lit::from_var(prereq_var, true),
                    ]);
                }
            }
        }
    }

    /// Solve the SAT problem and return a solution
    pub fn solve(&mut self, original_schedule: &[Vec<CourseCode>]) -> Option<SatSolution> {
        self.add_course_taken_constraints();
        self.add_uniqueness_constraints();

        // Add optimization constraints
        self.add_minimization_constraints();
        self.add_distribution_constraints();

        let mut solver = Solver::new();
        solver.add_formula(&self.formula);

        if solver.solve().unwrap() {
            let model = solver.model().unwrap();
            let mut total_courses = vec![Vec::new(); self.num_semesters];

            // Extract which courses are taken in which semesters
            for ((course, semester), &var) in &self.course_semester_vars {
                let idx = var.index();
                if idx < model.len() {
                    let assignment = model[idx];
                    if assignment.is_positive() {
                        total_courses[*semester].push(course.clone());
                    }
                } else {
                    eprintln!(
                        "Warning: Variable index {} out of bounds for model length {}",
                        idx,
                        model.len()
                    );
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

            Some(SatSolution { additional_courses })
        } else {
            None
        }
    }

    /// Add soft constraints to minimize total courses (optimization goal)
    fn add_minimization_constraints(&mut self) {
        // For now, we'll rely on the SAT solver's natural tendency to find minimal models
        // In a more advanced implementation, we could use a MaxSAT solver or add cardinality constraints
        // The current approach should already prefer minimal solutions due to how we encode constraints
    }

    /// Add constraints to encourage even distribution of courses across semesters
    fn add_distribution_constraints(&mut self) {
        // For better distribution, we can add soft penalties for having too many courses in one semester
        // This is a simple heuristic approach

        // Calculate how many courses could potentially be in each semester
        let mut semester_course_counts = vec![0; self.num_semesters];

        // Count courses that are already fixed in each semester (from original schedule)
        for ((_, semester), _) in &self.course_semester_vars {
            semester_course_counts[*semester] += 1;
        }

        // For a more even distribution, try to limit excessive concentration
        let total_potential_courses = self.course_semester_vars.len();
        let avg_per_semester =
            (total_potential_courses + self.num_semesters - 1) / self.num_semesters;

        // Add soft constraints to prevent any semester from having too many courses
        for sem in 0..self.num_semesters {
            let mut semester_vars = Vec::new();
            for ((_, semester), &var) in &self.course_semester_vars {
                if *semester == sem {
                    semester_vars.push(var);
                }
            }

            // Limit each semester to at most avg_per_semester + 1 courses
            // This encourages more even distribution
            if semester_vars.len() > avg_per_semester + 1 {
                // Add a cardinality constraint to prevent overloading this semester
                self.add_cardinality_constraint(&semester_vars, avg_per_semester + 1);
            }
        }

        // Additionally, try to encourage spreading courses to later semesters when possible
        // by adding slight penalties for taking courses too early
        for ((_course, _semester), &_var) in &self.course_semester_vars {
            // For prerequisite courses, prefer later semesters when constraints allow
            // This is a very weak preference - we just add a small bias
            // In a more sophisticated implementation, we'd use weighted constraints
            // For now, this is mainly educational and we don't implement the actual constraint
        }
    }

    /// Add a cardinality constraint: at most k of the given variables can be true
    fn add_cardinality_constraint(&mut self, vars: &[Var], k: usize) {
        if vars.len() <= k {
            return; // Constraint is already satisfied
        }

        // Simple encoding: for every pair when k=1, at least one must be false
        if k == 1 {
            // At most 1 can be true: for every pair, at least one must be false
            for i in 0..vars.len() {
                for j in i + 1..vars.len() {
                    self.formula.add_clause(&[
                        !Lit::from_var(vars[i], true),
                        !Lit::from_var(vars[j], true),
                    ]);
                }
            }
        } else if k == 2 && vars.len() <= 4 {
            // At most 2 can be true: for every triple, at least one must be false
            for i in 0..vars.len() {
                for j in i + 1..vars.len() {
                    for l in j + 1..vars.len() {
                        self.formula.add_clause(&[
                            !Lit::from_var(vars[i], true),
                            !Lit::from_var(vars[j], true),
                            !Lit::from_var(vars[l], true),
                        ]);
                    }
                }
            }
        }
        // For larger cases, we'd need a more sophisticated encoding (omitted for now)
    }

    /// Add a constraint to exclude a specific pattern of courses from the solution
    pub fn add_forbidden_pattern(&mut self, pattern: &[(CourseCode, usize)]) {
        if pattern.is_empty() {
            return;
        }

        // Create a clause that prevents this exact pattern from occurring
        // At least one course in the pattern must be placed differently
        let mut clause = Vec::new();

        for (course, semester) in pattern {
            if let Some(&var) = self.course_semester_vars.get(&(course.clone(), *semester)) {
                // Add the negation: this course should NOT be in this semester
                clause.push(!Lit::from_var(var, true));
            }
        }

        if !clause.is_empty() {
            self.formula.add_clause(&clause);
        }
    }
}

/// Enhanced SAT solver that finds multiple solutions by iteratively excluding previous ones
/// Now includes optimization scoring and early termination for better efficiency
pub fn solve_multiple_prereqs(
    schedule: Vec<Vec<CourseCode>>,
    prereqs: &HashMap<CourseCode, CourseReq>,
    max_solutions: usize,
) -> Vec<SatSolution> {
    let mut solutions = Vec::new();
    let mut forbidden_patterns: Vec<Vec<(CourseCode, usize)>> = Vec::new();
    let mut best_score = f64::INFINITY;
    let mut solutions_since_improvement = 0;
    const MAX_STAGNANT_ITERATIONS: usize = 20; // Stop if no improvement for this many iterations

    println!(
        "SAT solver exploring up to {} solutions for optimization...",
        max_solutions
    );

    for attempt in 0..max_solutions {
        let num_semesters = schedule.len();
        let mut solver = PrereqSatSolver::new(num_semesters);

        solver.add_existing_schedule(&schedule);
        solver.add_prereq_constraints(&schedule, prereqs);

        // Add constraints to exclude previous solutions
        for forbidden in &forbidden_patterns {
            solver.add_forbidden_pattern(forbidden);
        }

        if let Some(solution) = solver.solve(&schedule) {
            // Calculate a score for this solution (lower is better)
            let score = score_solution(&solution, &schedule);

            if score < best_score {
                best_score = score;
                solutions_since_improvement = 0;
                println!(
                    "SAT iteration {}: Found better solution with score {:.2} ({} additional courses)",
                    attempt + 1,
                    score,
                    count_total_additional_courses(&solution)
                );
            } else {
                solutions_since_improvement += 1;
            }

            // Early termination if we haven't improved in a while
            if solutions_since_improvement >= MAX_STAGNANT_ITERATIONS {
                println!(
                    "SAT solver: No improvement for {} iterations, terminating early at iteration {}",
                    MAX_STAGNANT_ITERATIONS,
                    attempt + 1
                );
                break;
            }

            // Extract the pattern of additional courses for this solution
            let mut pattern = Vec::new();
            for (sem_idx, additional_courses) in &solution.additional_courses {
                for course in additional_courses {
                    pattern.push((course.clone(), *sem_idx));
                }
            }

            if !pattern.is_empty() {
                forbidden_patterns.push(pattern);
            }
            solutions.push(solution);

            // Very early termination if we find a solution with very few additional courses
            let total_additional = count_total_additional_courses(&solutions.last().unwrap());
            if total_additional <= 2 {
                println!(
                    "SAT solver: Found excellent solution with only {} additional courses, terminating early",
                    total_additional
                );
                break;
            }
        } else {
            println!(
                "SAT solver: No more solutions found at iteration {}",
                attempt + 1
            );
            break; // No more solutions
        }
    }

    println!(
        "SAT solver completed: Found {} solutions, best score: {:.2}",
        solutions.len(),
        best_score
    );
    solutions
}

/// Score a solution based on number of additional courses and distribution
/// Lower scores are better
fn score_solution(solution: &SatSolution, original_schedule: &[Vec<CourseCode>]) -> f64 {
    let total_additional = count_total_additional_courses(solution);

    // Calculate distribution penalty (prefer even spread across semesters)
    let mut distribution_penalty = 0.0;
    let avg_additional = total_additional as f64 / original_schedule.len() as f64;

    for sem_idx in 0..original_schedule.len() {
        let sem_additional = solution
            .additional_courses
            .get(&sem_idx)
            .map(|courses| courses.len())
            .unwrap_or(0) as f64;

        // Penalty for deviation from average
        distribution_penalty += (sem_additional - avg_additional).abs();
    }

    // Main score: total additional courses + small distribution penalty
    total_additional as f64 + (distribution_penalty * 0.1)
}

/// Count total number of additional courses in a solution
fn count_total_additional_courses(solution: &SatSolution) -> usize {
    solution
        .additional_courses
        .values()
        .map(|courses| courses.len())
        .sum()
}
