use std::error::Error;

use crate::solution::{Solution};

#[derive(Clone, Copy)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<&str> for RPS {
    fn from(input: &str) -> RPS {
        match input {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissors,
            _ => unimplemented!(),
        }
    }
}

enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl From<&str> for Outcome {
    fn from(input: &str) -> Outcome {
        match input {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => unimplemented!(),
        }
    }
}

pub struct Day02;

fn score(selected: RPS, outcome: Outcome) -> i64 {
    selected as i64 + outcome as i64
}

impl Solution for Day02 {
    type F = i64;
    type S = i64;

    fn name(&self) -> &'static str { "Day 02" }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/day02.txt")
    }

    fn part_1(&self, input: &str) -> Result<i64, Box<dyn Error>> {
        let total = input
            .lines()
            .map(|line| {
                let mut choices = line.split(" ");
                (
                    choices.next().unwrap().into(),
                    choices.next().unwrap().into(),
                )
            })
            .map(|(opponent, selected): (RPS, RPS)| match opponent {
                RPS::Rock => match selected {
                    RPS::Rock => score(selected, Outcome::Draw),
                    RPS::Paper => score(selected, Outcome::Win),
                    RPS::Scissors => score(selected, Outcome::Lose),
                },
                RPS::Paper => match selected {
                    RPS::Rock => score(selected, Outcome::Lose),
                    RPS::Paper => score(selected, Outcome::Draw),
                    RPS::Scissors => score(selected, Outcome::Win),
                },
                RPS::Scissors => match selected {
                    RPS::Rock => score(selected, Outcome::Win),
                    RPS::Paper => score(selected, Outcome::Lose),
                    RPS::Scissors => score(selected, Outcome::Draw),
                },
            })
            .sum();

        Ok(total)
    }

    fn part_2(&self, input: &str) -> Result<i64, Box<dyn Error>> {
        let total = input
            .lines()
            .map(|line| {
                let mut choices = line.split(" ");
                (
                    choices.next().unwrap().into(),
                    choices.next().unwrap().into(),
                )
            })
            .map(|(opponent, outcome): (RPS, Outcome)| match opponent {
                RPS::Rock => match outcome {
                    Outcome::Lose => score(RPS::Scissors, outcome),
                    Outcome::Draw => score(RPS::Rock, outcome),
                    Outcome::Win => score(RPS::Paper, outcome),
                },
                RPS::Paper => match outcome {
                    Outcome::Lose => score(RPS::Rock, outcome),
                    Outcome::Draw => score(RPS::Paper, outcome),
                    Outcome::Win => score(RPS::Scissors, outcome),
                },
                RPS::Scissors => match outcome {
                    Outcome::Lose => score(RPS::Paper, outcome),
                    Outcome::Draw => score(RPS::Scissors, outcome),
                    Outcome::Win => score(RPS::Rock, outcome),
                },
            })
            .sum();

        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day02.part_1("A Y\nB X\nC Z").unwrap(),
            15
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day02.part_2("A Y\nB X\nC Z").unwrap(),
            12
        );
    }
}
