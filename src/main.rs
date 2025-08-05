mod args;
mod hex;

use args::Args;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufWriter};
use std::process;


fn main() {
    let args = Args::parse();

    // Setup output writer
    let writer: Box<dyn io::Write> = match &args.output {
        Some(path) => match File::create(path) {
            Ok(f) => Box::new(BufWriter::new(f)),
            Err(e) => {
                eprintln!("❌ Could not create output file: {e}");
                process::exit(1);
            }
        },
        None => Box::new(io::stdout()),
    };

    if let Err(e) = hex::hexdump(&args.input, writer, args.offset, args.length) {
        eprintln!("❌ Error: {e}");
        process::exit(1);
    }
}
