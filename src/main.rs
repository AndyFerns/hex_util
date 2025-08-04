use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process;

const BYTES_PER_LINE: usize = 16;

/// Function to convert a byte to a printable ASCII character
fn to_ascii(byte: u8) -> char {
    if byte.is_ascii_graphic() || byte == b' ' {
        byte as char
    } else {
        '.'
    }
}

fn hexdump(filename: &str) -> io::Result<()> {
    let mut file = File::open(filename)?;
    let mut buffer = [0u8; BYTES_PER_LINE];
    let mut offset = 0;

    while let Ok(n) = file.read(&mut buffer) {
        if n == 0 {
            break;
        }

        // Print offset
        print!("{:08x}: ", offset);

        // Print hex values
        for i in 0..BYTES_PER_LINE {
            if i < n {
                print!("{:02x} ", buffer[i]);
            } else {
                print!("   ");
            }

            if i == 7 {
                print!(" ");
            }
        }

        // Print ASCII values
        print!("|");
        for &b in &buffer[..n] {
            print!("{}", to_ascii(b));
        }
        println!("|");

        offset += n;
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: hexutil <file>");
        process::exit(1);
    }

    if let Err(e) = hexdump(&args[1]) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}