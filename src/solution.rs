use std::error::Error;
use std::fmt;

use log::info;

pub mod day01;
pub mod day02;

#[derive(Clone, Debug, PartialEq, clap::ValueEnum)]
pub enum Day {
    Day01,
    Day02,
    Day03,
}

pub trait Solution {
    type F: fmt::Display;
    type S: fmt::Display;

    fn name(&self) -> &'static str;
    fn default_input(&self) -> &'static str;

    fn part_1(&self, input: &str) -> Result<Self::F, Box<dyn Error>>;
    fn part_2(&self, input: &str) -> Result<Self::S, Box<dyn Error>>;

    fn run(&self, input: Option<String>) -> Result<(), Box<dyn Error>> {
        let input = input.unwrap_or(self.default_input().to_owned());
        let name = self.name();

        let part_1 = self.part_1(&input)?;
        info!("{name}: Part 1 - {part_1}");

        let part_2 = self.part_2(&input)?;
        info!("{name}: Part 2 - {part_2}");

        Ok(())
    }
}

pub fn run_solution(day: Day, input: Option<String>) -> Result<(), Box<dyn Error>>{
    match day {
        Day::Day01 => day01::Day01.run(input),
        Day::Day02 => day02::Day02.run(input),
        _ => unimplemented!(),
    }
}

pub fn run_all() -> Result<(), Box<dyn Error>>{
    day01::Day01.run(None)?;
    day02::Day02.run(None)?;

    Ok(())
}
