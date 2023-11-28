use crate::solution::{AocError, Solution};

pub struct Day01;

impl Solution for Day01 {
    type F = usize;
    type S = usize;

    fn meta(&self) -> (u32, u32) {
        (1, 2020)
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2020/day01.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let input: Vec<usize> = input
            .split("\n")
            .map(|num| num.parse::<usize>().expect("parsing"))
            .collect();

        for first_idx in 0..(input.len() - 1) {
            for second_idx in (first_idx + 1)..input.len() {
                if input[first_idx] + input[second_idx] == 2020 {
                    return Ok(input[first_idx] * input[second_idx]);
                }
            }
        }

        Err(AocError::logic("no solution"))
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let input: Vec<usize> = input
            .split("\n")
            .map(|num| num.parse::<usize>().expect("parsing"))
            .collect();

        for first_idx in 0..(input.len() - 2) {
            for second_idx in (first_idx + 1)..(input.len() - 1) {
                for third_idx in (second_idx + 1)..input.len() {
                    if input[first_idx] + input[second_idx] + input[third_idx] == 2020 {
                        return Ok(input[first_idx] * input[second_idx] * input[third_idx]);
                    }
                }
            }
        }

        Err(AocError::logic("no solution"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(Day01.part_1("1721\n979\n366\n299\n675\n1456"), Ok(514579));
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day01.part_2("1721\n979\n366\n299\n675\n1456"),
            Ok(241861950)
        );
    }
}
