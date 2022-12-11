use std::{
    collections::{HashMap, VecDeque},
    num::ParseIntError,
};

use crate::solution::{AocError, Solution};

pub struct Day11;

enum Argument {
    Number(u32),
    Old,
}

struct Monkey {
    items: VecDeque<u32>,
    remainders: VecDeque<HashMap<u32, u32>>,
    operation: Box<dyn Fn(u32, u32) -> u32>,
    argument: Argument,
    modulo: u32,
    next: (usize, usize),
    activity: usize,
}

impl Day11 {
    fn parse(input: &str) -> Result<Vec<Monkey>, AocError> {
        input
            .split("\n\n")
            .map(|chunk| {
                let mut lines = chunk.lines().skip(1);

                let mut line = lines
                    .next()
                    .ok_or_else(|| AocError::parse(chunk, "line 1"))?;
                let items: String = serde_scan::scan!("  Starting items: {}" <- line)
                    .map_err(|err| AocError::parse(line, err))?;
                let items: VecDeque<u32> = items
                    .split(", ")
                    .map(|item| item.parse::<u32>())
                    .collect::<Result<VecDeque<u32>, ParseIntError>>()
                    .map_err(|err| AocError::parse(line, err))?;

                line = lines
                    .next()
                    .ok_or_else(|| AocError::parse(chunk, "line 2"))?;
                let (operator, argument): (char, &str) =
                    serde_scan::scan!("  Operation: new = old {} {}" <- line)
                        .map_err(|err| AocError::parse(line, err))?;

                let argument = match argument {
                    "old" => Argument::Old,
                    number => Argument::Number(
                        number.parse().map_err(|err| AocError::parse(number, err))?,
                    ),
                };

                let operation: Box<dyn Fn(u32, u32) -> u32> = if operator == '*' {
                    Box::new(|a, b| -> u32 { a * b })
                } else {
                    Box::new(|a, b| -> u32 { a + b })
                };

                line = lines
                    .next()
                    .ok_or_else(|| AocError::parse(chunk, "line 3"))?;
                let modulo: u32 = serde_scan::scan!("  Test: divisible by {}" <- line)
                    .map_err(|err| AocError::parse(line, err))?;

                line = lines
                    .next()
                    .ok_or_else(|| AocError::parse(chunk, "line 4"))?;
                let right: usize = serde_scan::scan!("    If true: throw to monkey {}" <- line)
                    .map_err(|err| AocError::parse(line, err))?;

                line = lines
                    .next()
                    .ok_or_else(|| AocError::parse(chunk, "line 5"))?;
                let left: usize = serde_scan::scan!("    If false: throw to monkey {}" <- line)
                    .map_err(|err| AocError::parse(line, err))?;

                Ok(Monkey {
                    items,
                    operation,
                    argument,
                    modulo,
                    next: (left, right),
                    remainders: VecDeque::new(),
                    activity: 0,
                })
            })
            .collect()
    }
}

impl Solution for Day11 {
    type F = usize;
    type S = usize;

    fn name(&self) -> &'static str {
        "Day 11"
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/day11.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let mut monkeys = Self::parse(input)?;

        for _round in 1..=20 {
            for i in 0..monkeys.len() {
                while let Some(item) = monkeys[i].items.pop_front() {
                    monkeys[i].activity += 1;

                    let argument = match monkeys[i].argument {
                        Argument::Number(num) => num,
                        Argument::Old => item,
                    };

                    let worry_level = (monkeys[i].operation)(item, argument) / 3;

                    let target = if worry_level % monkeys[i].modulo == 0 {
                        monkeys[i].next.1
                    } else {
                        monkeys[i].next.0
                    };

                    monkeys[target].items.push_back(worry_level);
                }
            }
        }

        let mut activity: Vec<usize> = monkeys.iter().map(|monkey| monkey.activity).collect();
        activity.sort();

        Ok(activity.iter().rev().take(2).product())
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let mut monkeys = Self::parse(input)?;
        // Gather list of modulos the monkeys are interested in
        let modulos: Vec<u32> = monkeys.iter().map(|monkey| monkey.modulo).collect();

        // Populate the initial item remainders for every divider
        for monkey in monkeys.iter_mut() {
            for item in monkey.items.iter() {
                let mut item_remainders = HashMap::new();
                for modulo in modulos.iter() {
                    item_remainders.insert(*modulo, item % modulo);
                }
                monkey.remainders.push_back(item_remainders);
            }
        }

        for _round in 1..=10000 {
            for i in 0..monkeys.len() {
                while let Some(mut remainders) = monkeys[i].remainders.pop_front() {
                    monkeys[i].activity += 1;

                    for (modulo, remainder) in remainders.iter_mut() {
                        let argument = match monkeys[i].argument {
                            Argument::Number(num) => num,
                            Argument::Old => *remainder,
                        };

                        *remainder = (monkeys[i].operation)(*remainder, argument) % modulo;
                    }

                    let remainder = *remainders
                        .get(&monkeys[i].modulo)
                        .ok_or_else(|| AocError::logic("unknown remainder"))?;

                    let target = if remainder == 0 {
                        monkeys[i].next.1
                    } else {
                        monkeys[i].next.0
                    };

                    monkeys[target].remainders.push_back(remainders);
                }
            }
        }

        let mut activity: Vec<usize> = monkeys.iter().map(|monkey| monkey.activity).collect();
        activity.sort();

        Ok(activity.iter().rev().take(2).product())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1() {
        assert_eq!(
            Day11.part_1(
                &[
                    "Monkey 0:",
                    "  Starting items: 79, 98",
                    "  Operation: new = old * 19",
                    "  Test: divisible by 23",
                    "    If true: throw to monkey 2",
                    "    If false: throw to monkey 3",
                    "",
                    "Monkey 1:",
                    "  Starting items: 54, 65, 75, 74",
                    "  Operation: new = old + 6",
                    "  Test: divisible by 19",
                    "    If true: throw to monkey 2",
                    "    If false: throw to monkey 0",
                    "",
                    "Monkey 2:",
                    "  Starting items: 79, 60, 97",
                    "  Operation: new = old * old",
                    "  Test: divisible by 13",
                    "    If true: throw to monkey 1",
                    "    If false: throw to monkey 3",
                    "",
                    "Monkey 3:",
                    "  Starting items: 74",
                    "  Operation: new = old + 3",
                    "  Test: divisible by 17",
                    "    If true: throw to monkey 0",
                    "    If false: throw to monkey 1"
                ]
                .join("\n")
            ),
            Ok(10605)
        );
    }

    #[test]
    fn it_solves_part2() {
        assert_eq!(
            Day11.part_2(
                &[
                    "Monkey 0:",
                    "  Starting items: 79, 98",
                    "  Operation: new = old * 19",
                    "  Test: divisible by 23",
                    "    If true: throw to monkey 2",
                    "    If false: throw to monkey 3",
                    "",
                    "Monkey 1:",
                    "  Starting items: 54, 65, 75, 74",
                    "  Operation: new = old + 6",
                    "  Test: divisible by 19",
                    "    If true: throw to monkey 2",
                    "    If false: throw to monkey 0",
                    "",
                    "Monkey 2:",
                    "  Starting items: 79, 60, 97",
                    "  Operation: new = old * old",
                    "  Test: divisible by 13",
                    "    If true: throw to monkey 1",
                    "    If false: throw to monkey 3",
                    "",
                    "Monkey 3:",
                    "  Starting items: 74",
                    "  Operation: new = old + 3",
                    "  Test: divisible by 17",
                    "    If true: throw to monkey 0",
                    "    If false: throw to monkey 1"
                ]
                .join("\n")
            ),
            Ok(2713310158)
        );
    }
}
