use std::error::Error;
use std::fmt;

use log::{info, error};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

pub const MAX_DAYS: u8 = 6;

#[derive(Clone, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum Day {
    Day01,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
}

impl From<u8> for Day {
    fn from(num: u8) -> Self {
        match num {
            1 => Day::Day01,
            2 => Day::Day02,
            3 => Day::Day03,
            4 => Day::Day04,
            5 => Day::Day05,
            6 => Day::Day06,
            _ => unimplemented!(),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AocError(String);

impl AocError {
    fn logic<E: fmt::Display>(err: E) -> Self {
        AocError(format!("Logic error: {err}"))
    }
    fn parse<E: fmt::Display>(input: &str, err: E) -> Self {
        AocError(format!("Parse error at: {input}: {err}"))
    }
}

impl Error for AocError {}

impl std::fmt::Display for AocError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


pub trait Solution {
    type F: fmt::Display;
    type S: fmt::Display;

    fn name(&self) -> &'static str;
    fn default_input(&self) -> &'static str;

    fn part_1(&self, input: &str) -> Result<Self::F, AocError>;
    fn part_2(&self, input: &str) -> Result<Self::S, AocError>;

    fn run(&self, input: Option<String>) -> Vec<String> {
        let input = input.unwrap_or_else(|| self.default_input().to_owned());
        let name = self.name();
        let mut output = Vec::new();

        match self.part_1(&input) {
            Ok(result) => {
                let logline = format!("[{name}][Part 1] {result}");
                info!("{logline}");
                output.push(logline);
            },
            Err(err) => {
                let logline = format!("[{name}][Part 1] Error: {err}");
                error!("{logline}");
                output.push(logline);
            }
        };

        match self.part_2(&input) {
            Ok(result) => {
                let logline = format!("[{name}][Part 2] {result}");
                info!("{logline}");
                output.push(logline);
            },
            Err(err) => {
                let logline = format!("[{name}][Part 2] Error: {err}");
                error!("{logline}");
                output.push(logline);
            }
        };

        output
    }
}

pub fn run_solution(day: Day, input: Option<String>) -> Vec<String> {
    match day {
        Day::Day01 => day01::Day01.run(input),
        Day::Day02 => day02::Day02.run(input),
        Day::Day03 => day03::Day03.run(input),
        Day::Day04 => day04::Day04.run(input),
        Day::Day05 => day05::Day05.run(input),
        Day::Day06 => day06::Day06.run(input),
        _ => unimplemented!(),
    }
}

pub fn run_all() -> Vec<String> {
    let mut output = Vec::new();

    for day in 1..=MAX_DAYS {
        output.append(&mut run_solution(day.into(), None));
        output.push("--".to_string())
    }

    output
}
