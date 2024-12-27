use std::collections::HashMap;

use crate::solution::{AocError, Solution};

type Cache = HashMap<u64, HashMap<usize, usize>>;

fn parse(input: &str) -> Result<Vec<u64>, AocError> {
    input
        .split_ascii_whitespace()
        .map(|number| {
            number
                .parse::<u64>()
                .map_err(|err| AocError::parse(number, err))
        })
        .collect()
}

fn split(stone: u64) -> Option<(u64, u64)> {
    let digits = stone.ilog10() + 1;
    if digits % 2 != 0 {
        return None;
    }

    let divisor = 10u64.pow(digits / 2);
    let left = stone / divisor;
    let right = stone % divisor;

    Some((left, right))
}

fn simulate(stone: u64, steps: usize, cache: &mut Cache) -> usize {
    if steps == 0 {
        return 1;
    }

    if let Some(&count) = cache.get(&stone).and_then(|counts| counts.get(&steps)) {
        return count;
    }

    let count = if stone == 0 {
        simulate(1, steps - 1, cache)
    } else if let Some((left, right)) = split(stone) {
        simulate(left, steps - 1, cache) + simulate(right, steps - 1, cache)
    } else {
        simulate(stone * 2024, steps - 1, cache)
    };

    cache.entry(stone).or_default().insert(steps, count);

    count
}

pub struct Day11;
impl Solution for Day11 {
    type Part1 = usize;
    type Part2 = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day11.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let stones = parse(input)?;
        let mut cache = HashMap::new();

        let count = stones
            .into_iter()
            .map(|stone| simulate(stone, 25, &mut cache))
            .sum();

        Ok(count)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let stones = parse(input)?;
        let mut cache = HashMap::new();

        let count = stones
            .into_iter()
            .map(|stone| simulate(stone, 75, &mut cache))
            .sum();

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example_1() {
        assert_eq!(Day11.part_1("125 17"), Ok(55312));
    }
}
