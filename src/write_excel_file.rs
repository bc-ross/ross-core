use crate::schedule::Schedule;
use crate::{SAVEFILE_VERSION, TEMPLATE_PNG};
use anyhow::Result;
use rust_xlsxwriter::{Format, FormatAlign, Image, Workbook, Worksheet};
use savefile::save_to_mem;
use std::path::PathBuf;

fn pretty_print_sched_to_sheet(sched: &Schedule, sheet: &mut Worksheet) -> Result<()> {
    let semesters = sched.courses.len() + 1;
    let format = Format::new().set_align(FormatAlign::Center);

    sheet.merge_range(0, 0, 0, 1, "Incoming", &format)?;
    for col_idx in 1..semesters {
        sheet.merge_range(
            0,
            (col_idx * 2) as u16,
            0,
            ((col_idx * 2) + 1) as u16,
            &format!("Semester {}", col_idx),
            &format,
        )?;
    }

    for (row_idx, val) in sched.incoming.iter().enumerate() {
        sheet.write_string((row_idx + 1) as u32, 0, val.to_string())?;
        sheet.write_string(
            (row_idx + 1) as u32,
            1,
            sched
                .catalog
                .courses
                .get(val)
                .map(|(_, x, _)| x.map(|x| x.to_string()).unwrap_or("cr".into()))
                .ok_or(anyhow::anyhow!("Course lookup not found: {}", val))?,
        )?;
    }

    for (col_idx, field) in sched.courses.iter().enumerate() {
        for (row_idx, val) in field.iter().enumerate() {
            sheet.write_string(
                (row_idx + 1) as u32,
                ((col_idx + 1) * 2) as u16,
                val.to_string(),
            )?;
            sheet.write_string(
                (row_idx + 1) as u32,
                ((col_idx + 1) * 2 + 1) as u16,
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
    let mut workbook = Workbook::new();

    let schedule_sheet = workbook.add_worksheet().set_name("Schedule")?;
    pretty_print_sched_to_sheet(&sched, schedule_sheet)?;
    schedule_sheet.protect();

    let test_sheet = workbook.add_worksheet().set_name("Internals")?;
    embed_schedule_in_sheet(test_sheet, sched)?;
    test_sheet.protect();
    #[cfg(not(debug_assertions))]
    test_sheet.set_hidden(true);

    workbook.save(fname)?;
    Ok(())
}
