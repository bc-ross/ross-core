use anyhow::Result;
use polars::prelude::*;
use rust_xlsxwriter::{Format, FormatAlign, Workbook, Worksheet};
use std::{collections::HashMap, path::PathBuf};
use struct_field_names_as_array::FieldNamesAsArray;

use crate::read_self_zip::Course;

fn trim_titles(s: &str) -> String {
    if s.len() > 31 {
        s[..31].to_string()
    } else {
        s.to_string()
    }
}

fn write_df_to_sheet(df: &DataFrame, sheet: &mut Worksheet) -> Result<()> {
    for (col_idx, field) in df.get_columns().iter().enumerate() {
        sheet.write_string(0, col_idx as u16, field.name()).unwrap();

        let field = field.rechunk(); // field.iter() panics otherwise
        for (row_idx, val) in field.iter().enumerate() {
            match val {
                AnyValue::String(v) => sheet
                    .write_string((row_idx + 1) as u32, col_idx as u16, v)
                    .unwrap(),
                AnyValue::Int32(v) => sheet
                    .write_number((row_idx + 1) as u32, col_idx as u16, v as f64)
                    .unwrap(),
                AnyValue::UInt32(v) => sheet
                    .write_number((row_idx + 1) as u32, col_idx as u16, v as f64)
                    .unwrap(),
                AnyValue::Int64(v) => sheet
                    .write_number((row_idx + 1) as u32, col_idx as u16, v as f64)
                    .unwrap(),
                AnyValue::Float64(v) => sheet
                    .write_number((row_idx + 1) as u32, col_idx as u16, v)
                    .unwrap(),
                AnyValue::Null => sheet,
                _ => sheet
                    .write_string((row_idx + 1) as u32, col_idx as u16, dbg!(val).to_string())
                    .unwrap(),
            };
        }
    }
    Ok(())
}

fn pretty_print_df_to_sheet(df: &DataFrame, sheet: &mut Worksheet) -> Result<()> {
    let semesters = df.get_column_names().len() / Course::FIELD_NAMES_AS_ARRAY.len();
    let lf = df.clone().lazy();
    let exprs = (0..semesters)
        .map(|x| {
            when(
                col(&format!("semester-{}_code", x + 1))
                    .is_not_null()
                    .and(col(&format!("semester-{}_code", x + 1)).neq(lit(""))),
            )
            .then(col(&format!("semester-{}_code", x + 1)))
            .otherwise(
                when(
                    col(&format!("semester-{}_info", x + 1))
                        .is_not_null()
                        .and(col(&format!("semester-{}_info", x + 1)).neq(lit(""))),
                )
                .then(col(&format!("semester-{}_info", x + 1)))
                .otherwise(col(&format!("semester-{}_name", x + 1))),
            )
            .alias(&format!("semester-{}_prettyname", x + 1))
        })
        .collect::<Vec<_>>();
    let df = lf.with_columns(exprs).collect().unwrap();

    let format = Format::new().set_align(FormatAlign::Center);

    for col_idx in 0..semesters {
        sheet
            .merge_range(
                0,
                (col_idx * 2) as u16,
                0,
                ((col_idx * 2) + 1) as u16,
                &format!("Semester {}", col_idx + 1),
                &format,
            )
            .unwrap();
    }

    let keys = ["prettyname", "credit"];
    for (col_idx, field) in df
        .select((0..semesters).flat_map(|x| {
            keys.iter()
                .map(move |y| format!("semester-{}_{}", x + 1, y))
        }))
        .unwrap()
        .get_columns()
        .iter()
        .enumerate()
    {
        let max_len = field.iter().map(|s| s.to_string().len()).max().unwrap_or(0);
        sheet
            .set_column_width(col_idx as u16, max_len as f64 - 2.0)
            .unwrap();

        for (row_idx, val) in field.iter().filter(|x| !x.is_null()).enumerate() {
            match val {
                AnyValue::String(v) => sheet
                    .write_string((row_idx + 1) as u32, col_idx as u16, v)
                    .unwrap(),
                AnyValue::Int32(v) => sheet
                    .write_number((row_idx + 1) as u32, col_idx as u16, v as f64)
                    .unwrap(),
                AnyValue::Int64(v) => sheet
                    .write_number((row_idx + 1) as u32, col_idx as u16, v as f64)
                    .unwrap(),
                AnyValue::Float64(v) => sheet
                    .write_number((row_idx + 1) as u32, col_idx as u16, v)
                    .unwrap(),
                _ => sheet
                    .write_string((row_idx + 1) as u32, col_idx as u16, dbg!(val).to_string())
                    .unwrap(),
            };
        }
    }
    Ok(())
}

pub fn save_schedule(fname: &PathBuf, sched_df: &DataFrame, programs: &HashMap<String, DataFrame>) {
    let mut workbook = Workbook::new();

    let schedule_sheet = workbook.add_worksheet().set_name("Schedule").unwrap();
    pretty_print_df_to_sheet(sched_df, schedule_sheet).unwrap();
    schedule_sheet.protect();

    for (name, df) in programs {
        let sheet = workbook
            .add_worksheet()
            .set_name(trim_titles(name))
            .unwrap();
        write_df_to_sheet(df, sheet).unwrap();
        sheet.protect();
    }

    workbook.save(fname).unwrap();
}
