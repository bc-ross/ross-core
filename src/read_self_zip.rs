use anyhow::anyhow;
use num_enum::TryFromPrimitive;
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

use crate::schedule::Catalog;

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
    pub fn into_u32(self) -> u32 {
        self as u32
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

    fn push(&mut self, course: Course) -> anyhow::Result<()> {
        self.kind.push(course.kind.into_u32());
        self.credit.push(course.credit);
        self.name.push(course.name);
        self.code.push(course.code);
        self.url.push(course.url);
        self.info.push(course.info);
        Ok(())
    }

    fn into_df(self, sem_name: &str) -> anyhow::Result<DataFrame> {
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
struct ProgramRoot {
    #[serde(rename = "semester", default)]
    semesters: Vec<Semester>,
}

#[derive(Debug, Deserialize)]
struct Gened {
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "course", default)]
    courses: Vec<Course>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
struct GenedRoot {
    #[serde(rename = "gened", default)]
    geneds: Vec<Gened>,
}

fn semester_to_dataframe(semester: Semester) -> anyhow::Result<DataFrame> {
    let mut columns = CourseColumns::new();
    for course in semester.courses {
        columns.push(course)?;
    }
    columns.into_df(&semester.name)
}

fn parse_and_convert_xml_prog(xml_string: &str, _root_tag: &str) -> anyhow::Result<DataFrame> {
    let root: ProgramRoot = from_str(xml_string)?;
    let semester_dfs: anyhow::Result<Vec<DataFrame>> = root
        .semesters
        .into_iter()
        .map(semester_to_dataframe)
        .collect();

    Ok(concat_df_horizontal(&semester_dfs?)?)
}

fn gened_to_dataframe(gened: Gened) -> anyhow::Result<DataFrame> {
    let mut columns = CourseColumns::new();
    for course in gened.courses {
        columns.push(course)?;
    }
    columns.into_df(&gened.name)
}

fn parse_and_convert_xml_edreq(xml_string: &str, _root_tag: &str) -> anyhow::Result<DataFrame> {
    let root: GenedRoot = from_str(xml_string)?;
    let gened_dfs: anyhow::Result<Vec<DataFrame>> =
        root.geneds.into_iter().map(gened_to_dataframe).collect();

    Ok(concat_df_horizontal(&gened_dfs?)?)
}

pub fn load_catalog() -> anyhow::Result<Catalog> {
    let zip_path = env::current_exe()?;
    let mut files: HashMap<String, Vec<u8>> = HashMap::new();

    {
        let zip_file = File::open(zip_path)?;
        for entry in zip_file.read_zip()?.entries() {
            files.insert(entry.name.clone(), entry.bytes()?);
        }
    }

    let mut dataframes: HashMap<String, DataFrame> = HashMap::new();
    let mut geneds = None;

    for (file, contents) in files {
        let file_stem = path::Path::new(&file)
            .file_stem()
            .ok_or_else(|| anyhow::anyhow!("Failed to get file stem"))?
            .to_string_lossy();

        let xml_content = String::from_utf8_lossy(&contents);

        if file_stem == "General_Education" {
            let df = parse_and_convert_xml_edreq(&xml_content, "gened")?;
            geneds = Some(df);
        } else {
            let df = parse_and_convert_xml_prog(&xml_content, "semester")?;
            dataframes.insert(file_stem.to_string(), df);
        };

        // if root_tag == "gened" {
        //     continue;
        // }
    }

    Ok(Catalog {
        programs: dataframes,
        geneds: geneds.ok_or(anyhow!("no gened manifest for course catalog"))?,
    })
}
