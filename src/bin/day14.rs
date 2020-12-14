extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

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

            println!("[DEBUG]: value:   {:#038b} (decimal {:?})", value, value);
            println!("[DEBUG]: mask:    {:#038b} (decimal {:?})", mask, mask);
            println!("[DEBUG]: ignored: {:#038b} (decimal {:?})", ignored_bits, ignored_bits);

            // Set the bits mask is setting to zero, then OR
            let masked_value = (value & ignored_bits) | mask;

            println!("[DEBUG]: result:  {:#038b} (decimal {:?})", masked_value, masked_value);

            mem.insert(addr, masked_value);
        }
    }

    return mem.values().sum();
}

fn part_2(input: &str) -> u64 {
    let mut base_mask: u64 = 0x000000000;
    let mut included_bits: u64 = 0xfffffffff;

    let mut x_indices: Vec<usize> = Vec::new();

    let mut mem: HashMap<u64, u64> = HashMap::new();

    let mem_regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    for line in input.lines() {
        if line.starts_with("mask") {
            let raw_mask = line.split(" = ").nth(1).unwrap();

            let included_bits_str: String = raw_mask
                .chars()
                .map(|c| match c {
                    'X' => '0',
                    _bit => '1',
                })
                .collect();

            included_bits = u64::from_str_radix(&included_bits_str, 2).unwrap();

            x_indices = line
                .chars()
                .rev()
                .enumerate()
                .filter(|(_i, c)| c == &'X')
                .map(|(i, _c)| i)
                .collect();

            let mask_str: String = raw_mask
                .chars()
                .map(|c| match c {
                    'X' => '0',
                    bit => bit,
                })
                .collect();
            base_mask = u64::from_str_radix(&mask_str, 2).unwrap();
        } else if line.starts_with("mem") {
            let capture = mem_regex.captures(line).unwrap();
            let addr = capture[1].parse::<u64>().unwrap();
            let value = capture[2].parse::<u64>().unwrap();

            // With current masks x indices we can find 2^ their number combinations
            let x_combinations = 2usize.pow(x_indices.len() as u32);

            // Initialize a vector of masks. These will be modified to be the different combinations
            let mut masked_addrs: Vec<u64> =
                // This time we want to keep bits that weren't set to 'X' from the original address
                vec![(addr & included_bits) | base_mask; x_combinations];

            // Treat every combination as a number from 0..n.
            // This is to make iterating 'X' binary combinations easier, we can just iterate
            // decimal numbers, treat each bit as combination for each X and set the values
            // in each combinations respective masked_addr.
            for combination in 0..x_combinations {
                // For each bit we're setting set the bits of matching masked_addr with
                // the bits of this combination to correct indices.
                let comb_u64 = combination as u64;
                for (i, x_index) in x_indices.iter().enumerate() {
                    let i_u64 = i as u64;
                    let bit = (comb_u64 >> i_u64) & 1;
                    masked_addrs[combination] |= bit << x_index;
                }
            }

            println!(
                "[DEBUG]: decoded addr to {:?} addresses",
                x_combinations
            );

            println!("[DEBUG]: addr:    {:#038b} (decimal {:?})", addr, addr);
            for masked_addr in masked_addrs.iter() {
                assert_eq!(masked_addr < &2u64.pow(36), true);
                println!(
                    "[DEBUG]: decoded: {:#038b} (decimal {:?})",
                    masked_addr, masked_addr
                );
                mem.insert(*masked_addr, value);
            }

            let check_set: HashSet<&u64> = HashSet::from_iter(masked_addrs.iter());
            assert_eq!(check_set.len(), masked_addrs.len());
        }
    }

    return mem.values().sum();
}

fn main() -> () {
    let part_1_result = part_1(INPUT_FILE);
    let part_2_result = part_2(INPUT_FILE);

    println!("[INFO]: Part 1: {:?}", part_1_result);
    println!("[INFO]: Part 2: {:?}", part_2_result);
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

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            part_2(
                "mask = 000000000000000000000000000000X1001X\n\
                 mem[42] = 100\n\
                 mask = 00000000000000000000000000000000X0XX\n\
                 mem[26] = 1"
            ),
            208
        )
    }
}
