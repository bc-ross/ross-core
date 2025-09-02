use anyhow::{Result, anyhow};
use std::path::Path;

use ross_core::CC;
use ross_core::load_catalogs::CATALOGS;
use ross_core::read_excel_file::read_file;
use ross_core::schedule::CourseCode;
use ross_core::schedule::generate_schedule;
use ross_core::write_excel_file::save_schedule;

#[test]
fn test_reasons() -> Result<()> {
    let mut sched = generate_schedule(
        CATALOGS
            .first()
            .ok_or(anyhow!("no catalogs found"))?
            .programs
            .iter()
            .map(|x| dbg!(x.name.as_str()))
            // .take(1)
            .collect(),
        CATALOGS
            .first()
            .ok_or(anyhow!("no catalogs found"))?
            .clone(),
        Some(vec![CC!("PHYS", 4200), CC!("THEO", 1100)]), // None,
    )?;
    sched.validate()?;

    println!("Schedule valid? {}", sched.is_valid()?);
    println!("Reasons for courses:");
    println!("{:?}", sched.get_reasons()?);

    Ok(())
}
