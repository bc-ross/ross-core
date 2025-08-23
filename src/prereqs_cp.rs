use crate::geneds::{ElectiveReq, GenEd, are_geneds_satisfied};
use crate::schedule::{Catalog, CourseCode, Schedule};
use anyhow::Result;
use std::collections::{HashMap, HashSet};

/// Simple prerequisite solver using optimization principles
/// Now uses the unified SAT solver that handles prerequisites, geneds, and credits together
pub fn solve_prereqs_cp(sched: &Schedule) -> Result<Vec<Vec<Vec<CourseCode>>>> {
    use crate::prereqs_sat;
    let schedule = &sched.courses;
    let prereqs = &sched.catalog.prereqs;
    let courses = &sched.catalog.courses;
    let geneds = &sched.catalog.geneds;

    // Use the unified SAT solver that handles prerequisites, geneds, and credit constraints together
    let sat_solutions = prereqs_sat::solve_unified_schedule(
        schedule.clone(),
        prereqs,
        geneds,
        &sched.catalog,
        courses,
        prereqs_sat::MAX_SAT_ITERATIONS,
    );

    println!(
        "Unified SAT solver found {} complete solutions (prereqs + geneds + credits)",
        sat_solutions.len()
    );

    if sat_solutions.is_empty() {
        println!("No unified solutions found - returning empty schedule list");
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

    // The unified SAT solver already handles optimization, so just return the best solution(s)
    if let Some(best_solution) = schedule_solutions.first() {
        println!(
            "Unified solution found with {} semesters",
            best_solution.len()
        );
        for (i, sem) in best_solution.iter().enumerate() {
            let semester_credits: u32 = sem
                .iter()
                .filter_map(|course| courses.get(course).and_then(|(_, credits, _)| *credits))
                .sum();
            println!(
                "  Semester {}: {} courses, {} credits",
                i,
                sem.len(),
                semester_credits
            );
        }
        Ok(vec![best_solution.clone()])
    } else {
        println!("No valid unified solution found!");
        Ok(vec![])
    }
}
// =============================================================================
// LEGACY GENED PROCESSING FUNCTIONS (NO LONGER USED)
// The unified SAT solver now handles geneds directly, so these functions are deprecated
// =============================================================================

/// Find missing gened requirements in a schedule using smart selection
/// DEPRECATED: Geneds are now handled directly in the unified SAT solver
#[allow(dead_code)]
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
/// DEPRECATED: Geneds are now handled directly in the unified SAT solver
#[allow(dead_code)]
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
                ElectiveReq::Set(courses) => courses.iter().all(|c| current_courses.contains(c)),
                ElectiveReq::SetOpts(opts) => opts
                    .iter()
                    .any(|opt| opt.iter().all(|c| current_courses.contains(c))),
                ElectiveReq::Courses { num, courses } => {
                    let satisfied_count = courses
                        .iter()
                        .filter(|c| current_courses.contains(c))
                        .count();
                    satisfied_count >= *num
                }
                ElectiveReq::Credits { num, courses } => {
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
/// DEPRECATED: Geneds are now handled directly in the unified SAT solver
#[allow(dead_code)]
fn get_all_satisfying_courses(gened: &GenEd) -> Vec<CourseCode> {
    match gened {
        GenEd::Core { req, .. }
        | GenEd::Foundation { req, .. }
        | GenEd::SkillAndPerspective { req, .. } => {
            match req {
                ElectiveReq::Set(courses) => courses.clone(),
                ElectiveReq::SetOpts(opts) => {
                    // Return all courses from all options
                    opts.iter().flatten().cloned().collect()
                }
                ElectiveReq::Courses { courses, .. } | ElectiveReq::Credits { courses, .. } => {
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
/// DEPRECATED: Geneds are now handled directly in the unified SAT solver
#[allow(dead_code)]
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
        // let updated_current_courses_refs: HashSet<&CourseCode> =
        // updated_current_courses.iter().collect();

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
