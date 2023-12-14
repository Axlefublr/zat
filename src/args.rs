use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    pub file: PathBuf,
    #[arg(short, long)]
    pub start: Option<usize>,
    #[arg(short, long)]
    pub end: Option<usize>,
}
