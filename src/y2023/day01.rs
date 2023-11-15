use crate::solution::{AocError, Solution};

pub struct Day01;

impl Solution for Day01 {
    type F = u64;
    type S = u64;

    fn meta(&self) -> (u32, u32) {
        (1, 2023)
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/2022/day01.txt")
    }

    fn part_1(&self, _input: &str) -> Result<u64, AocError> {
        Ok(0)
    }

    fn part_2(&self, _input: &str) -> Result<u64, AocError> {
        Ok(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(Day01.part_1(""), Ok(0));
    }
}
