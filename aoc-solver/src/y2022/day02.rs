use crate::solution::{AocError, Solution};

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

    fn meta(&self) -> (u32, u32) {
        (2, 2022)
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2022/day02.txt")
    }

    fn part_1(&self, input: &str) -> Result<i64, AocError> {
        let mut total = 0;

        for line in input.lines() {
            let mut choices = line.split(' ');

            let opponent: RPS = choices
                .next()
                .ok_or_else(|| AocError::parse(line, "missing opponent"))?
                .into();
            let selected: RPS = choices
                .next()
                .ok_or_else(|| AocError::parse(line, "missing selected"))?
                .into();

            let outcome = match opponent {
                RPS::Rock => match selected {
                    RPS::Rock => Outcome::Draw,
                    RPS::Paper => Outcome::Win,
                    RPS::Scissors => Outcome::Lose,
                },
                RPS::Paper => match selected {
                    RPS::Rock => Outcome::Lose,
                    RPS::Paper => Outcome::Draw,
                    RPS::Scissors => Outcome::Win,
                },
                RPS::Scissors => match selected {
                    RPS::Rock => Outcome::Win,
                    RPS::Paper => Outcome::Lose,
                    RPS::Scissors => Outcome::Draw,
                },
            };

            total += score(selected, outcome);
        }

        Ok(total)
    }

    fn part_2(&self, input: &str) -> Result<i64, AocError> {
        let mut total = 0;

        for line in input.lines() {
            let mut choices = line.split(' ');

            let opponent: RPS = choices
                .next()
                .ok_or_else(|| AocError::parse(line, "missing opponent"))?
                .into();
            let outcome: Outcome = choices
                .next()
                .ok_or_else(|| AocError::parse(line, "missing outcome"))?
                .into();

            let selected = match opponent {
                RPS::Rock => match outcome {
                    Outcome::Lose => RPS::Scissors,
                    Outcome::Draw => RPS::Rock,
                    Outcome::Win => RPS::Paper,
                },
                RPS::Paper => match outcome {
                    Outcome::Lose => RPS::Rock,
                    Outcome::Draw => RPS::Paper,
                    Outcome::Win => RPS::Scissors,
                },
                RPS::Scissors => match outcome {
                    Outcome::Lose => RPS::Paper,
                    Outcome::Draw => RPS::Scissors,
                    Outcome::Win => RPS::Rock,
                },
            };

            total += score(selected, outcome);
        }

        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(Day02.part_1("A Y\nB X\nC Z"), Ok(15));
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(Day02.part_2("A Y\nB X\nC Z"), Ok(12));
    }
}
