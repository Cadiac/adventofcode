use std::error::Error;
use std::fmt;

use log::{error, info};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AocError(String);

impl AocError {
    pub fn logic<E>(err: E) -> Self
    where
        E: fmt::Display,
    {
        AocError(format!("Logic error: {err}"))
    }

    pub fn parse<I, E>(input: I, err: E) -> Self
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
    type A: fmt::Display;
    type B: fmt::Display;

    fn default_input(&self) -> &'static str;

    fn part_1(&self, input: &str) -> Result<Self::A, AocError>;
    fn part_2(&self, input: &str) -> Result<Self::B, AocError>;

    fn run(&self, input: Option<String>, day: u32, year: u32) -> Vec<String> {
        let input = input.unwrap_or_else(|| self.default_input().to_owned());
        let mut output = Vec::new();

        match self.part_1(&input) {
            Ok(result) => {
                let logline = format!("[{year}/{day:0>2}][Part 1] {result}");
                info!("{logline}");
                output.push(logline);
            }
            Err(err) => {
                let logline = format!("[{year}/{day:0>2}][Part 1] Error: {err}");
                error!("{logline}");
                output.push(logline);
            }
        };

        match self.part_2(&input) {
            Ok(result) => {
                let logline = format!("[{year}/{day:0>2}][Part 2] {result}");
                info!("{logline}");
                output.push(logline);
            }
            Err(err) => {
                let logline = format!("[{year}/{day:0>2}][Part 2] Error: {err}");
                error!("{logline}");
                output.push(logline);
            }
        };

        output
    }
}

pub trait Solver {
    fn run_solution(day: u8, input: Option<String>) -> Vec<String>;
    fn run_all() -> Vec<String>;
    fn get_source(day: u8) -> &'static str;
}
