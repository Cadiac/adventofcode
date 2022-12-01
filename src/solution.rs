use std::error::Error;
use std::fmt;

pub mod day01;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Answer {
    U64(u64),
    I64(i64),
    Str(String),
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Answer::U64(value) => write!(f, "{value}"),
            Answer::I64(value) => write!(f, "{value}"),
            Answer::Str(value) => write!(f, "{value}"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, clap::ValueEnum)]
pub enum Day {
    Day01,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
    Day08,
    Day09,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

pub trait Solution {
    fn part_1(&self) -> Result<Answer, Box<dyn Error>>;
    fn part_2(&self) -> Result<Answer, Box<dyn Error>>;
}

impl From<(Day, &str)> for Box<dyn Solution> {
    fn from((day, input): (Day, &str)) -> Box<dyn Solution> {
        match day {
            Day::Day01 => Box::new(day01::Day01::new(input)),
            _ => unimplemented!("{day:?}")
        }
    }
}

impl From<Day> for Box<dyn Solution> {
    fn from(day: Day) -> Box<dyn Solution> {
        match day {
            Day::Day01 => Box::new(day01::Day01::default()),
            _ => unimplemented!("{day:?}")
        }
    }
}