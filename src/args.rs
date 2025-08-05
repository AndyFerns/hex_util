use clap::Parser;

/// CLI tool to display hexdump of a file (like xxd)
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// File to read
    pub input: String,

    /// Output file (default: stdout)
    #[arg(short, long)]
    pub output: Option<String>,

    /// Offset in bytes to start reading
    #[arg(short, long, default_value_t = 0)]
    pub offset: usize,

    /// Maximum number of bytes to read
    #[arg(short, long)]
    pub length: Option<usize>,
}