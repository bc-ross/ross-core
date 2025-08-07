use std::collections::{HashMap, HashSet};

use crate::schedule::{Catalog, CourseCode, Schedule};
use anyhow::Result;
use savefile_derive::Savefile;
use serde::{Deserialize, Serialize};

const MAX_SKILLS_AND_PERSPECTIVES: u8 = 3;

#[derive(Savefile, Serialize, Deserialize, Debug, Clone)]
pub enum GenEd {
    Core { name: String, req: GenEdReq },
    Foundation { name: String, req: GenEdReq },
    SkillAndPerspective { name: String, req: GenEdReq },
}

#[derive(Savefile, Serialize, Deserialize, Debug, Clone)]
pub enum GenEdReq {
    Set(Vec<CourseCode>),
    SetOpts(Vec<Vec<CourseCode>>),
    Courses {
        num: usize,
        courses: Vec<CourseCode>,
    },
    Credits {
        num: u32,
        courses: Vec<CourseCode>,
    },
}

impl GenEdReq {
    pub fn fulfilled_courses(
        &self,
        all_codes: &HashSet<&CourseCode>,
        catalog: &Catalog,
    ) -> Option<HashSet<&CourseCode>> {
        match self {
            GenEdReq::Set(codes) => {
                let fulfilled: HashSet<_> = codes
                    .iter()
                    .filter(|code| all_codes.contains(*code))
                    .collect();
                if fulfilled.len() == codes.len() {
                    Some(fulfilled)
                } else {
                    None
                }
            }
            GenEdReq::SetOpts(opts) => {
                for opt in opts {
                    let fulfilled: HashSet<_> = opt
                        .iter()
                        .filter(|code| all_codes.contains(*code))
                        .collect();
                    if fulfilled.len() == opt.len() {
                        return Some(fulfilled);
                    }
                }
                None
            }
            GenEdReq::Courses { num, courses } => {
                let available: Vec<_> = courses
                    .iter()
                    .filter(|code| all_codes.contains(*code))
                    .collect();
                if available.len() >= *num {
                    // Return only the minimum number of courses needed
                    Some(available.into_iter().take(*num).collect())
                } else {
                    None
                }
            }
            GenEdReq::Credits { num, courses } => {
                let mut available: Vec<_> = courses
                    .iter()
                    .filter(|code| all_codes.contains(*code))
                    .collect();

                // Sort by credits to prioritize higher-credit courses for efficiency
                available.sort_by_key(|code| {
                    catalog
                        .courses
                        .get(code)
                        .and_then(|(_, creds, _)| *creds)
                        .unwrap_or(3)
                });
                available.reverse(); // Highest credits first

                let mut selected = HashSet::new();
                let mut total_credits = 0u32;

                for code in available {
                    let course_credits = catalog
                        .courses
                        .get(code)
                        .and_then(|(_, creds, _)| *creds)
                        .unwrap_or(3);

                    selected.insert(code);
                    total_credits += course_credits;

                    if total_credits >= *num {
                        break;
                    }
                }

                if total_credits >= *num {
                    Some(selected)
                } else {
                    None
                }
            }
        }
    }
}

pub fn are_geneds_satisfied(sched: &Schedule) -> Result<bool> {
    let all_codes = sched
        .courses
        .iter()
        .flatten()
        .collect::<HashSet<&CourseCode>>();

    println!("=== Checking gened satisfaction ===");
    println!("Total courses in schedule: {}", all_codes.len());

    // Separate geneds by type for different validation strategies
    let core_geneds: Vec<_> = sched
        .catalog
        .geneds
        .iter()
        .enumerate()
        .filter(|(_, gened)| matches!(gened, GenEd::Core { .. }))
        .collect();
    let foundation_geneds: Vec<_> = sched
        .catalog
        .geneds
        .iter()
        .enumerate()
        .filter(|(_, gened)| matches!(gened, GenEd::Foundation { .. }))
        .collect();
    let skill_perspective_geneds: Vec<_> = sched
        .catalog
        .geneds
        .iter()
        .enumerate()
        .filter(|(_, gened)| matches!(gened, GenEd::SkillAndPerspective { .. }))
        .collect();

    // Check Core geneds (no restrictions on overlap)
    for (gened_idx, gened) in core_geneds {
        if let GenEd::Core { req, name, .. } = gened {
            if req.fulfilled_courses(&all_codes, &sched.catalog).is_none() {
                println!("FAILED Core gened {}: {}", gened_idx, name);
                return Ok(false);
            } else {
                println!("PASSED Core gened {}: {}", gened_idx, name);
            }
        }
    }

    // Smart validation for Foundation geneds (no course can be used by multiple Foundation geneds)
    if !validate_foundation_geneds_smart(&foundation_geneds, &all_codes, &sched.catalog)? {
        return Ok(false);
    }

    // Smart validation for Skills & Perspective geneds (courses can be reused up to 3 times)
    if !validate_skills_perspective_geneds_smart(
        &skill_perspective_geneds,
        &all_codes,
        &sched.catalog,
    )? {
        return Ok(false);
    }

    println!("=== All geneds satisfied ===");
    Ok(true)
}

// Used for script_assistant crate
#[allow(dead_code)]
impl GenEd {
    pub fn all_course_codes(&self) -> Vec<CourseCode> {
        match self {
            GenEd::Core { req, .. } => req.all_course_codes(),
            GenEd::Foundation { req, .. } => req.all_course_codes(),
            GenEd::SkillAndPerspective { req, .. } => req.all_course_codes(),
        }
    }
}

impl GenEdReq {
    fn all_course_codes(&self) -> Vec<CourseCode> {
        let mut codes = Vec::new();
        self.collect_course_codes(&mut codes);
        codes.into_iter().map(|x| x.clone()).collect()
    }

    fn collect_course_codes<'a>(&'a self, codes: &mut Vec<&'a CourseCode>) {
        match self {
            GenEdReq::Set(courses) => {
                codes.extend(courses.iter());
            }
            GenEdReq::SetOpts(course_seqs) => {
                codes.extend(course_seqs.iter().flatten());
            }
            GenEdReq::Courses { courses, .. } | GenEdReq::Credits { courses, .. } => {
                codes.extend(courses.iter());
            }
        }
    }
}

/// Smart validation for Foundation geneds using constraint satisfaction
fn validate_foundation_geneds_smart(
    foundation_geneds: &[(usize, &GenEd)],
    all_codes: &HashSet<&CourseCode>,
    catalog: &Catalog,
) -> Result<bool> {
    // Try to find a valid assignment where no course is used by multiple Foundation geneds
    let foundation_assignments = find_foundation_assignment(foundation_geneds, all_codes, catalog);

    match foundation_assignments {
        Some(assignments) => {
            for (gened_idx, gened, used_courses) in assignments {
                if let GenEd::Foundation { name, .. } = gened {
                    println!(
                        "PASSED Foundation gened {}: {} (used {} courses: {:?})",
                        gened_idx,
                        name,
                        used_courses.len(),
                        used_courses.iter().collect::<Vec<_>>()
                    );
                }
            }
            Ok(true)
        }
        None => {
            println!("FAILED: Could not find valid Foundation gened assignment");
            // Show which geneds failed individually
            for (gened_idx, gened) in foundation_geneds {
                if let GenEd::Foundation { req, name, .. } = gened {
                    if req.fulfilled_courses(all_codes, catalog).is_none() {
                        println!(
                            "  Foundation gened {} ({}) cannot be satisfied individually",
                            gened_idx, name
                        );
                    } else {
                        println!(
                            "  Foundation gened {} ({}) CAN be satisfied individually",
                            gened_idx, name
                        );
                    }
                }
            }
            Ok(false)
        }
    }
}

/// Smart validation for Skills & Perspective geneds using constraint satisfaction
fn validate_skills_perspective_geneds_smart(
    skill_perspective_geneds: &[(usize, &GenEd)],
    all_codes: &HashSet<&CourseCode>,
    catalog: &Catalog,
) -> Result<bool> {
    // Try to find a valid assignment where no course is used more than 3 times
    let skill_assignments =
        find_skills_perspective_assignment(skill_perspective_geneds, all_codes, catalog);

    match skill_assignments {
        Some(assignments) => {
            for (gened_idx, gened, used_courses) in assignments {
                if let GenEd::SkillAndPerspective { name, .. } = gened {
                    println!(
                        "PASSED Skills & Perspective gened {}: {} (used {} courses: {:?})",
                        gened_idx,
                        name,
                        used_courses.len(),
                        used_courses.iter().collect::<Vec<_>>()
                    );
                }
            }
            Ok(true)
        }
        None => {
            println!("FAILED: Could not find valid Skills & Perspective gened assignment");
            Ok(false)
        }
    }
}

/// Find a valid assignment for Foundation geneds (no course overlap)
fn find_foundation_assignment<'a>(
    foundation_geneds: &[(usize, &'a GenEd)],
    all_codes: &HashSet<&'a CourseCode>,
    catalog: &Catalog,
) -> Option<Vec<(usize, &'a GenEd, HashSet<&'a CourseCode>)>> {
    // Use backtracking to find a valid assignment
    let mut assignments = Vec::new();
    let mut used_courses = HashSet::new();

    if backtrack_foundation_assignment(
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

/// Backtracking algorithm for Foundation gened assignment
fn backtrack_foundation_assignment<'a>(
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
            if backtrack_foundation_assignment(
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

/// Find a valid assignment for Skills & Perspective geneds (max 3 uses per course)
fn find_skills_perspective_assignment<'a>(
    skill_perspective_geneds: &[(usize, &'a GenEd)],
    all_codes: &HashSet<&'a CourseCode>,
    catalog: &Catalog,
) -> Option<Vec<(usize, &'a GenEd, HashSet<&'a CourseCode>)>> {
    // Use backtracking to find a valid assignment
    let mut assignments = Vec::new();
    let mut course_usage = HashMap::new();

    if backtrack_skills_assignment(
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

/// Backtracking algorithm for Skills & Perspective gened assignment
fn backtrack_skills_assignment<'a>(
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
            .filter(|course| course_usage.get(*course).unwrap_or(&0) < &MAX_SKILLS_AND_PERSPECTIVES)
            .cloned()
            .collect();

        // Try to satisfy this gened with available courses
        if let Some(fulfilled_courses) = req.fulfilled_courses(&available_codes, catalog) {
            // Try this assignment
            let backup_usage = course_usage.clone();
            for course in &fulfilled_courses {
                *course_usage.entry(course).or_insert(0) += 1;
            }
            assignments.push((gened_idx, gened, fulfilled_courses.clone()));

            // Recursively try to assign remaining geneds
            if backtrack_skills_assignment(
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
            *course_usage = backup_usage;
        }
    }

    false
}
