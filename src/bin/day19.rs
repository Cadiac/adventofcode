use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

const INPUT_FILE: &str = include_str!("../../inputs/day19.txt");

#[derive(Debug, Clone)]
struct TerminalRule {
    pub lhs: u64,
    pub rhs: char,
}

#[derive(Debug, Clone)]
struct ProductionRule {
    pub lhs: u64,
    pub non_terminal_a: u64,
    pub non_terminal_b: u64,
}

fn convert_to_cnf(input: &str) -> (Vec<TerminalRule>, Vec<ProductionRule>) {
    // https://www.geeksforgeeks.org/converting-context-free-grammar-chomsky-normal-form/
    // A context free grammar (CFG) is in Chomsky Normal Form (CNF) if all production rules satisfy one of the following conditions:

    // A non-terminal generating a terminal (e.g.; X->x)
    // A non-terminal generating two non-terminals (e.g.; X->YZ)
    // Start symbol generating ε. (e.g.; S-> ε)

    // At our example the only terminal rules are the ones ending to characters like "a".
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
        .map(|rule| rule.split(": ").next().unwrap().parse::<u64>().unwrap())
        .max()
        .unwrap();

    // Step 1. Eliminate start symbol from RHS.
    // If start symbol S is at the RHS of any production in the grammar, create a new production as:
    // S0->S
    // where S0 is the new start symbol.
    // No need to do this, "S" is not at the RHS of any production in the grammar.

    // Step 2. Eliminate null, unit and useless productions.
    // If CFG contains null, unit or useless production rules, eliminate them. You can refer the this article to eliminate these types of production rules.
    // We need to eliminate unit productions like 8: 42

    // Step 3. Eliminate terminals from RHS if they exist with other terminals or non-terminals. e.g,; production rule X->xY can be decomposed as:
    // X->ZY
    // Z->x
    // These don't exist, all terminal rules are just Z->x already

    let mut useless_unit_rules: Vec<(u64, u64)> = vec![];

    // Step 4. Eliminate RHS with more than two non-terminals.
    // e.g,; production rule X->XYZ can be decomposed as:
    // X->PZ
    // P->XY
    let mut production_rules: Vec<ProductionRule> = input
        .lines()
        .filter(|rule| !rule.contains('"'))
        .flat_map(|rule| {
            let mut lhs_rhs = rule.split(": ");
            let lhs = lhs_rhs.next().unwrap().parse().unwrap();
            let rhs = lhs_rhs.next().unwrap();

            rhs.split(" | ")
                .flat_map(|production| {
                    // println!("parsing production rule {:?}: {:?}", lhs, production);
                    let mut p: VecDeque<u64> = production
                        .split_whitespace()
                        .map(|p| p.parse::<u64>().unwrap())
                        .collect();

                    // This is an useless unit production rule. Deal with these later
                    if p.len() == 1 {
                        println!("useless unit production {:?}: {:?}", lhs, production);
                        useless_unit_rules.push((lhs, p[0]));
                        return vec![];
                    }

                    let mut rules: Vec<ProductionRule> = vec![];

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

    // println!("{:?}", terminal_rules);
    // println!("{:?}", production_rules);
    // println!("{:?}", useless_unit_rules);

    for (lhs, rhs) in useless_unit_rules {
        // find an existing rule to replace the right hand sides of the unit rules we
        // skipped earlier. In the example input we have useless rule
        // 8: 42
        // and a valid rules
        // 42: 40 69 | 127 5
        // and we want to replace the rule 8 with
        // 8: 40 69
        // 8: 127 5
        let existing_production: Vec<ProductionRule> = production_rules
            .iter()
            .filter(|rule| rule.lhs == rhs)
            .cloned()
            .collect();

        for rule in existing_production.iter() {
            println!(
                "Creating rule {:?}: {:?} {:?}",
                lhs, rule.non_terminal_a, rule.non_terminal_b
            );
            production_rules.push(ProductionRule {
                lhs: lhs,
                non_terminal_a: rule.non_terminal_a,
                non_terminal_b: rule.non_terminal_b,
            });
        }

        // We also need to do the same for terminal rules to handle situations like
        // 93: 127 | 40
        // 40: "a"
        // 127: "b"
        let existing_terminal: Vec<TerminalRule> = terminal_rules
            .iter()
            .filter(|rule| rule.lhs == rhs)
            .cloned()
            .collect();

        for rule in existing_terminal.iter() {
            println!("Creating terminal rule {:?}: {:?}", lhs, rule.rhs);
            terminal_rules.push(TerminalRule {
                lhs: lhs,
                rhs: rule.rhs,
            });
        }
    }

    (terminal_rules, production_rules)
}

fn cyk(w: &str, terminal_rules: Vec<TerminalRule>, production_rules: Vec<ProductionRule>) -> bool {
    // println!("Checking if string w {:?} is valid grammar", w);

    // https://www.geeksforgeeks.org/cyk-algorithm-for-context-free-grammar/
    // Let w be the n length string to be parsed. And G represent the set of rules in our grammar with start state S.
    let s = 0;

    let n = w.len() as u64;

    // Construct a table DP for size n × n.
    let mut dp: HashMap<(u64, u64), HashSet<u64>> = HashMap::new();

    // If w = e (empty string) and S -> e is a rule in G then we accept the string else we reject.
    if w.len() == 0 {
        return false;
    }

    // NOTE: indexing from 1 since this is full copy
    // For i = 1 to n:
    for (i, c) in w.chars().enumerate() {
        // For each variable A:
        for terminal_rule in terminal_rules.iter() {
            //     We check if A -> b is a rule and b = w_i for some i:
            if terminal_rule.rhs == c {
                //         If so, we place A in cell (i, i) of our table.
                dp.entry((i as u64 + 1, i as u64 + 1))
                    .or_insert(HashSet::new())
                    .insert(terminal_rule.lhs);
            }
        }
    }

    // For l = 2 to n:
    for l in 2..=n {
        // For i = 1 to n-l+1:
        for i in 1..=n - l + 1 {
            //     j = i+l-1
            let j = i + l - 1;
            //         For k = i to j-1:
            for k in i..=j - 1 {
                //         For each rule A -> BC:
                for production_rule in production_rules.iter() {
                    //         We check if (i, k) cell contains B and (k + 1, j) cell contains C:
                    if dp
                        .entry((i, k))
                        .or_insert(HashSet::new())
                        .contains(&production_rule.non_terminal_a)
                        && dp
                            .entry((k + 1, j))
                            .or_insert(HashSet::new())
                            .contains(&production_rule.non_terminal_b)
                    {
                        //             If so, we put A in cell (i, j) of our table.
                        dp.entry((i, j))
                            .or_insert(HashSet::new())
                            .insert(production_rule.lhs);
                    }
                }
            }
        }
    }

    // println!("{:?}", dp);
    // println!("Checking if (1, {:?}) contains root {:?}", n, s);

    // We check if S is in (1, n):
    if dp.entry((1, n)).or_insert(HashSet::new()).contains(&s) {
        println!("{:?} is accepted", w);
        // If so, we accept the string
        return true;
    } else {
        println!("{:?} is rejected", w);
        // Else, we reject.
        return false;
    }
}

fn part_1(input: &str) -> usize {
    let mut input_chunks = input.split("\n\n");

    // Convert input to Chomsky Normal Form (CNF)
    let (terminal_rules, production_rules) = convert_to_cnf(input_chunks.next().unwrap());

    // Check if grammar validates with CYK Algorithm
    input_chunks
        .next()
        .unwrap()
        .lines()
        .filter(|line| cyk(line, terminal_rules.clone(), production_rules.clone()))
        .count()
}

fn main() -> () {
    let part_1_result = part_1(INPUT_FILE);

    println!("[INFO]: Part 1: {:?}", part_1_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example_1() {
        assert_eq!(
            part_1(
                "0: 1 2\n\
                 1: \"a\"\n\
                 2: 1 3 | 3 1\n\
                 3: \"b\"\n\
                 \n\
                 aba\n\
                 aab\n\
                 abb"
            ),
            2
        );
    }

    #[test]
    fn it_solves_part1_example_2() {
        assert_eq!(
            part_1(
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
            2
        );
    }
}
