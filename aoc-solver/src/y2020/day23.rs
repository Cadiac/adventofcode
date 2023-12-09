use std::collections::VecDeque;

use crate::solution::{AocError, Solution};

pub struct Day23;

fn crab_game(mut cups: VecDeque<u32>, moves: u32) -> VecDeque<u32> {
    // Always treat the last cup of the vec as the current cup, and the first elements
    // of the queue as the ones clockwise next to them, looping around.
    cups.rotate_left(1);

    let max_cup = cups.iter().max().cloned().unwrap();
    let min_cup = cups.iter().min().cloned().unwrap();

    for _mov in 0..moves {
        // The crab picks up the three cups that are immediately clockwise of the current cup.
        // They are removed from the circle; cup spacing is adjusted as necessary to maintain the circle.
        let cup_1 = cups.pop_front().unwrap();
        let cup_2 = cups.pop_front().unwrap();
        let cup_3 = cups.pop_front().unwrap();

        // The crab selects a destination cup: the cup with a label equal to the current cup's label minus one.
        // If this would select one of the cups that was just picked up, the crab will keep subtracting one
        // until it finds a cup that wasn't just picked up. If at any point in this process the value goes
        // below the lowest value on any cup's label, it wraps around to the highest value on any cup's label instead.
        let mut dest_val = *cups.back().unwrap();

        let dest_idx = 'outer: loop {
            dest_val = if dest_val <= min_cup {
                max_cup
            } else {
                dest_val - 1
            };

            for (idx, cup) in cups.iter().enumerate() {
                if *cup == dest_val {
                    break 'outer idx;
                }
            }
        };

        // The crab places the cups it just picked up so that they are immediately clockwise of the destination cup.
        // They keep the same order as when they were picked up.
        cups.insert(dest_idx + 1, cup_3);
        cups.insert(dest_idx + 1, cup_2);
        cups.insert(dest_idx + 1, cup_1);

        // The crab selects a new current cup: the cup which is immediately clockwise of the current cup.
        cups.rotate_left(1);
    }

    cups
}

fn play(input: &str, moves: u32) -> Result<String, AocError> {
    let mut cups: VecDeque<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    cups = crab_game(cups, moves);

    while cups.front() != Some(&1) {
        cups.rotate_left(1);
    }

    let result = cups
        .into_iter()
        .skip(1)
        .map(|cup| cup.to_string())
        .collect::<Vec<String>>()
        .join("");

    Ok(result)
}

impl Solution for Day23 {
    type A = String;
    type B = u64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2020/day23.txt")
    }

    fn part_1(&self, input: &str) -> Result<String, AocError> {
        play(input, 100)
    }

    // With a release build this bruteforces the solution in roughly 1,5 hours on my laptop.
    // Perhaps not the solution this challenge wanted, but why not...
    fn part_2(&self, input: &str) -> Result<u64, AocError> {
        let moves = 10000000;

        let mut cups: VecDeque<u32> = input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        let max_cup = cups.iter().max().cloned().unwrap();
        for cup in (max_cup + 1)..=1000000 {
            cups.push_back(cup);
        }

        cups = crab_game(cups, moves);

        while cups.front() != Some(&1) {
            cups.rotate_left(1);
        }

        let mut iter = cups.into_iter().skip(1);
        let result = (iter.next().unwrap() as u64) * (iter.next().unwrap() as u64);

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example_10() {
        assert_eq!(play("389125467", 10), Ok(String::from("92658374")));
    }

    #[test]
    fn it_solves_part1_example_100() {
        assert_eq!(play("389125467", 100), Ok(String::from("67384529")));
    }

    #[ignore] // Takes 1,5 hours :D
    #[test]
    fn it_solves_part2_example_10000000() {
        assert_eq!(Day23.part_2("389125467"), Ok(149245887792));
    }
}
