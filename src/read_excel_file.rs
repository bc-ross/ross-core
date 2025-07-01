use anyhow::anyhow;
use calamine::{open_workbook_auto, Data, Range, Reader};
use polars::prelude::*;
use std::{collections::HashMap, iter::once, path::PathBuf};

use crate::schedule::{Catalog, Schedule, StandaloneSchedule};

struct Metadata {
    low_year: u32,
    programs: Vec<String>,
    sb_version: String,
}

pub fn read_file(fname: &PathBuf) -> anyhow::Result<StandaloneSchedule> {
    // Open workbook (auto detects xlsx or xls)
    let mut workbook = open_workbook_auto(fname)?;
    let mut df_map = HashMap::new();
    let mut sched_df = None;
    let mut gened_df = None;
    let mut metadata = None;
    // For each sheet (or pick by name)
    for sheet_name in workbook.sheet_names() {
        match sheet_name.as_str() {
            "Schedule" => (),
            "Schedule_Internal" => {
                if let Ok(range) = workbook.worksheet_range(&sheet_name) {
                    sched_df = Some(read_df_range(range)?);
                }
            }
            "Metadata" => {
                if let Ok(range) = workbook.worksheet_range(&sheet_name) {
                    metadata = Some(load_metadata(range)?);
                }
            }
            "General_Education" => {
                if let Ok(range) = workbook.worksheet_range(&sheet_name) {
                    gened_df = Some(read_df_range(range)?);
                }
            }
            _ => {
                if let Ok(range) = workbook.worksheet_range(&sheet_name) {
                    df_map.insert(sheet_name, read_df_range(range)?);
                }
            }
        }
    }

    if let Some(Metadata {
        programs,
        low_year,
        sb_version,
    }) = metadata
    {
        // TODO: check Schedulebot version compat

        let catalog = Catalog {
            programs: df_map,
            low_year,
            geneds: gened_df.ok_or(anyhow!("No geneds found"))?,
        };

        Ok(StandaloneSchedule::try_new(
            catalog,
            |catalog| -> anyhow::Result<_> {
                Ok(Schedule {
                    df: sched_df.ok_or(anyhow!("No geneds found"))?,
                    programs,
                    catalog,
                })
            },
        )?)
    } else {
        Err(anyhow!("no metadata found"))
    }
}

fn load_metadata(range: Range<Data>) -> anyhow::Result<Metadata> {
    let rows: Vec<Vec<&Data>> = range.rows().map(|row| row.iter().collect()).collect();
    let (header, data) = rows.split_first().unwrap();

    let mut programs: Option<Vec<String>> = None;
    let mut catalog = None;
    let mut sb_version = None;

    for (col, vals) in header
        .iter()
        .enumerate()
        .filter_map(|(idx, data)| match data {
            Data::String(s) => Some((idx, s.as_str())),
            Data::Empty => None,
            _ => None,
        })
        .map(|(col_idx, name)| {
            let column_iter = data.iter().map(move |row| &row[col_idx]);
            (name, column_iter)
        })
    {
        match col {
            "Programs" => {
                programs = Some(
                    vals.filter_map(|v| match v {
                        Data::String(s) => Some(s.to_string()),
                        _ => None,
                    })
                    .collect(),
                )
                .filter(|x: &Vec<String>| !x.is_empty());
                // println!("Programs: {:?}", programs);
            }
            "Catalog" => {
                catalog = vals
                    .filter_map(|v| match v {
                        Data::String(s) => {
                            s.to_string().split_once('-').map(|x| x.0.parse::<u32>())
                        }
                        _ => None,
                    })
                    .next()
                    .transpose()?;
                // .unwrap_or_else(|| "Unknown Catalog".to_string());
                // println!("Catalog: {}", catalog);
            }
            "Schedulebot" => {
                sb_version = vals
                    .filter_map(|v| match v {
                        Data::String(s) => Some(s.to_string()),
                        _ => None,
                    })
                    .next();
                // .unwrap_or_else(|| "Unknown Version".to_string());
                // println!("Schedulebot Version: {}", sb_version);
            }
            _ => (),
        }
    }

    Ok(Metadata {
        low_year: catalog.ok_or(anyhow!("Unknown Catalog Year"))?,
        sb_version: sb_version.ok_or(anyhow!("Unknown Schedulebot Version"))?,
        programs: programs.ok_or(anyhow!("No Programs Found"))?,
    })
}

fn read_df_range(range: Range<Data>) -> anyhow::Result<DataFrame> {
    let rows: Vec<Vec<&Data>> = range.rows().map(|row| row.iter().collect()).collect();

    let (header, data) = rows.split_first().unwrap();

    let columns: Vec<Column> = header
        .iter()
        .enumerate()
        .map(|(i, name)| build_typed_series(name, data.iter().map(|row| row[i])))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(DataFrame::new(columns)?)
}

fn build_typed_series<'a, I>(name: &Data, mut values: I) -> anyhow::Result<Column>
where
    I: Iterator<Item = &'a Data>,
{
    let col_name = if let Data::String(s) = &name {
        s
    } else {
        return Err(anyhow!("cannot identify column name"));
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
                    .map(|d| format!("{d:?}"))
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
