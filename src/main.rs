use anyhow::Result;
use indexmap::IndexMap;
use polars::prelude::*;
use std::{collections::HashMap, path::Path};

mod read_excel_file;
mod read_self_zip;
mod schedule;
mod write_excel_file;
use read_excel_file::read_file;
use read_self_zip::load_catalog;
use schedule::generate_schedule;
use write_excel_file::save_schedule;

fn main() -> Result<()> {
    const FNAME: &str = "schedulebot_test.xlsx";

    let catalog = load_catalog()?;
    let sched = generate_schedule(
        catalog.programs.keys().map(|x| x.as_str()).collect(),
        &catalog,
    )?;
    save_schedule(&Path::new(FNAME).to_path_buf(), &sched)?;

    println!("Excel file created: {}", FNAME);

    let new_df = read_file(&Path::new(FNAME).to_path_buf()).unwrap();
    println!("Read file: {}", FNAME);

    // save_schedule(&Path::new("output.xlsx").to_path_buf(), &sched.df, &new_df)?;

    println!("Excel file created: output.xlsx");
    Ok(())
}
