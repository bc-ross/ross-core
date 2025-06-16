use anyhow::Result;
use indexmap::IndexMap;
use polars::functions::concat_df_horizontal;
use polars::prelude::*;
use polars::prelude::*;
use quick_xml::de::from_str;
use rc_zip_sync::ReadZip;
use rust_xlsxwriter::{Format, FormatAlign, Workbook, Worksheet};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path;
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
pub struct Course {
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

pub fn load_programs() -> HashMap<String, DataFrame> {
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
        dataframes.insert(file_stem.to_string(), df);
    }
    return dataframes;
}
