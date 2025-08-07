use crate::geneds::{GenEd, GenEdReq, are_geneds_satisfied};
use crate::prereqs::CourseReq;
use crate::schedule::{Catalog, CourseCode, CourseTermOffering, Schedule};
use anyhow::Result;
use std::collections::{HashMap, HashSet};

// Normally 18 but adjusted to 18 for small-scale testing
const MAX_OVERLOAD_SEMESTER: u32 = 18; // Maximum credits per semester

/// CP-style solution that wraps SAT solver results
#[derive(Debug, Clone)]
pub struct CpSolution {
    pub schedule: Vec<Vec<CourseCode>>,
    pub total_credits: u32,
    pub additional_courses: HashMap<usize, Vec<CourseCode>>,
}

/// Simple prerequisite solver using optimization principles
/// This is currently a wrapper around the SAT solver with optimization-focused logic
pub fn solve_prereqs_cp(
    sched: &Schedule, // schedule: Vec<Vec<CourseCode>>,
                      // prereqs: &HashMap<CourseCode, CourseReq>,
                      // courses: &HashMap<CourseCode, (String, Option<u32>, CourseTermOffering)>,
) -> Result<Vec<Vec<Vec<CourseCode>>>> {
    // For now, we'll use the SAT solver as the backend but with optimization-focused logic
    // In the future, this can be replaced with a true CP/IP implementation

    use crate::prereqs_sat;
    let schedule = &sched.courses;
    let prereqs = &sched.catalog.prereqs;
    let courses = &sched.catalog.courses;

    // Use the SAT solver to get multiple valid solutions
    let sat_solutions = prereqs_sat::solve_multiple_prereqs(schedule.clone(), prereqs, 10);

    println!(
        "SAT solver found {} prerequisite solutions",
        sat_solutions.len()
    );
    if sat_solutions.is_empty() {
        println!("No SAT solutions found - returning empty schedule list");
        return Ok(vec![]); // Return empty if no solution found
    }

    // Convert SAT solutions to schedule format
    let mut schedule_solutions: Vec<Vec<Vec<CourseCode>>> = sat_solutions
        .iter()
        .map(|sat_sol| {
            let mut full_schedule = schedule.clone();

            // Add additional courses to their respective semesters
            for (sem_idx, courses) in &sat_sol.additional_courses {
                if *sem_idx < full_schedule.len() {
                    full_schedule[*sem_idx].extend(courses.clone());
                }
            }

            full_schedule
        })
        .collect();

    // Calculate geneds once for the first solution (they should be the same for all prerequisite solutions)
    let missing_geneds = if !schedule_solutions.is_empty() {
        let temp_schedule = Schedule {
            courses: schedule_solutions[0].clone(),
            programs: sched.programs.clone(),
            catalog: sched.catalog.clone(),
        };
        find_missing_geneds(&temp_schedule)
    } else {
        vec![]
    };

    // Apply the same gened courses to all solutions
    for solution in &mut schedule_solutions {
        // Improved gened placement: distribute courses to balance semesters
        place_geneds_balanced(solution, missing_geneds.clone(), courses);
    }

    // Apply optimization logic to choose the best solution
    let best_solution = find_best_solution(schedule_solutions.clone(), sched);

    match best_solution {
        Some(solution) => {
            println!("Best solution found with {} semesters", solution.len());
            for (i, sem) in solution.iter().enumerate() {
                println!("  Best Semester {}: {} courses", i, sem.len());
            }
            Ok(vec![solution])
        }
        None => {
            println!("No valid solution found!");
            Ok(vec![])
        }
    }
}

/// Get all solutions for analysis and comparison
pub fn solve_prereqs_cp_all_solutions(
    schedule: Vec<Vec<CourseCode>>,
    prereqs: &HashMap<CourseCode, CourseReq>,
    _courses: &HashMap<CourseCode, (String, Option<u32>, CourseTermOffering)>,
) -> Result<Vec<Vec<Vec<CourseCode>>>> {
    use crate::prereqs_sat;

    // Use the SAT solver to get multiple valid solutions
    let sat_solutions = prereqs_sat::solve_multiple_prereqs(schedule.clone(), prereqs, 10);

    if sat_solutions.is_empty() {
        return Ok(vec![]); // Return empty if no solution found
    }

    // Convert SAT solutions to schedule format
    let schedule_solutions: Vec<Vec<Vec<CourseCode>>> = sat_solutions
        .iter()
        .map(|sat_sol| {
            let mut full_schedule = schedule.clone();

            // Add additional courses to their respective semesters
            for (sem_idx, courses) in &sat_sol.additional_courses {
                if *sem_idx < full_schedule.len() {
                    full_schedule[*sem_idx].extend(courses.clone());
                }
            }

            full_schedule
        })
        .collect();

    Ok(schedule_solutions)
}

/// Find missing gened requirements in a schedule using smart selection
pub fn find_missing_geneds(sched: &Schedule) -> Vec<CourseCode> {
    // Check if geneds are already satisfied
    if are_geneds_satisfied(sched).unwrap_or(false) {
        return vec![];
    }

    // Get all courses currently in the schedule
    let current_courses: std::collections::HashSet<_> = sched.courses.iter().flatten().collect();

    // Find which geneds are unsatisfied and get all possible courses for each
    let mut unsatisfied_geneds = Vec::new();
    for (idx, gened) in sched.catalog.geneds.iter().enumerate() {
        if !is_gened_satisfied(gened, &current_courses, &sched.catalog) {
            let possible_courses = get_all_satisfying_courses(gened);
            unsatisfied_geneds.push((idx, gened.clone(), possible_courses));
        }
    }

    if unsatisfied_geneds.is_empty() {
        return vec![];
    }

    // Convert current courses to owned Vec for the cost calculation
    let current_courses_owned: Vec<CourseCode> = current_courses.into_iter().cloned().collect();

    // Use intelligent selection to find optimal course combination
    find_optimal_gened_courses(&unsatisfied_geneds, &sched.catalog, &current_courses_owned)
}

/// Check if a specific gened is satisfied by current courses
fn is_gened_satisfied(
    gened: &GenEd,
    current_courses: &std::collections::HashSet<&CourseCode>,
    catalog: &Catalog,
) -> bool {
    match gened {
        GenEd::Core { req, .. }
        | GenEd::Foundation { req, .. }
        | GenEd::SkillAndPerspective { req, .. } => {
            // We need to implement our own satisfaction check since fulfilled_courses is private
            match req {
                GenEdReq::Set(courses) => courses.iter().all(|c| current_courses.contains(c)),
                GenEdReq::SetOpts(opts) => opts
                    .iter()
                    .any(|opt| opt.iter().all(|c| current_courses.contains(c))),
                GenEdReq::Courses { num, courses } => {
                    let satisfied_count = courses
                        .iter()
                        .filter(|c| current_courses.contains(c))
                        .count();
                    satisfied_count >= *num
                }
                GenEdReq::Credits { num, courses } => {
                    let satisfied_credits: u32 = courses
                        .iter()
                        .filter(|c| current_courses.contains(c))
                        .filter_map(|c| catalog.courses.get(c).and_then(|(_, creds, _)| *creds))
                        .sum();
                    satisfied_credits >= *num
                }
            }
        }
    }
}

/// Get all courses that could potentially satisfy a gened
fn get_all_satisfying_courses(gened: &GenEd) -> Vec<CourseCode> {
    match gened {
        GenEd::Core { req, .. }
        | GenEd::Foundation { req, .. }
        | GenEd::SkillAndPerspective { req, .. } => {
            match req {
                GenEdReq::Set(courses) => courses.clone(),
                GenEdReq::SetOpts(opts) => {
                    // Return all courses from all options
                    opts.iter().flatten().cloned().collect()
                }
                GenEdReq::Courses { courses, .. } | GenEdReq::Credits { courses, .. } => {
                    courses.clone()
                }
            }
        }
    }
}

/// Find optimal combination of courses to satisfy all unsatisfied geneds
/// This uses constraint-aware selection that respects the same overlap rules as validation:
/// - Foundation geneds: no overlap between foundation courses
/// - Skills & Perspective geneds: limited reuse (MAX_SKILLS_AND_PERSPECTIVES times)
/// - Core geneds: can overlap with others
fn find_optimal_gened_courses(
    unsatisfied_geneds: &[(usize, GenEd, Vec<CourseCode>)],
    catalog: &Catalog,
    current_courses: &[CourseCode],
) -> Vec<CourseCode> {
    use crate::geneds::GenEd;

    println!("Finding optimal gened courses using overlap-aware backtracking...");

    // Create a comprehensive list of all possible courses for geneds
    let mut all_possible_courses = HashSet::new();
    for (_, _, possible_courses) in unsatisfied_geneds {
        all_possible_courses.extend(possible_courses.iter().cloned());
    }

    // Add current courses to the pool
    all_possible_courses.extend(current_courses.iter().cloned());

    // Convert to the format expected by backtracking functions
    let current_courses_refs: HashSet<&CourseCode> = current_courses.iter().collect();
    let all_courses_refs: HashSet<&CourseCode> = all_possible_courses.iter().collect();

    // Separate geneds by type for constraint-aware processing
    let mut core_geneds = Vec::new();
    let mut foundation_geneds = Vec::new();
    let mut skill_perspective_geneds = Vec::new();

    for (idx, gened, _) in unsatisfied_geneds {
        match gened {
            GenEd::Core { .. } => core_geneds.push((*idx, gened)),
            GenEd::Foundation { .. } => foundation_geneds.push((*idx, gened)),
            GenEd::SkillAndPerspective { .. } => skill_perspective_geneds.push((*idx, gened)),
        }
    }

    // Sort Foundation geneds by index to match validation order
    foundation_geneds.sort_by_key(|(idx, _)| *idx);

    println!(
        "Processing {} Core, {} Foundation, {} Skills&Perspective geneds",
        core_geneds.len(),
        foundation_geneds.len(),
        skill_perspective_geneds.len()
    );

    let mut selected_courses = Vec::new();

    // 1. Process Core geneds first (they have no overlap restrictions)
    for (idx, gened) in &core_geneds {
        if let GenEd::Core { req, name, .. } = gened {
            // Check if already satisfied
            if req
                .fulfilled_courses(&current_courses_refs, catalog)
                .is_some()
            {
                println!("Core gened {} ({}) already satisfied", idx, name);
                continue;
            }

            // Find courses to satisfy this gened
            if let Some(courses_needed) = req.fulfilled_courses(&all_courses_refs, catalog) {
                let new_courses: Vec<CourseCode> = courses_needed
                    .iter()
                    .filter(|course| !current_courses.contains(*course))
                    .cloned()
                    .cloned()
                    .collect();

                println!(
                    "Core gened {} ({}) needs {} new courses: {:?}",
                    idx,
                    name,
                    new_courses.len(),
                    new_courses
                );
                selected_courses.extend(new_courses);
            } else {
                println!("WARNING: Could not satisfy Core gened {} ({})", idx, name);
            }
        }
    }

    // 2. For Foundation geneds, ensure we have enough courses by selecting conservatively
    // We'll select courses for each Foundation gened individually first, then optimize overlaps
    if !foundation_geneds.is_empty() {
        println!("Pre-selecting Foundation gened courses to ensure coverage...");

        for (idx, gened) in &foundation_geneds {
            if let GenEd::Foundation { req, name, .. } = gened {
                // Check if already satisfied
                if req
                    .fulfilled_courses(&current_courses_refs, catalog)
                    .is_some()
                {
                    println!("Foundation gened {} ({}) already satisfied", idx, name);
                    continue;
                }

                // Find minimum courses needed for this specific Foundation gened
                if let Some(courses_needed) = req.fulfilled_courses(&all_courses_refs, catalog) {
                    let new_courses: Vec<CourseCode> = courses_needed
                        .iter()
                        .filter(|course| !current_courses.contains(*course))
                        .cloned()
                        .cloned()
                        .collect();

                    println!(
                        "Foundation gened {} ({}) needs {} new courses: {:?}",
                        idx,
                        name,
                        new_courses.len(),
                        new_courses
                    );
                    selected_courses.extend(new_courses);
                } else {
                    println!(
                        "WARNING: Could not satisfy Foundation gened {} ({})",
                        idx, name
                    );
                }
            }
        }
    }

    // 3. Use overlap-aware approach for Skills & Perspective geneds
    if !skill_perspective_geneds.is_empty() {
        println!("Using overlap-aware selection for Skills & Perspective geneds...");

        // Update current courses to include Foundation courses we selected
        let mut updated_current_courses = current_courses.to_vec();
        updated_current_courses.extend(selected_courses.iter().cloned());
        let updated_current_courses_refs: HashSet<&CourseCode> =
            updated_current_courses.iter().collect();

        // Handle Skills & Perspective geneds
        if let Some(skill_assignments) = find_skills_perspective_gened_assignment(
            &skill_perspective_geneds,
            &all_courses_refs,
            catalog,
        ) {
            for (idx, gened, used_courses) in skill_assignments {
                if let GenEd::SkillAndPerspective { name, .. } = gened {
                    let new_courses: Vec<CourseCode> = used_courses
                        .iter()
                        .filter(|course| !updated_current_courses.contains(*course))
                        .cloned()
                        .cloned()
                        .collect();

                    println!(
                        "Skills & Perspective gened {} ({}) assigned {} new courses: {:?}",
                        idx,
                        name,
                        new_courses.len(),
                        new_courses
                    );
                    selected_courses.extend(new_courses);
                }
            }
        } else {
            println!("WARNING: Could not find valid assignment for Skills & Perspective geneds");
        }
    }

    println!(
        "Total overlap-aware gened courses selected: {}",
        selected_courses.len()
    );
    selected_courses
}

/// Check if a gened is satisfied considering constraints
fn is_gened_fully_satisfied_constraint_aware(
    gened: &GenEd,
    all_courses: &[CourseCode],
    catalog: &Catalog,
    foundation_courses: &HashSet<CourseCode>,
    skill_perspective_usage: &HashMap<CourseCode, u8>,
) -> bool {
    let all_codes: HashSet<&CourseCode> = all_courses.iter().collect();

    match gened {
        GenEd::Core { req, .. } => req.fulfilled_courses(&all_codes, catalog).is_some(),
        GenEd::Foundation { req, .. } => {
            // Filter out courses already used by other foundation geneds
            let available_codes: HashSet<&CourseCode> = all_codes
                .iter()
                .filter(|course| !foundation_courses.contains(*course))
                .cloned()
                .collect();
            req.fulfilled_courses(&available_codes, catalog).is_some()
        }
        GenEd::SkillAndPerspective { req, .. } => {
            // Filter out courses that have been used more than 3 times
            let available_codes: HashSet<&CourseCode> = all_codes
                .iter()
                .filter(|course| skill_perspective_usage.get(*course).unwrap_or(&0) < &3)
                .cloned()
                .collect();
            req.fulfilled_courses(&available_codes, catalog).is_some()
        }
    }
}

/// Select courses to satisfy a specific GenEdReq
fn select_courses_for_requirement(
    req: &GenEdReq,
    available_courses: &[CourseCode],
    catalog: &Catalog,
) -> Vec<CourseCode> {
    match req {
        GenEdReq::Set(required_courses) => {
            // All courses in the set are required
            let mut result = Vec::new();
            for course in required_courses {
                if available_courses.contains(course) {
                    result.push(course.clone());
                } else {
                    // Can't satisfy this set requirement
                    return Vec::new();
                }
            }
            result
        }
        GenEdReq::SetOpts(options) => {
            // Pick the first option that can be fully satisfied
            for option in options {
                let can_satisfy = option
                    .iter()
                    .all(|course| available_courses.contains(course));
                if can_satisfy {
                    return option.clone();
                }
            }
            Vec::new()
        }
        GenEdReq::Courses { num, courses } => {
            // Select the first `num` available courses
            let result: Vec<_> = courses
                .iter()
                .filter(|course| available_courses.contains(course))
                .take(*num)
                .cloned()
                .collect();

            if result.len() >= *num {
                result
            } else {
                Vec::new() // Can't satisfy the requirement
            }
        }
        GenEdReq::Credits {
            num: credits,
            courses,
        } => {
            // Select courses until we have enough credits
            let mut selected = Vec::new();
            let mut total_credits = 0u32;

            for course in courses {
                if !available_courses.contains(course) {
                    continue;
                }

                let course_credits = catalog
                    .courses
                    .get(course)
                    .and_then(|(_, c, _)| *c)
                    .unwrap_or(3);

                selected.push(course.clone());
                total_credits += course_credits;

                if total_credits >= *credits {
                    break;
                }
            }

            if total_credits >= *credits {
                selected
            } else {
                Vec::new() // Can't satisfy the credit requirement
            }
        }
    }
}

/// Select courses for a Foundation requirement, being strategic to avoid conflicts with later Foundation geneds
fn select_courses_for_foundation_requirement(
    req: &GenEdReq,
    available_courses: &[CourseCode],
    catalog: &Catalog,
    all_geneds: &[&(usize, GenEd, Vec<CourseCode>)],
    current_gened_idx: usize,
) -> Vec<CourseCode> {
    match req {
        GenEdReq::Set(required_courses) => {
            // All courses in the set are required
            let mut result = Vec::new();
            for course in required_courses {
                if available_courses.contains(course) {
                    result.push(course.clone());
                } else {
                    // Can't satisfy this set requirement
                    return Vec::new();
                }
            }
            result
        }
        GenEdReq::SetOpts(options) => {
            // Pick the first option that can be fully satisfied
            for option in options {
                let can_satisfy = option
                    .iter()
                    .all(|course| available_courses.contains(course));
                if can_satisfy {
                    return option.clone();
                }
            }
            Vec::new()
        }
        GenEdReq::Courses { num, courses } => {
            // For Foundation geneds, be strategic about which courses to select
            // Prefer courses that are NOT needed by later Foundation geneds
            let mut prioritized_courses = Vec::new();
            let mut fallback_courses = Vec::new();

            // Get all later Foundation geneds
            let later_foundation_geneds: Vec<_> = all_geneds
                .iter()
                .filter(|(idx, gened, _)| {
                    *idx > current_gened_idx && matches!(gened, GenEd::Foundation { .. })
                })
                .collect();

            for course in courses {
                if !available_courses.contains(course) {
                    continue;
                }

                // Check if this course is needed by any later Foundation gened
                let needed_by_later = later_foundation_geneds
                    .iter()
                    .any(|(_, _later_gened, possible_courses)| possible_courses.contains(course));

                if needed_by_later {
                    fallback_courses.push(course.clone());
                } else {
                    prioritized_courses.push(course.clone());
                }
            }

            // Try to fulfill with prioritized courses first
            let mut selected = Vec::new();

            // Take from prioritized courses first
            for course in prioritized_courses {
                if selected.len() >= *num {
                    break;
                }
                selected.push(course);
            }

            // If we need more, take from fallback courses
            for course in fallback_courses {
                if selected.len() >= *num {
                    break;
                }
                selected.push(course);
            }

            if selected.len() >= *num {
                selected.truncate(*num);
                selected
            } else {
                Vec::new() // Can't satisfy the requirement
            }
        }
        GenEdReq::Credits {
            num: credits,
            courses,
        } => {
            // Similar strategic approach for credit-based requirements
            let mut prioritized_courses = Vec::new();
            let mut fallback_courses = Vec::new();

            // Get all later Foundation geneds
            let later_foundation_geneds: Vec<_> = all_geneds
                .iter()
                .filter(|(idx, gened, _)| {
                    *idx > current_gened_idx && matches!(gened, GenEd::Foundation { .. })
                })
                .collect();

            for course in courses {
                if !available_courses.contains(course) {
                    continue;
                }

                // Check if this course is needed by any later Foundation gened
                let needed_by_later = later_foundation_geneds
                    .iter()
                    .any(|(_, _later_gened, possible_courses)| possible_courses.contains(course));

                if needed_by_later {
                    fallback_courses.push(course.clone());
                } else {
                    prioritized_courses.push(course.clone());
                }
            }

            // Select courses until we have enough credits, preferring prioritized ones
            let mut selected = Vec::new();
            let mut total_credits = 0u32;

            // Try prioritized courses first
            for course in prioritized_courses {
                let course_credits = catalog
                    .courses
                    .get(&course)
                    .and_then(|(_, c, _)| *c)
                    .unwrap_or(3);

                selected.push(course);
                total_credits += course_credits;

                if total_credits >= *credits {
                    break;
                }
            }

            // If we need more credits, use fallback courses
            for course in fallback_courses {
                if total_credits >= *credits {
                    break;
                }

                let course_credits = catalog
                    .courses
                    .get(&course)
                    .and_then(|(_, c, _)| *c)
                    .unwrap_or(3);

                selected.push(course);
                total_credits += course_credits;
            }

            if total_credits >= *credits {
                selected
            } else {
                Vec::new() // Can't satisfy the credit requirement
            }
        }
    }
}

/// Calculate how much progress a course contributes to a specific gened
fn calculate_course_contribution(course: &CourseCode, gened: &GenEd, catalog: &Catalog) -> f64 {
    match gened {
        GenEd::Core { req, .. }
        | GenEd::Foundation { req, .. }
        | GenEd::SkillAndPerspective { req, .. } => match req {
            GenEdReq::Set(set_courses) => {
                // For Set, each course contributes 1/total_courses toward completion
                if set_courses.contains(course) {
                    1.0 / set_courses.len() as f64
                } else {
                    0.0
                }
            }
            GenEdReq::SetOpts(opts) => {
                // For SetOpts, if this course helps complete any option set, it contributes 1.0
                for opt in opts {
                    if opt.contains(course) {
                        return 1.0 / opt.len() as f64; // Contribution toward completing this option
                    }
                }
                0.0
            }
            GenEdReq::Courses {
                courses, num: _, ..
            } => {
                // For Courses, each relevant course contributes 1 unit
                if courses.contains(course) { 1.0 } else { 0.0 }
            }
            GenEdReq::Credits { courses, num, .. } => {
                // For Credits, contribution is based on credit value
                if courses.contains(course) {
                    let course_credits = catalog
                        .courses
                        .get(course)
                        .and_then(|(_, creds, _)| *creds)
                        .unwrap_or(3) as f64;
                    course_credits / *num as f64 // Contribution toward total credits needed
                } else {
                    0.0
                }
            }
        },
    }
}

/// Check if a gened is fully satisfied given current courses
fn is_gened_fully_satisfied(
    gened: &GenEd,
    current_courses: &[CourseCode],
    catalog: &Catalog,
) -> bool {
    match gened {
        GenEd::Core { req, .. }
        | GenEd::Foundation { req, .. }
        | GenEd::SkillAndPerspective { req, .. } => match req {
            GenEdReq::Set(set_courses) => {
                // All courses in the set must be present
                set_courses.iter().all(|c| current_courses.contains(c))
            }
            GenEdReq::SetOpts(opts) => {
                // At least one complete option set must be present
                opts.iter()
                    .any(|opt| opt.iter().all(|c| current_courses.contains(c)))
            }
            GenEdReq::Courses { courses, num, .. } => {
                // At least 'num' courses from the list must be present
                let count = courses
                    .iter()
                    .filter(|c| current_courses.contains(c))
                    .count();
                count >= *num
            }
            GenEdReq::Credits { courses, num, .. } => {
                // At least 'num' credits from the courses must be present
                let total_credits: u32 = courses
                    .iter()
                    .filter(|c| current_courses.contains(c))
                    .filter_map(|c| catalog.courses.get(c).and_then(|(_, creds, _)| *creds))
                    .sum();
                total_credits >= *num
            }
        },
    }
}

/// Count how many courses in the given list satisfy a specific gened
fn count_satisfied_courses_for_gened(gened: &GenEd, courses: &[CourseCode]) -> usize {
    match gened {
        GenEd::Core { req, .. }
        | GenEd::Foundation { req, .. }
        | GenEd::SkillAndPerspective { req, .. } => match req {
            GenEdReq::Set(set_courses) => {
                // For Set, return the number of courses from the set that we have
                // (but the gened is only satisfied when we have ALL of them)
                set_courses.iter().filter(|c| courses.contains(c)).count()
            }
            GenEdReq::SetOpts(opts) => {
                // For SetOpts, return 1 if any complete option set is satisfied, 0 otherwise
                if opts
                    .iter()
                    .any(|opt| opt.iter().all(|c| courses.contains(c)))
                {
                    1
                } else {
                    0
                }
            }
            GenEdReq::Courses {
                courses: gened_courses,
                ..
            } => {
                // For Courses, return the count of courses we have from the list
                gened_courses.iter().filter(|c| courses.contains(c)).count()
            }
            GenEdReq::Credits {
                courses: gened_courses,
                ..
            } => {
                // For Credits, return the count of courses we have from the list
                // (the actual credit calculation is done elsewhere)
                gened_courses.iter().filter(|c| courses.contains(c)).count()
            }
        },
    }
}

/// Find courses that can satisfy both Foundation and Skills & Perspective geneds
fn find_overlap_courses<'a>(
    foundation_geneds: &[(usize, &'a GenEd)],
    skill_perspective_geneds: &[(usize, &'a GenEd)],
    all_courses: &HashSet<&'a CourseCode>,
    _catalog: &Catalog,
) -> Vec<&'a CourseCode> {
    let mut overlap_courses = Vec::new();

    for course in all_courses {
        let mut satisfies_foundation = false;
        let mut satisfies_skill_perspective = false;

        // Check if this course can contribute to any Foundation gened
        for (_, gened) in foundation_geneds {
            if let GenEd::Foundation { req, .. } = gened {
                if course_can_contribute_to_req(course, req) {
                    satisfies_foundation = true;
                    break;
                }
            }
        }

        // Check if this course can contribute to any Skills & Perspective gened
        for (_, gened) in skill_perspective_geneds {
            if let GenEd::SkillAndPerspective { req, .. } = gened {
                if course_can_contribute_to_req(course, req) {
                    satisfies_skill_perspective = true;
                    break;
                }
            }
        }

        if satisfies_foundation && satisfies_skill_perspective {
            overlap_courses.push(*course);
        }
    }

    overlap_courses
}

/// Check if a course can contribute to a GenEdReq
fn course_can_contribute_to_req(course: &CourseCode, req: &GenEdReq) -> bool {
    match req {
        GenEdReq::Set(courses) => courses.contains(course),
        GenEdReq::SetOpts(opts) => opts.iter().any(|opt| opt.contains(course)),
        GenEdReq::Courses { courses, .. } | GenEdReq::Credits { courses, .. } => {
            courses.contains(course)
        }
    }
}

/// Optimally select overlap courses that can satisfy multiple geneds
fn select_overlap_courses_optimally<'a>(
    foundation_geneds: &[(usize, &'a GenEd)],
    skill_perspective_geneds: &[(usize, &'a GenEd)],
    overlap_courses: &[&'a CourseCode],
    current_courses: &HashSet<&'a CourseCode>,
    catalog: &Catalog,
) -> HashSet<&'a CourseCode> {
    let mut selected = HashSet::new();
    let mut satisfied_foundation_geneds = HashSet::new();
    let mut satisfied_skill_perspective_geneds = HashSet::new();

    // Greedily select overlap courses that satisfy the most unsatisfied geneds
    let mut remaining_overlap_courses = overlap_courses.to_vec();

    while !remaining_overlap_courses.is_empty() {
        let mut best_course = None;
        let mut best_score = 0;
        let mut best_idx = 0;

        for (idx, &course) in remaining_overlap_courses.iter().enumerate() {
            let mut score = 0;

            // Count how many Foundation geneds this course would help satisfy
            for (gened_idx, gened) in foundation_geneds {
                if satisfied_foundation_geneds.contains(gened_idx) {
                    continue;
                }

                if let GenEd::Foundation { req, .. } = gened {
                    if course_can_contribute_to_req(course, req) {
                        // Check if adding this course would satisfy the gened
                        let mut test_courses = current_courses.clone();
                        test_courses.extend(selected.iter().cloned());
                        test_courses.insert(course);

                        if req.fulfilled_courses(&test_courses, catalog).is_some() {
                            score += 10; // High value for completing a gened
                        } else {
                            score += 1; // Lower value for partial progress
                        }
                    }
                }
            }

            // Count how many Skills & Perspective geneds this course would help satisfy
            for (gened_idx, gened) in skill_perspective_geneds {
                if satisfied_skill_perspective_geneds.contains(gened_idx) {
                    continue;
                }

                if let GenEd::SkillAndPerspective { req, .. } = gened {
                    if course_can_contribute_to_req(course, req) {
                        // Check if adding this course would satisfy the gened
                        let mut test_courses = current_courses.clone();
                        test_courses.extend(selected.iter().cloned());
                        test_courses.insert(course);

                        if req.fulfilled_courses(&test_courses, catalog).is_some() {
                            score += 10; // High value for completing a gened
                        } else {
                            score += 1; // Lower value for partial progress
                        }
                    }
                }
            }

            if score > best_score {
                best_score = score;
                best_course = Some(course);
                best_idx = idx;
            }
        }

        if let Some(course) = best_course {
            selected.insert(course);
            remaining_overlap_courses.remove(best_idx);

            // Update satisfied geneds lists
            let mut test_courses = current_courses.clone();
            test_courses.extend(selected.iter().cloned());

            for (gened_idx, gened) in foundation_geneds {
                if let GenEd::Foundation { req, .. } = gened {
                    if req.fulfilled_courses(&test_courses, catalog).is_some() {
                        satisfied_foundation_geneds.insert(*gened_idx);
                    }
                }
            }

            for (gened_idx, gened) in skill_perspective_geneds {
                if let GenEd::SkillAndPerspective { req, .. } = gened {
                    if req.fulfilled_courses(&test_courses, catalog).is_some() {
                        satisfied_skill_perspective_geneds.insert(*gened_idx);
                    }
                }
            }

            // Stop if we've satisfied enough geneds or made good progress
            if satisfied_foundation_geneds.len() == foundation_geneds.len()
                && satisfied_skill_perspective_geneds.len() == skill_perspective_geneds.len()
            {
                break;
            }
        } else {
            // No more beneficial overlap courses
            break;
        }
    }

    selected
}

/// Find a valid assignment for Foundation geneds using backtracking (CP solver version)
fn find_foundation_gened_assignment<'a>(
    foundation_geneds: &[(usize, &'a GenEd)],
    all_codes: &HashSet<&'a CourseCode>,
    catalog: &Catalog,
) -> Option<Vec<(usize, &'a GenEd, HashSet<&'a CourseCode>)>> {
    // Use backtracking to find a valid assignment
    let mut assignments = Vec::new();
    let mut used_courses = HashSet::new();

    if backtrack_foundation_gened_assignment(
        foundation_geneds,
        all_codes,
        catalog,
        0,
        &mut assignments,
        &mut used_courses,
    ) {
        Some(assignments)
    } else {
        None
    }
}

/// Backtracking algorithm for Foundation gened assignment (CP solver version)
fn backtrack_foundation_gened_assignment<'a>(
    foundation_geneds: &[(usize, &'a GenEd)],
    all_codes: &HashSet<&'a CourseCode>,
    catalog: &Catalog,
    gened_index: usize,
    assignments: &mut Vec<(usize, &'a GenEd, HashSet<&'a CourseCode>)>,
    used_courses: &mut HashSet<&'a CourseCode>,
) -> bool {
    // Base case: all geneds assigned
    if gened_index >= foundation_geneds.len() {
        return true;
    }

    let (gened_idx, gened) = foundation_geneds[gened_index];
    if let GenEd::Foundation { req, .. } = gened {
        // Get available courses (not used by previous assignments)
        let available_codes: HashSet<_> = all_codes.difference(used_courses).cloned().collect();

        // Try to satisfy this gened with available courses
        if let Some(fulfilled_courses) = req.fulfilled_courses(&available_codes, catalog) {
            // Try this assignment
            let original_used_size = used_courses.len();
            used_courses.extend(fulfilled_courses.iter().cloned());
            assignments.push((gened_idx, gened, fulfilled_courses.clone()));

            // Recursively try to assign remaining geneds
            if backtrack_foundation_gened_assignment(
                foundation_geneds,
                all_codes,
                catalog,
                gened_index + 1,
                assignments,
                used_courses,
            ) {
                return true;
            }

            // Backtrack: undo this assignment
            assignments.pop();
            used_courses.retain(|course| !fulfilled_courses.contains(course));
            debug_assert_eq!(used_courses.len(), original_used_size);
        }
    }

    false
}

/// Find a valid assignment for Skills & Perspective geneds using backtracking (CP solver version)
fn find_skills_perspective_gened_assignment<'a>(
    skill_perspective_geneds: &[(usize, &'a GenEd)],
    all_codes: &HashSet<&'a CourseCode>,
    catalog: &Catalog,
) -> Option<Vec<(usize, &'a GenEd, HashSet<&'a CourseCode>)>> {
    // Use backtracking to find a valid assignment
    let mut assignments = Vec::new();
    let mut course_usage = HashMap::new();

    if backtrack_skills_gened_assignment(
        skill_perspective_geneds,
        all_codes,
        catalog,
        0,
        &mut assignments,
        &mut course_usage,
    ) {
        Some(assignments)
    } else {
        None
    }
}

/// Backtracking algorithm for Skills & Perspective gened assignment (CP solver version)
fn backtrack_skills_gened_assignment<'a>(
    skill_perspective_geneds: &[(usize, &'a GenEd)],
    all_codes: &HashSet<&'a CourseCode>,
    catalog: &Catalog,
    gened_index: usize,
    assignments: &mut Vec<(usize, &'a GenEd, HashSet<&'a CourseCode>)>,
    course_usage: &mut HashMap<&'a CourseCode, u8>,
) -> bool {
    // Base case: all geneds assigned
    if gened_index >= skill_perspective_geneds.len() {
        return true;
    }

    let (gened_idx, gened) = skill_perspective_geneds[gened_index];
    if let GenEd::SkillAndPerspective { req, .. } = gened {
        // Get available courses (not overused)
        let available_codes: HashSet<_> = all_codes
            .iter()
            .filter(|course| course_usage.get(*course).unwrap_or(&0) < &3)
            .cloned()
            .collect();

        // Try to satisfy this gened with available courses
        if let Some(fulfilled_courses) = req.fulfilled_courses(&available_codes, catalog) {
            // Try this assignment
            let original_usage = course_usage.clone();
            for course in &fulfilled_courses {
                *course_usage.entry(course).or_insert(0) += 1;
            }
            assignments.push((gened_idx, gened, fulfilled_courses.clone()));

            // Recursively try to assign remaining geneds
            if backtrack_skills_gened_assignment(
                skill_perspective_geneds,
                all_codes,
                catalog,
                gened_index + 1,
                assignments,
                course_usage,
            ) {
                return true;
            }

            // Backtrack: undo this assignment
            assignments.pop();
            *course_usage = original_usage;
        }
    }

    false
}

/// Intelligently place gened courses to balance semesters while respecting prerequisites
fn place_geneds_balanced(
    solution: &mut [Vec<CourseCode>],
    missing_geneds: Vec<CourseCode>,
    courses: &HashMap<CourseCode, (String, Option<u32>, CourseTermOffering)>,
) {
    // First, we need to sort the missing geneds by their prerequisite dependencies
    // Courses with no prerequisites should be placed first
    let mut geneds_with_deps = Vec::new();
    for gened_course in missing_geneds {
        geneds_with_deps.push(gened_course);
    }

    // Sort courses by dependency order - this is a simple heuristic
    geneds_with_deps.sort_by_key(|course| {
        // Count the number of prerequisite courses this course depends on
        count_total_prerequisites(course, solution)
    });

    // Place each gened course in the earliest possible semester that respects prerequisites
    for missing_course in geneds_with_deps {
        let course_credits = courses
            .get(&missing_course)
            .and_then(|(_, c, _)| *c)
            .unwrap_or(3);

        // Find the earliest semester where this course can be placed
        let earliest_valid_semester = find_earliest_valid_semester(&missing_course, solution);

        // Among semesters >= earliest_valid_semester, pick the one with lowest load
        let mut best_semester_idx = earliest_valid_semester;
        let mut min_resulting_load = u32::MAX;

        for sem_idx in earliest_valid_semester..solution.len() {
            let current_load: u32 = solution[sem_idx]
                .iter()
                .map(|course| courses.get(course).and_then(|(_, c, _)| *c).unwrap_or(3))
                .sum();
            let resulting_load = current_load + course_credits;

            // Prefer semesters under the overload limit and with lower load
            if resulting_load <= MAX_OVERLOAD_SEMESTER && resulting_load < min_resulting_load {
                best_semester_idx = sem_idx;
                min_resulting_load = resulting_load;
            }
        }

        // If no semester under the limit is found, use the earliest valid semester anyway
        if min_resulting_load == u32::MAX {
            best_semester_idx = earliest_valid_semester;
        }

        // Place the course in the selected semester
        if best_semester_idx < solution.len() {
            solution[best_semester_idx].push(missing_course);
        } else {
            // If we run out of semesters, add to the last one
            solution[solution.len() - 1].push(missing_course);
        }
    }
}

/// Count total number of prerequisite courses (simple heuristic for ordering)
fn count_total_prerequisites(course: &CourseCode, _schedule: &[Vec<CourseCode>]) -> usize {
    // Simple heuristic: courses with higher course numbers tend to have more prerequisites
    // In a real implementation, we'd traverse the actual prerequisite graph
    match &course.code {
        crate::schedule::CourseCodeSuffix::Number(num) => *num,
        crate::schedule::CourseCodeSuffix::Unique(num) => *num,
        crate::schedule::CourseCodeSuffix::Special(_) => 9999, // Put special courses last
    }
}

/// Find the earliest semester where a course can be placed without violating prerequisites
fn find_earliest_valid_semester(course: &CourseCode, _schedule: &[Vec<CourseCode>]) -> usize {
    // For now, use a simple heuristic:
    // - Basic courses (1000-1999) can go in semester 0
    // - Intermediate courses (2000-2999) need at least semester 1
    // - Advanced courses (3000+) need at least semester 2
    // This is a simplified approach; ideally we'd check actual prerequisites

    let course_num = match &course.code {
        crate::schedule::CourseCodeSuffix::Number(num) => *num,
        crate::schedule::CourseCodeSuffix::Unique(num) => *num,
        crate::schedule::CourseCodeSuffix::Special(_) => 1000, // Treat special courses as basic
    };

    if course_num < 2000 {
        0 // Basic courses can start in semester 0
    } else if course_num < 3000 {
        1 // Intermediate courses need at least semester 1
    } else {
        2 // Advanced courses need at least semester 2
    }
}

/// Find the best solution based on optimization criteria:
/// 1. Minimize total credits
/// 2. Balance credits across semesters
/// 3. Minimize number of additional courses
fn find_best_solution(
    solutions: Vec<Vec<Vec<CourseCode>>>,
    sched: &Schedule,
) -> Option<Vec<Vec<CourseCode>>> {
    if solutions.is_empty() {
        return None;
    }

    let mut best_solution = &solutions[0];
    let mut best_score = evaluate_solution(&solutions[0], sched);

    for solution in &solutions[1..] {
        let score = evaluate_solution(solution, sched);
        if score < best_score {
            best_score = score;
            best_solution = solution;
        }
    }

    Some(best_solution.clone())
}

/// Evaluate a solution based on optimization criteria
/// Returns a score where lower is better
fn evaluate_solution(solution: &[Vec<CourseCode>], sched: &Schedule) -> f64 {
    let mut total_credits = 0;
    let mut semester_credits = Vec::new();

    // Calculate total credits and per-semester credits
    for semester in solution {
        let mut semester_total = 0;
        for course_code in semester {
            if let Some((_, Some(credits), _)) = sched.catalog.courses.get(course_code) {
                semester_total += credits;
            }
        }
        semester_credits.push(semester_total);
        total_credits += semester_total;
    }

    // Calculate credit balance (variance from ideal)
    let ideal_credits_per_semester = total_credits as f64 / solution.len() as f64;
    let balance_penalty: f64 = semester_credits
        .iter()
        .map(|&credits| (credits as f64 - ideal_credits_per_semester).abs())
        .sum();

    // Scoring formula: prioritize total credits, then balance
    total_credits as f64 + (balance_penalty * 0.1)
}
