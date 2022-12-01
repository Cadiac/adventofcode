use std::error::Error;

pub mod day01;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Answer {
    U64(u64),
    I64(i64),
    Text(String),
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

impl From<(Day, String)> for Box<dyn Solution> {
    fn from((day, input): (Day, String)) -> Box<dyn Solution> {
        match day {
            Day::Day01 => Box::new(day01::Day01::new(&input)),
            _ => unimplemented!("{day:?}")
        }
    }
}