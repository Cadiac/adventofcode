use crate::solution::{AocError, Solution};
use itertools::Itertools;
use std::{collections::HashMap, iter};

pub struct Day08;

#[derive(PartialEq, Eq, Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
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
                    let left_right = lr_str.strip_suffix(')').and_then(|s| s.split_once(", "));

                    match left_right {
                        Some(lr) => Some((id, lr)),
                        None => None,
                    }
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
    type F = u32;
    type S = u32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day08.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
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

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        // TODO:
        // calculate how long does it take to end up on all z's or return to starting position
        // least common divider
        unimplemented!();

        let (instructions, nodes) = parse(input)?;

        let mut steps = 0;
        let mut currents: Vec<String> = nodes
            .keys()
            .filter(|id| id.ends_with('A'))
            .cloned()
            .collect();

        for instruction in instructions.chars().cycle() {
            if currents.iter().all(|current| current.ends_with('Z')) {
                break;
            }

            let mut new_currents = Vec::with_capacity(currents.len());

            for current in currents.into_iter() {
                let node = nodes
                    .get(&current)
                    .ok_or(AocError::logic(format!("Unknown node {current}")))?;

                let next = match instruction {
                    'L' => node.left.clone(),
                    'R' => node.right.clone(),
                    _ => {
                        return Err(AocError::logic(format!(
                            "Unknown instruction {instruction}"
                        )))
                    }
                };

                new_currents.push(next)
            }

            currents = new_currents;
            steps += 1;
        }

        Ok(steps)
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
}
