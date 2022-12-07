use clap::{Parser, ValueHint};
use env_logger::Env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use aoc::solution::{run_all, run_solution};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Day of the Solution
    #[clap(short, long, value_name = "<1-25>")]
    day: Option<u8>,

    /// Path to the input file
    #[clap(short, long, value_hint = ValueHint::FilePath)]
    file: Option<PathBuf>,

    /// Print game actions debug output (slow)
    #[clap(short, long, action)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Args::parse();
    init_logger(cli.verbose);

    match cli.day {
        None => {
            run_all();
        }
        Some(day) => {
            let input = match cli.file {
                None => None,
                Some(path) => {
                    let mut file = File::open(&path)?;
                    let mut input = String::new();
                    file.read_to_string(&mut input)?;

                    Some(input)
                }
            };

            run_solution(day, input);
        }
    };

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
