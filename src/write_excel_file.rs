use crate::SAVEFILE_VERSION;
use crate::schedule::Schedule;
use anyhow::Result;
use rust_xlsxwriter::{Image, Workbook, Worksheet};
use savefile::save_to_mem;
use std::path::PathBuf;

// use crate::VERSION; //, read_self_zip::Course, schedule::Schedule};

pub static TEMPLATE_PNG: &[u8] = include_bytes!("../assets/template.png");

fn trim_titles(s: &str) -> String {
    s.chars().take(31).collect()
}

// fn pretty_print_df_to_sheet(df: &DataFrame, sheet: &mut Worksheet) -> Result<()> {
//     let semesters = df.get_column_names().len() / Course::FIELD_NAMES_AS_ARRAY.len();
//     let lf = df.clone().lazy();
//     let exprs = (0..semesters)
//         .map(|x| {
//             when(
//                 col(format!("semester-{}_code", x + 1))
//                     .is_not_null()
//                     .and(col(format!("semester-{}_code", x + 1)).neq(lit(""))),
//             )
//             .then(col(format!("semester-{}_code", x + 1)))
//             .otherwise(
//                 when(
//                     col(format!("semester-{}_info", x + 1))
//                         .is_not_null()
//                         .and(col(format!("semester-{}_info", x + 1)).neq(lit(""))),
//                 )
//                 .then(col(format!("semester-{}_info", x + 1)))
//                 .otherwise(col(format!("semester-{}_name", x + 1))),
//             )
//             .alias(format!("semester-{}_prettyname", x + 1))
//         })
//         .collect::<Vec<_>>();
//     let df = lf.with_columns(exprs).collect()?;
//     let format = Format::new().set_align(FormatAlign::Center);

//     for col_idx in 0..semesters {
//         sheet.merge_range(
//             0,
//             (col_idx * 2) as u16,
//             0,
//             ((col_idx * 2) + 1) as u16,
//             &format!("Semester {}", col_idx + 1),
//             &format,
//         )?;
//     }

//     let keys = ["prettyname", "credit"];
//     let selected_columns = (0..semesters)
//         .flat_map(|x| {
//             keys.iter()
//                 .map(move |y| format!("semester-{}_{}", x + 1, y))
//         })
//         .collect::<Vec<_>>();
//     let selected_df = df.select(selected_columns)?;

//     for (col_idx, field) in selected_df.get_columns().iter().enumerate() {
//         let field = field.as_materialized_series();
//         let max_len = field.iter().map(|s| s.to_string().len()).max().unwrap_or(0);
//         sheet.set_column_width(col_idx as u16, max_len as f64 - 2.0)?;

//         for (row_idx, val) in field.iter().filter(|x| !x.is_null()).enumerate() {
//             match val {
//                 AnyValue::String(v) => {
//                     sheet.write_string((row_idx + 1) as u32, col_idx as u16, v)?
//                 }
//                 AnyValue::Int32(v) => {
//                     sheet.write_number((row_idx + 1) as u32, col_idx as u16, v as f64)?
//                 }
//                 AnyValue::Int64(v) => {
//                     sheet.write_number((row_idx + 1) as u32, col_idx as u16, v as f64)?
//                 }
//                 AnyValue::Float64(v) => {
//                     sheet.write_number((row_idx + 1) as u32, col_idx as u16, v)?
//                 }
//                 _ => sheet.write_string((row_idx + 1) as u32, col_idx as u16, val.to_string())?,
//             };
//         }
//     }
//     Ok(())
// }

fn embed_schedule_in_sheet(sheet: &mut Worksheet, sched: &Schedule) -> Result<()> {
    sheet.insert_image(
        0,
        0,
        &Image::new_from_buffer(&[TEMPLATE_PNG, &save_to_mem(SAVEFILE_VERSION, sched)?].concat())?,
    )?;

    Ok(())
}

pub fn save_schedule(fname: &PathBuf, sched: &Schedule) -> Result<()> {
    // let pad_col = Column::full_null(
    //     "PadColumn".into(),
    //     sched.programs.len() - 1,
    //     &DataType::String,
    // );R
    // let mut cat_col = Column::new(
    //     "Catalog".into(),
    //     vec![format!(
    //         "{}-{}",
    //         sched.catalog.low_year,
    //         sched.catalog.low_year + 1
    //     )],
    // );
    // let mut sched_col = Column::new("Schedulebot".into(), vec![VERSION]);
    // cat_col.append(&pad_col)?;
    // sched_col.append(&pad_col)?;

    // let meta_df = DataFrame::new(vec![
    //     Column::new("Programs".into(), &sched.programs),
    //     cat_col,
    //     sched_col,
    // ])?;

    let mut workbook = Workbook::new();

    // let schedule_sheet = workbook.add_worksheet().set_name("Schedule")?;
    // pretty_print_df_to_sheet(&sched.df, schedule_sheet)?;
    // schedule_sheet.protect();

    let test_sheet = workbook.add_worksheet().set_name("TESTING EMBED")?;
    embed_schedule_in_sheet(test_sheet, sched)?;
    test_sheet.protect();
    // #[cfg(not(debug_assertions))]
    // test_sheet.set_hidden(true);

    // for (name, df) in &sched.catalog.programs {
    //     let sheet = workbook.add_worksheet().set_name(trim_titles(name))?;
    //     write_df_to_sheet(df, sheet)?;
    //     sheet.protect();
    //     #[cfg(not(debug_assertions))]
    //     sheet.set_hidden(true);
    // }

    workbook.save(fname)?;
    Ok(())
}
