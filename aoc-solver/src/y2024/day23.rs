use std::collections::{HashMap, HashSet};

use crate::solution::{AocError, Solution};

fn parse(input: &str) -> Result<HashMap<&str, HashSet<&str>>, AocError> {
    let mut network: HashMap<&str, HashSet<&str>> = HashMap::new();

    for row in input.lines() {
        let (first, second) = row
            .split_once("-")
            .ok_or_else(|| AocError::parse(row, "Invalid network node"))?;

        network.entry(first).or_default().insert(second);
        network.entry(second).or_default().insert(first);
    }

    Ok(network)
}

pub struct Day23;
impl Solution for Day23 {
    type A = u32;
    type B = String;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day23.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let network = parse(input)?;

        let mut networks_of_three: HashSet<Vec<&str>> = HashSet::new();

        for (first, edges) in network.iter() {
            if first.starts_with("t") {
                for second in edges {
                    for third in network[second].intersection(edges) {
                        let mut group = vec![*first, *second, *third];
                        group.sort();
                        networks_of_three.insert(group);
                    }
                }
            }
        }

        Ok(networks_of_three.len() as u32)
    }

    fn part_2(&self, input: &str) -> Result<String, AocError> {
        let network = parse(input)?;

        let mut largest = network
            .iter()
            .map(|(current, edges)| {
                let mut interconnected = vec![*current];

                for edge in edges {
                    if interconnected
                        .iter()
                        .all(|node| network[edge].contains(node))
                    {
                        interconnected.push(*edge);
                    }
                }

                interconnected
            })
            .max_by(|a, b| a.len().cmp(&b.len()))
            .ok_or_else(|| AocError::logic("No groups found"))?;

        largest.sort();
        let password = largest.join(",");

        Ok(password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day23.part_1(
                "kh-tc\n\
                 qp-kh\n\
                 de-cg\n\
                 ka-co\n\
                 yn-aq\n\
                 qp-ub\n\
                 cg-tb\n\
                 vc-aq\n\
                 tb-ka\n\
                 wh-tc\n\
                 yn-cg\n\
                 kh-ub\n\
                 ta-co\n\
                 de-co\n\
                 tc-td\n\
                 tb-wq\n\
                 wh-td\n\
                 ta-ka\n\
                 td-qp\n\
                 aq-cg\n\
                 wq-ub\n\
                 ub-vc\n\
                 de-ta\n\
                 wq-aq\n\
                 wq-vc\n\
                 wh-yn\n\
                 ka-de\n\
                 kh-ta\n\
                 co-tc\n\
                 wh-qp\n\
                 tb-vc\n\
                 td-yn"
            ),
            Ok(7)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day23.part_2(
                "kh-tc\n\
                 qp-kh\n\
                 de-cg\n\
                 ka-co\n\
                 yn-aq\n\
                 qp-ub\n\
                 cg-tb\n\
                 vc-aq\n\
                 tb-ka\n\
                 wh-tc\n\
                 yn-cg\n\
                 kh-ub\n\
                 ta-co\n\
                 de-co\n\
                 tc-td\n\
                 tb-wq\n\
                 wh-td\n\
                 ta-ka\n\
                 td-qp\n\
                 aq-cg\n\
                 wq-ub\n\
                 ub-vc\n\
                 de-ta\n\
                 wq-aq\n\
                 wq-vc\n\
                 wh-yn\n\
                 ka-de\n\
                 kh-ta\n\
                 co-tc\n\
                 wh-qp\n\
                 tb-vc\n\
                 td-yn"
            ),
            Ok(String::from("co,de,ka,ta"))
        );
    }
}
