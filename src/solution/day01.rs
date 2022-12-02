use std::error::Error;

use crate::solution::{Solution};

pub struct Day01;

fn parse(input: &str) -> Vec<u64> {
    input
        .split("\n\n")
        .map(|chunk| chunk.lines().map(|line| line.parse::<u64>().unwrap()).sum())
        .collect()
}

impl Solution for Day01 {
    type F = u64;
    type S = u64;

    fn name(&self) -> &'static str { "Day 01" }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/day01.txt")
    }

    fn part_1(&self, input: &str) -> Result<u64, Box<dyn Error>> {
        let elves = parse(input);
        let most = elves.iter().max().unwrap();

        Ok(*most)
    }

    fn part_2(&self, input: &str) -> Result<u64, Box<dyn Error>> {
        let elves = parse(input);
        let mut sorted = elves.clone();
        sorted.sort();

        let top_3 = sorted.iter().rev().take(3).sum();

        Ok(top_3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day01.part_1(
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
            .unwrap(),
            24000
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day01.part_2(
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
            .unwrap(),
            45000
        );
    }
}
