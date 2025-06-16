use anyhow::Result;
use indexmap::IndexMap;
use polars::functions::concat_df_horizontal;
use polars::prelude::*;
use quick_xml::de::from_str;
use rc_zip_sync::ReadZip;
use rust_xlsxwriter::{Format, FormatAlign, Workbook, Worksheet};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::{env, path};
use struct_field_names_as_array::FieldNamesAsArray;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
struct Semester {
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "course", default)]
    courses: Vec<Course>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
struct Root {
    #[serde(rename = "semester", default)]
    semesters: Vec<Semester>,
}

fn semester_to_dataframe(semester: &Semester) -> DataFrame {
    let mut columns: HashMap<String, Vec<String>> = HashMap::new();
    let keys = Course::FIELD_NAMES_AS_ARRAY;

    for key in &keys {
        columns.insert(format!("{}_{}", semester.name, key), Vec::new());
    }

    for course in &semester.courses {
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

fn parse_and_convert_xml(xml_string: &str, _root_tag: &str) -> Result<DataFrame> {
    let root: Root = from_str(xml_string).unwrap();
    let semester_dfs: Vec<DataFrame> = root.semesters.iter().map(semester_to_dataframe).collect();

    Ok(concat_df_horizontal(&semester_dfs).unwrap())
}

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
                AnyValue::Int64(v) => sheet
                    .write_number((row_idx + 1) as u32, col_idx as u16, v as f64)
                    .unwrap(),
                AnyValue::Float64(v) => sheet
                    .write_number((row_idx + 1) as u32, col_idx as u16, v)
                    .unwrap(),
                AnyValue::Null => sheet,
                _ => sheet
                    .write_string((row_idx + 1) as u32, col_idx as u16, val.to_string())
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
                    .write_string((row_idx + 1) as u32, col_idx as u16, val.to_string())
                    .unwrap(),
            };
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let zip_path = env::current_exe().unwrap();
    let mut files: HashMap<String, Vec<u8>> = HashMap::new();

    {
        let zipf = File::open(zip_path).unwrap();
        for entry in zipf.read_zip().unwrap().entries() {
            files.insert(entry.name.clone(), entry.bytes().unwrap());
        }
    }

    let mut dataframes: HashMap<String, DataFrame> = HashMap::new();

    for (file, contents) in files {
        let file_stem = path::Path::new(&file)
            .file_stem()
            .unwrap()
            .to_string_lossy();
        let root_tag = if file_stem == "General_Education" {
            "gened"
        } else {
            "semester"
        };

        if root_tag == "gened" {
            continue;
        }

        let xml_content = String::from_utf8_lossy(&contents);
        let df = parse_and_convert_xml(&xml_content, root_tag).unwrap();
        dataframes.insert(trim_titles(&file_stem), df);
    }

    let mut workbook = Workbook::new();
    let full_df_list: Vec<DataFrame> = dataframes.values().cloned().collect();

    let mut all_columns: IndexMap<String, DataType> = IndexMap::new();
    for df in &full_df_list {
        all_columns.extend(
            df.get_column_names()
                .iter()
                .map(|x| (*x).to_owned())
                .zip(df.dtypes()),
        );
    }

    let dfs_aligned: Vec<DataFrame> = full_df_list
        .into_iter()
        .map(|mut df| {
            for (col, dtype) in &all_columns {
                if !df.get_column_names().contains(&col.as_str()) {
                    let s = Series::full_null(col, df.height(), dtype);
                    df.with_column(s).unwrap();
                }
            }
            df.select(all_columns.iter().map(|x| x.0).collect::<Vec<_>>())
                .unwrap()
        })
        .collect();

    let full_df = concat(
        &dfs_aligned
            .into_iter()
            .map(|x| x.lazy())
            .collect::<Vec<_>>(),
        UnionArgs::default(),
    )
    .unwrap()
    .collect()
    .unwrap();

    let schedule_sheet = workbook.add_worksheet().set_name("Schedule").unwrap();
    pretty_print_df_to_sheet(&full_df, schedule_sheet).unwrap();
    schedule_sheet.protect();

    for (name, df) in &dataframes {
        let sheet = workbook.add_worksheet().set_name(name).unwrap();
        write_df_to_sheet(df, sheet).unwrap();
        sheet.protect();
    }

    workbook.save("test_hidden2.xlsx").unwrap();
    println!("Excel file created: test_hidden2.xlsx");
    Ok(())
}
