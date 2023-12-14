use crate::args::Args;
use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod args;

fn main() -> Result<(), Box<dyn Error>> {
    let Args { file, start, end } = Args::parse();

    let contents: Box<dyn io::Read> = match file {
        Some(path) => Box::new(File::open(path)?),
        None => Box::new(io::stdin()),
    };
    let start = start.unwrap_or_default().max(1) - 1;
    let end = end.unwrap_or(usize::MAX).max(1) - 1;
    let lines = BufReader::new(contents)
        .lines()
        .enumerate()
        .filter(|&(index, _)| index >= start && index <= end)
        .map(|(_, line)| line.unwrap()); // invalid utf8 and IO problems shouldn't be silent

    for line in lines {
        println!("{line}");
    }

    Ok(())
}
