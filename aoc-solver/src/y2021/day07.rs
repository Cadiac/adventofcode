use crate::solution::{AocError, Solution};

pub struct Day07;

fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|coord| coord.parse::<usize>().unwrap())
        .collect()
}

impl Solution for Day07 {
    type F = usize;
    type S = usize;

    fn meta(&self) -> (u32, u32) {
        (7, 2021)
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2021/day07.txt")
    }

    fn part_1(&self, input: &str) -> Result<Self::F, AocError> {
        let depths = parse(input);

        let max_depth = *depths.iter().max().unwrap_or(&0);

        let mut costs = vec![0; max_depth + 1];

        for initial in depths {
            for (target, cost) in costs.iter_mut().enumerate().take(max_depth + 1) {
                *cost += if target > initial {
                    target - initial
                } else {
                    initial - target
                };
            }
        }

        costs.into_iter().min().ok_or(AocError::logic("No minimum"))
    }

    fn part_2(&self, input: &str) -> Result<Self::S, AocError> {
        let depths = parse(input);

        let max_depth = *depths.iter().max().unwrap_or(&0);

        let mut costs = vec![0; max_depth + 1];

        for initial in depths {
            for (target, cost) in costs.iter_mut().enumerate().take(max_depth + 1) {
                let n = if target > initial {
                    target - initial
                } else {
                    initial - target
                };
                // https://en.wikipedia.org/wiki/Triangular_number
                *cost += n * (n + 1) / 2;
            }
        }

        costs.into_iter().min().ok_or(AocError::logic("No minimum"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_examples() {
        assert_eq!(Day07.part_1("16,1,2,0,4,2,7,1,2,14"), Ok(37));
    }

    #[test]
    fn it_solves_part2_examples() {
        assert_eq!(Day07.part_2("16,1,2,0,4,2,7,1,2,14"), Ok(168));
    }
}
