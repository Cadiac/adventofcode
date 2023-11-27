use crate::solution::{AocError, Solution};

pub struct Day06;

fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|initial_timer| initial_timer.parse::<usize>().unwrap())
        .collect()
}

// Each fish produces offspring independent of other fish, so each fish
// with a certain initial timer end up producing the same number of total fish.
// Similarly, at any given day each fish with equal current timers will end up
// producing the same amount of total fish, so only the counts of fish with
// each different timer need to be considered.
fn solve(input: &str, days: usize) -> u64 {
    let mut fish_generations: [u64; 9] = [0; 9];

    for initial_timer in parse(input).into_iter() {
        fish_generations[initial_timer] += 1;
    }

    for _day in 0..days {
        let mut next_fish_generations = [0; 9];

        for timer in 0..fish_generations.len() {
            let count = fish_generations[timer];

            if timer == 0 {
                // The fish are ready to produce offspring -
                // after this day each fish with this internal timer resets to 6,
                // and they each create a new lanternfish with an internal timer of 8.
                next_fish_generations[6] += count;
                next_fish_generations[8] += count;
            } else {
                // Normally their internal timer just decreases by one
                next_fish_generations[timer - 1] += count;
            }
        }

        fish_generations = next_fish_generations;
    }

    fish_generations.iter().sum()
}

impl Solution for Day06 {
    type F = u64;
    type S = u64;

    fn meta(&self) -> (u32, u32) {
        (6, 2021)
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2021/day06.txt")
    }

    fn part_1(&self, input: &str) -> Result<Self::F, AocError> {
        Ok(solve(input, 80))
    }

    fn part_2(&self, input: &str) -> Result<Self::S, AocError> {
        Ok(solve(input, 256))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_examples() {
        assert_eq!(solve("3,4,3,1,2", 18), 26);
        assert_eq!(solve("3,4,3,1,2", 80), 5934);
        assert_eq!(Day06.part_1("3,4,3,1,2"), Ok(5934));
    }

    #[test]
    fn it_solves_part2_examples() {
        assert_eq!(Day06.part_2("3,4,3,1,2"), Ok(26984457539));
    }
}
