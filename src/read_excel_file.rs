use anyhow::anyhow;
use calamine::{open_workbook_auto, Data, Reader};
use polars::prelude::*;
use std::{collections::HashMap, iter::once, path::PathBuf};

pub fn read_file(fname: &PathBuf) -> anyhow::Result<HashMap<String, DataFrame>> {
    // Open workbook (auto detects xlsx or xls)
    let mut workbook = open_workbook_auto(fname)?;
    let mut df_map = HashMap::new();
    // For each sheet (or pick by name)
    for sheet_name in workbook.sheet_names() {
        if sheet_name == "Schedule" {
            continue; // Skips cover sheet
        }

        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            let rows: Vec<Vec<&Data>> = range.rows().map(|row| row.iter().collect()).collect();

            let (header, data) = rows.split_first().unwrap();

            let columns: Vec<Column> = header
                .iter()
                .enumerate()
                .map(|(i, name)| build_typed_series(name, data.iter().map(|row| row[i])))
                .collect::<Result<Vec<_>, _>>()?;

            let df = DataFrame::new(columns)?;

            df_map.insert(sheet_name, df);
        }
    }

    Ok(df_map)
}

fn build_typed_series<'a, I>(name: &Data, mut values: I) -> anyhow::Result<Column>
where
    I: Iterator<Item = &'a Data>,
{
    let col_name = if let Data::String(s) = name {
        s
    } else {
        "UNKNOWN"
    };

    // Find first non-empty to decide
    if let Some(dtype) = values.next() {
        // |v| !matches!(v, Data::Empty));

        match dtype {
            Data::Int(_) => {
                if col_name.ends_with("_kind") {
                    let v = once(dtype)
                        .chain(values)
                        .map(|d| match d {
                            Data::Int(i) => Some(*i),
                            Data::Empty => None,
                            _ => None,
                        })
                        .map(|x| x.map(|v| (v as u8) as u32))
                        .collect::<Vec<_>>();
                    Ok(Column::new(col_name.into(), v))
                } else {
                    let v = once(dtype)
                        .chain(values)
                        .map(|d| match d {
                            Data::Int(i) => Some(*i),
                            Data::Empty => None,
                            _ => None,
                        })
                        .collect::<Vec<_>>();
                    Ok(Column::new(col_name.into(), v))
                }
            }
            Data::Float(_) => {
                if col_name.ends_with("_kind") {
                    let v = once(dtype)
                        .chain(values)
                        .map(|d| match d {
                            Data::Float(i) => Some(*i),
                            Data::Empty => None,
                            _ => None,
                        })
                        .map(|x| x.map(|v| (v as u8) as u32))
                        .collect::<Vec<_>>();
                    Ok(Column::new(col_name.into(), v))
                } else {
                    let v = once(dtype)
                        .chain(values)
                        .map(|d| match d {
                            Data::Float(f) => Some(*f),
                            Data::Empty => None,
                            _ => None,
                        })
                        .collect::<Vec<_>>();
                    Ok(Column::new(col_name.into(), v))
                }
            }
            Data::Bool(_) => {
                let v = once(dtype)
                    .chain(values)
                    .map(|d| match d {
                        Data::Bool(b) => Some(*b),
                        Data::Empty => None,
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                Ok(Column::new(col_name.into(), v))
            }
            Data::String(_) => {
                let v = once(dtype)
                    .chain(values)
                    .map(|d| match d {
                        Data::String(s) => Some(s.as_str()),
                        Data::Empty => None,
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                Ok(Column::new(col_name.into(), v))
            }
            Data::Error(_) => {
                // If you want, handle as string
                let v = once(dtype)
                    .chain(values)
                    .map(|d| format!("{:?}", d))
                    .collect::<Vec<_>>();
                Ok(Column::new(col_name.into(), v))
            }
            _ => {
                // all empty? fallback to empty strings
                Ok(Column::new(
                    col_name.into(),
                    vec![""; once(dtype).chain(values).count()],
                ))
            }
        }
    } else {
        Err(anyhow!("blank column"))
    }
}
