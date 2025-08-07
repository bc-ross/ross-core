use crate::prereqs::CourseReq;
use crate::schedule::{CourseCode, CourseTermOffering};
use anyhow::Result;
use std::any;
use std::collections::HashMap;

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
    schedule: Vec<Vec<CourseCode>>,
    prereqs: &HashMap<CourseCode, CourseReq>,
    courses: &HashMap<CourseCode, (String, Option<u32>, CourseTermOffering)>,
) -> Result<Vec<Vec<Vec<CourseCode>>>> {
    // For now, we'll use the SAT solver as the backend but with optimization-focused logic
    // In the future, this can be replaced with a true CP/IP implementation

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

    // Apply optimization logic to choose the best solution
    let best_solution = find_best_solution(schedule_solutions.clone(), courses);

    Ok(vec![best_solution])
}

/// Get all solutions for analysis and comparison
pub fn solve_prereqs_cp_all_solutions(
    schedule: Vec<Vec<CourseCode>>,
    prereqs: &HashMap<CourseCode, CourseReq>,
    courses: &HashMap<CourseCode, (String, Option<u32>, CourseTermOffering)>,
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

/// Find the best solution based on optimization criteria:
/// 1. Minimize total credits
/// 2. Balance credits across semesters
/// 3. Minimize number of additional courses
fn find_best_solution(
    solutions: Vec<Vec<Vec<CourseCode>>>,
    courses: &HashMap<CourseCode, (String, Option<u32>, CourseTermOffering)>,
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
            let score = calculate_solution_score(&solution, courses);
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
fn calculate_solution_score(
    solution: &[Vec<CourseCode>],
    courses: &HashMap<CourseCode, (String, Option<u32>, CourseTermOffering)>,
) -> f64 {
    let mut total_credits = 0;
    let mut semester_credits = Vec::new();
    let mut total_courses = 0;

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
        .filter(|&&credits| credits > 18)
        .map(|&credits| (credits - 18) as f64 * 100000.0) // Hefty penalty for overload
        .sum();

    credit_penalty + balance_penalty + course_penalty + overload_penalty
}

/// Test function for the CP solver that uses optimization principles
pub fn test_cp_solver() {
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

    // Create a schedule that violates prerequisites
    let schedule = vec![
        vec![math_calc1.clone()], // Semester 0: Calc 1
        vec![],                   // Semester 1: Empty
        vec![phys_em.clone()],    // Semester 2: E&M Physics (missing prereqs!)
    ];

    // Set up prerequisites
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
    solve_schedule_cp(schedule, &prereqs, &courses).unwrap();
}

pub fn solve_schedule_cp(
    schedule: Vec<Vec<CourseCode>>,
    prereqs: &HashMap<CourseCode, CourseReq>,
    courses: &HashMap<CourseCode, (String, Option<u32>, CourseTermOffering)>,
) -> Result<Vec<Vec<CourseCode>>> {
    println!("Testing optimization-focused CP solver:");
    println!("Original schedule with prerequisite violations:");
    for (i, sem) in schedule.iter().enumerate() {
        println!("  Semester {}: {:?}", i, sem);
    }

    // First, show all solutions found
    match solve_prereqs_cp_all_solutions(schedule.clone(), &prereqs, &courses) {
        Ok(all_solutions) => {
            println!(
                "\nCP solver found {} total solution(s):",
                all_solutions.len()
            );
            for (sol_idx, solution) in all_solutions.iter().enumerate() {
                println!("Solution {}:", sol_idx + 1);
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
                let score = calculate_solution_score(solution, &courses);
                println!("  Optimization score: {:.2}", score);
            }
        }
        Err(e) => {
            println!("CP solver failed to get all solutions: {}", e);
        }
    }

    // Then show the optimized choice
    println!("\nNow finding the BEST solution via optimization:");
    solve_prereqs_cp(schedule, &prereqs, &courses).and_then(|mut solutions| {
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
            let score = calculate_solution_score(solution, &courses);
            println!("  Optimization score: {:.2}", score);
        }
        solutions
            .pop()
            .ok_or(anyhow::anyhow!("No optimized solution found"))
    })
}
