use anyhow::Result;
use indexmap::IndexMap;
use polars::prelude::*;
use std::collections::HashMap;

mod read_self_zip;
mod write_excel_file;
use read_self_zip::load_programs;
use write_excel_file::save_schedule;

fn generate_schedule_df(dfs: &HashMap<String, DataFrame>) -> DataFrame {
    let full_df_list: Vec<DataFrame> = dfs.values().cloned().collect();

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

    concat(
        dfs_aligned
            .into_iter()
            .map(|x| x.lazy())
            .collect::<Vec<_>>(),
        UnionArgs::default(),
    )
    .unwrap()
    .collect()
    .unwrap()
}

fn main() -> Result<()> {
    let dataframes = load_programs();
    let full_df = generate_schedule_df(&dataframes);
    save_schedule(&full_df, &dataframes);

    println!("Excel file created: test_hidden2.xlsx");
    Ok(())
}
