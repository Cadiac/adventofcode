use std::error::Error;

use crate::solution::{Answer, Solution};

const DEFAULT_INPUT: &str = include_str!("../../inputs/day01.txt");

pub struct Day01 {
    elves: Vec<u64>,
}

impl Day01 {
    pub fn new(input: &str) -> Self {
        Self {
            elves: Day01::parse(input),
        }
    }

    pub fn parse(input: &str) -> Vec<u64> {
        input
            .split("\n\n")
            .map(|chunk| chunk.lines().map(|line| line.parse::<u64>().unwrap()).sum())
            .collect()
    }
}

impl Default for Day01 {
    fn default() -> Self {
        Self {
            elves: Day01::parse(DEFAULT_INPUT),
        }
    }
}

impl Solution for Day01 {
    fn part_1(&self) -> Result<Answer, Box<dyn Error>> {
        let most = self.elves.iter().max().unwrap();

        Ok(Answer::U64(*most))
    }

    fn part_2(&self) -> Result<Answer, Box<dyn Error>> {
        let mut sorted = self.elves.clone();
        sorted.sort();

        let top_3 = sorted.iter().rev().take(3).sum();

        Ok(Answer::U64(top_3))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day01::new(
                "1000\n\
                2000\n\
                3000\n\
                \n\
                4000\n\
                \n\
                5000\n\
                6000\n\
                \n\
                7000\n\
                8000\n\
                9000\n\
                \n\
                10000"
            )
            .part_1()
            .unwrap(),
            Answer::U64(24000)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day01::new(
                "1000\n\
                2000\n\
                3000\n\
                \n\
                4000\n\
                \n\
                5000\n\
                6000\n\
                \n\
                7000\n\
                8000\n\
                9000\n\
                \n\
                10000"
            )
            .part_2()
            .unwrap(),
            Answer::U64(45000)
        );
    }
}
