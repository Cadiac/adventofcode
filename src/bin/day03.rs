const INPUT_FILE: &str = include_str!("../../inputs/day03.txt");

fn part_1(input: &str) -> u32 {
    let mut lines = input.lines().peekable();
    let bits_length = lines.peek().unwrap_or(&"").len();

    let all_numbers: Vec<Vec<u32>> = input
        .lines()
        .map(|binary_str| {
            binary_str
                .chars()
                .map(|bit| bit.to_digit(2).unwrap())
                .collect()
        })
        .collect();

    let mut one_bits_per_index: Vec<u32> = vec![0; bits_length];

    for index in 0..bits_length {
        one_bits_per_index[index] = all_numbers.iter().map(|bits| bits[index]).sum();
    }

    let most_common_bits: String = one_bits_per_index
        .iter()
        .map(|one_bits| if all_numbers.len() as u32 - one_bits <= *one_bits { '1' } else { '0' })
        .collect();

    let decimal_most_common = u32::from_str_radix(most_common_bits.as_str(), 2).unwrap();
    let max_decimal_value = u32::from_str_radix(
        (vec!['1'; bits_length]).iter().collect::<String>().as_str(),
        2,
    ).unwrap();

    let decimal_least_common = max_decimal_value - decimal_most_common;

    decimal_most_common * decimal_least_common
}

fn candidate_to_decimal(candidates: Vec<Vec<u32>>) -> std::result::Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(
        candidates[0]
            .iter()
            .map(|bit| char::from_digit(*bit, 2).unwrap())
            .collect::<String>()
            .as_str(),
        2,
    )
}

fn part_2(input: &str) -> u32 {
    let all_numbers: Vec<Vec<u32>> = input
        .lines()
        .map(|binary_str| {
            binary_str
                .chars()
                .map(|bit| bit.to_digit(2).unwrap())
                .collect()
        })
        .collect();

    let mut ogr_candidates = all_numbers.clone();
    let mut ogr_bit_index = 0;

    while ogr_candidates.len() > 1 {
        let one_bits_at_index: u32 = ogr_candidates.iter().map(|bits| bits[ogr_bit_index]).sum();

        // 1 is most common
        if ogr_candidates.len() as u32 - one_bits_at_index <= one_bits_at_index {
            ogr_candidates = ogr_candidates
                .into_iter()
                .filter(|bits| bits[ogr_bit_index] == 1)
                .collect();
        // 0 is most common
        } else {
            ogr_candidates = ogr_candidates
                .into_iter()
                .filter(|bits| bits[ogr_bit_index] == 0)
                .collect();
        }

        ogr_bit_index += 1;
    }

    let mut co2_scubber_candidates = all_numbers;
    let mut c02_bit_index = 0;

    while co2_scubber_candidates.len() > 1 {
        let one_bits_at_index: u32 = co2_scubber_candidates
            .iter()
            .map(|bits| bits[c02_bit_index])
            .sum();

        // 1 is least common
        if co2_scubber_candidates.len() as u32 - one_bits_at_index > one_bits_at_index {
            co2_scubber_candidates = co2_scubber_candidates
                .into_iter()
                .filter(|bits| bits[c02_bit_index] == 1)
                .collect();
        // 0 is least common
        } else {
            co2_scubber_candidates = co2_scubber_candidates
                .into_iter()
                .filter(|bits| bits[c02_bit_index] == 0)
                .collect();
        }

        c02_bit_index += 1;
    }

    let ogr_decimal = candidate_to_decimal(ogr_candidates).unwrap();
    let co2_scrubber_decimal = candidate_to_decimal(co2_scubber_candidates).unwrap();

    ogr_decimal * co2_scrubber_decimal
}

fn main() {
    let part_1_result = part_1(INPUT_FILE);
    println!("[INFO]: Part 1: {:?}", part_1_result);

    let part_2_result = part_2(INPUT_FILE);
    println!("[INFO]: Part 2: {:?}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            part_1(
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
            198
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            part_2(
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
            230
        );
    }
}
