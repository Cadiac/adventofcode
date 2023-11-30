use std::collections::HashSet;
use std::collections::VecDeque;

use crate::solution::{AocError, Solution};

#[derive(Debug, Clone)]
struct TerminalRule {
    pub lhs: u32,
    pub rhs: char,
}

#[derive(Debug, Clone)]
struct ProductionRule {
    pub lhs: u32,
    pub non_terminal_a: u32,
    pub non_terminal_b: u32,
}

pub struct Day19;

// Adapted from https://www.geeksforgeeks.org/converting-context-free-grammar-chomsky-normal-form/
// A context free grammar (CFG) is in Chomsky Normal Form (CNF) if all production rules satisfy one of the following conditions:
//  - A non-terminal generating a terminal (e.g.; X->x)
//  - A non-terminal generating two non-terminals (e.g.; X->YZ)
//  - Start symbol generating ε. (e.g.; S-> ε)
fn convert_to_cnf(input: &str) -> (Vec<TerminalRule>, Vec<ProductionRule>) {
    // At our example the only terminal rules are the characters like "a".
    let mut terminal_rules: Vec<TerminalRule> = input
        .lines()
        .filter(|rule| rule.contains('"'))
        .map(|rule| {
            let mut iter = rule.split(": ");
            let lhs = iter.next().unwrap().parse().unwrap();
            let rhs = iter.next().unwrap().chars().nth(1).unwrap();

            TerminalRule { lhs: lhs, rhs: rhs }
        })
        .collect();

    let mut max_rule_id = input
        .lines()
        .map(|rule| rule.split(": ").next().unwrap().parse::<u32>().unwrap())
        .max()
        .unwrap();

    let mut useless_unit_rules: Vec<(u32, u32)> = vec![];

    let mut production_rules: Vec<ProductionRule> = input
        .lines()
        .filter(|rule| !rule.contains('"'))
        .flat_map(|rule| {
            let mut lhs_rhs = rule.split(": ");
            let lhs = lhs_rhs.next().unwrap().parse().unwrap();
            let rhs = lhs_rhs.next().unwrap();

            rhs.split(" | ")
                .flat_map(|production| {
                    let mut p: VecDeque<u32> = production
                        .split_whitespace()
                        .map(|p| p.parse::<u32>().unwrap())
                        .collect();

                    // This is an useless unit production rule. Deal with these later
                    if p.len() == 1 {
                        useless_unit_rules.push((lhs, p[0]));
                        return vec![];
                    }

                    let mut rules: Vec<ProductionRule> = vec![];

                    // Eliminate RHS with more than two non-terminals.
                    // e.g,; production rule X->XYZ can be decomposed as:
                    // X->PZ
                    // P->XY
                    while p.len() > 2 {
                        max_rule_id += 1;

                        let a = p.pop_front().unwrap();
                        let b = p.pop_front().unwrap();
                        p.push_front(max_rule_id);

                        rules.push(ProductionRule {
                            lhs: max_rule_id,
                            non_terminal_a: a,
                            non_terminal_b: b,
                        });
                    }

                    assert_eq!(p.len(), 2);

                    rules.push(ProductionRule {
                        lhs: lhs,
                        non_terminal_a: p[0],
                        non_terminal_b: p[1],
                    });

                    rules
                })
                .collect::<Vec<ProductionRule>>()
        })
        .collect();

    // We need to eliminate unit productions like 8: 42
    for (lhs, rhs) in useless_unit_rules {
        // Find an existing rule to replace the right hand sides of the unit rules we
        // skipped earlier. In the example input we have useless rule
        //  8: 42
        // and a valid rules
        //  42: 40 69 | 127 5
        // and we want to replace the rule 8 with
        //  8: 40 69
        //  8: 127 5
        let existing_production: Vec<ProductionRule> = production_rules
            .iter()
            .filter(|rule| rule.lhs == rhs)
            .cloned()
            .collect();

        for rule in existing_production.iter() {
            production_rules.push(ProductionRule {
                lhs: lhs,
                non_terminal_a: rule.non_terminal_a,
                non_terminal_b: rule.non_terminal_b,
            });
        }

        // We also need to do the same for terminal rules to handle situations like
        //  93: 127 | 40
        //  40: "a"
        //  127: "b"
        let existing_terminal: Vec<TerminalRule> = terminal_rules
            .iter()
            .filter(|rule| rule.lhs == rhs)
            .cloned()
            .collect();

        for rule in existing_terminal.iter() {
            terminal_rules.push(TerminalRule {
                lhs: lhs,
                rhs: rule.rhs,
            });
        }
    }

    (terminal_rules, production_rules)
}

fn cyk(
    w: &str,
    terminal_rules: &Vec<TerminalRule>,
    production_rules: &Vec<ProductionRule>,
) -> bool {
    // Adapted from https://www.geeksforgeeks.org/cyk-algorithm-for-context-free-grammar/
    let n = w.len();

    let mut dp: Vec<Vec<HashSet<u32>>> = vec![vec![HashSet::new(); production_rules.len()]; n];

    for (s, c) in w.chars().enumerate() {
        for terminal_rule in terminal_rules.iter() {
            if terminal_rule.rhs == c {
                dp[0][s].insert(terminal_rule.lhs);
            }
        }
    }

    // This should run for O(|G|*n^3)
    for i in 1..n {
        for j in 0..(n - i) {
            for k in 0..i {
                for production_rule in production_rules.iter() {
                    if dp[k][j].contains(&production_rule.non_terminal_a)
                        && dp[i - k - 1][j + k + 1].contains(&production_rule.non_terminal_b)
                    {
                        dp[i][j].insert(production_rule.lhs);
                    }
                }
            }
        }
    }

    dp[n - 1][0].contains(&0)
}

fn count_valid_grammar(input: &str) -> usize {
    let mut input_chunks = input.split("\n\n");

    // Convert the input to Chomsky Normal Form (CNF)
    let (terminal_rules, production_rules) = convert_to_cnf(input_chunks.next().unwrap());

    // Check if grammar validates with CYK Algorithm
    input_chunks
        .next()
        .unwrap()
        .lines()
        .filter(|line| cyk(line, &terminal_rules, &production_rules))
        .count()
}

impl Solution for Day19 {
    type F = usize;
    type S = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2020/day19.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let count = count_valid_grammar(input);

        Ok(count)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        // Replace rules 8: 42 and 11: 42 31 with the following:
        let mut i = input.replace("8: 42", "8: 42 | 42 8");
        i = i.replace("11: 42 31", "11: 42 31 | 42 11 31");

        let count = count_valid_grammar(i.as_str());

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example_1() {
        assert_eq!(
            Day19.part_1(
                "0: 1 2\n\
                 1: \"a\"\n\
                 2: 1 3 | 3 1\n\
                 3: \"b\"\n\
                 \n\
                 aba\n\
                 aab\n\
                 abb"
            ),
            Ok(2)
        );
    }

    #[test]
    fn it_solves_part1_example_2() {
        assert_eq!(
            Day19.part_1(
                "0: 4 1 5\n\
                 1: 2 3 | 3 2\n\
                 2: 4 4 | 5 5\n\
                 3: 4 5 | 5 4\n\
                 4: \"a\"\n\
                 5: \"b\"\n\
                 \n\
                 ababbb\n\
                 bababa\n\
                 abbbab\n\
                 aaabbb\n\
                 aaaabbb"
            ),
            Ok(2)
        );
    }

    #[test]
    fn it_solves_part2_example_with_part1() {
        assert_eq!(
            Day19.part_1(
                "42: 9 14 | 10 1\n\
                 9: 14 27 | 1 26\n\
                 10: 23 14 | 28 1\n\
                 1: \"a\"\n\
                 11: 42 31\n\
                 5: 1 14 | 15 1\n\
                 19: 14 1 | 14 14\n\
                 12: 24 14 | 19 1\n\
                 16: 15 1 | 14 14\n\
                 31: 14 17 | 1 13\n\
                 6: 14 14 | 1 14\n\
                 2: 1 24 | 14 4\n\
                 0: 8 11\n\
                 13: 14 3 | 1 12\n\
                 15: 1 | 14\n\
                 17: 14 2 | 1 7\n\
                 23: 25 1 | 22 14\n\
                 28: 16 1\n\
                 4: 1 1\n\
                 20: 14 14 | 1 15\n\
                 3: 5 14 | 16 1\n\
                 27: 1 6 | 14 18\n\
                 14: \"b\"\n\
                 21: 14 1 | 1 14\n\
                 25: 1 1 | 1 14\n\
                 22: 14 14\n\
                 8: 42\n\
                 26: 14 22 | 1 20\n\
                 18: 15 15\n\
                 7: 14 5 | 1 21\n\
                 24: 14 1\n\
                 \n\
                 abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa\n\
                 bbabbbbaabaabba\n\
                 babbbbaabbbbbabbbbbbaabaaabaaa\n\
                 aaabbbbbbaaaabaababaabababbabaaabbababababaaa\n\
                 bbbbbbbaaaabbbbaaabbabaaa\n\
                 bbbababbbbaaaaaaaabbababaaababaabab\n\
                 ababaaaaaabaaab\n\
                 ababaaaaabbbaba\n\
                 baabbaaaabbaaaababbaababb\n\
                 abbbbabbbbaaaababbbbbbaaaababb\n\
                 aaaaabbaabaaaaababaa\n\
                 aaaabbaaaabbaaa\n\
                 aaaabbaabbaaaaaaabbbabbbaaabbaabaaa\n\
                 babaaabbbaaabaababbaabababaaab\n\
                 aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"
            ),
            Ok(3)
        )
    }

    #[test]
    fn it_solves_part2_example_with_part2() {
        assert_eq!(
            Day19.part_2(
                "42: 9 14 | 10 1\n\
                 9: 14 27 | 1 26\n\
                 10: 23 14 | 28 1\n\
                 1: \"a\"\n\
                 11: 42 31\n\
                 5: 1 14 | 15 1\n\
                 19: 14 1 | 14 14\n\
                 12: 24 14 | 19 1\n\
                 16: 15 1 | 14 14\n\
                 31: 14 17 | 1 13\n\
                 6: 14 14 | 1 14\n\
                 2: 1 24 | 14 4\n\
                 0: 8 11\n\
                 13: 14 3 | 1 12\n\
                 15: 1 | 14\n\
                 17: 14 2 | 1 7\n\
                 23: 25 1 | 22 14\n\
                 28: 16 1\n\
                 4: 1 1\n\
                 20: 14 14 | 1 15\n\
                 3: 5 14 | 16 1\n\
                 27: 1 6 | 14 18\n\
                 14: \"b\"\n\
                 21: 14 1 | 1 14\n\
                 25: 1 1 | 1 14\n\
                 22: 14 14\n\
                 8: 42\n\
                 26: 14 22 | 1 20\n\
                 18: 15 15\n\
                 7: 14 5 | 1 21\n\
                 24: 14 1\n\
                 \n\
                 abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa\n\
                 bbabbbbaabaabba\n\
                 babbbbaabbbbbabbbbbbaabaaabaaa\n\
                 aaabbbbbbaaaabaababaabababbabaaabbababababaaa\n\
                 bbbbbbbaaaabbbbaaabbabaaa\n\
                 bbbababbbbaaaaaaaabbababaaababaabab\n\
                 ababaaaaaabaaab\n\
                 ababaaaaabbbaba\n\
                 baabbaaaabbaaaababbaababb\n\
                 abbbbabbbbaaaababbbbbbaaaababb\n\
                 aaaaabbaabaaaaababaa\n\
                 aaaabbaaaabbaaa\n\
                 aaaabbaabbaaaaaaabbbabbbaaabbaabaaa\n\
                 babaaabbbaaabaababbaabababaaab\n\
                 aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"
            ),
            Ok(12)
        )
    }
}
