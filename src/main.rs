use anyhow::{Result, anyhow};
use std::path::Path;

mod load_catalogs;
mod prereqs;
mod prereqs_sat;
mod prereqs_cp;
mod read_excel_file;
mod schedule;
mod schedule_sorter;
mod version;
mod write_excel_file;

use load_catalogs::CATALOGS;
use read_excel_file::read_file;
use schedule::generate_schedule;
pub use version::{SAVEFILE_VERSION, VERSION};
use write_excel_file::save_schedule;

pub static TEMPLATE_PNG: &[u8] = include_bytes!("../assets/template.png");

fn main() -> Result<()> {
    // Test the CP solver
    println!("=== Testing CP Solver ===");
    prereqs_cp::test_cp_solver();
    println!();

    // Test the SAT solver
    println!("=== Testing SAT Solver ===");
    prereqs_sat::test_prereq_sat();
    println!();

    const FNAME: &str = "ross_test.xlsx";

    let sched = generate_schedule(
        CATALOGS
            .first()
            .ok_or(anyhow!("no catalogs found"))?
            .programs
            .iter()
            .map(|x| x.name.as_str())
            .collect(),
        CATALOGS
            .first()
            .ok_or(anyhow!("no catalogs found"))?
            .clone(),
    )?;
    save_schedule(&Path::new(FNAME).to_path_buf(), &sched)?;

    println!("Excel file created: {FNAME}");
    // println!("{}", catalogs.first().ok_or(anyhow!("no catalogs found"))?);

    let _new_sched = read_file(&Path::new(FNAME).to_path_buf())?;
    // dbg!(new_sched);
    println!("Read file: {FNAME}");

    // save_schedule(
    //     &Path::new("output.xlsx").to_path_buf(),
    //     new_sched.borrow_schedule(),
    // )?;

    // println!("Excel file created: output.xlsx");
    Ok(())
}
