use clap::Parser;
use std::path::PathBuf;

/// Print a range of lines of a file.
/// Both --start and --end are inclusive.
/// Line numbers are 1 indexed, but 0 also means 1."
#[derive(Parser)]
#[command(author, version)]
pub struct Args {
    pub file: Option<PathBuf>,
    #[arg(short, long, value_name = "NUM")]
    pub start: Option<usize>,
    #[arg(short, long, value_name = "NUM")]
    pub end: Option<usize>,
}
