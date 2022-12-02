use std::error::Error;

use crate::solution::{Solution};

pub struct Day03;

impl Solution for Day03 {
    type F = i64;
    type S = i64;

    fn name(&self) -> &'static str { "Day 03" }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/day03.txt")
    }

    fn part_1(&self, input: &str) -> Result<i64, Box<dyn Error>> {
        unimplemented!();
    }

    fn part_2(&self, input: &str) -> Result<i64, Box<dyn Error>> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day03.part_1("A Y\nB X\nC Z").unwrap(),
            15
        );
    }

    #[test]
    fn it_solves_part2_example() {
        unimplemented!();
    }
}
