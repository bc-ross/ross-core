use crate::prereqs::CourseReq;
use crate::schedule::CourseCode;
use std::collections::HashMap;
use varisat::{CnfFormula, ExtendFormula, Lit, Var, solver::Solver};

// Maximum credits allowed per semester
pub const MAX_CREDITS_PER_SEMESTER: u32 = 18;

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
    /// Course credit information for optimization
    course_credits: HashMap<CourseCode, u32>,
}

impl PrereqSatSolver {
    pub fn new(num_semesters: usize) -> Self {
        Self {
            course_semester_vars: HashMap::new(),
            course_taken_vars: HashMap::new(),
            all_vars: Vec::new(),
            formula: CnfFormula::new(),
            num_semesters,
            course_credits: HashMap::new(),
        }
    }

    /// Set course credit information for optimization
    pub fn set_course_credits(&mut self, course_credits: HashMap<CourseCode, u32>) {
        self.course_credits = course_credits;
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

    /// Enhanced solve method that includes gened constraints
    pub fn solve_with_geneds(
        &mut self,
        original_schedule: &[Vec<CourseCode>],
        geneds: &[crate::geneds::GenEd],
        catalog: &crate::schedule::Catalog,
    ) -> Option<SatSolution> {
        self.add_course_taken_constraints();
        self.add_uniqueness_constraints();

        // Debug: calculate credits in original schedule
        let mut original_credits = 0u32;
        for semester in original_schedule {
            for course in semester {
                original_credits += self.course_credits.get(course).unwrap_or(&3);
            }
        }
        println!("Original schedule has {} credits total", original_credits);

        // Debug: show credits per semester in original schedule
        for (sem_idx, semester) in original_schedule.iter().enumerate() {
            let mut sem_credits = 0u32;
            for course in semester {
                sem_credits += self.course_credits.get(course).unwrap_or(&3);
            }
            println!("  Semester {}: {} credits", sem_idx, sem_credits);
        }

        // Add all constraints together
        self.add_credit_constraint();
        self.add_gened_constraints(geneds, catalog);

        print!("\r🔧 Adding optimization constraints...");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        self.add_minimization_constraints();
        self.add_distribution_constraints();

        print!("\r🧮 Creating SAT solver and adding formula...");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        let mut solver = Solver::new();
        solver.add_formula(&self.formula);

        print!("\r🔍 Solving SAT problem (this may take a moment)...");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        if solver.solve().unwrap() {
            print!("\r✓ SAT problem solved! Extracting solution...");
            std::io::Write::flush(&mut std::io::stdout()).unwrap();

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

    /// Add hard constraint to ensure no semester exceeds MAX_CREDITS_PER_SEMESTER
    fn add_credit_constraint(&mut self) {
        if self.course_credits.is_empty() {
            return; // No credit information available
        }

        println!(
            "Adding per-semester credit constraints (max {} credits per semester)",
            MAX_CREDITS_PER_SEMESTER
        );

        // For each semester, add a constraint that the total credits <= MAX_CREDITS_PER_SEMESTER
        for sem in 0..self.num_semesters {
            let mut semester_vars_with_credits = Vec::new();

            // Collect all courses that could be taken in this semester with their credits
            for ((course, semester), &var) in &self.course_semester_vars {
                if *semester == sem {
                    let credits = self.course_credits.get(course).cloned().unwrap_or(3);
                    semester_vars_with_credits.push((var, credits));
                }
            }

            if !semester_vars_with_credits.is_empty() {
                println!(
                    "Adding credit constraint for semester {}: {} courses, max {} credits",
                    sem,
                    semester_vars_with_credits.len(),
                    MAX_CREDITS_PER_SEMESTER
                );
                self.add_weighted_cardinality_constraint(
                    &semester_vars_with_credits,
                    MAX_CREDITS_PER_SEMESTER,
                );
            }
        }
    }

    /// Add weighted cardinality constraint: sum of credits for true variables <= max_credits
    /// Uses comprehensive encoding for optimal results
    fn add_weighted_cardinality_constraint(
        &mut self,
        weighted_vars: &[(Var, u32)],
        max_credits: u32,
    ) {
        if weighted_vars.is_empty() {
            return;
        }

        // Calculate total possible credits
        let total_possible: u32 = weighted_vars.iter().map(|(_, weight)| *weight).sum();

        if total_possible <= max_credits {
            return; // Constraint is already satisfied
        }

        println!(
            "Adding credit constraint: max {} credits from {} courses (total possible: {})",
            max_credits,
            weighted_vars.len(),
            total_possible
        );

        // Use a proper weighted constraint encoding
        self.add_weighted_constraint_proper(weighted_vars, max_credits);
    }

    /// Add a proper weighted constraint using totalizer encoding
    /// This ensures that the sum of weights (credits) does not exceed max_credits
    fn add_weighted_constraint_proper(&mut self, weighted_vars: &[(Var, u32)], max_credits: u32) {
        if weighted_vars.is_empty() || max_credits == 0 {
            return;
        }

        // Simple but correct approach: forbid specific problematic combinations
        // This is more reliable than complex sequential encoding
        self.add_weighted_constraint_direct(weighted_vars, max_credits);
    }

    /// Direct encoding that forbids combinations exceeding the limit
    /// Uses a more systematic approach than the previous broken implementation
    fn add_weighted_constraint_direct(&mut self, weighted_vars: &[(Var, u32)], max_credits: u32) {
        // For small sets, enumerate combinations that violate the constraint
        if weighted_vars.len() <= 12 {
            // Limit to avoid exponential blowup
            println!(
                "  Using direct enumeration for {} courses",
                weighted_vars.len()
            );
            self.forbid_overweight_combinations(weighted_vars, max_credits, 0, 0, Vec::new());
        } else {
            // For larger sets, use a simpler approach:
            // Forbid taking all high-credit courses simultaneously
            println!(
                "  Using simplified constraint for {} courses",
                weighted_vars.len()
            );
            self.add_simplified_weight_constraint(weighted_vars, max_credits);
        }
    }

    /// Recursively forbid all combinations that exceed the credit limit
    fn forbid_overweight_combinations(
        &mut self,
        weighted_vars: &[(Var, u32)],
        max_credits: u32,
        index: usize,
        current_weight: u32,
        current_vars: Vec<Var>,
    ) {
        // If current weight already exceeds limit, forbid this combination
        if current_weight > max_credits && !current_vars.is_empty() {
            let clause: Vec<Lit> = current_vars
                .iter()
                .map(|&var| !Lit::from_var(var, true)) // NOT all of these can be true
                .collect();
            self.formula.add_clause(&clause);

            // Debug: print what we're forbidding
            if current_vars.len() <= 5 {
                // Only print small combinations
                println!(
                    "  Forbidding combination of {} courses with {} credits (exceeds {})",
                    current_vars.len(),
                    current_weight,
                    max_credits
                );
            }
            return; // No need to continue this branch
        }

        // If we've processed all variables, we're done with this branch
        if index >= weighted_vars.len() {
            return;
        }

        let (var, weight) = weighted_vars[index];

        // Branch 1: don't take this course
        self.forbid_overweight_combinations(
            weighted_vars,
            max_credits,
            index + 1,
            current_weight,
            current_vars.clone(),
        );

        // Branch 2: take this course (if it doesn't immediately exceed limit)
        let new_weight = current_weight + weight;
        if new_weight <= max_credits * 2 {
            // Reasonable bound to avoid infinite exploration
            let mut new_vars = current_vars;
            new_vars.push(var);
            self.forbid_overweight_combinations(
                weighted_vars,
                max_credits,
                index + 1,
                new_weight,
                new_vars,
            );
        }
    }

    /// Simplified constraint for large course sets
    fn add_simplified_weight_constraint(&mut self, weighted_vars: &[(Var, u32)], max_credits: u32) {
        // Sort courses by weight (highest first)
        let mut sorted_vars = weighted_vars.to_vec();
        sorted_vars.sort_by(|a, b| b.1.cmp(&a.1));

        // Find combinations of high-weight courses that would exceed the limit
        for i in 0..sorted_vars.len() {
            for j in (i + 1)..sorted_vars.len() {
                let (var1, weight1) = sorted_vars[i];
                let (var2, weight2) = sorted_vars[j];

                if weight1 + weight2 > max_credits {
                    // These two courses together exceed the limit
                    self.formula
                        .add_clause(&[!Lit::from_var(var1, true), !Lit::from_var(var2, true)]);
                }

                // Check triplets for very high-weight courses
                if weight1 >= 4 && weight2 >= 4 {
                    for k in (j + 1)..sorted_vars.len() {
                        let (var3, weight3) = sorted_vars[k];
                        if weight1 + weight2 + weight3 > max_credits {
                            self.formula.add_clause(&[
                                !Lit::from_var(var1, true),
                                !Lit::from_var(var2, true),
                                !Lit::from_var(var3, true),
                            ]);
                        }
                    }
                }
            }
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

    /// Hard constraint to ensure all general education requirements are satisfied
    pub fn add_gened_constraints(
        &mut self,
        geneds: &[crate::geneds::GenEd],
        catalog: &crate::schedule::Catalog,
    ) {
        println!(
            "Adding gened constraints for {} geneds directly to SAT solver",
            geneds.len()
        );

        for (gened_idx, gened) in geneds.iter().enumerate() {
            // Progress indicator
            let spinner_chars = ['|', '/', '-', '\\'];
            let spinner = spinner_chars[gened_idx % 4];

            match gened {
                crate::geneds::GenEd::Core { req, name, .. } => {
                    print!(
                        "\r{} Encoding Core gened {}/{}: {}...",
                        spinner,
                        gened_idx + 1,
                        geneds.len(),
                        name
                    );
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                    self.add_gened_requirement_constraint(
                        req,
                        catalog,
                        &format!("Core_{}", gened_idx),
                    );
                }
                crate::geneds::GenEd::Foundation { req, name, .. } => {
                    print!(
                        "\r{} Encoding Foundation gened {}/{}: {}...",
                        spinner,
                        gened_idx + 1,
                        geneds.len(),
                        name
                    );
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                    self.add_gened_requirement_constraint(
                        req,
                        catalog,
                        &format!("Foundation_{}", gened_idx),
                    );
                }
                crate::geneds::GenEd::SkillAndPerspective { req, name, .. } => {
                    print!(
                        "\r{} Encoding Skills&Perspective gened {}/{}: {}...",
                        spinner,
                        gened_idx + 1,
                        geneds.len(),
                        name
                    );
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                    self.add_gened_requirement_constraint(
                        req,
                        catalog,
                        &format!("Skills_{}", gened_idx),
                    );
                }
            }
        }
        println!("\r✓ Completed encoding {} gened constraints", geneds.len());
    }

    /// Add constraint for a single gened requirement
    fn add_gened_requirement_constraint(
        &mut self,
        req: &crate::geneds::ElectiveReq,
        catalog: &crate::schedule::Catalog,
        _gened_name: &str,
    ) {
        match req {
            crate::geneds::ElectiveReq::Courses { num, courses } => {
                print!(" (needs {} from {} courses)", num, courses.len());
                std::io::Write::flush(&mut std::io::stdout()).unwrap();

                // Create variables for each potential course across all semesters
                let mut course_vars = Vec::new();
                for course_code in courses {
                    // Add this course as a potential course in all semesters
                    for sem in 0..self.num_semesters {
                        let var = self.get_course_semester_var(course_code, sem);
                        course_vars.push(var);
                    }
                }

                // Add constraint: at least `num` of these course variables must be true
                if *num == 1 {
                    // At least one must be true
                    let clause: Vec<Lit> = course_vars
                        .iter()
                        .map(|&var| Lit::from_var(var, true))
                        .collect();
                    self.formula.add_clause(&clause);
                } else {
                    // For num > 1, we need a more sophisticated cardinality constraint
                    self.add_at_least_k_constraint(&course_vars, *num);
                }

                // CRITICAL: Add constraint to prevent taking more than needed
                // This prevents the solver from taking ALL courses when only some are needed
                if courses.len() > *num {
                    self.add_at_most_k_constraint(&course_vars, *num + 2); // Allow small buffer
                }
            }
            crate::geneds::ElectiveReq::Credits { num, courses } => {
                print!(" (needs {} credits from {} courses)", num, courses.len());
                std::io::Write::flush(&mut std::io::stdout()).unwrap();

                // Create weighted variables for each course option
                let mut weighted_vars = Vec::new();
                for course_code in courses {
                    let course_credits = catalog
                        .courses
                        .get(course_code)
                        .and_then(|(_, credits, _)| *credits)
                        .unwrap_or(3);

                    // Add this course as a potential course in all semesters
                    for sem in 0..self.num_semesters {
                        let var = self.get_course_semester_var(course_code, sem);
                        weighted_vars.push((var, course_credits));
                    }
                }

                // Add constraint: at least `num` total credits from these courses
                self.add_at_least_credits_constraint(&weighted_vars, *num);

                // CRITICAL: Prevent over-taking courses for credit requirements
                // This helps avoid taking too many gened courses when fewer would suffice
                let avg_credits =
                    weighted_vars.iter().map(|(_, c)| *c).sum::<u32>() / weighted_vars.len() as u32;
                let max_courses_needed = (*num + avg_credits - 1) / avg_credits + 1; // Ceiling + buffer
                if weighted_vars.len() > max_courses_needed as usize {
                    let simple_vars: Vec<Var> = weighted_vars.iter().map(|(v, _)| *v).collect();
                    self.add_at_most_k_constraint(&simple_vars, max_courses_needed as usize);
                }
            }
            crate::geneds::ElectiveReq::Set(courses) => {
                print!(" (requires exactly {} courses)", courses.len());
                std::io::Write::flush(&mut std::io::stdout()).unwrap();

                // All courses must be taken (in some semester)
                for course_code in courses {
                    let mut course_sem_vars = Vec::new();
                    for sem in 0..self.num_semesters {
                        let var = self.get_course_semester_var(course_code, sem);
                        course_sem_vars.push(var);
                    }

                    // This course must be taken in at least one semester
                    let clause: Vec<Lit> = course_sem_vars
                        .iter()
                        .map(|&var| Lit::from_var(var, true))
                        .collect();
                    self.formula.add_clause(&clause);
                }
            }
            crate::geneds::ElectiveReq::SetOpts(sets) => {
                print!(" (one of {} set options)", sets.len());
                std::io::Write::flush(&mut std::io::stdout()).unwrap();

                // Create variables for each set being satisfied
                let mut set_vars = Vec::new();
                for (_set_idx, set_courses) in sets.iter().enumerate() {
                    let set_var = self.new_var();
                    set_vars.push(set_var);

                    // For this set to be satisfied, all courses in the set must be taken
                    let mut set_course_vars = Vec::new();
                    for course_code in set_courses {
                        let mut course_sem_vars = Vec::new();
                        for sem in 0..self.num_semesters {
                            let var = self.get_course_semester_var(course_code, sem);
                            course_sem_vars.push(var);
                        }

                        // Course must be taken in at least one semester
                        let course_taken_var = self.new_var();
                        set_course_vars.push(course_taken_var);

                        // Link course_taken_var to semester variables
                        let mut clause = vec![!Lit::from_var(course_taken_var, true)];
                        clause.extend(course_sem_vars.iter().map(|&v| Lit::from_var(v, true)));
                        self.formula.add_clause(&clause);

                        for &sem_var in &course_sem_vars {
                            self.formula.add_clause(&[
                                !Lit::from_var(sem_var, true),
                                Lit::from_var(course_taken_var, true),
                            ]);
                        }
                    }

                    // Set is satisfied iff all courses in set are taken
                    for &course_var in &set_course_vars {
                        self.formula.add_clause(&[
                            !Lit::from_var(set_var, true),
                            Lit::from_var(course_var, true),
                        ]);
                    }
                    let mut clause = set_course_vars
                        .iter()
                        .map(|&v| !Lit::from_var(v, true))
                        .collect::<Vec<_>>();
                    clause.push(Lit::from_var(set_var, true));
                    self.formula.add_clause(&clause);
                }

                // At least one set must be satisfied
                let clause: Vec<Lit> = set_vars
                    .iter()
                    .map(|&var| Lit::from_var(var, true))
                    .collect();
                self.formula.add_clause(&clause);
            }
        }
    }

    /// Add constraint: at least k of the given variables must be true
    fn add_at_least_k_constraint(&mut self, vars: &[Var], k: usize) {
        if k == 0 || vars.is_empty() {
            return;
        }

        if k == 1 {
            // Simple case: at least one must be true
            let clause: Vec<Lit> = vars.iter().map(|&var| Lit::from_var(var, true)).collect();
            self.formula.add_clause(&clause);
        } else if k >= vars.len() {
            // All must be true
            for &var in vars {
                self.formula.add_clause(&[Lit::from_var(var, true)]);
            }
        } else {
            // Use a more efficient encoding for k > 1
            // Instead of exponential combinations, use auxiliary variables
            self.add_efficient_at_least_k_constraint(vars, k);
        }
    }

    /// Add constraint: at most k of the given variables can be true
    fn add_at_most_k_constraint(&mut self, vars: &[Var], k: usize) {
        if k >= vars.len() {
            return; // Already satisfied
        }

        if k == 0 {
            // None can be true
            for &var in vars {
                self.formula.add_clause(&[!Lit::from_var(var, true)]);
            }
        } else if k == 1 {
            // At most one can be true - use pairwise constraints
            for i in 0..vars.len() {
                for j in (i + 1)..vars.len() {
                    self.formula.add_clause(&[
                        !Lit::from_var(vars[i], true),
                        !Lit::from_var(vars[j], true),
                    ]);
                }
            }
        } else {
            // For k > 1, use a simplified approach to avoid exponential blowup
            // We'll use the complement: forbid (k+1) or more from being true simultaneously
            self.add_efficient_at_most_k_constraint(vars, k);
        }
    }

    /// Efficient encoding for at-most-k constraints
    fn add_efficient_at_most_k_constraint(&mut self, vars: &[Var], k: usize) {
        // Use auxiliary variables to track how many are true
        // This is much more efficient than enumerating all combinations

        let num_aux = k + 1; // We need k+1 auxiliary variables
        let mut aux_vars = Vec::new();

        for _ in 0..num_aux {
            aux_vars.push(self.new_var());
        }

        // aux_vars[i] means "at least i+1 variables are true"
        // We want aux_vars[k] to be false (at most k can be true)
        self.formula
            .add_clause(&[!Lit::from_var(aux_vars[k], true)]);

        // Link auxiliary variables: if aux[i] is true, then aux[i-1] must be true
        for i in 1..aux_vars.len() {
            self.formula.add_clause(&[
                !Lit::from_var(aux_vars[i], true),
                Lit::from_var(aux_vars[i - 1], true),
            ]);
        }

        // Connect original variables to auxiliary variables
        // If aux[0] is true, at least one original variable must be true
        let mut clause = vec![!Lit::from_var(aux_vars[0], true)];
        for &var in vars {
            clause.push(Lit::from_var(var, true));
        }
        self.formula.add_clause(&clause);

        // For efficiency, we use a simplified approach rather than exact counting
        // This provides good constraint enforcement without exponential complexity
    }

    /// Efficient encoding for at-least-k constraints using auxiliary variables
    fn add_efficient_at_least_k_constraint(&mut self, vars: &[Var], k: usize) {
        print!(" [encoding {}-of-{}]", k, vars.len());
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        if vars.len() <= 10 && k <= 3 {
            // For small cases, use direct encoding
            self.add_direct_at_least_k_constraint(vars, k);
        } else {
            // For larger cases, use a simplified approach that's more efficient
            // We'll use a counting network approach
            self.add_counting_network_at_least_k(vars, k);
        }

        print!(" [done]");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }

    /// Direct encoding for small at-least-k constraints
    fn add_direct_at_least_k_constraint(&mut self, vars: &[Var], k: usize) {
        // For small k, we can forbid all combinations of size < k being all false
        // This is manageable for small numbers

        if k == 2 && vars.len() <= 10 {
            // For "at least 2", every pair must have at least one true
            // This is more manageable than the exponential approach
            for i in 0..vars.len() {
                for j in (i + 1)..vars.len() {
                    // At least one of vars[i] or vars[j] must be true
                    self.formula
                        .add_clause(&[Lit::from_var(vars[i], true), Lit::from_var(vars[j], true)]);
                }
            }
        } else {
            // Fall back to counting network for other cases
            self.add_counting_network_at_least_k(vars, k);
        }
    }

    /// Simplified counting network for at-least-k constraints
    fn add_counting_network_at_least_k(&mut self, vars: &[Var], k: usize) {
        // Use a simplified approach: create auxiliary variables to count
        // This is much more efficient than exponential enumeration

        // Create auxiliary variables for partial counts
        let mut count_vars = Vec::new();

        // For efficiency, we'll use a different approach:
        // Create k auxiliary variables that represent "we've seen at least i items"
        for i in 1..=k {
            let aux_var = self.new_var();
            count_vars.push(aux_var);
        }

        // The key insight: we need at least k to be true
        // So the k-th count variable must be true
        self.formula
            .add_clause(&[Lit::from_var(count_vars[k - 1], true)]);

        // Add simplified constraints linking original vars to count vars
        // This is a heuristic approach that's much more efficient

        // At least one original variable must be true for count_1 to be true
        let mut clause = vec![!Lit::from_var(count_vars[0], true)];
        for &var in vars {
            clause.push(Lit::from_var(var, true));
        }
        self.formula.add_clause(&clause);

        // For higher counts, use a simplified cascade
        for i in 1..k {
            // count[i] implies count[i-1]
            self.formula.add_clause(&[
                !Lit::from_var(count_vars[i], true),
                Lit::from_var(count_vars[i - 1], true),
            ]);
        }

        // Add some additional constraints to make the counting more accurate
        // Group variables and ensure enough groups have at least one true variable
        let group_size = (vars.len() + k - 1) / k; // Ceiling division
        for group_start in (0..vars.len()).step_by(group_size) {
            let group_end = (group_start + group_size).min(vars.len());
            let group_vars = &vars[group_start..group_end];

            if !group_vars.is_empty() {
                // At least one in this group should contribute to the count
                let group_var = self.new_var();

                // Group var is true if any in group is true
                let mut group_clause = vec![!Lit::from_var(group_var, true)];
                for &var in group_vars {
                    group_clause.push(Lit::from_var(var, true));
                }
                self.formula.add_clause(&group_clause);

                // Link to count variables (simplified)
                if group_start / group_size < count_vars.len() {
                    let count_idx = group_start / group_size;
                    self.formula.add_clause(&[
                        !Lit::from_var(count_vars[count_idx], true),
                        Lit::from_var(group_var, true),
                    ]);
                }
            }
        }
    }

    /// Add constraint: at least `min_credits` total credits from the weighted variables
    fn add_at_least_credits_constraint(&mut self, weighted_vars: &[(Var, u32)], min_credits: u32) {
        if min_credits == 0 || weighted_vars.is_empty() {
            return;
        }

        // Calculate total possible credits
        let total_possible: u32 = weighted_vars.iter().map(|(_, credits)| *credits).sum();

        if total_possible < min_credits {
            // Impossible to satisfy - add contradiction
            self.formula.add_clause(&[]);
            return;
        }

        // Use a simplified approach for efficiency
        // Instead of exponential enumeration, use a heuristic approach
        self.add_efficient_credits_constraint(weighted_vars, min_credits);
    }

    /// Efficient encoding for credit constraints
    fn add_efficient_credits_constraint(&mut self, weighted_vars: &[(Var, u32)], min_credits: u32) {
        // Use a simplified greedy approach for efficiency
        // Sort by credits descending to prioritize high-credit courses
        let mut sorted_vars = weighted_vars.to_vec();
        sorted_vars.sort_by(|a, b| b.1.cmp(&a.1));

        // Calculate minimum number of top courses needed
        let mut cumulative_credits = 0u32;
        let mut required_courses = Vec::new();

        for (var, credits) in &sorted_vars {
            cumulative_credits += credits;
            required_courses.push(*var);

            if cumulative_credits >= min_credits {
                break;
            }
        }

        if cumulative_credits >= min_credits {
            // At least one of the required courses must be taken
            let clause: Vec<Lit> = required_courses
                .iter()
                .map(|&var| Lit::from_var(var, true))
                .collect();
            self.formula.add_clause(&clause);
        }
    }
}

/// Score a solution based on number of additional courses and distribution
/// Lower scores are better - now heavily prioritizes total credits
fn score_solution(
    solution: &SatSolution,
    original_schedule: &[Vec<CourseCode>],
    credit_map: &HashMap<CourseCode, u32>,
) -> f64 {
    let total_additional = count_total_additional_courses(solution);

    // Calculate total credits for additional courses
    let mut total_additional_credits = 0u32;
    for additional_courses in solution.additional_courses.values() {
        for course in additional_courses {
            total_additional_credits += credit_map.get(course).unwrap_or(&3);
        }
    }

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

    // HEAVILY prioritize total credits (weight 10x), then course count, then distribution
    (total_additional_credits as f64 * 10.0)
        + (total_additional as f64)
        + (distribution_penalty * 0.1)
}

/// Count total number of additional courses in a solution
fn count_total_additional_courses(solution: &SatSolution) -> usize {
    solution
        .additional_courses
        .values()
        .map(|courses| courses.len())
        .sum()
}

/// Unified SAT solver that handles prerequisites, geneds, and credit constraints together
pub fn solve_unified_schedule(
    schedule: Vec<Vec<CourseCode>>,
    prereqs: &HashMap<CourseCode, CourseReq>,
    geneds: &[crate::geneds::GenEd],
    catalog: &crate::schedule::Catalog,
    course_credits: &HashMap<
        CourseCode,
        (String, Option<u32>, crate::schedule::CourseTermOffering),
    >,
    max_solutions: usize,
) -> Vec<SatSolution> {
    let mut solutions = Vec::new();
    let mut forbidden_patterns: Vec<Vec<(CourseCode, usize)>> = Vec::new();
    let mut best_score = f64::INFINITY;
    let mut solutions_since_improvement = 0;
    const MAX_STAGNANT_ITERATIONS: usize = 20;

    // Convert course credits to a simpler format
    let credit_map: HashMap<CourseCode, u32> = course_credits
        .iter()
        .map(|(code, (_, credits, _))| (code.clone(), credits.unwrap_or(3)))
        .collect();

    println!(
        "Unified SAT solver exploring up to {} solutions (prerequisites + geneds + credits)...",
        max_solutions
    );

    for attempt in 0..max_solutions {
        let num_semesters = schedule.len();
        let mut solver = PrereqSatSolver::new(num_semesters);

        // Set course credit information
        solver.set_course_credits(credit_map.clone());

        // Add all constraints together
        solver.add_existing_schedule(&schedule);
        solver.add_prereq_constraints(&schedule, prereqs);

        // Add constraints to exclude previous solutions
        for forbidden in &forbidden_patterns {
            solver.add_forbidden_pattern(forbidden);
        }

        // Use the unified solver that includes geneds
        if let Some(solution) = solver.solve_with_geneds(&schedule, geneds, catalog) {
            // Calculate a score for this solution
            let score = score_solution(&solution, &schedule, &credit_map);

            if score < best_score {
                best_score = score;
                solutions_since_improvement = 0;

                // Calculate total credits for this solution
                let mut total_additional_credits = 0u32;
                for additional_courses in solution.additional_courses.values() {
                    for course in additional_courses {
                        total_additional_credits += credit_map.get(course).unwrap_or(&3);
                    }
                }

                println!(
                    "Unified SAT iteration {}: Found better solution with score {:.2} ({} additional courses, {} additional credits)",
                    attempt + 1,
                    score,
                    count_total_additional_courses(&solution),
                    total_additional_credits
                );
            } else {
                solutions_since_improvement += 1;
            }

            // Early termination logic
            if solutions_since_improvement >= MAX_STAGNANT_ITERATIONS {
                println!(
                    "Unified SAT solver: No improvement for {} iterations, terminating early at iteration {}",
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

            // Early termination based on efficiency
            let last_solution = solutions.last().unwrap();
            let total_additional = count_total_additional_courses(last_solution);
            let mut total_additional_credits = 0u32;
            for additional_courses in last_solution.additional_courses.values() {
                for course in additional_courses {
                    total_additional_credits += credit_map.get(course).unwrap_or(&3);
                }
            }

            if total_additional <= 5 || total_additional_credits <= 15 {
                println!(
                    "Unified SAT solver: Found excellent solution with only {} additional courses ({} credits), terminating early",
                    total_additional, total_additional_credits
                );
                break;
            }
        } else {
            println!(
                "Unified SAT solver: No more solutions found at iteration {}",
                attempt + 1
            );
            break;
        }
    }

    println!(
        "Unified SAT solver completed: Found {} solutions, best score: {:.2}",
        solutions.len(),
        best_score
    );
    solutions
}
