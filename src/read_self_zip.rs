use anyhow::Result;
use num_enum::TryFromPrimitive;
use polars::functions::concat_df_horizontal;
use polars::prelude::*;
use quick_xml::de::from_str;
use rc_zip_sync::ReadZip;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::hash::Hash;
use std::path;
use struct_field_names_as_array::FieldNamesAsArray;
use strum_macros::EnumString;

#[repr(u32)]
#[derive(
    Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq, EnumString, TryFromPrimitive,
)]
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

impl CourseKind {
    #[inline]
    pub fn as_u32(self) -> u32 {
        self as u32
    }

    // #[inline]
    // pub fn as_u32(self) -> u32 {
    //     self.as_u8().into()
    // }
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

struct CourseColumns {
    kind: Vec<u32>,
    credit: Vec<i32>,
    name: Vec<Option<String>>,
    code: Vec<Option<String>>,
    url: Vec<Option<String>>,
    info: Vec<Option<String>>,
}

impl CourseColumns {
    fn new() -> Self {
        Self {
            kind: Vec::new(),
            credit: Vec::new(),
            name: Vec::new(),
            code: Vec::new(),
            url: Vec::new(),
            info: Vec::new(),
        }
    }

    fn push(&mut self, x: Course) -> anyhow::Result<()> {
        self.kind.push(x.kind.as_u32());
        self.credit.push(x.credit);
        self.name.push(x.name);
        self.code.push(x.code);
        self.url.push(x.url);
        self.info.push(x.info);
        Ok(())
    }

    fn to_df(self, sem_name: &str) -> anyhow::Result<DataFrame> {
        Ok(DataFrame::new(vec![
            Series::new(&format!("{}_kind", sem_name), self.kind),
            Series::new(&format!("{}_credit", sem_name), self.credit),
            Series::new(&format!("{}_name", sem_name), self.name),
            Series::new(&format!("{}_code", sem_name), self.code),
            Series::new(&format!("{}_url", sem_name), self.url),
            Series::new(&format!("{}_info", sem_name), self.info),
        ])?)
    }
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

    let mut cols = CourseColumns::new();
    for course in courses {
        cols.push(course).unwrap();
    }
    cols.to_df(&sem_name).unwrap()
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
