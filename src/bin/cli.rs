use clap::{Parser, ValueHint};
use env_logger::Env;
use std::io::Read;
use std::{path::PathBuf};
use std::error::Error;
use std::fs::{File};

use aoc::solution::{Solution, Day};

#[macro_use]
extern crate log;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Day of the Solution
    #[clap(short, long, value_enum)]
    day: Day,

    /// Print game actions debug output (slow)
    #[clap(short, long, action)]
    verbose: bool,

    /// Path to the input file
    #[clap(short, long, value_hint = ValueHint::FilePath, conflicts_with="stdin")]
    file: Option<PathBuf>,

    /// Read input from stdin
    #[clap(short, long)]
    stdin: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Args::parse();
    init_logger(cli.verbose);

    let solver: Box<dyn Solution> = match cli.file {
        Some(path) => {
            let mut file = File::open(&path)?;
            let mut input = String::new();
            file.read_to_string(&mut input)?;
        
            (cli.day, input.as_str()).into()
        },
        None => {
            cli.day.into()
        }
    };

    let part_1 = solver.part_1()?;
    info!("Part 1: {part_1}");

    let part_2 = solver.part_2()?;
    info!("Part 2: {part_2}");

    Ok(())
}

fn init_logger(verbose: bool) {
    let default_level = if verbose { "debug" } else { "info" };

    env_logger::Builder::from_env(
        Env::default()
            .filter_or("LOG_LEVEL", default_level)
            .write_style_or("LOG_STYLE", "always"),
    )
    .format_timestamp(None)
    .format_module_path(false)
    .init();
}
