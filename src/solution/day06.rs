use itertools::Itertools;

use crate::solution::{AocError, Solution};

pub struct Day06;

fn find_marker(signal: Vec<char>, marker_len: usize) -> Result<usize, AocError> {
    signal
        .windows(marker_len)
        .position(|slice| slice.iter().unique().count() == marker_len)
        .map_or_else(
            || Err(AocError::logic("no marker detected")),
            |index| Ok(index + marker_len),
        )
}

impl Solution for Day06 {
    type F = usize;
    type S = usize;

    fn name(&self) -> &'static str {
        "Day 06"
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/day06.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        find_marker(input.chars().collect(), 4)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        find_marker(input.chars().collect(), 14)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(Day06.part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), Ok(5));
        assert_eq!(Day06.part_1("nppdvjthqldpwncqszvftbrmjlhg"), Ok(6));
        assert_eq!(Day06.part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Ok(10));
        assert_eq!(Day06.part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Ok(11));
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(Day06.part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Ok(19));
        assert_eq!(Day06.part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), Ok(23));
        assert_eq!(Day06.part_2("nppdvjthqldpwncqszvftbrmjlhg"), Ok(23));
        assert_eq!(Day06.part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Ok(29));
        assert_eq!(Day06.part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Ok(26));
    }
}
