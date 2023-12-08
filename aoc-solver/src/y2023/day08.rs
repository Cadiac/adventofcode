use std::collections::{HashMap, HashSet};

use crate::solution::{AocError, Solution};

pub struct Day08;

#[derive(PartialEq, Eq, Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn lcm_of_vector(numbers: Vec<u64>) -> u64 {
    numbers
        .iter()
        .fold(1, |acc, &num| lcm(acc, std::cmp::max(num, 1)))
}

fn parse(input: &str) -> Result<(String, HashMap<String, Node>), AocError> {
    let mut lines = input.trim().lines();

    let instructions = lines
        .next()
        .ok_or(AocError::parse(input, "Missing instructions"))?
        .to_owned();

    let nodes = lines
        .skip(1)
        .map(|line| {
            let (id, (left, right)) = line
                .split_once(" = (")
                .and_then(|(id, lr_str)| {
                    lr_str
                        .strip_suffix(')')
                        .and_then(|s| s.split_once(", "))
                        .map(|lr| (id, lr))
                })
                .ok_or(AocError::parse(line, "Invalid node"))?;

            Ok((
                id.to_owned(),
                Node {
                    id: id.to_owned(),
                    left: left.to_owned(),
                    right: right.to_owned(),
                },
            ))
        })
        .collect::<Result<_, _>>()?;

    Ok((instructions, nodes))
}

impl Solution for Day08 {
    type F = u64;
    type S = u64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day08.txt")
    }

    fn part_1(&self, input: &str) -> Result<u64, AocError> {
        let (instructions, nodes) = parse(input)?;

        let mut steps = 0;
        let mut current = "AAA";
        for instruction in instructions.chars().cycle() {
            if current == "ZZZ" {
                break;
            }

            let node = nodes
                .get(current)
                .ok_or(AocError::logic(format!("Unknown node {current}")))?;

            match instruction {
                'L' => current = &node.left,
                'R' => current = &node.right,
                _ => {
                    return Err(AocError::logic(format!(
                        "Unknown instruction {instruction}"
                    )))
                }
            }

            steps += 1;
        }

        Ok(steps)
    }

    fn part_2(&self, input: &str) -> Result<u64, AocError> {
        let (instructions, nodes) = parse(input)?;

        let mut ends = Vec::new();

        for start in nodes.keys().filter(|id| id.ends_with('A')) {
            let mut current = start;
            let mut current_ends = Vec::new();

            let mut visited: HashSet<(usize, &str)> = HashSet::new();

            for (steps, (index, instruction)) in
                instructions.chars().enumerate().cycle().enumerate()
            {
                if !visited.insert((index, current)) {
                    break;
                }

                if current.ends_with('Z') {
                    current_ends.push(steps as u64);
                }

                let node = nodes
                    .get(current)
                    .ok_or(AocError::logic(format!("Unknown node {current}")))?;

                match instruction {
                    'L' => current = &node.left,
                    'R' => current = &node.right,
                    _ => {
                        return Err(AocError::logic(format!(
                            "Unknown instruction {instruction}"
                        )))
                    }
                }
            }

            ends.push(lcm_of_vector(current_ends));
        }

        let all_at_z = lcm_of_vector(ends);

        Ok(all_at_z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_simple() {
        assert_eq!(
            Day08.part_1(
                "RL\n\
                \n\
                AAA = (BBB, CCC)\n\
                BBB = (DDD, EEE)\n\
                CCC = (ZZZ, GGG)\n\
                DDD = (DDD, DDD)\n\
                EEE = (EEE, EEE)\n\
                GGG = (GGG, GGG)\n\
                ZZZ = (ZZZ, ZZZ)"
            ),
            Ok(2)
        );
    }

    #[test]
    fn it_solves_part1_looping() {
        assert_eq!(
            Day08.part_1(
                "LLR\n\
                \n\
                AAA = (BBB, BBB)\n\
                BBB = (AAA, ZZZ)\n\
                ZZZ = (ZZZ, ZZZ)"
            ),
            Ok(6)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day08.part_2(
                "LR\n\
                \n\
                11A = (11B, XXX)\n\
                11B = (XXX, 11Z)\n\
                11Z = (11B, XXX)\n\
                22A = (22B, XXX)\n\
                22B = (22C, 22C)\n\
                22C = (22Z, 22Z)\n\
                22Z = (22B, 22B)\n\
                XXX = (XXX, XXX)"
            ),
            Ok(6)
        );
    }

    #[test]
    fn it_solves_part2_real_input() {
        assert_eq!(Day08.part_2(Day08.default_input()), Ok(8906539031197));
    }
}
