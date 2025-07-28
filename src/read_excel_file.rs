use std::path::PathBuf;

use crate::schedule::Schedule;

use crate::SAVEFILE_VERSION;
use crate::write_excel_file::TEMPLATE_PNG;
use anyhow::{Result, bail};
use savefile::prelude::*;
use umya_spreadsheet::reader::xlsx;

pub fn read_file(fname: &PathBuf) -> Result<Schedule> {
    // Open workbook
    let workbook = xlsx::read(fname)?;

    // Get the "TESTING EMBED" sheet
    let sheet = workbook
        .get_sheet_by_name("TESTING EMBED")
        .ok_or_else(|| anyhow::anyhow!("Sheet 'TESTING EMBED' not found"))?;

    // Find image at cell A1 (row 1, col 1)
    let image = sheet
        .get_image((1, 1))
        .ok_or_else(|| anyhow::anyhow!("No image found at cell A1"))?;

    // Get image bytes
    let img_bytes = image.get_image_data();

    // Assuming the embedded data is appended to the PNG, find the start of the appended data
    // This assumes TEMPLATE_PNG is the prefix, so skip its length
    let template_len = TEMPLATE_PNG.len();
    if img_bytes.len() <= template_len {
        bail!("Image does not contain embedded data");
    }
    let embedded_bytes = &img_bytes[template_len..];

    // Deserialize Player
    let sched: Schedule = load_from_mem(embedded_bytes, SAVEFILE_VERSION)?;
    Ok(sched)
}
