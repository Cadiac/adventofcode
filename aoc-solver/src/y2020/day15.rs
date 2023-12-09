use std::collections::HashMap;

use crate::solution::{AocError, Solution};

pub struct Day15;

fn memory_game(input: &str, turns: u32) -> u32 {
    let mut mem: HashMap<u32, u32> = HashMap::new();

    let mut turn = 1;
    let mut next_spoken = 0;

    // The initial words never seem to repeat
    for i in input.trim().split(',') {
        let num = i.parse::<u32>().unwrap();
        mem.insert(num, turn);
        turn += 1;
    }

    while turn < turns {
        if let Some(last_spoken_at) = mem.insert(next_spoken, turn) {
            next_spoken = turn - last_spoken_at;
        } else {
            next_spoken = 0;
        }

        turn += 1;
    }

    next_spoken
}

impl Solution for Day15 {
    type A = u32;
    type B = u32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2020/day15.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let result = memory_game(input, 2020);

        Ok(result)
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        let result = memory_game(input, 30000000);

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_examples() {
        assert_eq!(memory_game("0,3,6", 2020), 436);
        assert_eq!(memory_game("1,3,2", 2020), 1);
        assert_eq!(memory_game("2,1,3", 2020), 10);
        assert_eq!(memory_game("1,2,3", 2020), 27);
        assert_eq!(memory_game("2,3,1", 2020), 78);
        assert_eq!(memory_game("3,2,1", 2020), 438);
        assert_eq!(memory_game("3,1,2", 2020), 1836);
    }

    #[test]
    #[ignore]
    fn it_solves_part2_examples() {
        assert_eq!(memory_game("0,3,6", 30000000), 175594);
        assert_eq!(memory_game("1,3,2", 30000000), 2578);
        assert_eq!(memory_game("2,1,3", 30000000), 3544142);
        assert_eq!(memory_game("1,2,3", 30000000), 261214);
        assert_eq!(memory_game("2,3,1", 30000000), 6895259);
        assert_eq!(memory_game("3,2,1", 30000000), 18);
        assert_eq!(memory_game("3,1,2", 30000000), 362);
    }
}
