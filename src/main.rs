use anyhow::Result;
use glob::glob;
use polars::functions::concat_df_horizontal as concat_df;
use polars::prelude::*;
use quick_xml::de::from_str;
use rust_xlsxwriter::{Workbook, Worksheet};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
/// Define your CourseKind — adjust as needed.
#[derive(Debug, Deserialize)]
// #[serde(rename_all = "lowercase")]
#[serde(rename_all = "PascalCase")]
enum CourseKind {
    Degree,
    GenEd,
    GenEdStub,
    Elective,
    ElectiveStub,
}

/// One course element
#[derive(Debug, Deserialize)]
struct Course {
    // #[serde(rename = "kind")]
    kind: String,
    #[serde(rename = "credit")]
    credit: i32,
    #[serde(flatten)]
    other: HashMap<String, String>,
}

/// A semester block in the XML
#[derive(Debug, Deserialize)]
struct Semester {
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "course", default)]
    courses: Vec<Course>,
}

/// Root XML container
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
struct Root {
    #[serde(rename = "$value")]
    semesters: Vec<Semester>,
}

/// Parses XML and builds a Polars DataFrame for this file
fn parse_and_convert_xml(xml_string: &str, _root_tag: &str) -> Result<DataFrame> {
    let root: Root = from_str(xml_string)?;

    let mut semester_dfs = Vec::new();

    for semester in &root.semesters {
        let mut all_rows = Vec::new();
        for course in &semester.courses {
            let mut row: Vec<(&str, String)> = course
                .other
                .iter()
                .map(|(k, v)| (k.as_str(), v.clone()))
                .collect();
            row.push(("kind", course.kind.clone()));
            row.push(("credit", course.credit.to_string()));
            all_rows.push(row);
        }

        if let Some(first) = all_rows.first() {
            let mut columns: HashMap<&str, Vec<String>> = HashMap::new();
            for &(key, _) in first {
                columns.insert(key, Vec::new());
            }

            for row in &all_rows {
                for &(key, ref value) in row {
                    columns.get_mut(key).unwrap().push(value.clone());
                }
            }

            let mut fields = Vec::new();
            for (key, values) in columns {
                fields.push(Series::new(&format!("{}:{}", semester.name, key), values));
            }

            let df = DataFrame::new(fields)?;
            semester_dfs.push(df);
        }
    }

    let df = concat_df(&semester_dfs)?;
    Ok(df)
}

/// Trim title to Excel sheet name max length.
fn trim_titles(s: &str) -> String {
    if s.len() > 31 {
        s[..31].to_string()
    } else {
        s.to_string()
    }
}

fn main() -> Result<()> {
    let password = "plzdontgraduate";

    let xml_files: Vec<_> = glob("scraped_programs/*.xml")?
        .filter_map(Result::ok)
        .collect();

    let mut dataframes: HashMap<String, DataFrame> = HashMap::new();

    for file in &xml_files {
        let file_stem = file.file_stem().unwrap().to_string_lossy();
        let root_tag = if file_stem == "General_Education" {
            "gened"
        } else {
            "semester"
        };

        let xml_content = fs::read_to_string(file)?;
        let df = parse_and_convert_xml(&xml_content, root_tag)?;

        dataframes.insert(trim_titles(&file_stem), df);
    }

    // Create Excel workbook
    let mut workbook = Workbook::new();

    // Create combined "Schedule" sheet
    let mut full_df_list = Vec::new();
    for df in dataframes.values() {
        full_df_list.push(df.clone());
    }
    let full_df = concat_df(&full_df_list)?;

    let mut schedule_sheet = workbook.add_worksheet().set_name("Schedule")?;
    write_df_to_sheet(&full_df, &mut schedule_sheet)?;

    // Add each sheet + protect + hide
    for (name, df) in &dataframes {
        let mut sheet = workbook.add_worksheet().set_name(name)?;

        write_df_to_sheet(df, &mut sheet)?;

        // let options = ProtectionOptions::new().with_password(password);
        sheet.protect();

        sheet.set_hidden(true);
    }

    // Save workbook
    workbook.save("test_hidden2.xlsx")?;

    println!("Excel file created: test_hidden2.xlsx");
    Ok(())
}

/// Write a Polars DataFrame to an xlsxwriter worksheet.
fn write_df_to_sheet(df: &DataFrame, sheet: &mut Worksheet) -> Result<()> {
    for (col_idx, field) in df.get_columns().iter().enumerate() {
        // Write header
        sheet.write_string(0, col_idx as u16, field.name())?;

        // Write rows
        for (row_idx, val) in field.iter().enumerate() {
            match val {
                AnyValue::String(v) => {
                    sheet.write_string((row_idx + 1) as u32, col_idx as u16, v)?
                }
                AnyValue::Int32(v) => {
                    sheet.write_number((row_idx + 1) as u32, col_idx as u16, v as f64)?
                }
                AnyValue::Int64(v) => {
                    sheet.write_number((row_idx + 1) as u32, col_idx as u16, v as f64)?
                }
                AnyValue::Float64(v) => {
                    sheet.write_number((row_idx + 1) as u32, col_idx as u16, v)?
                }
                _ => sheet.write_string((row_idx + 1) as u32, col_idx as u16, &val.to_string())?,
            };
        }
    }
    Ok(())
}
