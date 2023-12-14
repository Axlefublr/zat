use std::error::Error;
use std::fs;

use crate::args::Args;
use clap::Parser;

mod args;

fn main() -> Result<(), Box<dyn Error>> {
    let Args { file, start, end } = Args::parse();
    let contents = fs::read_to_string(file)?;
    let start = start.unwrap_or_default().max(1) - 1;
    let end = end.unwrap_or(usize::MAX).max(1) - 1;
    let lines: Vec<_> = contents
        .lines()
        .enumerate()
        .filter(|&(index, _)| index >= start && index <= end)
        .map(|(_, line)| line)
        .collect();
    print!("{}", lines.join("\n"));
    Ok(())
}
