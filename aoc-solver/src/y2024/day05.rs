use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;

use crate::solution::{AocError, Solution};

type Rules = HashMap<(u32, u32), Ordering>;
type Updates = Vec<Vec<u32>>;

fn parse(input: &str) -> Result<(Rules, Updates), AocError> {
    let (rules_input, updates_input) = input
        .split_once("\n\n")
        .ok_or_else(|| AocError::parse(input, "Missing rules or updates sections!"))?;

    let rules: Vec<_> = rules_input
        .trim()
        .lines()
        .map(|line| {
            let (before, after) = line
                .split_once('|')
                .ok_or_else(|| AocError::parse(line, "Invalid rule"))?;

            let before = before
                .parse::<u32>()
                .map_err(|err| AocError::parse(before, err))?;
            let after = after
                .parse::<u32>()
                .map_err(|err| AocError::parse(after, err))?;

            Ok([
                ((before, after), Ordering::Less),
                ((after, before), Ordering::Greater),
            ])
        })
        .try_collect()?;

    let rules: Rules = rules.into_iter().flatten().collect();

    let updates = updates_input
        .trim()
        .lines()
        .map(|line| {
            let numbers: Vec<u32> = line
                .split(',')
                .map(|number| {
                    number
                        .parse::<u32>()
                        .map_err(|err| AocError::parse(number, err))
                })
                .try_collect()?;

            Ok(numbers)
        })
        .try_collect()?;

    Ok((rules, updates))
}

pub struct Day05;
impl Solution for Day05 {
    type Part1 = u32;
    type Part2 = u32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day05.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let (rules, updates) = parse(input)?;

        let correct_middle_sum = updates
            .into_iter()
            .filter(|update| {
                update.iter().enumerate().all(|(i, current)| {
                    let is_correct_before = update[..i]
                        .iter()
                        .all(|check| rules.get(&(*current, *check)) == Some(&Ordering::Greater));

                    let is_correct_after = update[i + 1..]
                        .iter()
                        .all(|check| rules.get(&(*current, *check)) == Some(&Ordering::Less));

                    is_correct_before && is_correct_after
                })
            })
            .map(|update| update[update.len() / 2])
            .sum();

        Ok(correct_middle_sum)
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        let (rules, updates) = parse(input)?;

        let incorrect_middle_sum = updates
            .into_iter()
            .filter(|update| {
                update.iter().enumerate().any(|(i, current)| {
                    let is_incorrect_before = update[..i]
                        .iter()
                        .any(|check| rules.get(&(*current, *check)) != Some(&Ordering::Greater));

                    let is_incorrect_after = update[i + 1..]
                        .iter()
                        .any(|check| rules.get(&(*current, *check)) != Some(&Ordering::Less));

                    is_incorrect_before || is_incorrect_after
                })
            })
            .map(|mut incorrect| {
                incorrect.sort_by(|a, b| rules[&(*a, *b)]);
                incorrect[incorrect.len() / 2]
            })
            .sum();

        Ok(incorrect_middle_sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day05.part_1(
                "47|53\n\
                 97|13\n\
                 97|61\n\
                 97|47\n\
                 75|29\n\
                 61|13\n\
                 75|53\n\
                 29|13\n\
                 97|29\n\
                 53|29\n\
                 61|53\n\
                 97|53\n\
                 61|29\n\
                 47|13\n\
                 75|47\n\
                 97|75\n\
                 47|61\n\
                 75|61\n\
                 47|29\n\
                 75|13\n\
                 53|13\n\
                 \n\
                 75,47,61,53,29\n\
                 97,61,53,29,13\n\
                 75,29,13\n\
                 75,97,47,61,53\n\
                 61,13,29\n\
                 97,13,75,29,47"
            ),
            Ok(143)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day05.part_2(
                "47|53\n\
                 97|13\n\
                 97|61\n\
                 97|47\n\
                 75|29\n\
                 61|13\n\
                 75|53\n\
                 29|13\n\
                 97|29\n\
                 53|29\n\
                 61|53\n\
                 97|53\n\
                 61|29\n\
                 47|13\n\
                 75|47\n\
                 97|75\n\
                 47|61\n\
                 75|61\n\
                 47|29\n\
                 75|13\n\
                 53|13\n\
                 \n\
                 75,47,61,53,29\n\
                 97,61,53,29,13\n\
                 75,29,13\n\
                 75,97,47,61,53\n\
                 61,13,29\n\
                 97,13,75,29,47"
            ),
            Ok(123)
        );
    }
}
