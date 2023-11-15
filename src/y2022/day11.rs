use std::{
    collections::{HashMap, VecDeque},
    num::ParseIntError,
};

use crate::solution::{AocError, Solution};

pub struct Day11;

#[derive(Default)]
enum Argument {
    Number(u32),
    #[default]
    Old,
}

struct Monkey {
    items: VecDeque<u32>,
    remainders: VecDeque<HashMap<u32, u32>>,
    operation: Box<dyn Fn(u32, u32) -> u32>,
    argument: Argument,
    modulus: u32,
    next: (usize, usize),
    activity: u64,
}

impl Default for Monkey {
    fn default() -> Self {
        Monkey {
            items: VecDeque::default(),
            remainders: VecDeque::default(),
            operation: Box::new(|x, y| x + y),
            argument: Argument::default(),
            modulus: 0,
            next: (0, 0),
            activity: 0,
        }
    }
}

enum Parser {
    Id,
    StartingItems,
    Operation,
    TestDivisible,
    TestTrue,
    TestFalse,
    End,
}

impl Day11 {
    fn parse(input: &str) -> Result<Vec<Monkey>, AocError> {
        let mut state = Parser::Id;

        let mut monkeys: Vec<Monkey> = Vec::new();
        let mut current = Monkey::default();

        for line in input.lines() {
            match state {
                Parser::Id => state = Parser::StartingItems,

                Parser::StartingItems => {
                    current.items = line
                        .strip_prefix("  Starting items: ")
                        .ok_or_else(|| AocError::parse(line, "prefix"))?
                        .split(", ")
                        .map(|item| item.parse::<u32>())
                        .collect::<Result<VecDeque<u32>, ParseIntError>>()
                        .map_err(|err| AocError::parse(line, err))?;

                    state = Parser::Operation;
                }

                Parser::Operation => {
                    let (operator, argument): (char, &str) =
                        serde_scan::scan!("  Operation: new = old {} {}" <- line)
                            .map_err(|err| AocError::parse(line, err))?;

                    let operation: Box<dyn Fn(u32, u32) -> u32> = if operator == '*' {
                        Box::new(|a, b| -> u32 { a * b })
                    } else {
                        Box::new(|a, b| -> u32 { a + b })
                    };

                    let argument = match argument {
                        "old" => Argument::Old,
                        number => Argument::Number(
                            number.parse().map_err(|err| AocError::parse(number, err))?,
                        ),
                    };

                    current.operation = operation;
                    current.argument = argument;

                    state = Parser::TestDivisible;
                }

                Parser::TestDivisible => {
                    current.modulus = serde_scan::scan!("  Test: divisible by {}" <- line)
                        .map_err(|err| AocError::parse(line, err))?;

                    state = Parser::TestTrue;
                }

                Parser::TestTrue => {
                    current.next.1 = serde_scan::scan!("    If true: throw to monkey {}" <- line)
                        .map_err(|err| AocError::parse(line, err))?;

                    state = Parser::TestFalse;
                }

                Parser::TestFalse => {
                    current.next.0 = serde_scan::scan!("    If false: throw to monkey {}" <- line)
                        .map_err(|err| AocError::parse(line, err))?;

                    monkeys.push(current);
                    current = Monkey::default();

                    state = Parser::End;
                }

                Parser::End => {
                    state = Parser::Id;
                }
            }
        }

        Ok(monkeys)
    }
}

impl Solution for Day11 {
    type F = u64;
    type S = u64;

    fn meta(&self) -> (u32, u32) {
        (12, 2022)
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/2022/day11.txt")
    }

    fn part_1(&self, input: &str) -> Result<u64, AocError> {
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

                    let target = if worry_level % monkeys[i].modulus == 0 {
                        monkeys[i].next.1
                    } else {
                        monkeys[i].next.0
                    };

                    monkeys[target].items.push_back(worry_level);
                }
            }
        }

        let mut activity: Vec<u64> = monkeys.iter().map(|monkey| monkey.activity).collect();
        activity.sort();

        Ok(activity.iter().rev().take(2).product())
    }

    fn part_2(&self, input: &str) -> Result<u64, AocError> {
        let mut monkeys = Self::parse(input)?;

        // Gather list of modulus for each monkey
        let moduli: Vec<u32> = monkeys.iter().map(|monkey| monkey.modulus).collect();

        // Populate the initial item remainders for every divider
        for monkey in monkeys.iter_mut() {
            for item in monkey.items.iter() {
                let mut item_remainders = HashMap::new();
                for modulus in moduli.iter() {
                    item_remainders.insert(*modulus, item % modulus);
                }
                monkey.remainders.push_back(item_remainders);
            }
        }

        for _round in 1..=10000 {
            for i in 0..monkeys.len() {
                while let Some(mut remainders) = monkeys[i].remainders.pop_front() {
                    monkeys[i].activity += 1;

                    // Recalculate the item's remainders for each monkey's modulus
                    for (modulus, remainder) in remainders.iter_mut() {
                        let argument = match monkeys[i].argument {
                            Argument::Number(num) => num,
                            Argument::Old => *remainder,
                        };

                        *remainder = (monkeys[i].operation)(*remainder, argument) % modulus;
                    }

                    let remainder = *remainders
                        .get(&monkeys[i].modulus)
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

        let mut activity: Vec<u64> = monkeys.iter().map(|monkey| monkey.activity).collect();
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
