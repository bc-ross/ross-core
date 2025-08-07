use crate::geneds::{GenEd, GenEdReq, are_geneds_satisfied};
use crate::prereqs::CourseReq;
use crate::schedule::{Catalog, CourseCode, CourseTermOffering, Schedule};
use anyhow::Result;
use std::any;
use std::collections::HashMap;

// Normally 18 but adjusted to 9 for small-scale testing
const MAX_OVERLOAD_SEMESTER: u32 = 9; // Maximum credits per semester

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

    // Now handle geneds for each solution
    for solution in &mut schedule_solutions {
        let temp_schedule = Schedule {
            courses: solution.clone(),
            programs: sched.programs.clone(),
            catalog: sched.catalog.clone(),
        };

        let missing_geneds = find_missing_geneds(&temp_schedule);

        // Improved gened placement: distribute courses to balance semesters
        place_geneds_balanced(solution, missing_geneds, courses);
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
/// This uses a greedy approach that considers course overlap and total prerequisite cost
fn find_optimal_gened_courses(
    unsatisfied_geneds: &[(usize, GenEd, Vec<CourseCode>)],
    catalog: &Catalog,
    already_in_schedule: &[CourseCode],
) -> Vec<CourseCode> {
    let mut selected_courses = Vec::new();
    let mut remaining_geneds = unsatisfied_geneds.to_vec();

    // Track how many courses we've selected for each gened that needs multiple courses
    let mut gened_progress: std::collections::HashMap<usize, usize> =
        std::collections::HashMap::new();

    // Initialize progress based on courses already in schedule
    for (gened_idx, gened, _) in &remaining_geneds {
        let already_satisfied_count = count_satisfied_courses_for_gened(gened, already_in_schedule);
        if already_satisfied_count > 0 {
            gened_progress.insert(*gened_idx, already_satisfied_count);
        }
    }

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

    // Greedy selection: prioritize courses with best cost/benefit ratio
    let mut iteration_count = 0;
    let max_iterations = 50; // Prevent infinite loops

    while !remaining_geneds.is_empty() && iteration_count < max_iterations {
        iteration_count += 1;

        let mut best_course = None;
        let mut best_score = f64::NEG_INFINITY;
        let mut best_geneds_affected = Vec::new();
        let mut best_total_courses = Vec::new();

        for (course, gened_indices) in &course_to_geneds {
            // Skip if this course is already selected or in the schedule
            if selected_courses.contains(course) || already_in_schedule.contains(course) {
                continue;
            }

            // Count how many unsatisfied geneds this course would help satisfy
            let mut affected_geneds = Vec::new();
            let mut benefit_score = 0.0;

            for &gened_idx in gened_indices {
                if let Some((_, gened, _)) = remaining_geneds
                    .iter()
                    .find(|(idx, _, _)| *idx == gened_idx)
                {
                    // Check if this gened can be helped by this course
                    let courses_needed = get_courses_needed_for_gened(gened);
                    let current_progress = gened_progress.get(&gened_idx).unwrap_or(&0);

                    if *current_progress < courses_needed {
                        // Check if this course actually contributes to this gened
                        if can_course_satisfy_gened(course, gened) {
                            affected_geneds.push(gened_idx);

                            // Calculate benefit: how much closer this gets us to completion
                            if courses_needed == 1 {
                                benefit_score += 1.0; // Fully satisfies the gened
                            } else {
                                // Partial satisfaction - weight by how much it helps
                                let completion_ratio =
                                    (*current_progress + 1) as f64 / courses_needed as f64;
                                benefit_score += completion_ratio;
                            }
                        }
                    }
                }
            }

            if !affected_geneds.is_empty() {
                // Calculate total courses needed (including unsatisfied prerequisites)
                // Combine already selected courses with courses already in the schedule
                let mut all_existing_courses = selected_courses.clone();
                all_existing_courses.extend_from_slice(already_in_schedule);

                let total_courses_needed =
                    calculate_total_course_cost(course, catalog, &all_existing_courses);

                // Calculate benefit/cost ratio
                let total_credits: f64 = total_courses_needed
                    .iter()
                    .map(|c| {
                        catalog
                            .courses
                            .get(c)
                            .and_then(|(_, creds, _)| *creds)
                            .unwrap_or(3) as f64
                    })
                    .sum();

                // Score = (benefit score * 1000) / (total credits needed)
                // Higher score is better (more benefit per credit)
                let score = if total_credits > 0.0 {
                    (benefit_score * 1000.0) / total_credits
                } else {
                    benefit_score * 1000.0
                };

                if score > best_score {
                    best_score = score;
                    best_course = Some(course.clone());
                    best_geneds_affected = affected_geneds;
                    best_total_courses = total_courses_needed;
                }
            }
        }

        if let Some(course) = best_course {
            // Add all needed courses (including prerequisites)
            for needed_course in &best_total_courses {
                if !selected_courses.contains(needed_course) {
                    selected_courses.push(needed_course.clone());
                }
            }

            // Update progress for affected geneds
            for &gened_idx in &best_geneds_affected {
                let current = gened_progress.get(&gened_idx).unwrap_or(&0);
                gened_progress.insert(gened_idx, current + 1);
            }

            // Check if any geneds are now fully satisfied and remove them
            let mut geneds_to_remove = Vec::new();
            for &gened_idx in &best_geneds_affected {
                if let Some((_, gened, _)) = remaining_geneds
                    .iter()
                    .find(|(idx, _, _)| *idx == gened_idx)
                {
                    let courses_needed = get_courses_needed_for_gened(gened);
                    let current_progress = gened_progress.get(&gened_idx).unwrap_or(&0);

                    if *current_progress >= courses_needed {
                        geneds_to_remove.push(gened_idx);
                    }
                }
            }

            // Remove fully satisfied geneds
            remaining_geneds.retain(|(gened_idx, _, _)| !geneds_to_remove.contains(gened_idx));

            // Update course_to_geneds map to remove fully satisfied geneds
            for geneds_list in course_to_geneds.values_mut() {
                geneds_list.retain(|idx| !geneds_to_remove.contains(idx));
            }

            // Remove the selected course from course_to_geneds to prevent re-selection
            course_to_geneds.remove(&course);

            println!(
                "Selected {} to help satisfy {} gened(s) (cost: {} total courses, score: {:.2})",
                format!("{}-{}", course.stem, course.code),
                best_geneds_affected.len(),
                best_total_courses.len(),
                best_score
            );
        } else {
            // No course found, something went wrong
            println!(
                "No more beneficial courses found, stopping gened selection (iteration {})",
                iteration_count
            );
            break;
        }

        // Add extra termination condition if we've satisfied all geneds
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

    selected_courses
}

/// Count how many courses in the given list satisfy a specific gened
fn count_satisfied_courses_for_gened(gened: &GenEd, courses: &[CourseCode]) -> usize {
    match gened {
        GenEd::Core { req, .. }
        | GenEd::Foundation { req, .. }
        | GenEd::SkillAndPerspective { req, .. } => match req {
            GenEdReq::Set(set_courses) => {
                if set_courses.iter().all(|c| courses.contains(c)) {
                    1
                } else {
                    0
                }
            }
            GenEdReq::SetOpts(opts) => opts
                .iter()
                .filter(|opt| opt.iter().all(|c| courses.contains(c)))
                .count(),
            GenEdReq::Courses {
                courses: gened_courses,
                ..
            } => gened_courses.iter().filter(|c| courses.contains(c)).count(),
            GenEdReq::Credits {
                courses: gened_courses,
                ..
            } => gened_courses.iter().filter(|c| courses.contains(c)).count(),
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
                GenEdReq::Set(_) => 1,     // Need all courses in the set (treated as 1 unit)
                GenEdReq::SetOpts(_) => 1, // Need one of the option sets
                GenEdReq::Courses { num, .. } => *num, // Need exactly num courses
                GenEdReq::Credits { .. } => 1, // Need enough courses to meet credit requirement (simplified)
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

/// Intelligently place gened courses to balance semesters
fn place_geneds_balanced(
    solution: &mut [Vec<CourseCode>],
    missing_geneds: Vec<CourseCode>,
    courses: &HashMap<CourseCode, (String, Option<u32>, CourseTermOffering)>,
) {
    // Sort semesters by current credit load (ascending)
    let mut semester_loads: Vec<(usize, u32)> = solution
        .iter()
        .enumerate()
        .map(|(idx, semester)| {
            let credits = semester
                .iter()
                .map(|course| courses.get(course).and_then(|(_, c, _)| *c).unwrap_or(3))
                .sum();
            (idx, credits)
        })
        .collect();

    // Sort by credits (lowest first) to fill lighter semesters first
    semester_loads.sort_by_key(|(_, credits)| *credits);

    // Place each gened course in the semester with the least load
    for missing_course in missing_geneds {
        let course_credits = courses
            .get(&missing_course)
            .and_then(|(_, c, _)| *c)
            .unwrap_or(3);

        // Find the semester with the lowest load that can accommodate this course
        let mut best_semester_idx = None;
        let mut min_resulting_load = u32::MAX;

        for &(sem_idx, current_load) in &semester_loads {
            let resulting_load = current_load + course_credits;

            // Prefer semesters under the overload limit
            if resulting_load <= MAX_OVERLOAD_SEMESTER && resulting_load < min_resulting_load {
                best_semester_idx = Some(sem_idx);
                min_resulting_load = resulting_load;
            }
        }

        // If no semester can accommodate without overload, place in the least loaded one anyway
        if best_semester_idx.is_none() {
            best_semester_idx = Some(semester_loads[0].0);
            min_resulting_load = semester_loads[0].1 + course_credits;
        }

        if let Some(sem_idx) = best_semester_idx {
            solution[sem_idx].push(missing_course);

            // Update the load for this semester in our tracking
            for load_entry in &mut semester_loads {
                if load_entry.0 == sem_idx {
                    load_entry.1 = min_resulting_load;
                    break;
                }
            }

            // Re-sort to maintain order by load
            semester_loads.sort_by_key(|(_, credits)| *credits);
        }
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
