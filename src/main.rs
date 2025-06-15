use anyhow::Result;
use glob::glob;
use polars::prelude::concat as concat_df;
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

#[derive(Debug, Deserialize)]
struct Course {
    kind: String,
    credit: i32,
    code: Option<String>,
    url: Option<String>,
    info: Option<String>,
    name: Option<String>,
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
    #[serde(rename = "semester", default)]
    semesters: Vec<Semester>,
}

// Make separate gened!

// use polars::prelude::*;
// use std::collections::HashMap;

fn semester_to_dataframe(semester: &Semester) -> DataFrame {
    let mut columns: HashMap<&str, Vec<String>> = HashMap::new();

    let keys = ["kind", "credit", "name", "code", "url", "info"];

    for key in &keys {
        columns.insert(key, Vec::new());
    }

    for course in &semester.courses {
        columns.get_mut("kind").unwrap().push(course.kind.clone());
        columns
            .get_mut("credit")
            .unwrap()
            .push(course.credit.to_string());
        columns
            .get_mut("name")
            .unwrap()
            .push(course.name.clone().unwrap_or_default());
        columns
            .get_mut("code")
            .unwrap()
            .push(course.code.clone().unwrap_or_default());
        columns
            .get_mut("url")
            .unwrap()
            .push(course.url.clone().unwrap_or_default());
        columns
            .get_mut("info")
            .unwrap()
            .push(course.info.clone().unwrap_or_default());
    }

    let series: Vec<Series> = keys
        .iter()
        .map(|key| Series::new(*key, &columns[key]))
        .collect();

    DataFrame::new(series).unwrap()
}

/// Parses XML and builds a Polars DataFrame for this file
fn parse_and_convert_xml(xml_string: &str, _root_tag: &str) -> Result<DataFrame> {
    let root: Root = from_str(xml_string).unwrap();

    let mut semester_dfs = Vec::new();

    for semester in &root.semesters {
        semester_dfs.push(semester_to_dataframe(semester));
    }
    // dbg!(_root_tag);
    // dbg!(&semester_dfs);
    let semester_dfs: Vec<LazyFrame> = semester_dfs.into_iter().map(|x| x.lazy()).collect();

    let df = concat_df(&semester_dfs, UnionArgs::default()).unwrap();
    Ok(df.collect().unwrap())
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

    let xml_files: Vec<_> = glob("scraped_programs/*.xml")
        .unwrap()
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

        if (root_tag == "gened") {
            continue; // FIXME
        }

        let xml_content = fs::read_to_string(file).unwrap();
        let df = parse_and_convert_xml(&xml_content, root_tag).unwrap();

        dataframes.insert(trim_titles(&file_stem), df);
    }

    // Create Excel workbook
    let mut workbook = Workbook::new();

    // Create combined "Schedule" sheet
    let mut full_df_list = Vec::new();
    for df in dataframes.values() {
        full_df_list.push(df.clone());
    }
    let full_df_list: Vec<LazyFrame> = full_df_list.into_iter().map(|x| x.lazy()).collect();
    let full_df = concat_df(&full_df_list, UnionArgs::default())
        .unwrap()
        .collect()
        .unwrap();

    let mut schedule_sheet = workbook.add_worksheet().set_name("Schedule").unwrap();
    write_df_to_sheet(&full_df, &mut schedule_sheet).unwrap();

    // Add each sheet + protect + hide
    for (name, df) in &dataframes {
        let mut sheet = workbook.add_worksheet().set_name(name).unwrap();

        write_df_to_sheet(df, &mut sheet).unwrap();

        // let options = ProtectionOptions::new().with_password(password);
        sheet.protect();

        sheet.set_hidden(true);
    }

    // Save workbook
    workbook.save("test_hidden2.xlsx").unwrap();

    println!("Excel file created: test_hidden2.xlsx");
    Ok(())
}

/// Write a Polars DataFrame to an xlsxwriter worksheet.
fn write_df_to_sheet(df: &DataFrame, sheet: &mut Worksheet) -> Result<()> {
    for (col_idx, field) in df.get_columns().iter().enumerate() {
        // Write header
        sheet.write_string(0, col_idx as u16, field.name()).unwrap();

        // Write rows
        dbg!(&field);
        for (row_idx, val) in field.iter().enumerate() {
            dbg!(&row_idx, &val);
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
                    .write_string((row_idx + 1) as u32, col_idx as u16, &val.to_string())
                    .unwrap(),
            };
        }
    }
    Ok(())
}
