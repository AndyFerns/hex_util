use colored::*;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};

const BYTES_PER_LINE: usize = 16;

/// Converts a byte to a printable ASCII character or '.' if non-printable
fn to_ascii(byte: u8) -> char {
    if byte.is_ascii_graphic() || byte == b' ' {
        byte as char
    } else {
        '.'
    }
}

/// Writes a hexdump to a given output
pub fn hexdump(
    input_path: &str,
    mut output: Box<dyn Write>,
    offset: usize,
    length: Option<usize>,
) -> io::Result<()> {
    let mut file = File::open(input_path)?;
    let _ = file.seek(SeekFrom::Start(offset as u64))?;

    let mut buffer = [0u8; BYTES_PER_LINE];
    let mut total_read = 0;
    let mut current_offset = offset;

    while let Ok(n) = file.read(&mut buffer) {
        if n == 0 || (length.is_some() && total_read >= length.unwrap()) {
            break;
        }

        let line_len = if let Some(max) = length {
            std::cmp::min(n, max - total_read)
        } else {
            n
        };

        write!(output, "{:08x}: ", current_offset)?;

        for i in 0..BYTES_PER_LINE {
            if i < line_len {
                write!(output, "{} ", format!("{:02x}", buffer[i]).green())?;
            } else {
                write!(output, "   ")?;
            }

            if i == 7 {
                write!(output, " ")?;
            }
        }

        write!(output, "|")?;
        for &b in &buffer[..line_len] {
            write!(output, "{}", to_ascii(b).to_string().yellow())?;
        }
        writeln!(output, "|")?;

        total_read += line_len;
        current_offset += line_len;
    }

    Ok(())
}
