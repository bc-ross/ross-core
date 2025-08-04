use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::path::Path;

mod prereqs;
mod read_excel_file;
mod read_self_zip;
mod schedule;
mod write_excel_file;

use read_excel_file::read_file;
use read_self_zip::CATALOGS;
use schedule::generate_schedule;
use write_excel_file::save_schedule;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const SAVEFILE_VERSION: u32 = 0;

fn main() -> Result<()> {
    const FNAME: &str = "schedulebot_test.xlsx";

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

    let new_sched = read_file(&Path::new(FNAME).to_path_buf())?;
    dbg!(new_sched);
    println!("Read file: {FNAME}");

    // save_schedule(
    //     &Path::new("output.xlsx").to_path_buf(),
    //     new_sched.borrow_schedule(),
    // )?;

    // println!("Excel file created: output.xlsx");
    Ok(())
}
