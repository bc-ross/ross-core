use anyhow::Result;
use glob::glob;
use indexmap::IndexMap;
use polars::functions::concat_df_horizontal;
use polars::prelude::concat as concat_df;
use polars::prelude::*;
use quick_xml::de::from_str;
use rust_xlsxwriter::{Workbook, Worksheet};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use struct_field_names_as_array::FieldNamesAsArray;
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

#[derive(Debug, Deserialize, FieldNamesAsArray)]
struct Course {
    kind: String,
    credit: i32,
    name: Option<String>,
    code: Option<String>,
    url: Option<String>,
    info: Option<String>,
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

// const KEYS: = ["kind", "credit", "name", "code", "url", "info"];

// use polars::prelude::*;
// use std::collections::HashMap;

fn semester_to_dataframe(semester: &Semester) -> DataFrame {
    let mut columns: HashMap<String, Vec<String>> = HashMap::new();

    let keys = Course::FIELD_NAMES_AS_ARRAY; // ["kind", "credit", "name", "code", "url", "info"];

    for key in &keys {
        columns.insert(format!("{}_{}", semester.name, key), Vec::new());
    }

    for course in &semester.courses {
        // let mut col_name = format!("{}_kind", semester.name);
        columns
            .get_mut(&format!("{}_kind", semester.name))
            .unwrap()
            .push(course.kind.clone());
        columns
            .get_mut(&format!("{}_credit", semester.name))
            .unwrap()
            .push(course.credit.to_string());
        columns
            .get_mut(&format!("{}_name", semester.name))
            .unwrap()
            .push(course.name.clone().unwrap_or_default());
        columns
            .get_mut(&format!("{}_code", semester.name))
            .unwrap()
            .push(course.code.clone().unwrap_or_default());
        columns
            .get_mut(&format!("{}_url", semester.name))
            .unwrap()
            .push(course.url.clone().unwrap_or_default());
        columns
            .get_mut(&format!("{}_info", semester.name))
            .unwrap()
            .push(course.info.clone().unwrap_or_default());
    }

    let series: Vec<Series> = keys
        .iter()
        .map(|key| {
            Series::new(
                &format!("{}_{}", semester.name, key),
                &columns[&format!("{}_{}", semester.name, key)],
            )
        })
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
    // let semester_dfs: Vec<LazyFrame> = semester_dfs.into_iter().map(|x| x.lazy()).collect();

    let df = concat_df_horizontal(&semester_dfs).unwrap();
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
    // dbg!(&full_df_list);

    // 1️⃣ Find union of all column names
    let mut all_columns: IndexMap<String, DataType> = IndexMap::new();
    for df in &full_df_list {
        all_columns.extend(
            df.get_column_names()
                .iter()
                .map(|x| (*x).to_owned())
                .zip(df.dtypes()),
        );
    }
    // all_columns = all_columns.into_iter().sort

    let mut dfs_aligned = vec![];

    // 2️⃣ For each df, add missing columns with nulls
    for df in full_df_list {
        let mut df = df;
        for (col, dtype) in &all_columns {
            if !df.get_column_names().contains(&col.as_str()) {
                // Add Series of nulls, same length as df height
                let s = Series::full_null(col, df.height(), dtype);
                df.with_column(s)?;
            }
        }
        // Optional: sort columns to match union order
        let df = df.select(&all_columns.iter().map(|x| x.0).collect::<Vec<_>>())?;
        dfs_aligned.push(df);
    }

    let full_df_list: Vec<LazyFrame> = dfs_aligned.into_iter().map(|x| x.lazy()).collect();
    let full_df = concat_df(&full_df_list, UnionArgs::default())
        .unwrap()
        .collect()
        .unwrap();

    let mut schedule_sheet = workbook.add_worksheet().set_name("Schedule").unwrap();
    pretty_print_df_to_sheet(&full_df, &mut schedule_sheet).unwrap();
    schedule_sheet.protect();

    // Add each sheet + protect + hide
    for (name, df) in &dataframes {
        let mut sheet = workbook.add_worksheet().set_name(name).unwrap();

        write_df_to_sheet(df, &mut sheet).unwrap();

        // let options = ProtectionOptions::new().with_password(password);
        sheet.protect();

        // sheet.set_hidden(true); // FIXME
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
        // dbg!(&field);
        let field = field.rechunk();
        // dbg!(field.iter().collect::<Vec<_>>());
        for (row_idx, val) in field.iter().enumerate() {
            // dbg!(&row_idx, &val);
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
                AnyValue::Null => sheet,
                _ => sheet
                    .write_string((row_idx + 1) as u32, col_idx as u16, &val.to_string())
                    .unwrap(),
            };
        }
    }
    Ok(())
}

/// Write a Polars DataFrame to an xlsxwriter worksheet.
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

    for col_idx in 0..semesters {
        // Write header
        sheet
            .write_string(
                0,
                (col_idx * 2) as u16,
                format!("Semester {}", col_idx + 1),
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
        // Write rows
        // dbg!(&field);
        let field = field.rechunk();
        // dbg!(field.iter().collect::<Vec<_>>());
        for (row_idx, val) in field.iter().filter(|x| !x.is_null()).enumerate() {
            // dbg!(&row_idx, &val);
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
                // AnyValue::Null => sheet,
                _ => sheet
                    .write_string((row_idx + 1) as u32, col_idx as u16, &val.to_string())
                    .unwrap(),
            };
        }
    }
    Ok(())
}

// use polars::prelude::*;

// fn main() -> PolarsResult<()> {
//     // Example DataFrame
//     let mut df = df! [
//         "a-code" => &[Some("A123"), None, Some("C789"), None],
//         "a-kind" => &[Some("X"), Some("Y"), None, Some("Z")]
//     ]?;
//     let prefs = ["a"];

//     // Use coalesce to get first non-null from code or kind
//     let lf = df.lazy();
//     let exprs = prefs
//         .iter()
//         .map(|x| {
//             when(col(&format!("{}-code", x)).is_not_null())
//                 .then(col(&format!("{}-code", x)))
//                 .otherwise(col(&format!("{}-kind", x)))
//                 .alias(&format!("{}-result", x))
//         })
//         .collect::<Vec<_>>();
//     let new_df = lf.with_columns(exprs);
//     println!("{}", new_df.collect().unwrap());

//     Ok(())
// }
