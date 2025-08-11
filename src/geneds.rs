use crate::schedule::{Catalog, CourseCode, Schedule};
use anyhow::Result;
use savefile_derive::Savefile;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Savefile, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum GenEd {
    Core { name: String, req: GenEdReq },
    Foundation { name: String, req: GenEdReq },
    SkillAndPerspective { name: String, req: GenEdReq },
}

#[derive(Clone, Debug, Savefile, Serialize, Deserialize, Hash, Eq, PartialEq)]
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

// Helper: for a GenEdReq, return a set of courses from schedule that can be used to satisfy it, or None if not possible
fn satisfy_req<'a>(
    req: &'a GenEdReq,
    sched_courses: &HashSet<&'a CourseCode>,
    catalog: &Catalog,
) -> Option<HashSet<&'a CourseCode>> {
    match req {
        GenEdReq::Set(codes) => {
            let set: HashSet<_> = codes
                .iter()
                .filter_map(|c| sched_courses.get(c))
                .copied()
                .collect();
            if set.len() == codes.len() {
                Some(set)
            } else {
                None
            }
        }
        GenEdReq::SetOpts(opts) => {
            for opt in opts {
                let set: HashSet<_> = opt
                    .iter()
                    .filter_map(|c| sched_courses.get(c))
                    .copied()
                    .collect();
                if set.len() == opt.len() {
                    return Some(set);
                }
            }
            None
        }
        GenEdReq::Courses { num, courses } => {
            let available: Vec<_> = courses
                .iter()
                .filter_map(|c| sched_courses.get(c))
                .copied()
                .collect();
            if available.len() >= *num {
                Some(available.into_iter().take(*num).collect())
            } else {
                None
            }
        }
        GenEdReq::Credits { num, courses } => {
            let mut available: Vec<_> = courses
                .iter()
                .filter_map(|c| sched_courses.get(c))
                .copied()
                .collect();
            // Sort by credits descending
            available.sort_by_key(|c| {
                -(catalog
                    .courses
                    .get(*c)
                    .and_then(|(_, cr, _)| *cr)
                    .unwrap_or(0) as i32)
            });
            let mut selected = HashSet::new();
            let mut total = 0u32;
            for c in available {
                let cr = catalog
                    .courses
                    .get(c)
                    .and_then(|(_, cr, _)| *cr)
                    .unwrap_or(0) as u32;
                selected.insert(c);
                total += cr;
                if total >= *num {
                    break;
                }
            }
            if total >= *num { Some(selected) } else { None }
        }
    }
}

pub fn are_geneds_satisfied(sched: &Schedule) -> Result<bool> {
    let sched_courses: HashSet<&CourseCode> = sched.courses.iter().flatten().collect();
    // 1. Core: each must be satisfied, overlap allowed
    for gened in sched.catalog.geneds.iter() {
        if let GenEd::Core { req, name } = gened {
            if satisfy_req(req, &sched_courses, &sched.catalog).is_none() {
                println!("FAILED Core gened: {}", name);
                return Ok(false);
            }
        }
    }
    // 2. Foundation: each must be satisfied, but no course can be used for more than one Foundation
    let mut foundation_reqs = vec![];
    let mut foundation_names = vec![];
    for gened in sched.catalog.geneds.iter() {
        if let GenEd::Foundation { req, name } = gened {
            foundation_reqs.push(req);
            foundation_names.push(name);
        }
    }
    // Try all possible assignments of courses to foundation geneds (backtracking)
    fn assign_foundations<'a>(
        reqs: &[&'a GenEdReq],
        idx: usize,
        used: &mut HashSet<&'a CourseCode>,
        sched_courses: &HashSet<&'a CourseCode>,
        catalog: &Catalog,
    ) -> bool {
        if idx == reqs.len() {
            return true;
        }
        if let Some(candidates) = satisfy_req(reqs[idx], sched_courses, catalog) {
            // Try all possible ways to assign courses to this foundation, not using any in 'used'
            let mut available: Vec<_> = candidates.difference(used).copied().collect();
            // If not enough, fail
            if available.len() < candidates.len() {
                return false;
            }
            // Try all combinations (greedy: just pick one set)
            for comb in available
                .iter()
                .copied()
                .collect::<Vec<_>>()
                .windows(candidates.len())
            {
                let mut new_used = used.clone();
                new_used.extend(comb.iter().copied());
                if assign_foundations(reqs, idx + 1, &mut new_used, sched_courses, catalog) {
                    return true;
                }
            }
            false
        } else {
            false
        }
    }
    if !foundation_reqs.is_empty() {
        let mut used = HashSet::new();
        if !assign_foundations(
            &foundation_reqs,
            0,
            &mut used,
            &sched_courses,
            &sched.catalog,
        ) {
            println!("FAILED Foundation geneds");
            return Ok(false);
        }
    }
    // 3. Skills & Perspectives: each must be satisfied, but no course can be used for more than 3 S&Ps
    let mut sp_reqs = vec![];
    let mut sp_names = vec![];
    for gened in sched.catalog.geneds.iter() {
        if let GenEd::SkillAndPerspective { req, name } = gened {
            sp_reqs.push(req);
            sp_names.push(name);
        }
    }
    // For each S&P, get the set of courses that could satisfy it
    let mut sp_course_counts: HashMap<&CourseCode, usize> = HashMap::new();
    for req in &sp_reqs {
        if let Some(courses) = satisfy_req(req, &sched_courses, &sched.catalog) {
            for c in courses {
                *sp_course_counts.entry(c).or_insert(0) += 1;
            }
        } else {
            println!("FAILED S&P gened");
            return Ok(false);
        }
    }
    // No course can be used for more than 3 S&Ps
    if sp_course_counts.values().any(|&v| v > 3) {
        println!("FAILED S&P overlap");
        return Ok(false);
    }
    println!("=== All geneds satisfied ===");
    Ok(true)
}
