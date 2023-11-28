use std::collections::VecDeque;

use crate::solution::{AocError, Solution};

pub struct Day09;

fn solve_part1(input: &str, preamble_len: usize) -> Result<usize, AocError> {
    let mut checksum: VecDeque<usize> = input
        .lines()
        .take(preamble_len)
        .map(|line| usize::from_str_radix(line, 10).unwrap())
        .collect();

    let first_invalid = input
        .lines()
        .skip(preamble_len)
        .map(|line| usize::from_str_radix(line, 10).unwrap())
        .find(|number| {
            for first in 0..(checksum.len() - 1) {
                for second in (first + 1)..(checksum.len()) {
                    if checksum[first] != checksum[second]
                        && checksum[first] + checksum[second] == *number
                    {
                        // Found two numbers that sum up to the current number.
                        // Push current to checksum and pop from the front
                        checksum.pop_front();
                        checksum.push_back(*number);

                        return false;
                    }
                }
            }

            return true;
        });

    first_invalid.ok_or(AocError::logic("No solution"))
}

fn solve_part2(input: &str, preamble_len: usize) -> Result<usize, AocError> {
    let first_invalid = solve_part1(input, preamble_len)?;

    let list: Vec<usize> = input
        .lines()
        .map(|line| usize::from_str_radix(line, 10).unwrap())
        .collect();

    for (index, number) in list.iter().enumerate() {
        let mut sum = *number;
        for other in (index + 1)..(list.len()) {
            if sum > first_invalid {
                break;
            }

            sum += list[other];

            if sum == first_invalid {
                // To find the encryption weakness, add together the smallest and largest number
                // in this contiguous range; in this example, these are 15 and 47, producing 62.
                let min = list[index..=other].iter().min().unwrap();
                let max = list[index..=other].iter().max().unwrap();

                return Ok(min + max);
            }
        }
    }

    Err(AocError::logic("No solution"))
}

impl Solution for Day09 {
    type F = usize;
    type S = usize;

    fn meta(&self) -> (u32, u32) {
        (9, 2020)
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2020/day09.txt")
    }

    fn part_1(&self, input: &str) -> Result<Self::F, AocError> {
        solve_part1(input, 25)
    }

    fn part_2(&self, input: &str) -> Result<Self::S, AocError> {
        solve_part2(input, 25)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_examples() {
        assert_eq!(
            solve_part1(
                "35\n\
                 20\n\
                 15\n\
                 25\n\
                 47\n\
                 40\n\
                 62\n\
                 55\n\
                 65\n\
                 95\n\
                 102\n\
                 117\n\
                 150\n\
                 182\n\
                 127\n\
                 219\n\
                 299\n\
                 277\n\
                 309\n\
                 576",
                5
            ),
            Ok(127)
        );
    }

    #[test]
    fn it_solves_part2_examples() {
        assert_eq!(
            solve_part2(
                "35\n\
                 20\n\
                 15\n\
                 25\n\
                 47\n\
                 40\n\
                 62\n\
                 55\n\
                 65\n\
                 95\n\
                 102\n\
                 117\n\
                 150\n\
                 182\n\
                 127\n\
                 219\n\
                 299\n\
                 277\n\
                 309\n\
                 576",
                5
            ),
            Ok(62)
        );
    }
}
