use anyhow::Result;
use indexmap::IndexMap;
use ouroboros::self_referencing;
use polars::prelude::*;
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone)]
pub struct Catalog {
    pub programs: HashMap<String, DataFrame>,
    pub geneds: DataFrame,
    pub low_year: u32,
}

impl fmt::Display for Catalog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "<BC {}-{} Catalog>",
            self.low_year,
            self.low_year + 1
        ))
    }
}

#[derive(Debug, Clone)]
pub struct Schedule<'a> {
    pub df: DataFrame,
    pub programs: Vec<String>,
    pub catalog: &'a Catalog,
}

#[self_referencing(pub_extras)]
pub struct StandaloneSchedule {
    pub catalog: Catalog,
    #[borrows(catalog)]
    #[covariant]
    pub schedule: Schedule<'this>,
}

pub fn generate_schedule<'k>(programs: Vec<&str>, catalog: &'k Catalog) -> Result<Schedule<'k>> {
    // (catalog: )
    let full_df_list: Vec<DataFrame> = catalog
        .programs
        .iter()
        .filter_map(|(k, v)| programs.contains(&k.as_str()).then_some(v))
        .cloned()
        .collect();

    let mut all_columns: IndexMap<String, DataType> = IndexMap::new();
    for df in full_df_list.iter() {
        all_columns.extend(
            df.get_column_names()
                .iter()
                .map(|x| (*x).to_string())
                .zip(df.dtypes()),
        );
    }

    let dfs_aligned = full_df_list
        .into_iter()
        .map(|mut df| {
            for (col, dtype) in &all_columns {
                if !df.get_column_names().contains(&&PlSmallStr::from_str(col)) {
                    let s = Column::full_null(col.into(), df.height(), dtype);
                    df.with_column(s)?;
                }
            }
            Ok(df.select(all_columns.iter().map(|x| x.0).collect::<Vec<_>>())?)
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(Schedule {
        df: concat(
            dfs_aligned
                .into_iter()
                .map(|x| x.lazy())
                .collect::<Vec<_>>(),
            UnionArgs::default(),
        )?
        .collect()?,
        programs: programs.into_iter().map(|v| v.to_string()).collect(),
        catalog,
    })
}
