use std::error::Error;
use itertools::Itertools;

use crate::solution::Solution;

pub struct Day06;

fn find_marker(signal: Vec<char>, marker_len: usize) -> usize {
    let index = signal.windows(marker_len)
        .position(|slice| slice.iter().unique().count() == marker_len)
        .unwrap();

    index + marker_len
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

    fn part_1(&self, input: &str) -> Result<usize, Box<dyn Error>> {
        Ok(find_marker(input.chars().collect(), 4))
    }

    fn part_2(&self, input: &str) -> Result<usize, Box<dyn Error>> {
        Ok(find_marker(input.chars().collect(), 14))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(Day06.part_1("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 5);
        assert_eq!(Day06.part_1("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 6);
        assert_eq!(Day06.part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(), 10);
        assert_eq!(Day06.part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(), 11);
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(Day06.part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap(), 19);
        assert_eq!(Day06.part_2("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 23);
        assert_eq!(Day06.part_2("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 23);
        assert_eq!(Day06.part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(), 29);
        assert_eq!(Day06.part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(), 26);
    }
}
