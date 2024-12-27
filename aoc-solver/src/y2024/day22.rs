use std::collections::HashMap;

use itertools::Itertools;

use crate::solution::{AocError, Solution};

fn parse(input: &str) -> Result<Vec<u64>, AocError> {
    input
        .lines()
        .map(|row| row.parse::<u64>().map_err(|err| AocError::parse(row, err)))
        .try_collect()
}

fn evolve(mut secret: u64) -> u64 {
    // Calculate the result of multiplying the secret number by 64.
    // Then, mix this result into the secret number.
    // Finally, prune the secret number.
    secret ^= secret * 64;
    secret %= 16777216;

    // Calculate the result of dividing the secret number by 32.
    // Then, mix this result into the secret number.
    // Finally, prune the secret number.
    secret ^= secret / 32;
    secret %= 16777216;

    // Calculate the result of multiplying the secret number by 2048.
    // Then, mix this result into the secret number.
    // Finally, prune the secret number.
    secret ^= secret * 2048;
    secret %= 16777216;

    secret
}

pub struct Day22;
impl Solution for Day22 {
    type Part1 = u64;
    type Part2 = u64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day22.txt")
    }

    fn part_1(&self, input: &str) -> Result<u64, AocError> {
        let secrets = parse(input)?;

        let sum = secrets
            .into_iter()
            .map(|initial_secret| (0..2000).fold(initial_secret, |secret, _| evolve(secret)))
            .sum();

        Ok(sum)
    }

    fn part_2(&self, input: &str) -> Result<u64, AocError> {
        let secrets = parse(input)?;

        let every_buyer_sequences = secrets
            .into_iter()
            .map(|mut secret| {
                let mut prices = Vec::new();
                let mut diffs = Vec::new();

                for _ in 1..2000 {
                    let prev_price = secret % 10;
                    secret = evolve(secret);
                    let price = secret % 10;

                    prices.push(price);
                    diffs.push(price as i64 - prev_price as i64);
                }

                let price_by_sequence = diffs.windows(4).enumerate().fold(
                    HashMap::new(),
                    |mut sequences, (i, window)| {
                        sequences.entry(window.to_vec()).or_insert(prices[i + 3]);
                        sequences
                    },
                );

                price_by_sequence
            })
            .collect::<Vec<_>>();

        let mut bananas_by_sequence: HashMap<Vec<i64>, u64> = HashMap::new();

        for sequences in every_buyer_sequences {
            for (sequence, price) in sequences.into_iter() {
                *bananas_by_sequence.entry(sequence).or_insert(0) += price;
            }
        }

        let most_bananas = bananas_by_sequence
            .into_values()
            .max()
            .ok_or_else(|| AocError::logic("No bananas?"))?;

        Ok(most_bananas)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day22.part_1(
                "1\n\
                 10\n\
                 100\n\
                 2024"
            ),
            Ok(37327623)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day22.part_2(
                "1\n\
                 2\n\
                 3\n\
                 2024"
            ),
            Ok(23)
        );
    }

    #[test]
    fn it_evolves_correctly() {
        assert_eq!(evolve(123), 15887950);
    }
}
