use anyhow::{Result, anyhow};
use std::path::Path;

mod geneds;
mod load_catalogs;
mod model;
mod prereqs;
mod read_excel_file;
mod schedule;
mod version;
mod write_excel_file;

use load_catalogs::CATALOGS;
use schedule::CourseCode;
use schedule::generate_schedule;
pub use version::{SAVEFILE_VERSION, VERSION};
use write_excel_file::save_schedule;

pub static TEMPLATE_PNG: &[u8] = include_bytes!("../assets/template.png");
pub const MAX_CREDITS_PER_SEMESTER: i64 = 18;

fn main() -> Result<()> {
    const FNAME: &str = "ross_test.xlsx";

    let sched = generate_schedule(
        CATALOGS
            .first()
            .ok_or(anyhow!("no catalogs found"))?
            .programs
            .iter()
            .map(|x| x.name.as_str())
            // .take(1)
            .collect(),
        CATALOGS
            .first()
            .ok_or(anyhow!("no catalogs found"))?
            .clone(),
        Some(vec![CC!("THEO", 1100)]), // None,
    )?;

    println!("Final schedule (two-stage, balanced):");
    println!("Final schedule (two-stage, balanced):");
    let mut sched_credits = 0;
    for (s, semester) in std::iter::once(&sched.incoming)
        .chain(sched.courses.iter())
        .enumerate()
    {
        if s == 0 {
            println!("Semester 0 (incoming only):");
        } else {
            println!("Semester {}", s);
        }
        let mut sem_credits = 0;
        for code in semester {
            // Look up credits from catalog
            let credits = sched
                .catalog
                .courses
                .get(code)
                .and_then(|(_, cr, _)| *cr)
                .unwrap_or(0);
            let credits = sched
                .catalog
                .courses
                .get(code)
                .and_then(|(_, cr, _)| *cr)
                .unwrap_or(0);
            println!("  {} ({} credits)", code, credits);
            sem_credits += credits;
        }
        println!("  Credits: {}", sem_credits);
        if s > 0 {
            sched_credits += sem_credits;
        }
    }
    println!("Total credits (excluding incoming): {}", sched_credits);

    save_schedule(&Path::new(FNAME).to_path_buf(), &sched)?;

    println!(
        "Excel file created: {FNAME} with {} schedule",
        if sched.is_valid()? {
            "valid"
        } else {
            "invalid"
        }
    );

    Ok(())
}
