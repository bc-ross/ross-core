use anyhow::Result;
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

const KIND_COLS: [&str; 1] = ["kind"];
const INT_COLS: [&str; 1] = ["credit"];

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

trait Unzip6<A, B, C, D, E, F> {
    fn unzip6(
        self,
        a_name: &str,
        b_name: &str,
        c_name: &str,
        d_name: &str,
        e_name: &str,
        f_name: &str,
    ) -> (Series, Series, Series, Series, Series, Series);
}

impl<I, A, B, C, D, E, F> Unzip6<A, B, C, D, E, F> for I
where
    I: Iterator<Item = (A, B, C, D, E, F)>,
    Series: NamedFromOwned<Vec<A>>,
    Series: NamedFromOwned<Vec<B>>,
    Series: NamedFromOwned<Vec<C>>,
    Series: NamedFromOwned<Vec<D>>,
    Series: NamedFromOwned<Vec<E>>,
    Series: NamedFromOwned<Vec<F>>,
{
    fn unzip6(
        self,
        a_name: &str,
        b_name: &str,
        c_name: &str,
        d_name: &str,
        e_name: &str,
        f_name: &str,
    ) -> (Series, Series, Series, Series, Series, Series) {
        let mut a = Vec::new();
        let mut b = Vec::new();
        let mut c = Vec::new();
        let mut d = Vec::new();
        let mut e = Vec::new();
        let mut f = Vec::new();
        for (u, v, w, x, y, z) in self {
            a.push(u);
            b.push(v);
            c.push(w);
            d.push(x);
            e.push(y);
            f.push(z);
        }
        (
            Series::from_vec(a_name, a),
            Series::from_vec(b_name, b),
            Series::from_vec(c_name, c),
            Series::from_vec(d_name, d),
            Series::from_vec(e_name, e),
            Series::from_vec(f_name, f),
        )
    }
}

fn semester_to_dataframe(semester: Semester) -> DataFrame {
    let Semester {
        name: sem_name,
        courses,
    } = semester;
    let mut str_cols: HashMap<String, Vec<String>> = HashMap::new();
    let mut int_cols: HashMap<String, Vec<i32>> = HashMap::new();
    let mut kind_cols: HashMap<String, Vec<CourseKind>> = HashMap::new();

    let keys = Course::FIELD_NAMES_AS_ARRAY;

    let mut columns: HashMap<String, Vec<String>> = HashMap::new();

    // for key in &keys {
    //     if KIND_COLS.contains(key) {
    //         kind_cols
    //     } else if INT_COLS.contains(key) {
    //         int_cols
    //     } else {
    //         str_cols
    //     }
    //     .insert(format!("{}_{}", sem_name, key), Vec::new());
    // }

    // for course in courses {
    //     let Course {
    //         kind,
    //         credit,
    //         name,
    //         code,
    //         url,
    //         info,
    //     } = course;
    //     columns
    //         .get_mut(&format!("{}_kind", sem_name))
    //         .unwrap()
    //         .0
    //         .push(kind.to_string());
    //     columns
    //         .get_mut(&format!("{}_credit", sem_name))
    //         .unwrap()
    //         .push(credit.to_string());
    //     columns
    //         .get_mut(&format!("{}_name", sem_name))
    //         .unwrap()
    //         .push(name.unwrap_or_default());
    //     columns
    //         .get_mut(&format!("{}_code", sem_name))
    //         .unwrap()
    //         .push(code.unwrap_or_default());
    //     columns
    //         .get_mut(&format!("{}_url", sem_name))
    //         .unwrap()
    //         .push(url.unwrap_or_default());
    //     columns
    //         .get_mut(&format!("{}_info", sem_name))
    //         .unwrap()
    //         .push(info.unwrap_or_default());
    // }
    // Series::from_iter(iter);

    courses
        .into_iter()
        .map(|x| {
            let Course {
                kind,
                credit,
                name,
                code,
                url,
                info,
            } = x;
            (
                kind.to_string(),
                credit,
                name.unwrap_or_default(),
                code.unwrap_or_default(),
                url.unwrap_or_default(),
                info.unwrap_or_default(),
            )
        })
        .unzip6();

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
