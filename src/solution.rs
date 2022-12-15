use std::error::Error;
use std::fmt;

use log::{error, info};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;

pub const MAX_DAYS: u8 = 15;

pub fn run_solution(day: u8, input: Option<String>) -> Vec<String> {
    match day {
        1 => day01::Day01.run(input),
        2 => day02::Day02.run(input),
        3 => day03::Day03.run(input),
        4 => day04::Day04.run(input),
        5 => day05::Day05.run(input),
        6 => day06::Day06.run(input),
        7 => day07::Day07.run(input),
        8 => day08::Day08.run(input),
        9 => day09::Day09.run(input),
        10 => day10::Day10.run(input),
        11 => day11::Day11.run(input),
        12 => day12::Day12.run(input),
        13 => day13::Day13.run(input),
        14 => day14::Day14.run(input),
        15 => day15::Day15.run(input),
        _ => unimplemented!(),
    }
}

pub fn run_all() -> Vec<String> {
    let mut output = Vec::new();

    for day in 1..=MAX_DAYS {
        output.append(&mut run_solution(day, None));
        output.push("--".to_string())
    }

    output
}


#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AocError(String);

impl AocError {
    fn logic<E>(err: E) -> Self
    where
        E: fmt::Display,
    {
        AocError(format!("Logic error: {err}"))
    }

    fn parse<I, E>(input: I, err: E) -> Self
    where
        I: fmt::Display,
        E: fmt::Display,
    {
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
            }
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
            }
            Err(err) => {
                let logline = format!("[{name}][Part 2] Error: {err}");
                error!("{logline}");
                output.push(logline);
            }
        };

        output
    }
}
