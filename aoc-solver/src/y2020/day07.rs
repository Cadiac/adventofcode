use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::solution::{AocError, Solution};

pub struct Day07;

#[derive(Debug, Clone)]
struct Rule {
    children: Vec<(usize, String)>,
    parents: Vec<String>,
}

fn build_rules_tree(input: &str) -> HashMap<String, Rule> {
    let input_regex = Regex::new(r"(.+) bags contain (.+).").unwrap();
    let bag_regex = Regex::new(r"^(\d+) (.+) bags?$").unwrap();

    let mut rules: HashMap<String, Rule> = HashMap::new();

    for capture in input_regex.captures_iter(input) {
        let outer_name = capture.get(1).unwrap().as_str().to_string();
        let inner_bags = capture.get(2).unwrap().as_str();

        let rule = rules.entry(outer_name.clone()).or_insert(Rule {
            children: Vec::new(),
            parents: Vec::new(),
        });

        let mut children: Vec<(usize, String)> = Vec::new();

        for inner_bag in inner_bags.split(", ") {
            if inner_bag == "no other bags" {
                break;
            }
            let bag_cap = bag_regex.captures(inner_bag).unwrap();

            let amount = bag_cap[1].parse::<usize>().unwrap();
            let inner_bag_name = bag_cap[2].parse::<String>().unwrap();

            children.push((amount, inner_bag_name));
        }

        rule.children = children.clone();

        // Also populate parents for all nodes
        for (_amount, child_name) in children {
            rules
                .entry(child_name.clone())
                .and_modify(|child| child.parents.push(outer_name.clone()))
                .or_insert(Rule {
                    children: Vec::new(),
                    parents: vec![outer_name.clone()],
                });
        }
    }

    rules
}

fn find_ancestors(current: String, rules: HashMap<String, Rule>) -> HashSet<String> {
    let current_rule = rules.get(&current).unwrap();

    let mut ancestors: HashSet<String> = HashSet::new();

    for parent in current_rule.parents.iter() {
        ancestors.insert(parent.clone());
        ancestors.extend(find_ancestors(parent.clone(), rules.clone()));
    }

    ancestors
}

fn find_inner_bags_count(current: String, rules: HashMap<String, Rule>) -> usize {
    let current_rule = rules.get(&current).unwrap();

    let sum = current_rule
        .children
        .iter()
        .map(|(count, child)| count + count * find_inner_bags_count(child.clone(), rules.clone()))
        .sum();

    sum
}

impl Solution for Day07 {
    type A = usize;
    type B = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2020/day07.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let rules = build_rules_tree(input);
        let ancestors = find_ancestors(String::from("shiny gold"), rules);

        Ok(ancestors.len())
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let rules = build_rules_tree(input);
        let count = find_inner_bags_count(String::from("shiny gold"), rules);

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day07.part_1(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
                 dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
                 bright white bags contain 1 shiny gold bag.\n\
                 muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
                 shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
                 dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
                 vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
                 faded blue bags contain no other bags.\n\
                 dotted black bags contain no other bags."
            ),
            Ok(4)
        );
    }

    #[test]
    fn it_solves_part2_example_1() {
        assert_eq!(
            Day07.part_2(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
                 dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
                 bright white bags contain 1 shiny gold bag.\n\
                 muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
                 shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
                 dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
                 vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
                 faded blue bags contain no other bags.\n\
                 dotted black bags contain no other bags."
            ),
            Ok(32)
        );
    }

    #[test]
    fn it_solves_part2_example_2() {
        assert_eq!(
            Day07.part_2(
                "shiny gold bags contain 2 dark red bags.\n\
                 dark red bags contain 2 dark orange bags.\n\
                 dark orange bags contain 2 dark yellow bags.\n\
                 dark yellow bags contain 2 dark green bags.\n\
                 dark green bags contain 2 dark blue bags.\n\
                 dark blue bags contain 2 dark violet bags.\n\
                 dark violet bags contain no other bags."
            ),
            Ok(126)
        );
    }
}
