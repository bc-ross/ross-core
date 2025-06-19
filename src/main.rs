use anyhow::Result;
use indexmap::IndexMap;
use polars::prelude::*;
use std::{collections::HashMap, path::Path};

mod read_excel_file;
mod read_self_zip;
mod schedule;
mod write_excel_file;
use read_excel_file::read_file;
use read_self_zip::load_programs;
use schedule::generate_schedule_df;
use write_excel_file::save_schedule;

fn main() -> Result<()> {
    const FNAME: &str = "schedulebot_test.xlsx";

    let dataframes = load_programs()?;
    let full_df = generate_schedule_df(&dataframes);
    save_schedule(&Path::new(FNAME).to_path_buf(), &full_df, &dataframes)?;

    println!("Excel file created: {}", FNAME);

    let new_df = read_file(&Path::new(FNAME).to_path_buf()).unwrap();
    println!("Read file: {}", FNAME);

    save_schedule(&Path::new("output.xlsx").to_path_buf(), &full_df, &new_df)?;

    println!("Excel file created: output.xlsx");
    Ok(())
}
