use std::error::Error;

use crate::solution::{Solution, Answer};

const INPUT_FILE: &str = include_str!("../../inputs/day02.txt");

pub struct Day01 {
    input: String,
}

impl Day01 {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_owned(),
        }
    }
}

impl Default for Day01 {
    fn default() -> Self {
        Self {
            input: INPUT_FILE.to_owned(),
        }
    }
}

impl Solution for Day01 {
    fn part_1(&self) -> Result<Answer, Box<dyn Error>> {
        let count = self
            .input
            .lines()
            .map(|depth| depth.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
            .windows(2)
            .filter(|depths| depths[0] < depths[1])
            .count();

        Ok(Answer::U64(count as u64))
    }

    fn part_2(&self) -> Result<Answer, Box<dyn Error>> {
        let count = self
            .input
            .lines()
            .map(|depth| depth.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
            .windows(3)
            .map(|depths| depths.iter().sum())
            .collect::<Vec<u32>>()
            .windows(2)
            .filter(|depths| depths[0] < depths[1])
            .count();

        Ok(Answer::U64(count as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day01::new(
                "199\n\
                    200\n\
                    208\n\
                    210\n\
                    200\n\
                    207\n\
                    240\n\
                    269\n\
                    260\n\
                    263"
            ).part_1().unwrap(),
            Answer::U64(7)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day01::new(
                "199\n\
                    200\n\
                    208\n\
                    210\n\
                    200\n\
                    207\n\
                    240\n\
                    269\n\
                    260\n\
                    263"
            ).part_2().unwrap(),
            Answer::U64(5)
        );
    }
}
