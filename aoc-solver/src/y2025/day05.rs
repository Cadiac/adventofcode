use itertools::Itertools;

use crate::solution::{AocError, Solution};

pub struct Day05;

struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn merge_with(&mut self, other: Range) {
        self.start = self.start.min(other.start);
        self.end = self.end.max(other.end);
    }
}

fn parse(input: &str) -> Result<(Vec<Range>, Vec<u64>), AocError> {
    let (fresh_str, available_str) = input
        .trim()
        .split_once("\n\n")
        .ok_or(AocError::parse(input, "missing chunks"))?;

    let fresh = fresh_str
        .lines()
        .map(|line| {
            let (start_str, end_str) = line
                .split_once("-")
                .ok_or(AocError::parse(input, "invalid range"))?;

            let start = start_str
                .parse::<u64>()
                .map_err(|err| AocError::parse(line, err))?;

            let end = end_str
                .parse::<u64>()
                .map_err(|err| AocError::parse(line, err))?;

            Ok(Range { start, end })
        })
        .try_collect()?;

    let available = available_str
        .lines()
        .map(|line| {
            line.parse::<u64>()
                .map_err(|err| AocError::parse(line, err))
        })
        .try_collect()?;

    Ok((fresh, available))
}

fn is_fresh(ingredient: u64, valid: &[Range]) -> bool {
    valid
        .iter()
        .any(|range| ingredient >= range.start && ingredient <= range.end)
}

fn is_overlapping(a: &Range, b: &Range) -> bool {
    a.end >= b.start && a.start <= b.end
}

fn simplify(fresh: &mut Vec<Range>) -> bool {
    for i in 0..fresh.len() {
        for j in i + 1..fresh.len() {
            if is_overlapping(&fresh[i], &fresh[j]) {
                let removed = fresh.swap_remove(j);
                fresh[i].merge_with(removed);
                return true;
            }
        }
    }

    false
}

impl Solution for Day05 {
    type Part1 = usize;
    type Part2 = u64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2025/day05.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let (fresh, available) = parse(input)?;

        let result = available
            .into_iter()
            .filter(|ingredient| is_fresh(*ingredient, &fresh))
            .count();

        Ok(result)
    }

    fn part_2(&self, input: &str) -> Result<u64, AocError> {
        let (mut fresh, _available) = parse(input)?;

        while simplify(&mut fresh) {}

        let result = fresh.iter().map(|range| range.end - range.start + 1).sum();

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day05.part_1(
                "3-5\n\
                 10-14\n\
                 16-20\n\
                 12-18\n\
                 \n\
                 1\n\
                 5\n\
                 8\n\
                 11\n\
                 17\n\
                 32"
            ),
            Ok(3)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day05.part_2(
                "3-5\n\
                 10-14\n\
                 16-20\n\
                 12-18\n\
                 \n\
                 1\n\
                 5\n\
                 8\n\
                 11\n\
                 17\n\
                 32"
            ),
            Ok(14)
        );
    }

    #[test]
    fn it_finds_overlapping_ranges() {
        assert!(is_overlapping(
            &Range { start: 10, end: 14 },
            &Range { start: 12, end: 18 }
        ));

        assert!(!is_overlapping(
            &Range { start: 10, end: 14 },
            &Range { start: 15, end: 18 }
        ));
    }

    #[test]
    fn it_merges_ranges_1() {
        let mut a = Range { start: 10, end: 14 };
        let b = Range { start: 12, end: 18 };

        a.merge_with(b);
        assert_eq!(a.start, 10);
        assert_eq!(a.end, 18);
    }

    #[test]
    fn it_merges_ranges_2() {
        let mut a = Range { start: 10, end: 14 };
        let b = Range { start: 12, end: 13 };

        a.merge_with(b);
        assert_eq!(a.start, 10);
        assert_eq!(a.end, 14);
    }

    #[test]
    fn it_merges_ranges_3() {
        let mut a = Range { start: 12, end: 14 };
        let b = Range { start: 10, end: 13 };

        a.merge_with(b);
        assert_eq!(a.start, 10);
        assert_eq!(a.end, 14);
    }
}
