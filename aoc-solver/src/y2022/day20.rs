use crate::solution::{AocError, Solution};
use std::collections::VecDeque;

pub struct Day20;

impl Day20 {
    fn parse(input: &str) -> Result<VecDeque<(usize, i64)>, AocError> {
        let mut sequence = VecDeque::new();

        for (index, line) in input.lines().enumerate() {
            let value = line.parse().map_err(|err| AocError::parse(line, err))?;

            sequence.push_back((index, value));
        }

        Ok(sequence)
    }

    fn mix(sequence: &mut VecDeque<(usize, i64)>) -> Result<(), AocError> {
        for index in 0..sequence.len() {
            let pos = sequence
                .iter()
                .position(|(original_index, _)| *original_index == index)
                .ok_or_else(|| AocError::logic("position not found"))?;

            if let Some(current) = sequence.remove(pos) {
                let target = (pos as i64 + current.1).rem_euclid(sequence.len() as i64);

                if target > sequence.len() as i64 || target == 0 {
                    sequence.push_back(current);
                } else {
                    sequence.insert(target as usize, current)
                }
            }
        }

        Ok(())
    }

    fn checksum(mixed: VecDeque<(usize, i64)>) -> i64 {
        let numbers: Vec<_> = mixed
            .iter()
            .cycle()
            .skip_while(|(_, value)| *value != 0)
            .map(|(_, value)| value)
            .take(3001)
            .collect();

        numbers[1000] + numbers[2000] + numbers[3000]
    }
}

impl Solution for Day20 {
    type F = i64;
    type S = i64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2022/day20.txt")
    }

    fn part_1(&self, input: &str) -> Result<i64, AocError> {
        let mut sequence = Day20::parse(input)?;

        Day20::mix(&mut sequence)?;

        Ok(Day20::checksum(sequence))
    }

    fn part_2(&self, input: &str) -> Result<i64, AocError> {
        let sequence = Day20::parse(input)?;
        let decryption_key = 811589153;

        let mut mixed: VecDeque<(usize, i64)> = sequence
            .into_iter()
            .map(|(_, value)| value * decryption_key)
            .enumerate()
            .collect();

        for _round in 0..10 {
            Day20::mix(&mut mixed)?;
        }

        Ok(Day20::checksum(mixed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1\n2\n-3\n3\n-2\n0\n4";

    #[test]
    fn it_solves_part1() {
        assert_eq!(Day20.part_1(INPUT), Ok(3));
    }

    #[test]
    fn it_solves_part2() {
        assert_eq!(Day20.part_2(INPUT), Ok(1623178306));
    }
}
