use std::collections::HashMap;

use crate::solution::{AocError, Solution};

pub struct Day14;

fn parse(input: &str) -> (Vec<char>, HashMap<[char; 2], char>) {
    let mut lines = input.lines();
    let polymer_template = lines.next().unwrap().chars().collect();

    // Discard the empty line between template and rules
    lines.next();

    let pair_insertion_rules: HashMap<[char; 2], char> = lines
        .map(|line| {
            let parts: Vec<&str> = line.split(" -> ").collect();
            assert_eq!(parts.len(), 2);

            let mut adjacent_chars = parts[0].chars();
            let inserted_char = parts[1].chars().next().unwrap();

            (
                [
                    adjacent_chars.next().unwrap(),
                    adjacent_chars.next().unwrap(),
                ],
                inserted_char,
            )
        })
        .collect();

    (polymer_template, pair_insertion_rules)
}

fn polymer_counts_by_element(polymer: Vec<char>) -> HashMap<char, usize> {
    let mut counts_by_element: HashMap<char, usize> = HashMap::new();

    for element in polymer {
        *counts_by_element.entry(element).or_insert(0) += 1;
    }

    counts_by_element
}

fn element_counts_after_steps(
    mut polymer: Vec<char>,
    pair_insertion_rules: &HashMap<[char; 2], char>,
    steps: usize,
) -> Vec<char> {
    let last = *polymer.last().unwrap();

    for _step in 0..steps {
        polymer = polymer
            .windows(2)
            .flat_map(|pair| {
                if let Some(element) = pair_insertion_rules.get(&[pair[0], pair[1]]) {
                    return vec![pair[0], *element];
                }
                vec![pair[0]]
            })
            // Add the last element into the polymer, it never changes
            .chain(std::iter::once(last))
            .collect();
    }

    polymer
}

fn solve(input: &str, steps: usize) -> usize {
    // Approach: Consider all rules individually (NN, NC, CB...) and see what they procude after half of the steps.
    // Each pair of elements will always individually produce the same amount of elements over steps as other similar pairs.

    // Then, after constructing the elements after half steps for each rule, simulate the polymer for half steps
    // (which seems to be easily doable at steps = 40 / 2)

    // Take the simulated polymer after half steps, split it into pairs and look up what each pair produces after another
    // half steps.

    // This could perhaps further be improved by doing something similar a few times?

    let (mut polymer_template, pair_insertion_rules) = parse(input);

    let halfway = steps / 2;

    let mut counts_by_element_by_rule: HashMap<[char; 2], HashMap<char, usize>> = HashMap::new();

    for rule in pair_insertion_rules.keys().cloned() {
        let mut polymer = vec![rule[0], rule[1]];
        polymer = element_counts_after_steps(polymer, &pair_insertion_rules, halfway);

        let counts_by_element = polymer_counts_by_element(polymer);
        counts_by_element_by_rule.insert(rule, counts_by_element);
    }

    // Run the original polymer template simulation for half steps
    polymer_template = element_counts_after_steps(polymer_template, &pair_insertion_rules, halfway);

    let mut final_counts_by_element: HashMap<char, usize> = HashMap::new();

    // And then check what counts the 2-length windows produce by rules
    let mut is_first = true;
    polymer_template.windows(2).for_each(|pair| {
        if let Some(counts) = counts_by_element_by_rule.get(&[pair[0], pair[1]]) {
            for (element, count) in counts {
                *final_counts_by_element.entry(*element).or_insert(0) += count;
            }
            if is_first {
                is_first = false;
            } else {
                // If this isn't the first window drop the first element produced, like
                // _N_BBBNC -> only consider the part BBBNC since previous pair already
                // included the N as its last element.
                *final_counts_by_element.entry(pair[0]).or_insert(0) -= 1;
            }
        } else {
            // We should always know a rule for every pair
            unreachable!();
        }
    });

    let most_common_count = final_counts_by_element.values().max().unwrap();
    let least_common_count = final_counts_by_element.values().min().unwrap();

    most_common_count - least_common_count
}

impl Solution for Day14 {
    type F = usize;
    type S = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2021/day14.txt")
    }

    fn part_1(&self, input: &str) -> Result<Self::F, AocError> {
        let result = solve(input, 10);
        Ok(result)
    }

    fn part_2(&self, input: &str) -> Result<Self::S, AocError> {
        let result = solve(input, 40);
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            solve(
                "NNCB\n\
                 \n\
                 CH -> B\n\
                 HH -> N\n\
                 CB -> H\n\
                 NH -> C\n\
                 HB -> C\n\
                 HC -> B\n\
                 HN -> C\n\
                 NN -> C\n\
                 BH -> H\n\
                 NC -> B\n\
                 NB -> B\n\
                 BN -> B\n\
                 BB -> N\n\
                 BC -> B\n\
                 CC -> N\n\
                 CN -> C",
                10
            ),
            1588
        );
    }

    #[test]
    fn it_solves_part1_small_example_with_part2() {
        assert_eq!(
            solve(
                "NNCB\n\
                 \n\
                 CH -> B\n\
                 HH -> N\n\
                 CB -> H\n\
                 NH -> C\n\
                 HB -> C\n\
                 HC -> B\n\
                 HN -> C\n\
                 NN -> C\n\
                 BH -> H\n\
                 NC -> B\n\
                 NB -> B\n\
                 BN -> B\n\
                 BB -> N\n\
                 BC -> B\n\
                 CC -> N\n\
                 CN -> C",
                4
            ),
            18 // 23 B, 5 H
        );
    }

    #[test]
    fn it_solves_part1_example_with_part2() {
        assert_eq!(
            solve(
                "NNCB\n\
                 \n\
                 CH -> B\n\
                 HH -> N\n\
                 CB -> H\n\
                 NH -> C\n\
                 HB -> C\n\
                 HC -> B\n\
                 HN -> C\n\
                 NN -> C\n\
                 BH -> H\n\
                 NC -> B\n\
                 NB -> B\n\
                 BN -> B\n\
                 BB -> N\n\
                 BC -> B\n\
                 CC -> N\n\
                 CN -> C",
                10
            ),
            1588
        );
    }

    #[test]
    #[ignore]
    fn it_solves_part2_example() {
        assert_eq!(
            solve(
                "NNCB\n\
                 \n\
                 CH -> B\n\
                 HH -> N\n\
                 CB -> H\n\
                 NH -> C\n\
                 HB -> C\n\
                 HC -> B\n\
                 HN -> C\n\
                 NN -> C\n\
                 BH -> H\n\
                 NC -> B\n\
                 NB -> B\n\
                 BN -> B\n\
                 BB -> N\n\
                 BC -> B\n\
                 CC -> N\n\
                 CN -> C",
                40
            ),
            2188189693529
        );
    }
}
