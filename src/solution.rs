use std::error::Error;
use std::fmt;

use log::info;

pub mod day01;
pub mod day02;
// pub mod day03;

#[derive(Clone, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum Day {
    Day01,
    Day02,
    Day03,
    Day04,
}

pub trait Solution {
    type F: fmt::Display;
    type S: fmt::Display;

    fn name(&self) -> &'static str;
    fn default_input(&self) -> &'static str;

    fn part_1(&self, input: &str) -> Result<Self::F, Box<dyn Error>>;
    fn part_2(&self, input: &str) -> Result<Self::S, Box<dyn Error>>;

    fn run(&self, input: Option<String>) -> Result<Vec<String>, Box<dyn Error>> {
        let input = input.unwrap_or_else(|| self.default_input().to_owned());
        let name = self.name();
        let mut output = Vec::new();

        let part_1 = self.part_1(&input)?;

        let result = format!("{name}: Part 1 - {part_1}");
        info!("{result}");
        output.push(result);

        let part_2 = self.part_2(&input)?;

        let result = format!("{name}: Part 2 - {part_2}");
        info!("{result}");
        output.push(result);

        Ok(output)
    }
}

pub fn run_solution(day: Day, input: Option<String>) -> Result<Vec<String>, Box<dyn Error>>{
    match day {
        Day::Day01 => day01::Day01.run(input),
        Day::Day02 => day02::Day02.run(input),
        // Day::Day03 => day03::Day03.run(input),
        _ => unimplemented!(),
    }
}

pub fn run_all() -> Result<Vec<String>, Box<dyn Error>>{
    let mut output = Vec::new();

    output.append(&mut day01::Day01.run(None)?);
    output.append(&mut day02::Day02.run(None)?);
    // day03::Day03.run(None)?;

    Ok(output)
}