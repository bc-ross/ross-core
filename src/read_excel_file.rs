use std::{collections::HashMap, hash::Hash, path::PathBuf};

use calamine::{open_workbook_auto, Data, DataType, Reader};
use polars::prelude::*;

pub fn read_file(fname: &PathBuf) -> anyhow::Result<HashMap<String, DataFrame>> {
    // Open workbook (auto detects xlsx or xls)
    let mut workbook = open_workbook_auto(fname)?;
    let mut df_map = HashMap::new();
    // For each sheet (or pick by name)
    for sheet_name in workbook.sheet_names().to_owned() {
        if sheet_name == "Schedule" {
            continue; // Skips cover sheet
        }

        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            println!("Processing sheet: {sheet_name}");

            // Extract rows as Vec<Vec<String>>
            let rows: Vec<Vec<String>> = range
                .rows()
                .map(|row| row.iter().map(cell_to_string).collect())
                .collect();

            // Split header and data
            let (header, data) = rows.split_first().unwrap();

            // Transpose to columns for Polars
            let columns: Vec<Series> = header
                .iter()
                .enumerate()
                .map(|(i, name)| {
                    let col: Vec<String> = data
                        .iter()
                        .map(|row| row.get(i).unwrap_or(&"".to_string()).clone())
                        .collect();
                    Series::new(name, col)
                })
                .collect();

            let df = DataFrame::new(columns)?;
            df_map.insert(sheet_name, df);
        }
    }

    Ok(df_map)
}

fn cell_to_string(cell: &Data) -> String {
    match cell {
        Data::Empty => "".to_string(),
        Data::String(s) => s.clone(),
        Data::Float(f) => f.to_string(),
        Data::Int(i) => i.to_string(),
        Data::Bool(b) => b.to_string(),
        _ => "?".to_string(),
    }
}
