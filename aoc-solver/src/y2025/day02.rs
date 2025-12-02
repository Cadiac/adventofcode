use itertools::Itertools;

use crate::solution::{AocError, Solution};

pub struct Day02;

fn parse(input: &str) -> Result<Vec<(u64, u64)>, AocError> {
    input
        .trim()
        .split(',')
        .map(|range| {
            let (start_str, end_str) = range
                .split_once('-')
                .ok_or(AocError::parse(range, "missing separator"))?;

            let start = start_str
                .parse::<u64>()
                .map_err(|err| AocError::parse(start_str, err))?;

            let end = end_str
                .parse::<u64>()
                .map_err(|err| AocError::parse(end_str, err))?;

            Ok((start, end))
        })
        .try_collect()
}

fn is_invalid_part1(id: u64) -> bool {
    let digits = id.ilog10() + 1;
    if !(digits).is_multiple_of(2) {
        return false;
    }

    let mid = 10u64.pow(digits / 2);
    let beginning = id / mid;
    let end = id - beginning * mid;

    beginning == end
}

fn is_invalid_part2(id: u64) -> bool {
    let len = id.ilog10() as usize + 1;
    let digits: Vec<char> = id.to_string().chars().collect();

    for chunk_size in 1..=(len / 2) {
        let chunks: Vec<_> = digits.chunks(chunk_size).collect();
        let first = chunks[0];
        let is_invalid = chunks.iter().all(|&chunk| chunk == first);

        if is_invalid {
            return true;
        }
    }

    false
}

impl Solution for Day02 {
    type Part1 = u64;
    type Part2 = u64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2025/day02.txt")
    }

    fn part_1(&self, input: &str) -> Result<u64, AocError> {
        let ranges = parse(input)?;

        let invalid = ranges
            .into_iter()
            .flat_map(|(start, end)| start..=end)
            .filter(|&id| is_invalid_part1(id))
            .sum();

        Ok(invalid)
    }

    fn part_2(&self, input: &str) -> Result<u64, AocError> {
        let ranges = parse(input)?;

        let invalid = ranges
            .into_iter()
            .flat_map(|(start, end)| start..=end)
            .filter(|&id| is_invalid_part2(id))
            .sum();

        Ok(invalid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_part1_valid_products() {
        assert!(!is_invalid_part1(12));
        assert!(!is_invalid_part1(123));
        assert!(!is_invalid_part1(11233));
        assert!(!is_invalid_part1(38593862));
    }

    #[test]
    fn it_finds_part1_invalid_products() {
        assert!(is_invalid_part1(55));
        assert!(is_invalid_part1(6464));
        assert!(is_invalid_part1(123123));
        assert!(is_invalid_part1(1188511885));
    }

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day02.part_1(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            ),
            Ok(1227775554)
        );
    }

    #[test]
    fn it_finds_part2_valid_products() {
        assert!(!is_invalid_part2(12));
        assert!(!is_invalid_part2(123));
        assert!(!is_invalid_part2(11233));
        assert!(!is_invalid_part2(38593862));
    }

    #[test]
    fn it_finds_part2_invalid_products() {
        assert!(is_invalid_part2(111));
        assert!(is_invalid_part2(1010));
        assert!(is_invalid_part2(1188511885));
        assert!(is_invalid_part2(222222));
        assert!(is_invalid_part2(565656));
        assert!(is_invalid_part2(824824824));
        assert!(is_invalid_part2(2121212121));
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day02.part_2(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            ),
            Ok(4174379265)
        );
    }
}
