use std::collections::HashMap;

const INPUT_LOWER_BOUND: u32 = 271973;
const INPUT_UPPER_BOUND: u32 = 785961;

fn is_valid_part1(combination: u32) -> bool {
    let mut has_pair = false;

    let combination_vec: Vec<u32> = combination.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    
    if combination_vec[0] == combination_vec[1] { has_pair = true; }
    if combination_vec[0] > combination_vec[1] { return false; }

    if combination_vec[1] == combination_vec[2] { has_pair = true; }
    if combination_vec[1] > combination_vec[2] { return false; }

    if combination_vec[2] == combination_vec[3] { has_pair = true; }
    if combination_vec[2] > combination_vec[3] { return false; }

    if combination_vec[3] == combination_vec[4] { has_pair = true; }
    if combination_vec[3] > combination_vec[4] { return false; }

    if combination_vec[4] == combination_vec[5] { has_pair = true; }
    if combination_vec[4] > combination_vec[5] { return false; }

    return has_pair;
}

fn is_valid_part2(combination: u32) -> bool {
    let mut pair_hits: HashMap<u32, u32> = HashMap::new();

    let digits: Vec<u32> = combination.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    if digits[0] > digits[1] { return false; }
    if digits[1] > digits[2] { return false; }
    if digits[2] > digits[3] { return false; }
    if digits[3] > digits[4] { return false; }
    if digits[4] > digits[5] { return false; }

    // println!("{:?} is in order", digits);

    if digits[0] == digits[1] {
        *pair_hits.entry(digits[0]).or_insert(0) += 1;
    }

    if digits[1] == digits[2] {
        *pair_hits.entry(digits[1]).or_insert(0) += 1;
    }

    if digits[2] == digits[3] {
        *pair_hits.entry(digits[2]).or_insert(0) += 1;
    }

    if digits[3] == digits[4] {
        *pair_hits.entry(digits[3]).or_insert(0) += 1;
    }

    if digits[4] == digits[5] {
        *pair_hits.entry(digits[4]).or_insert(0) += 1;
    }

    // println!("{:?} pair hits", pair_hits);

    for hit in pair_hits.values() {
        if *hit == 1u32 {
            return true;
        }
    }

    return false;
}

fn part_1(lower_bound: u32, upper_bound: u32) -> u32 {
    let mut legit_combinations = 0;
    for combination in lower_bound..upper_bound {
        if is_valid_part1(combination) { legit_combinations += 1; }
    }

    return legit_combinations;
}

fn part_2(lower_bound: u32, upper_bound: u32) -> u32 {
    let mut legit_combinations = 0;
    for combination in lower_bound..upper_bound {
        if is_valid_part2(combination) { legit_combinations += 1; }
    }

    return legit_combinations;
}

fn main() -> () {
    let part1 = part_1(INPUT_LOWER_BOUND, INPUT_UPPER_BOUND);
    let part2 = part_2(INPUT_LOWER_BOUND, INPUT_UPPER_BOUND);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_examples() {
        assert_eq!(is_valid_part1(111123), true);
        assert_eq!(is_valid_part1(111111), true);
        assert_eq!(is_valid_part1(223450), false);
        assert_eq!(is_valid_part1(123789), false);
    }

    #[test]
    fn it_solves_part2_examples() {
        assert_eq!(is_valid_part2(112233), true);
        assert_eq!(is_valid_part2(123444), false);
        assert_eq!(is_valid_part2(111122), true);
        assert_eq!(is_valid_part2(123444), false);
        assert_eq!(is_valid_part2(443444), false);
        assert_eq!(is_valid_part2(445558), true);
        assert_eq!(is_valid_part2(567788), true);
    }
}
