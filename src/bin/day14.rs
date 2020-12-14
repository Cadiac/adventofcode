extern crate regex;

use regex::Regex;
use std::collections::HashMap;

const INPUT_FILE: &str = include_str!("../../inputs/day14.txt");

fn part_1(input: &str) -> u64 {
    let mut mask: u64 = 0x000000000;
    let mut ignored_bits: u64 = 0xfffffffff;

    let mut mem: HashMap<usize, u64> = HashMap::new();

    let mem_regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    for line in input.lines() {
        if line.starts_with("mask") {
            let value = line.split(" = ").nth(1).unwrap();

            let ignored_bits_str: String = value
                .chars()
                .map(|c| match c {
                    'X' => '1',
                    _bit => '0',
                })
                .collect();

            ignored_bits = u64::from_str_radix(&ignored_bits_str, 2).unwrap();

            let mask_str: String = value
                .chars()
                .map(|c| match c {
                    'X' => '0',
                    bit => bit,
                })
                .collect();
            mask = u64::from_str_radix(&mask_str, 2).unwrap();
        } else if line.starts_with("mem") {
            let capture = mem_regex.captures(line).unwrap();
            let addr = capture[1].parse::<usize>().unwrap();
            let value = capture[2].parse::<u64>().unwrap();

            // println!("[DEBUG]: value:   {:36b} (decimal {:?}", value, value);
            // println!("[DEBUG]: mask:    {:36b} (decimal {:?}", mask, mask);
            // println!("[DEBUG]: ignored: {:36b} (decimal {:?}", ignored_bits, ignored_bits);

            // Set the bits mask is setting to zero, then OR
            let masked_value = (value & ignored_bits) | mask;

            // println!("[DEBUG]: result:  {:36b} (decimal {:?}", masked_value, masked_value);

            mem.insert(addr, masked_value);
        }
    }

    return mem.values().sum();
}

fn main() -> () {
    let part_1_result = part_1(INPUT_FILE);
    println!("[INFO]: Part 1: {:?}", part_1_result);

    // let part_2_result = part_2(INPUT_FILE);
    // println!("[INFO]: Part 2: {:?}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            part_1(
                "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
                 mem[8] = 11\n\
                 mem[7] = 101\n\
                 mem[8] = 0"
            ),
            165
        )
    }
}
