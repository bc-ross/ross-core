use crate::geneds::{GenEd, GenEdReq, are_geneds_satisfied};
use crate::prereqs::CourseReq;
use crate::schedule::{Catalog, CourseCode, CourseTermOffering, Schedule};
use anyhow::Result;
use std::collections::HashMap;

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

    if sat_solutions.is_empty() {
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

    Ok(vec![best_solution])
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
/// This uses a greedy approach that heavily favors course overlap and minimal additions
fn find_optimal_gened_courses(
    unsatisfied_geneds: &[(usize, GenEd, Vec<CourseCode>)],
    catalog: &Catalog,
    already_in_schedule: &[CourseCode],
) -> Vec<CourseCode> {
    let mut selected_courses = Vec::new();
    let mut remaining_geneds = unsatisfied_geneds.to_vec();

    // Build a map of course -> list of geneds it can satisfy
    let mut course_to_geneds: std::collections::HashMap<CourseCode, Vec<usize>> =
        std::collections::HashMap::new();

    for (gened_idx, _gened, possible_courses) in &remaining_geneds {
        for course in possible_courses {
            course_to_geneds
                .entry(course.clone())
                .or_default()
                .push(*gened_idx);
        }
    }

    // Track which geneds we've already satisfied
    let mut satisfied_geneds = std::collections::HashSet::new();

    // Initialize with already satisfied geneds
    for (gened_idx, gened, _) in &remaining_geneds {
        let current_courses: Vec<_> = already_in_schedule.iter().cloned().collect();
        if is_gened_fully_satisfied(gened, &current_courses, catalog) {
            satisfied_geneds.insert(*gened_idx);
        }
    }

    // Remove already satisfied geneds
    remaining_geneds.retain(|(gened_idx, _, _)| !satisfied_geneds.contains(gened_idx));

    let mut iteration_count = 0;
    let max_iterations = 20; // Much more conservative limit

    while !remaining_geneds.is_empty() && iteration_count < max_iterations {
        iteration_count += 1;

        let mut best_course = None;
        let mut best_score = f64::NEG_INFINITY;
        let mut best_geneds_affected = Vec::new();

        // Sort course keys to make iteration deterministic
        let mut course_keys: Vec<_> = course_to_geneds.keys().collect();
        course_keys.sort_by(|a, b| {
            a.stem.cmp(&b.stem).then_with(|| {
                a.code
                    .partial_cmp(&b.code)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
        });

        for course in course_keys {
            let gened_indices = &course_to_geneds[course];

            // Skip if this course is already selected or in the schedule
            if selected_courses.contains(course) || already_in_schedule.contains(course) {
                continue;
            }

            // Count how many unsatisfied geneds this course would help satisfy
            let mut affected_geneds = Vec::new();
            let mut geneds_that_would_be_completed = 0;

            for &gened_idx in gened_indices {
                if satisfied_geneds.contains(&gened_idx) {
                    continue; // Skip already satisfied geneds
                }

                if let Some((_, gened, _)) = remaining_geneds
                    .iter()
                    .find(|(idx, _, _)| *idx == gened_idx)
                {
                    if can_course_satisfy_gened(course, gened) {
                        // Check if this course would complete the gened
                        let mut test_courses = already_in_schedule.to_vec();
                        test_courses.extend(selected_courses.iter().cloned());
                        test_courses.push(course.clone());

                        if is_gened_fully_satisfied(gened, &test_courses, catalog) {
                            geneds_that_would_be_completed += 1;
                        }
                        affected_geneds.push(gened_idx);
                    }
                }
            }

            if !affected_geneds.is_empty() {
                // Score heavily favors courses that:
                // 1. Complete multiple geneds (massive bonus)
                // 2. Affect many geneds
                // 3. Have low credit cost
                let course_credits = catalog
                    .courses
                    .get(course)
                    .and_then(|(_, creds, _)| *creds)
                    .unwrap_or(3) as f64;

                // Heavily favor courses that complete geneds, especially multiple ones
                let completion_bonus = if geneds_that_would_be_completed > 0 {
                    1000.0
                        * geneds_that_would_be_completed as f64
                        * geneds_that_would_be_completed as f64
                } else {
                    0.0
                };

                // Bonus for affecting multiple geneds (even if not completing them)
                let overlap_bonus = (affected_geneds.len() as f64 - 1.0) * 100.0;

                // Score = completion bonus + overlap bonus - credit penalty
                let score = completion_bonus + overlap_bonus - course_credits;

                if score > best_score {
                    best_score = score;
                    best_course = Some(course.clone());
                    best_geneds_affected = affected_geneds;
                }
            }
        }

        if let Some(course) = best_course {
            selected_courses.push(course.clone());

            // Check which geneds this course completes and mark them as satisfied
            let mut test_courses = already_in_schedule.to_vec();
            test_courses.extend(selected_courses.iter().cloned());

            for &gened_idx in &best_geneds_affected {
                if let Some((_, gened, _)) = remaining_geneds
                    .iter()
                    .find(|(idx, _, _)| *idx == gened_idx)
                {
                    if is_gened_fully_satisfied(gened, &test_courses, catalog) {
                        satisfied_geneds.insert(gened_idx);
                    }
                }
            }

            // Remove satisfied geneds from remaining list
            remaining_geneds.retain(|(gened_idx, _, _)| !satisfied_geneds.contains(gened_idx));

            // Update course_to_geneds map to remove satisfied geneds
            for geneds_list in course_to_geneds.values_mut() {
                geneds_list.retain(|idx| !satisfied_geneds.contains(idx));
            }

            println!(
                "Selected {} to help satisfy {} gened(s) (would complete {} geneds, score: {:.2})",
                format!("{}-{}", course.stem, course.code),
                best_geneds_affected.len(),
                best_geneds_affected
                    .iter()
                    .filter(|&&gened_idx| satisfied_geneds.contains(&gened_idx))
                    .count(),
                best_score
            );
        } else {
            println!(
                "No more beneficial courses found, stopping gened selection (iteration {})",
                iteration_count
            );
            break;
        }

        if remaining_geneds.is_empty() {
            println!("All geneds satisfied after {} iterations", iteration_count);
            break;
        }
    }

    if iteration_count >= max_iterations {
        println!(
            "Warning: Reached maximum iterations ({}) in gened selection",
            max_iterations
        );
    }

    println!("Total gened courses selected: {}", selected_courses.len());
    selected_courses
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

/// Check if a specific course can contribute to satisfying a gened
fn can_course_satisfy_gened(course: &CourseCode, gened: &GenEd) -> bool {
    match gened {
        GenEd::Core { req, .. }
        | GenEd::Foundation { req, .. }
        | GenEd::SkillAndPerspective { req, .. } => match req {
            GenEdReq::Set(courses) => courses.contains(course),
            GenEdReq::SetOpts(opts) => opts.iter().any(|opt| opt.contains(course)),
            GenEdReq::Courses { courses, .. } | GenEdReq::Credits { courses, .. } => {
                courses.contains(course)
            }
        },
    }
}

/// Get the number of courses needed to satisfy a gened
fn get_courses_needed_for_gened(gened: &GenEd) -> usize {
    match gened {
        GenEd::Core { req, .. }
        | GenEd::Foundation { req, .. }
        | GenEd::SkillAndPerspective { req, .. } => {
            match req {
                GenEdReq::Set(courses) => courses.len(), // Need all courses in the set
                GenEdReq::SetOpts(opts) => {
                    // Need all courses from one of the option sets (use the smallest set as estimate)
                    opts.iter().map(|opt| opt.len()).min().unwrap_or(1)
                }
                GenEdReq::Courses { num, .. } => *num, // Need exactly num courses
                GenEdReq::Credits { courses, num } => {
                    // For credits, estimate conservatively - assume we need most/all courses
                    // This is used for rough estimation in some algorithms
                    // The actual credit requirement is handled in other functions
                    std::cmp::min(courses.len(), (*num as usize / 3).max(1)) // Assume 3 credits per course
                }
            }
        }
    }
}

/// Calculate the total courses needed for a given course (including unsatisfied prerequisites)
/// Returns only courses that aren't already selected or don't satisfy other needed geneds
fn calculate_total_course_cost(
    course: &CourseCode,
    catalog: &Catalog,
    already_selected: &[CourseCode],
) -> Vec<CourseCode> {
    let mut needed_courses = Vec::new();
    let mut to_process = vec![course.clone()];
    let mut processed = std::collections::HashSet::new();

    while let Some(current_course) = to_process.pop() {
        if processed.contains(&current_course) || already_selected.contains(&current_course) {
            continue;
        }

        processed.insert(current_course.clone());
        needed_courses.push(current_course.clone());

        // Check if this course has prerequisites
        if let Some(prereq) = catalog.prereqs.get(&current_course) {
            let prereq_courses = extract_prerequisite_courses(prereq);
            for prereq_course in prereq_courses {
                if !processed.contains(&prereq_course) && !already_selected.contains(&prereq_course)
                {
                    to_process.push(prereq_course);
                }
            }
        }
    }

    needed_courses
}

/// Calculate current progress for a gened based on courses already taken
fn calculate_gened_progress(
    gened: &GenEd,
    current_courses: &[CourseCode],
    catalog: &Catalog,
) -> f64 {
    match gened {
        GenEd::Core { req, .. }
        | GenEd::Foundation { req, .. }
        | GenEd::SkillAndPerspective { req, .. } => match req {
            GenEdReq::Set(set_courses) => {
                // For Set, progress is the fraction of courses we have from the set
                let satisfied_count = set_courses
                    .iter()
                    .filter(|c| current_courses.contains(c))
                    .count();
                satisfied_count as f64 / set_courses.len() as f64
            }
            GenEdReq::SetOpts(opts) => {
                // For SetOpts, we check if any complete option set is satisfied
                for opt in opts {
                    if opt.iter().all(|c| current_courses.contains(c)) {
                        return 1.0; // One complete option set is satisfied
                    }
                }
                // If no complete set is satisfied, find the best partial progress
                let mut best_progress: f64 = 0.0;
                for opt in opts {
                    let satisfied_count =
                        opt.iter().filter(|c| current_courses.contains(c)).count();
                    let progress = satisfied_count as f64 / opt.len() as f64;
                    best_progress = best_progress.max(progress);
                }
                best_progress
            }
            GenEdReq::Courses {
                courses, num: _, ..
            } => {
                // For Courses, progress is the count of courses we have (up to the required number)
                let satisfied_count = courses
                    .iter()
                    .filter(|c| current_courses.contains(c))
                    .count();
                satisfied_count as f64
            }
            GenEdReq::Credits {
                courses, num: _, ..
            } => {
                // For Credits, progress is the total credits we have from the course list
                let total_credits: u32 = courses
                    .iter()
                    .filter(|c| current_courses.contains(c))
                    .filter_map(|c| catalog.courses.get(c).and_then(|(_, creds, _)| *creds))
                    .sum();
                total_credits as f64
            }
        },
    }
}

/// Get the target progress value for a gened (when it's considered fully satisfied)
fn get_gened_target_progress(gened: &GenEd, _catalog: &Catalog) -> f64 {
    match gened {
        GenEd::Core { req, .. }
        | GenEd::Foundation { req, .. }
        | GenEd::SkillAndPerspective { req, .. } => match req {
            GenEdReq::Set(_) => 1.0,                      // Need 100% of the set
            GenEdReq::SetOpts(_) => 1.0,                  // Need one complete option set (100%)
            GenEdReq::Courses { num, .. } => *num as f64, // Need exactly 'num' courses
            GenEdReq::Credits { num, .. } => *num as f64, // Need exactly 'num' credits
        },
    }
}

/// Recursively add prerequisites for the given courses
fn expand_with_prerequisites(courses: Vec<CourseCode>, catalog: &Catalog) -> Vec<CourseCode> {
    let mut all_needed_courses = Vec::new();
    let mut to_process = courses;
    let mut processed = std::collections::HashSet::new();

    while let Some(course) = to_process.pop() {
        if processed.contains(&course) {
            continue;
        }

        processed.insert(course.clone());
        all_needed_courses.push(course.clone());

        // Check if this course has prerequisites
        if let Some(prereq) = catalog.prereqs.get(&course) {
            let prereq_courses = extract_prerequisite_courses(prereq);
            for prereq_course in prereq_courses {
                if !processed.contains(&prereq_course) {
                    to_process.push(prereq_course);
                }
            }
        }
    }

    // Return in dependency order (prerequisites first)
    all_needed_courses.reverse();
    all_needed_courses
}

/// Extract all courses mentioned in a prerequisite requirement
fn extract_prerequisite_courses(req: &CourseReq) -> Vec<CourseCode> {
    match req {
        CourseReq::PreCourse(course) | CourseReq::CoCourse(course) => vec![course.clone()],
        CourseReq::PreCourseGrade(course, _) | CourseReq::CoCourseGrade(course, _) => {
            vec![course.clone()]
        }
        CourseReq::And(reqs) | CourseReq::Or(reqs) => {
            reqs.iter().flat_map(extract_prerequisite_courses).collect()
        }
        CourseReq::Program(_) | CourseReq::Instructor | CourseReq::None => vec![],
    }
}

/// Find the best solution based on optimization criteria:
/// 1. Minimize total credits
/// 2. Balance credits across semesters
/// 3. Minimize number of additional courses
fn find_best_solution(
    solutions: Vec<Vec<Vec<CourseCode>>>,
    ref_sched: &Schedule,
) -> Vec<Vec<CourseCode>> {
    if solutions.is_empty() {
        return vec![];
    }

    if solutions.len() == 1 {
        return solutions.into_iter().next().unwrap();
    }

    // Score each solution based on optimization criteria
    let mut scored_solutions: Vec<(f64, Vec<Vec<CourseCode>>)> = solutions
        .into_iter()
        .map(|solution| {
            let score = calculate_solution_score(&solution, ref_sched);
            (score, solution)
        })
        .collect();

    // Sort by score (lower is better)
    scored_solutions.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // Return the best solution
    scored_solutions.into_iter().next().unwrap().1
}

/// Calculate a score for a solution (lower is better)
/// Combines multiple optimization objectives
fn calculate_solution_score(solution: &[Vec<CourseCode>], ref_sched: &Schedule) -> f64 {
    let mut total_credits = 0;
    let mut semester_credits = Vec::new();
    let mut total_courses = 0;

    let courses = &ref_sched.catalog.courses;

    // Calculate credits per semester
    for semester in solution {
        let mut sem_credits = 0;
        for course in semester {
            total_courses += 1;
            if let Some((_, Some(credits), _)) = courses.get(course) {
                sem_credits += credits;
                total_credits += credits;
            } else {
                sem_credits += 3; // Default assumption
                total_credits += 3;
            }
        }
        semester_credits.push(sem_credits);
    }

    // Objective 1: Minimize total credits (weight: 1000)
    let credit_penalty = total_credits as f64 * 1000.0;

    // Objective 2: Balance semesters (weight: 100)
    let balance_penalty = if !semester_credits.is_empty() {
        let avg_credits = total_credits as f64 / semester_credits.len() as f64;
        let variance: f64 = semester_credits
            .iter()
            .map(|&credits| {
                let diff = credits as f64 - avg_credits;
                diff * diff
            })
            .sum::<f64>()
            / semester_credits.len() as f64;
        variance * 100.0
    } else {
        0.0
    };

    // Objective 3: Minimize total courses (weight: 10)
    let course_penalty = total_courses as f64 * 10.0;

    // Objective 4: Ensure all semester are under 18 credits (oneshot)
    let overload_penalty: f64 = semester_credits
        .iter()
        .filter(|&&credits| credits > MAX_OVERLOAD_SEMESTER)
        .map(|&credits| (credits - MAX_OVERLOAD_SEMESTER) as f64 * 100000.0) // Hefty penalty for overload
        .sum();

    // Objective 5: Ensure schedule is valid (oneshot)
    let validation_penalty: f64 = !(Schedule {
        courses: solution.to_vec(),
        programs: ref_sched.programs.clone(),
        catalog: ref_sched.catalog.clone(),
    })
    .is_valid()
    .unwrap() as u8 as f64
        * 1000000.0;

    credit_penalty + balance_penalty + course_penalty + overload_penalty + validation_penalty
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
    // - Intermediate courses (2000-2999) can go in semester 1
    // - Advanced courses (3000+) can go in semester 2
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

/// Test function for the CP solver that uses optimization principles
pub fn test_cp_solver() {
    use crate::geneds::{GenEd, GenEdReq};
    use crate::schedule::CourseCode;
    use std::collections::HashMap;

    // Create test courses
    let math_calc1 = CourseCode {
        stem: "MATH".to_string(),
        code: 1350.into(),
    };
    let math_calc2 = CourseCode {
        stem: "MATH".to_string(),
        code: 1360.into(),
    };
    let phys_basic = CourseCode {
        stem: "PHYS".to_string(),
        code: 1500.into(),
    };
    let phys_mech = CourseCode {
        stem: "PHYS".to_string(),
        code: 2100.into(),
    };
    let phys_em = CourseCode {
        stem: "PHYS".to_string(),
        code: 2200.into(),
    };
    let math_linalg = CourseCode {
        stem: "MATH".to_string(),
        code: 2400.into(),
    };

    // Add some gened courses
    let engl_comp = CourseCode {
        stem: "ENGL".to_string(),
        code: 1100.into(),
    };
    let theo_intro = CourseCode {
        stem: "THEO".to_string(),
        code: 1100.into(),
    };
    let phil_natural = CourseCode {
        stem: "PHIL".to_string(),
        code: 2100.into(),
    };
    // Add a multi-gened course that can satisfy both Foundation and SkillAndPerspective
    let theo_advanced = CourseCode {
        stem: "THEO".to_string(),
        code: 3100.into(),
    };
    // Add a gened course with prerequisites
    let phil_ethics = CourseCode {
        stem: "PHIL".to_string(),
        code: 3200.into(),
    };
    // Add a simple alternative that doesn't need prerequisites
    let theo_simple = CourseCode {
        stem: "THEO".to_string(),
        code: 2000.into(),
    };

    // Create a schedule that violates prerequisites and is missing geneds
    let schedule = vec![
        vec![math_calc1.clone()], // Semester 0: Calc 1
        vec![],                   // Semester 1: Empty
        vec![phys_em.clone()],    // Semester 2: E&M Physics (missing prereqs!)
    ];

    // Set up prerequisites (including for gened courses)
    let mut prereqs = HashMap::new();
    prereqs.insert(math_calc2.clone(), CourseReq::PreCourse(math_calc1.clone()));
    prereqs.insert(phys_mech.clone(), CourseReq::PreCourse(math_calc1.clone()));
    prereqs.insert(
        phys_em.clone(),
        CourseReq::Or(vec![
            CourseReq::And(vec![
                CourseReq::PreCourse(phys_mech.clone()),
                CourseReq::PreCourse(math_calc2.clone()),
            ]),
            CourseReq::PreCourse(math_linalg.clone()),
        ]),
    );
    // Add prerequisite for advanced theology (requires intro theology)
    prereqs.insert(
        theo_advanced.clone(),
        CourseReq::PreCourse(theo_intro.clone()),
    );
    // Add prerequisite for ethics (requires natural philosophy)
    prereqs.insert(
        phil_ethics.clone(),
        CourseReq::PreCourse(phil_natural.clone()),
    );

    // Set up geneds (enhanced example with overlapping courses and prerequisites)
    let geneds = vec![
        GenEd::Core {
            name: "English Composition".to_string(),
            req: GenEdReq::Courses {
                num: 1,
                courses: vec![engl_comp.clone()],
            },
        },
        GenEd::Core {
            name: "Intro to Theology".to_string(),
            req: GenEdReq::Courses {
                num: 1,
                courses: vec![theo_intro.clone()],
            },
        },
        GenEd::Foundation {
            name: "Faith Foundation".to_string(),
            req: GenEdReq::Courses {
                num: 1,
                courses: vec![theo_advanced.clone(), phil_natural.clone()], // Multiple options
            },
        },
        GenEd::SkillAndPerspective {
            name: "Ethics and Philosophy".to_string(),
            req: GenEdReq::Courses {
                num: 2,
                courses: vec![
                    theo_advanced.clone(), // Satisfies 2 geneds but needs prereq
                    phil_ethics.clone(),   // Needs prereq (phil_natural)
                    theo_simple.clone(),   // Simple option with no prereqs!
                ],
            },
        },
    ];

    // Course catalog
    let mut courses = HashMap::new();
    courses.insert(
        math_calc1.clone(),
        ("Calculus I".to_string(), Some(4), CourseTermOffering::Both),
    );
    courses.insert(
        math_calc2.clone(),
        ("Calculus II".to_string(), Some(4), CourseTermOffering::Both),
    );
    courses.insert(
        phys_mech.clone(),
        (
            "Physics Mechanics".to_string(),
            Some(3),
            CourseTermOffering::Both,
        ),
    );
    courses.insert(
        phys_em.clone(),
        ("Physics E&M".to_string(), Some(3), CourseTermOffering::Both),
    );
    courses.insert(
        math_linalg.clone(),
        (
            "Math Linear Algebra".to_string(),
            Some(3),
            CourseTermOffering::Both,
        ),
    );
    courses.insert(
        phys_basic.clone(),
        (
            "Physics Basics".to_string(),
            Some(3),
            CourseTermOffering::Both,
        ),
    );
    // Add gened courses to catalog
    courses.insert(
        engl_comp.clone(),
        (
            "English Composition".to_string(),
            Some(3),
            CourseTermOffering::Both,
        ),
    );
    courses.insert(
        theo_intro.clone(),
        (
            "Introduction to Theology".to_string(),
            Some(3),
            CourseTermOffering::Both,
        ),
    );
    courses.insert(
        phil_natural.clone(),
        (
            "Natural Philosophy".to_string(),
            Some(3),
            CourseTermOffering::Both,
        ),
    );
    courses.insert(
        theo_advanced.clone(),
        (
            "Advanced Theology".to_string(),
            Some(3),
            CourseTermOffering::Both,
        ),
    );
    courses.insert(
        phil_ethics.clone(),
        (
            "Philosophy Ethics".to_string(),
            Some(4),
            CourseTermOffering::Both,
        ),
    );
    courses.insert(
        theo_simple.clone(),
        (
            "Simple Theology".to_string(),
            Some(3),
            CourseTermOffering::Both,
        ),
    );

    solve_schedule_cp(&Schedule {
        courses: schedule,
        programs: vec![],
        catalog: Catalog {
            geneds: geneds,
            prereqs: prereqs,
            programs: vec![],
            courses: courses,
            low_year: 0,
        },
    })
    .unwrap();
}

pub fn solve_schedule_cp(
    sched: &Schedule,
    // schedule: Vec<Vec<CourseCode>>,
    // prereqs: &HashMap<CourseCode, CourseReq>,
    // courses: &HashMap<CourseCode, (String, Option<u32>, CourseTermOffering)>,
) -> Result<Vec<Vec<CourseCode>>> {
    let schedule = &sched.courses;
    let prereqs = &sched.catalog.prereqs;
    let courses = &sched.catalog.courses;

    println!("Testing optimization-focused CP solver:");
    println!("Original schedule with prerequisite violations:");
    for (i, sem) in schedule.iter().enumerate() {
        println!("  Semester {}: {:?}", i, sem);
    }

    // First, show all solutions found (prerequisite solutions only)
    match solve_prereqs_cp_all_solutions(schedule.clone(), &prereqs, &courses) {
        Ok(all_solutions) => {
            println!(
                "\nCP solver found {} prerequisite solution(s):",
                all_solutions.len()
            );
            for (sol_idx, solution) in all_solutions.iter().enumerate() {
                println!("Prereq Solution {}:", sol_idx + 1);
                let mut total_credits = 0;
                for (sem_idx, semester) in solution.iter().enumerate() {
                    let sem_credits: u32 = semester
                        .iter()
                        .map(|course| courses.get(course).and_then(|(_, c, _)| *c).unwrap_or(3))
                        .sum();
                    total_credits += sem_credits;
                    println!(
                        "  Semester {}: {:?} ({} credits)",
                        sem_idx, semester, sem_credits
                    );
                }
                println!("  Total credits: {}", total_credits);

                // Check if geneds are satisfied
                let temp_schedule = Schedule {
                    courses: solution.clone(),
                    programs: vec![],
                    catalog: sched.catalog.clone(),
                };
                let geneds_satisfied = are_geneds_satisfied(&temp_schedule).unwrap_or(false);
                println!("  Geneds satisfied: {}", geneds_satisfied);

                // Check if geneds are satisfied
                let temp_schedule = Schedule {
                    courses: solution.clone(),
                    programs: vec![],
                    catalog: sched.catalog.clone(),
                };
                let geneds_satisfied = are_geneds_satisfied(&temp_schedule).unwrap_or(false);
                println!("  Geneds satisfied: {}", geneds_satisfied);

                // Calculate solution score
                let score = calculate_solution_score(solution, sched);
                println!("  Optimization score: {:.2}", score);
            }
        }
        Err(e) => {
            println!("CP solver failed to get prerequisite solutions: {}", e);
        }
    }

    // Then show the optimized choice (with geneds)
    println!("\nNow finding the BEST solution via optimization (with geneds):");
    solve_prereqs_cp(sched).and_then(|mut solutions| {
        println!("Optimized solution chosen:");
        for (sol_idx, solution) in solutions.iter().enumerate() {
            println!("Best Solution {}:", sol_idx + 1);
            let mut total_credits = 0;
            for (sem_idx, semester) in solution.iter().enumerate() {
                let sem_credits: u32 = semester
                    .iter()
                    .map(|course| courses.get(course).and_then(|(_, c, _)| *c).unwrap_or(3))
                    .sum();
                total_credits += sem_credits;
                println!(
                    "  Semester {}: {:?} ({} credits)",
                    sem_idx, semester, sem_credits
                );
            }
            println!("  Total credits: {}", total_credits);

            // Calculate solution score
            let score = calculate_solution_score(solution, sched);
            println!("  Optimization score: {:.2}", score);
            println!(
                "  Valid schedule? {}",
                Schedule {
                    courses: solution.clone(),
                    programs: sched.programs.clone(),
                    catalog: sched.catalog.clone(),
                }
                .is_valid()
                .unwrap_or(false)
            );
        }
        solutions
            .pop()
            .ok_or(anyhow::anyhow!("No optimized solution found"))
    })
}

/// Test specifically for multi-course geneds
pub fn test_multi_course_gened() {
    use crate::geneds::{GenEd, GenEdReq};
    use crate::schedule::CourseCode;
    use std::collections::HashMap;

    println!("=== Testing Multi-Course Gened Functionality ===");

    // Create simple courses
    let course_a = CourseCode {
        stem: "TEST".to_string(),
        code: 100.into(),
    };
    let course_b = CourseCode {
        stem: "TEST".to_string(),
        code: 200.into(),
    };
    let course_c = CourseCode {
        stem: "TEST".to_string(),
        code: 300.into(),
    };

    // Empty schedule
    let schedule = vec![vec![], vec![], vec![]];

    // No prerequisites
    let prereqs = HashMap::new();

    // Create a gened that requires 2 courses
    let geneds = vec![GenEd::Core {
        name: "Multi-Course Test".to_string(),
        req: GenEdReq::Courses {
            num: 2,
            courses: vec![course_a.clone(), course_b.clone(), course_c.clone()],
        },
    }];

    // Course catalog
    let mut courses = HashMap::new();
    courses.insert(
        course_a.clone(),
        (
            "Test Course A".to_string(),
            Some(3),
            CourseTermOffering::Both,
        ),
    );
    courses.insert(
        course_b.clone(),
        (
            "Test Course B".to_string(),
            Some(3),
            CourseTermOffering::Both,
        ),
    );
    courses.insert(
        course_c.clone(),
        (
            "Test Course C".to_string(),
            Some(3),
            CourseTermOffering::Both,
        ),
    );

    let test_schedule = Schedule {
        courses: schedule,
        programs: vec![],
        catalog: Catalog {
            geneds: geneds,
            prereqs: prereqs,
            programs: vec![],
            courses: courses,
            low_year: 0,
        },
    };

    // Test the gened finding logic
    let missing_geneds = find_missing_geneds(&test_schedule);
    println!(
        "Missing geneds returned {} courses: {:?}",
        missing_geneds.len(),
        missing_geneds
            .iter()
            .map(|c| format!("{}-{}", c.stem, c.code))
            .collect::<Vec<_>>()
    );

    // Should return exactly 2 courses since the gened requires 2
    if missing_geneds.len() == 2 {
        println!("✓ Correctly identified need for 2 courses");
    } else {
        println!(
            "✗ Expected 2 courses but got {} courses",
            missing_geneds.len()
        );
    }

    // Test the full solver
    match solve_prereqs_cp(&test_schedule) {
        Ok(solutions) => {
            if let Some(solution) = solutions.first() {
                let final_schedule = Schedule {
                    courses: solution.clone(),
                    programs: vec![],
                    catalog: test_schedule.catalog.clone(),
                };

                let total_courses: usize = solution.iter().map(|sem| sem.len()).sum();
                println!("Final solution has {} total courses", total_courses);

                // Check if geneds are satisfied
                let geneds_satisfied = are_geneds_satisfied(&final_schedule).unwrap_or(false);
                println!("Geneds satisfied: {}", geneds_satisfied);

                if geneds_satisfied && total_courses >= 2 {
                    println!("✓ Multi-course gened test PASSED");
                } else {
                    println!("✗ Multi-course gened test FAILED");
                }
            } else {
                println!("✗ No solution found");
            }
        }
        Err(e) => {
            println!("✗ Solver error: {}", e);
        }
    }

    println!("=== End Multi-Course Gened Test ===\n");
}

/// Test function for all GenEdReq variants
pub fn test_all_gened_variants() {
    use crate::geneds::{GenEd, GenEdReq};
    use crate::schedule::CourseCode;
    use std::collections::HashMap;

    println!("=== Testing All GenEdReq Variants ===");

    // Create test courses
    let course_a = CourseCode {
        stem: "TEST".to_string(),
        code: 100.into(),
    };
    let course_b = CourseCode {
        stem: "TEST".to_string(),
        code: 200.into(),
    };
    let course_c = CourseCode {
        stem: "TEST".to_string(),
        code: 300.into(),
    };
    let course_d = CourseCode {
        stem: "TEST".to_string(),
        code: 400.into(),
    };
    let course_e = CourseCode {
        stem: "MATH".to_string(),
        code: 100.into(),
    };
    let course_f = CourseCode {
        stem: "MATH".to_string(),
        code: 200.into(),
    };

    // Empty schedule
    let schedule = vec![vec![], vec![], vec![]];
    let prereqs = HashMap::new();

    // Create geneds with different requirement types
    let geneds = vec![
        // Set: requires ALL courses in the set
        GenEd::Core {
            name: "Required Set".to_string(),
            req: GenEdReq::Set(vec![course_a.clone(), course_b.clone()]),
        },
        // SetOpts: requires ALL courses from ONE of the option sets
        GenEd::Foundation {
            name: "Set Options".to_string(),
            req: GenEdReq::SetOpts(vec![
                vec![course_c.clone()],                   // Option 1: just course C
                vec![course_d.clone(), course_e.clone()], // Option 2: courses D and E
            ]),
        },
        // Courses: requires 2 courses from the list
        GenEd::SkillAndPerspective {
            name: "Multi-Course".to_string(),
            req: GenEdReq::Courses {
                num: 2,
                courses: vec![course_c.clone(), course_d.clone(), course_e.clone()],
            },
        },
        // Credits: requires 6 credits from the list
        GenEd::Core {
            name: "Credit Based".to_string(),
            req: GenEdReq::Credits {
                num: 6,
                courses: vec![course_f.clone(), course_a.clone(), course_b.clone()],
            },
        },
    ];

    // Course catalog with different credit values
    let mut courses = HashMap::new();
    courses.insert(
        course_a.clone(),
        (
            "Test Course A".to_string(),
            Some(3),
            CourseTermOffering::Both,
        ),
    );
    courses.insert(
        course_b.clone(),
        (
            "Test Course B".to_string(),
            Some(3),
            CourseTermOffering::Both,
        ),
    );
    courses.insert(
        course_c.clone(),
        (
            "Test Course C".to_string(),
            Some(4),
            CourseTermOffering::Both,
        ),
    );
    courses.insert(
        course_d.clone(),
        (
            "Test Course D".to_string(),
            Some(2),
            CourseTermOffering::Both,
        ),
    );
    courses.insert(
        course_e.clone(),
        (
            "Math Course E".to_string(),
            Some(4),
            CourseTermOffering::Both,
        ),
    );
    courses.insert(
        course_f.clone(),
        (
            "Math Course F".to_string(),
            Some(3),
            CourseTermOffering::Both,
        ),
    );

    let test_schedule = Schedule {
        courses: schedule,
        programs: vec![],
        catalog: Catalog {
            geneds: geneds,
            prereqs: prereqs,
            programs: vec![],
            courses: courses,
            low_year: 0,
        },
    };

    // Test the gened finding logic
    let missing_geneds = find_missing_geneds(&test_schedule);
    println!(
        "Missing geneds returned {} courses: {:?}",
        missing_geneds.len(),
        missing_geneds
            .iter()
            .map(|c| format!("{}-{}", c.stem, c.code))
            .collect::<Vec<_>>()
    );

    // Test the full solver
    match solve_prereqs_cp(&test_schedule) {
        Ok(solutions) => {
            if let Some(solution) = solutions.first() {
                let final_schedule = Schedule {
                    courses: solution.clone(),
                    programs: vec![],
                    catalog: test_schedule.catalog.clone(),
                };

                let total_courses: usize = solution.iter().map(|sem| sem.len()).sum();
                println!("Final solution has {} total courses", total_courses);

                // Check if geneds are satisfied
                let geneds_satisfied = are_geneds_satisfied(&final_schedule).unwrap_or(false);
                println!("Geneds satisfied: {}", geneds_satisfied);

                if geneds_satisfied {
                    println!("✓ All GenEdReq variants test PASSED");
                } else {
                    println!("✗ All GenEdReq variants test FAILED - geneds not satisfied");
                }

                // Show the final schedule
                for (sem_idx, semester) in solution.iter().enumerate() {
                    if !semester.is_empty() {
                        println!(
                            "  Semester {}: {:?}",
                            sem_idx,
                            semester
                                .iter()
                                .map(|c| format!("{}-{}", c.stem, c.code))
                                .collect::<Vec<_>>()
                        );
                    }
                }
            } else {
                println!("✗ No solution found");
            }
        }
        Err(e) => {
            println!("✗ Solver error: {}", e);
        }
    }

    println!("=== End All GenEdReq Variants Test ===\n");
}
