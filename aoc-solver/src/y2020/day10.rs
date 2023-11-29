use std::collections::HashMap;
use std::collections::HashSet;

use crate::solution::{AocError, Solution};

pub struct Day10;

#[derive(Debug, Default, Clone)]
pub struct Solver {
    pub cache: HashMap<i64, i64>,
    pub dbja: i64, // Device's built-in joltage adapter
    pub adapters: HashSet<i64>,
    pub possible_next_adapters: HashMap<i64, HashSet<i64>>,
}

impl Solver {
    #[inline]
    fn new(input: &str) -> Solver {
        let mut adapters: HashSet<i64> = input
            .lines()
            .map(|line| line.parse::<i64>().unwrap())
            .collect();

        // Add the charging outlet to the list of adapters to find possible adapters from
        adapters.insert(0);

        // Device's built-in joltage adapter
        let dbja = *adapters.iter().max().unwrap_or(&0);

        // For part two construct this HashMap. In the end this was probably unnecessary,
        // caching the recursion results was all that I really needed.
        let possible_next_adapters: HashMap<i64, HashSet<i64>> = adapters
            .iter()
            .map(|adapter| {
                let mut next_adapters: HashSet<i64> = HashSet::new();
                if adapters.contains(&(adapter + 1)) {
                    next_adapters.insert(adapter + 1);
                }
                if adapters.contains(&(adapter + 2)) {
                    next_adapters.insert(adapter + 2);
                }
                if adapters.contains(&(adapter + 3)) {
                    next_adapters.insert(adapter + 3);
                }
                (*adapter, next_adapters)
            })
            .collect();

        Solver {
            cache: HashMap::new(),
            adapters: adapters,
            dbja: dbja,
            possible_next_adapters: possible_next_adapters,
        }
    }

    #[inline]
    fn part_1(&self) -> i64 {
        let mut jolts = 0;
        let mut differences_of_1 = 0;
        let mut differences_of_3 = 0;

        while jolts < self.dbja {
            if self.adapters.contains(&(jolts + 1)) {
                jolts += 1;
                differences_of_1 += 1;
            } else if self.adapters.contains(&(jolts + 2)) {
                jolts += 2;
            } else if self.adapters.contains(&(jolts + 3)) {
                jolts += 3;
                differences_of_3 += 1;
            } else {
                panic!("[ERROR]: Stuck at {:?}", jolts);
            }
        }

        // the dbja counts as an adapter with +3 jolts
        differences_of_1 * (differences_of_3 + 1)
    }

    #[inline]
    fn find_complete_chains(&mut self, jolts: i64) -> i64 {
        if jolts >= self.dbja {
            self.cache.insert(jolts, 1);
            return 1;
        }

        self.possible_next_adapters
            .get(&jolts)
            .unwrap()
            .clone()
            .iter()
            .map(|adapter| match self.cache.get(&adapter) {
                Some(solution) => *solution,
                None => {
                    let branch_chains = self.find_complete_chains(*adapter);
                    self.cache.insert(*adapter, branch_chains);
                    branch_chains
                }
            })
            .sum()
    }

    fn part_2(&mut self) -> i64 {
        self.find_complete_chains(0)
    }
}

impl Solution for Day10 {
    type F = i64;
    type S = i64;

    fn meta(&self) -> (u32, u32) {
        (10, 2020)
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2020/day10.txt")
    }

    fn part_1(&self, input: &str) -> Result<Self::F, AocError> {
        let solver = Solver::new(input);
        let result = solver.part_1();

        Ok(result)
    }

    fn part_2(&self, input: &str) -> Result<Self::S, AocError> {
        let mut solver = Solver::new(input);
        let result = solver.part_2();

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example_1() {
        assert_eq!(
            Day10.part_1(
                "16\n\
                10\n\
                15\n\
                5\n\
                1\n\
                11\n\
                7\n\
                19\n\
                6\n\
                12\n\
                4",
            ),
            Ok(7 * 5)
        );
    }

    #[test]
    fn it_solves_part1_example_2() {
        assert_eq!(
            Day10.part_1(
                "28\n\
                33\n\
                18\n\
                42\n\
                31\n\
                14\n\
                46\n\
                20\n\
                48\n\
                47\n\
                24\n\
                23\n\
                49\n\
                45\n\
                19\n\
                38\n\
                39\n\
                11\n\
                1\n\
                32\n\
                25\n\
                35\n\
                8\n\
                17\n\
                7\n\
                9\n\
                4\n\
                2\n\
                34\n\
                10\n\
                3",
            ),
            Ok(22 * 10)
        );
    }

    #[test]
    fn it_solves_part2_example_1() {
        assert_eq!(
            Day10.part_2(
                "16\n\
                10\n\
                15\n\
                5\n\
                1\n\
                11\n\
                7\n\
                19\n\
                6\n\
                12\n\
                4",
            ),
            Ok(8)
        );
    }

    #[test]
    fn it_solves_part2_example_2() {
        assert_eq!(
            Day10.part_2(
                "28\n\
                33\n\
                18\n\
                42\n\
                31\n\
                14\n\
                46\n\
                20\n\
                48\n\
                47\n\
                24\n\
                23\n\
                49\n\
                45\n\
                19\n\
                38\n\
                39\n\
                11\n\
                1\n\
                32\n\
                25\n\
                35\n\
                8\n\
                17\n\
                7\n\
                9\n\
                4\n\
                2\n\
                34\n\
                10\n\
                3",
            ),
            Ok(19208)
        );
    }
}
