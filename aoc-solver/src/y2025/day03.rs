use crate::solution::{AocError, Solution};

pub struct Day03;

fn parse(input: &str) -> Result<Vec<Vec<u32>>, AocError> {
    input
        .trim()
        .lines()
        .map(|batteries| {
            batteries
                .chars()
                .map(|battery| {
                    battery
                        .to_digit(10)
                        .ok_or(AocError::parse(battery, "invalid joltage"))
                })
                .collect()
        })
        .collect()
}

fn find_max_joltage(banks: &[u32], digits: usize) -> Option<u64> {
    if digits > banks.len() {
        return None;
    }

    let mut skip = 0;
    let mut result = 0;

    for i in 0..digits {
        let remaining = digits - i;
        let end = banks.len() - (remaining - 1);
        let slice = &banks[skip..end];

        let (offset, &max) = slice
            .iter()
            .enumerate()
            // max_by_key returns the last element
            .rev()
            .max_by_key(|&(_, v)| v)?;

        skip += offset + 1;
        result = result * 10 + max as u64;
    }

    Some(result)
}

impl Solution for Day03 {
    type Part1 = u64;
    type Part2 = u64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2025/day03.txt")
    }

    fn part_1(&self, input: &str) -> Result<u64, AocError> {
        let total = parse(input)?
            .iter()
            .flat_map(|bank| find_max_joltage(bank, 2))
            .sum();

        Ok(total)
    }

    fn part_2(&self, input: &str) -> Result<u64, AocError> {
        let total = parse(input)?
            .iter()
            .flat_map(|bank| find_max_joltage(bank, 12))
            .sum();

        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_max_joltage_1() {
        assert_eq!(
            find_max_joltage(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 2),
            Some(98)
        );
    }

    #[test]
    fn it_finds_max_joltage_2() {
        assert_eq!(
            find_max_joltage(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 2),
            Some(89)
        )
    }

    #[test]
    fn it_finds_max_joltage_3() {
        assert_eq!(
            find_max_joltage(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 2),
            Some(78)
        )
    }

    #[test]
    fn it_finds_max_joltage_4() {
        assert_eq!(
            find_max_joltage(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 2),
            Some(92)
        )
    }

    #[test]
    fn it_finds_max_joltage_5() {
        assert_eq!(
            find_max_joltage(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 12),
            Some(987654321111)
        );
    }

    #[test]
    fn it_finds_max_joltage_6() {
        assert_eq!(
            find_max_joltage(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 12),
            Some(811111111119)
        );
    }

    #[test]
    fn it_finds_max_joltage_7() {
        assert_eq!(
            find_max_joltage(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 12),
            Some(434234234278)
        );
    }

    #[test]
    fn it_finds_max_joltage_8() {
        assert_eq!(
            find_max_joltage(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 12),
            Some(888911112111)
        );
    }

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day03.part_1(
                "987654321111111\n\
                 811111111111119\n\
                 234234234234278\n\
                 818181911112111"
            ),
            Ok(357)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day03.part_2(
                "987654321111111\n\
                 811111111111119\n\
                 234234234234278\n\
                 818181911112111"
            ),
            Ok(3121910778619)
        );
    }
}
