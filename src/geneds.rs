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
    let sched_courses: HashSet<&CourseCode> = std::iter::once(&sched.incoming)
        .chain(sched.courses.iter())
        .flatten()
        .collect();
    // 1. Core: each must be satisfied, overlap allowed
    let mut all_core_ok = true;
    for gened in sched.catalog.geneds.iter() {
        if let GenEd::Core { req, name } = gened {
            if satisfy_req(req, &sched_courses, &sched.catalog).is_none() {
                println!("FAILED Core gened: {}", name);
                all_core_ok = false;
            } else {
                println!("Core gened satisfied: {}", name);
            }
        }
    }
    if !all_core_ok {
        return Ok(false);
    }

    // 2. Foundation: each must be satisfied, but no course can be used for more than one Foundation
    let mut foundation_reqs = vec![];
    let mut foundation_names = vec![];
    for gened in sched.catalog.geneds.iter() {
        if let GenEd::Foundation { req, name } = gened {
            foundation_reqs.push(req);
            foundation_names.push(name);
            // Print eligible courses for this Foundation
            let eligible: Vec<_> = match req {
                GenEdReq::Set(codes) => codes.clone(),
                GenEdReq::SetOpts(opts) => opts.iter().flatten().cloned().collect(),
                GenEdReq::Courses { courses, .. } => courses.clone(),
                GenEdReq::Credits { courses, .. } => courses.clone(),
            };
            println!("Foundation '{}': eligible courses: {:?}", name, eligible);
            // Print which eligible courses are present in the schedule
            let sched_courses: HashSet<&CourseCode> = sched.courses.iter().flatten().collect();
            let present: Vec<_> = eligible
                .iter()
                .filter(|c| sched_courses.contains(c))
                .collect();
            println!("Foundation '{}': scheduled courses: {:?}", name, present);
        }
    }
    // Try all possible assignments of courses to foundation geneds (backtracking)
    fn assign_foundations<'a>(
        reqs: &[&'a GenEdReq],
        names: &[&'a String],
        idx: usize,
        used: &mut HashSet<&'a CourseCode>,
        sched_courses: &HashSet<&'a CourseCode>,
        catalog: &Catalog,
        fail_foundation: &mut Option<String>,
    ) -> bool {
        if idx == reqs.len() {
            return true;
        }
        if let Some(candidates) = satisfy_req(reqs[idx], sched_courses, catalog) {
            let available: Vec<_> = candidates.difference(used).copied().collect();
            println!(
                "[FOUNDATION DIAG] {}: candidates = {:?}, used = {:?}, available = {:?}",
                names[idx], candidates, used, available
            );
            if available.len() < candidates.len() {
                println!(
                    "[FOUNDATION DIAG] Not enough available for {}: need {}, have {}",
                    names[idx],
                    candidates.len(),
                    available.len()
                );
                *fail_foundation = Some(names[idx].clone());
                return false;
            }
            // Try all combinations of candidates from available (order doesn't matter)
            fn combinations<'b, T: Clone>(
                data: &'b [T],
                k: usize,
                out: &mut Vec<T>,
                res: &mut Vec<Vec<T>>,
            ) {
                if out.len() == k {
                    res.push(out.clone());
                    return;
                }
                for i in 0..data.len() {
                    out.push(data[i].clone());
                    let rest = &data[i + 1..];
                    combinations(rest, k, out, res);
                    out.pop();
                }
            }
            let mut combs = Vec::new();
            let mut out = Vec::new();
            combinations(&available, candidates.len(), &mut out, &mut combs);
            if combs.is_empty() {
                println!("[FOUNDATION DIAG] No valid combinations for {}", names[idx]);
            }
            for comb in &combs {
                let mut new_used = used.clone();
                new_used.extend(comb.iter().copied());
                if assign_foundations(
                    reqs,
                    names,
                    idx + 1,
                    &mut new_used,
                    sched_courses,
                    catalog,
                    fail_foundation,
                ) {
                    return true;
                }
            }
            println!(
                "[FOUNDATION DIAG] All combinations failed for {}",
                names[idx]
            );
            *fail_foundation = Some(names[idx].clone());
            false
        } else {
            println!(
                "[FOUNDATION DIAG] satisfy_req returned None for {}",
                names[idx]
            );
            *fail_foundation = Some(names[idx].clone());
            false
        }
    }
    if !foundation_reqs.is_empty() {
        let mut used = HashSet::new();
        let mut fail_foundation = None;
        let req_refs: Vec<&GenEdReq> = foundation_reqs.iter().map(|r| *r).collect();
        let name_refs: Vec<&String> = foundation_names.iter().map(|r| *r).collect();
        if !assign_foundations(
            &req_refs,
            &name_refs,
            0,
            &mut used,
            &sched_courses,
            &sched.catalog,
            &mut fail_foundation,
        ) {
            if let Some(fail) = fail_foundation {
                println!("FAILED Foundation gened: {}", fail);
            } else {
                println!("FAILED Foundation geneds (unknown)");
            }
            return Ok(false);
        } else {
            for name in &foundation_names {
                println!("Foundation gened satisfied: {}", name);
            }
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
    let mut all_sp_ok = true;
    let mut sp_course_counts: HashMap<&CourseCode, usize> = HashMap::new();
    for (i, req) in sp_reqs.iter().enumerate() {
        if let Some(courses) = satisfy_req(req, &sched_courses, &sched.catalog) {
            for c in courses {
                *sp_course_counts.entry(c).or_insert(0) += 1;
            }
            println!("S&P gened satisfied: {}", sp_names[i]);
        } else {
            println!("FAILED S&P gened: {}", sp_names[i]);
            all_sp_ok = false;
        }
    }
    if !all_sp_ok {
        return Ok(false);
    }
    // No course can be used for more than 3 S&Ps
    let mut sp_overlap_fail = false;
    for (c, count) in &sp_course_counts {
        if *count > 3 {
            println!("FAILED S&P overlap: course {} used for {} S&Ps", c, count);
            sp_overlap_fail = true;
        }
    }
    if sp_overlap_fail {
        return Ok(false);
    }
    println!("=== All geneds satisfied ===");
    Ok(true)
}
