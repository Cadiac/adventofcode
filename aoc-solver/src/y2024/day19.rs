use std::collections::HashMap;

use crate::solution::{AocError, Solution};

fn parse(input: &str) -> Result<(Vec<String>, Vec<String>), AocError> {
    let (towels, designs) = input
        .split_once("\n\n")
        .ok_or_else(|| AocError::parse(input, "Missing towel and designs sections"))?;

    let towels: Vec<String> = towels.split(", ").map(|towel| towel.to_owned()).collect();
    let designs: Vec<String> = designs.lines().map(|towel| towel.to_owned()).collect();

    Ok((towels, designs))
}

fn count_possible(towels: &[String], design: &str, cache: &mut HashMap<String, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(&count) = cache.get(design) {
        return count;
    }

    let possible = towels
        .iter()
        .filter_map(|towel| design.strip_suffix(towel))
        .map(|remaining| count_possible(towels, remaining, cache))
        .sum();

    cache.insert(design.to_owned(), possible);

    possible
}

pub struct Day19;
impl Solution for Day19 {
    type A = usize;
    type B = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day19.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let (towels, designs) = parse(input)?;

        let mut cache = HashMap::new();
        let possible = designs
            .iter()
            .filter(|design| count_possible(&towels, design, &mut cache) > 0)
            .count();

        Ok(possible)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let (towels, designs) = parse(input)?;

        let mut cache = HashMap::new();
        let possible = designs
            .iter()
            .map(|design| count_possible(&towels, design, &mut cache))
            .sum();

        Ok(possible)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day19.part_1(
                "r, wr, b, g, bwu, rb, gb, br\n\
                 \n\
                 brwrr\n\
                 bggr\n\
                 gbbr\n\
                 rrbgbr\n\
                 ubwu\n\
                 bwurrg\n\
                 brgr\n\
                 bbrgwb"
            ),
            Ok(6)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day19.part_2(
                "r, wr, b, g, bwu, rb, gb, br\n\
                 \n\
                 brwrr\n\
                 bggr\n\
                 gbbr\n\
                 rrbgbr\n\
                 ubwu\n\
                 bwurrg\n\
                 brgr\n\
                 bbrgwb"
            ),
            Ok(16)
        );
    }
}
