use anyhow::Result;
use indexmap::IndexMap;
use polars::prelude::*;
use std::{collections::HashMap, path::Path};

pub struct Schedule {
    pub df: DataFrame,
    pub programs: Vec<String>,
    pub catalog: HashMap<String, DataFrame>,
}

pub fn generate_schedule(
    programs: Vec<&str>,
    catalog: &HashMap<String, DataFrame>,
) -> Result<Schedule> {
    // (catalog: )
    let full_df_list: Vec<DataFrame> = catalog
        .into_iter()
        .filter_map(|(k, v)| programs.contains(&k.as_str()).then_some(v))
        .cloned()
        .collect();

    let mut all_columns: IndexMap<String, DataType> = IndexMap::new();
    for df in full_df_list.iter() {
        all_columns.extend(
            df.get_column_names()
                .iter()
                .map(|x| (*x).to_owned())
                .zip(df.dtypes()),
        );
    }

    let dfs_aligned = full_df_list
        .into_iter()
        .map(|mut df| {
            for (col, dtype) in &all_columns {
                if !df.get_column_names().contains(&col.as_str()) {
                    let s = Series::full_null(col, df.height(), dtype);
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
        catalog: catalog.clone(),
    })
}
