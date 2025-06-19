use anyhow::Result;
use polars::functions::concat_df_horizontal;
use polars::prelude::*;
use quick_xml::de::from_str;
use rc_zip_sync::ReadZip;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path;
use struct_field_names_as_array::FieldNamesAsArray;
use strum_macros::EnumString;

#[derive(Debug)]
pub enum ColumnSeed {
    Kind(Vec<CourseKind>),
    String(Vec<String>),
    Int(Vec<i32>),
}

impl ColumnSeed {
    fn kind() -> Self {
        Self::Kind(Vec::new())
    }

    fn string() -> Self {
        Self::String(Vec::new())
    }

    fn int() -> Self {
        Self::Int(Vec::new())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq, EnumString)]
#[serde(rename_all = "PascalCase")]
pub enum CourseKind {
    Degree = 0,
    GenEd = 1,
    GenEdStub = 3,
    Elective = 4,
    ElectiveStub = 5,
}

impl std::fmt::Display for CourseKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

#[derive(Debug, Deserialize, FieldNamesAsArray)]
pub struct Course {
    kind: CourseKind,
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

fn semester_to_dataframe(semester: Semester) -> DataFrame {
    let Semester {
        name: sem_name,
        courses,
    } = semester;
    let mut columns: HashMap<String, ColumnSeed> = HashMap::new();
    let keys = Course::FIELD_NAMES_AS_ARRAY;

    for key in &keys {
        columns.insert(
            format!("{}_{}", sem_name, key),
            match *key {
                "kind" => ColumnSeed::kind(),
                "credit" => ColumnSeed::int(),
                _ => ColumnSeed::string(),
            },
        );
    }

    for course in courses {
        let Course {
            kind,
            credit,
            name,
            code,
            url,
            info,
        } = course;
        columns
            .get_mut(&format!("{}_kind", sem_name))
            .unwrap()
            .0
            .push(kind.to_string());
        columns
            .get_mut(&format!("{}_credit", sem_name))
            .unwrap()
            .push(credit.to_string());
        columns
            .get_mut(&format!("{}_name", sem_name))
            .unwrap()
            .push(name.unwrap_or_default());
        columns
            .get_mut(&format!("{}_code", sem_name))
            .unwrap()
            .push(code.unwrap_or_default());
        columns
            .get_mut(&format!("{}_url", sem_name))
            .unwrap()
            .push(url.unwrap_or_default());
        columns
            .get_mut(&format!("{}_info", sem_name))
            .unwrap()
            .push(info.unwrap_or_default());
    }

    let series: Vec<Series> = keys
        .iter()
        .map(|key| {
            Series::new(
                &format!("{}_{}", sem_name, key),
                &columns[&format!("{}_{}", sem_name, key)],
            )
        })
        .collect();

    DataFrame::new(series).unwrap()
}

fn parse_and_convert_xml(xml_string: &str, _root_tag: &str) -> Result<DataFrame> {
    let root: Root = from_str(xml_string).unwrap();
    let semester_dfs: Vec<DataFrame> = root
        .semesters
        .into_iter()
        .map(semester_to_dataframe)
        .collect();

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
    dataframes
}
