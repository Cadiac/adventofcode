use crate::solution::{AocError, Solution};

#[derive(Clone, Copy)]
enum Rps {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<&str> for Rps {
    fn from(input: &str) -> Rps {
        match input {
            "A" | "X" => Rps::Rock,
            "B" | "Y" => Rps::Paper,
            "C" | "Z" => Rps::Scissors,
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

fn score(selected: Rps, outcome: Outcome) -> i64 {
    selected as i64 + outcome as i64
}

impl Solution for Day02 {
    type A = i64;
    type B = i64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2022/day02.txt")
    }

    fn part_1(&self, input: &str) -> Result<i64, AocError> {
        let mut total = 0;

        for line in input.lines() {
            let mut choices = line.split(' ');

            let opponent: Rps = choices
                .next()
                .ok_or_else(|| AocError::parse(line, "missing opponent"))?
                .into();
            let selected: Rps = choices
                .next()
                .ok_or_else(|| AocError::parse(line, "missing selected"))?
                .into();

            let outcome = match opponent {
                Rps::Rock => match selected {
                    Rps::Rock => Outcome::Draw,
                    Rps::Paper => Outcome::Win,
                    Rps::Scissors => Outcome::Lose,
                },
                Rps::Paper => match selected {
                    Rps::Rock => Outcome::Lose,
                    Rps::Paper => Outcome::Draw,
                    Rps::Scissors => Outcome::Win,
                },
                Rps::Scissors => match selected {
                    Rps::Rock => Outcome::Win,
                    Rps::Paper => Outcome::Lose,
                    Rps::Scissors => Outcome::Draw,
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

            let opponent: Rps = choices
                .next()
                .ok_or_else(|| AocError::parse(line, "missing opponent"))?
                .into();
            let outcome: Outcome = choices
                .next()
                .ok_or_else(|| AocError::parse(line, "missing outcome"))?
                .into();

            let selected = match opponent {
                Rps::Rock => match outcome {
                    Outcome::Lose => Rps::Scissors,
                    Outcome::Draw => Rps::Rock,
                    Outcome::Win => Rps::Paper,
                },
                Rps::Paper => match outcome {
                    Outcome::Lose => Rps::Rock,
                    Outcome::Draw => Rps::Paper,
                    Outcome::Win => Rps::Scissors,
                },
                Rps::Scissors => match outcome {
                    Outcome::Lose => Rps::Paper,
                    Outcome::Draw => Rps::Scissors,
                    Outcome::Win => Rps::Rock,
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
