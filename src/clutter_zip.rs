use std::fs;
use std::io::{self, Read};

/// Scan arbitrary bytes for ZIP Local File Headers (LFH) and yield (filename, contents)
pub fn scan_zip_entries(data: &[u8]) -> impl Iterator<Item = io::Result<(String, Vec<u8>)>> + '_ {
    ZipEntryIterator { data, pos: 0 }
}

struct ZipEntryIterator<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Iterator for ZipEntryIterator<'a> {
    type Item = io::Result<(String, Vec<u8>)>;

    fn next(&mut self) -> Option<Self::Item> {
        const LFH_SIG: &[u8] = b"PK\x03\x04";

        // Search for next LFH
        let idx = self.data[self.pos..]
            .windows(4)
            .position(|w| w == LFH_SIG)?;

        let abs = self.pos + idx;

        // Validate header length
        if abs + 30 > self.data.len() {
            return Some(Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Incomplete LFH",
            )));
        }

        // Parse LFH fields
        let file_name_len =
            u16::from_le_bytes(self.data[abs + 26..abs + 28].try_into().unwrap()) as usize;
        let extra_len =
            u16::from_le_bytes(self.data[abs + 28..abs + 30].try_into().unwrap()) as usize;

        let name_start = abs + 30;
        let name_end = name_start + file_name_len;

        if name_end > self.data.len() {
            return Some(Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "LFH file name out of bounds",
            )));
        }

        let file_name = match String::from_utf8(self.data[name_start..name_end].to_vec()) {
            Ok(s) => s,
            Err(_) => {
                return Some(Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid UTF-8 filename",
                )))
            }
        };

        let data_start = name_end + extra_len;

        if abs + 22 > self.data.len() {
            return Some(Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "LFH too short",
            )));
        }

        let compressed_size =
            u32::from_le_bytes(self.data[abs + 18..abs + 22].try_into().unwrap()) as usize;

        if data_start + compressed_size > self.data.len() {
            return Some(Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "File data truncated",
            )));
        }

        let contents = self.data[data_start..data_start + compressed_size].to_vec();

        // Advance for next search
        self.pos = data_start + compressed_size;

        Some(Ok((file_name, contents)))
    }
}
