use crate::schedule::Schedule;
use crate::{SAVEFILE_VERSION, TEMPLATE_PNG};
use anyhow::Result;
use rust_xlsxwriter::{Format, FormatAlign, Image, Workbook, Worksheet};
use savefile::save_to_mem;
use std::path::PathBuf;

fn pretty_print_sched_to_sheet(sched: &Schedule, sheet: &mut Worksheet) -> Result<()> {
    let semesters = sched.courses.len();
    let format = Format::new().set_align(FormatAlign::Center);

    for col_idx in 0..semesters {
        sheet.merge_range(
            0,
            (col_idx * 2) as u16,
            0,
            ((col_idx * 2) + 1) as u16,
            &format!("Semester {}", col_idx + 1),
            &format,
        )?;
    }

    for (col_idx, field) in sched.courses.iter().enumerate() {
        for (row_idx, val) in field.iter().enumerate() {
            sheet.write_string((row_idx + 1) as u32, (col_idx * 2) as u16, val.to_string())?;
            sheet.write_string(
                (row_idx + 1) as u32,
                ((col_idx * 2) + 1) as u16,
                sched
                    .catalog
                    .courses
                    .get(&val)
                    .map(|(_, x, _)| x.map(|credits| credits.to_string()).unwrap_or("cr".into()))
                    .ok_or(anyhow::anyhow!("Course lookup not found: {}", val))?,
            )?;

            // TODO: How to add credit??
        }
    }
    sheet.autofit();

    Ok(())
}

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

    let schedule_sheet = workbook.add_worksheet().set_name("Schedule")?;
    pretty_print_sched_to_sheet(&sched, schedule_sheet)?;
    schedule_sheet.protect();

    let test_sheet = workbook.add_worksheet().set_name("Internals")?;
    embed_schedule_in_sheet(test_sheet, sched)?;
    test_sheet.protect();
    #[cfg(not(debug_assertions))]
    test_sheet.set_hidden(true);

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
