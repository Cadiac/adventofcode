use crate::solution::{AocError, Solution};

pub struct Day03;

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|binary_str| {
            binary_str
                .chars()
                .map(|bit| bit.to_digit(2).unwrap())
                .collect()
        })
        .collect()
}

fn candidate_to_decimal(
    candidates: Vec<Vec<u32>>,
) -> std::result::Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(
        candidates[0]
            .iter()
            .map(|bit| char::from_digit(*bit, 2).unwrap())
            .collect::<String>()
            .as_str(),
        2,
    )
}

// `expected_value` is the value a candidate must have to be kept when the most common bit value is `1`.
// For finding the most common value this is `1`, and for finding the least common value this is `0`.
fn filter_by_bit_criteria(mut candidates: Vec<Vec<u32>>, expected_value: u32) -> Vec<Vec<u32>> {
    let mut bit_index = 0;

    while candidates.len() > 1 {
        let one_bits_at_index: u32 = candidates.iter().map(|bits| bits[bit_index]).sum();
        let total_bits_at_index = candidates.len() as u32;

        candidates = candidates
            .into_iter()
            .filter(|bits| {
                if total_bits_at_index - one_bits_at_index <= one_bits_at_index {
                    // `1` is most common -> `0` is least common
                    bits[bit_index] == expected_value
                } else {
                    // `0` is most common -> `1` is least common
                    bits[bit_index] != expected_value
                }
            })
            .collect();

        bit_index += 1;
    }

    candidates
}

impl Solution for Day03 {
    type F = u32;
    type S = u32;

    fn meta(&self) -> (u32, u32) {
        (3, 2021)
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2021/day03.txt")
    }

    fn part_1(&self, input: &str) -> Result<Self::F, AocError> {
        let bits_length = input.lines().next().unwrap_or("").len();
        let numbers = parse(input);

        let mut one_bits_per_index: Vec<u32> = vec![0; bits_length];

        for index in 0..bits_length {
            one_bits_per_index[index] = numbers.iter().map(|bits| bits[index]).sum();
        }

        let most_common_bits: String = one_bits_per_index
            .iter()
            .map(|one_bits| {
                if numbers.len() as u32 - one_bits <= *one_bits {
                    '1'
                } else {
                    '0'
                }
            })
            .collect();

        // Max decimal value this amount of '1' bits ('111111111') can represent
        let max_decimal_value = u32::from_str_radix(
            (vec!['1'; bits_length]).iter().collect::<String>().as_str(),
            2,
        )
        .unwrap();

        let decimal_most_common = u32::from_str_radix(most_common_bits.as_str(), 2).unwrap();
        let decimal_least_common = max_decimal_value - decimal_most_common;

        Ok(decimal_most_common * decimal_least_common)
    }

    fn part_2(&self, input: &str) -> Result<Self::S, AocError> {
        let numbers = parse(input);

        let ogr_candidates = filter_by_bit_criteria(numbers.clone(), 1);
        let co2_scubber_candidates = filter_by_bit_criteria(numbers, 0);

        let ogr_decimal = candidate_to_decimal(ogr_candidates).unwrap();
        let co2_scrubber_decimal = candidate_to_decimal(co2_scubber_candidates).unwrap();

        Ok(ogr_decimal * co2_scrubber_decimal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day03.part_1(
                "00100\n\
                11110\n\
                10110\n\
                10111\n\
                10101\n\
                01111\n\
                00111\n\
                11100\n\
                10000\n\
                11001\n\
                00010\n\
                01010"
            ),
            Ok(198)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day03.part_2(
                "00100\n\
                    11110\n\
                    10110\n\
                    10111\n\
                    10101\n\
                    01111\n\
                    00111\n\
                    11100\n\
                    10000\n\
                    11001\n\
                    00010\n\
                    01010"
            ),
            Ok(230)
        );
    }
}
