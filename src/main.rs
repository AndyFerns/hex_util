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
