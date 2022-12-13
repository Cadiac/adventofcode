use std::{cmp::Ordering};

use nom::{
    branch::alt,
    character::complete::{char, space0, digit1},
    combinator::{cut, map},
    error::{context},
    multi::separated_list0,
    sequence::{preceded, terminated},
    IResult,
};

use crate::solution::{AocError, Solution};

#[derive(Debug)]
struct Pair {
    left: Packet,
    right: Packet,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    Value(u32),
    Array(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Packet::Value(value) => match other {
                Packet::Value(other) => other.cmp(&value),
                Packet::Array(other) => other.cmp(&vec![self.clone()]),
            },
            Packet::Array(list) => match other {
                Packet::Value(_) => Packet::Array(vec![other.clone()]).cmp(&self),
                Packet::Array(other) => other.cmp(&list),
            },
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    preceded(
        space0,
        alt((
            map(parse_value, |value: u32| Packet::Value(value)),
            map(parse_array, |values: Vec<Packet>| Packet::Array(values)),
        )),
    )(input)
}

fn parse_value(input: &str) -> IResult<&str, u32> {
    map(digit1, |digits: &str| digits.parse().unwrap())(input)
}

fn parse_array(input: &str) -> IResult<&str, Vec<Packet>> {
    context(
        "array",
        preceded(
            char('['),
            cut(terminated(
                separated_list0(preceded(space0, char(',')), parse_packet),
                preceded(space0, char(']')),
            )),
        ),
    )(input)
}

pub struct Day13;

impl Day13 {
    fn parse(input: &str) -> Result<Vec<Pair>, AocError> {
        let mut pairs = Vec::new();
        for chunk in input.split("\n\n") {
            let mut iter = chunk.lines();

            let i = iter.next().unwrap();
            let (_, left) = parse_packet(i)
                .map_err(|err| AocError::parse(i, err))?;

            let i = iter.next().unwrap();
            let (_, right) = parse_packet(i)
                .map_err(|err| AocError::parse(i, err))?;

            pairs.push(Pair{ left, right });
        }

        println!("{pairs:?}");

        Ok(pairs)
    }
}

impl Solution for Day13 {
    type F = usize;
    type S = usize;

    fn name(&self) -> &'static str {
        "Day 13"
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/day13.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let pairs = Self::parse(input)?;

        unimplemented!()
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let pairs = Self::parse(input)?;

        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1() {
        assert_eq!(
            Day13.part_1(
                "[1,1,3,1,1]\n\
                 [1,1,5,1,1]\n\
                 \n\
                 [[1],[2,3,4]]\n\
                 [[1],4]\n\
                 \n\
                 [9]\n\
                 [[8,7,6]]\n\
                 \n\
                 [[4,4],4,4]\n\
                 [[4,4],4,4,4]\n\
                 \n\
                 [7,7,7,7]\n\
                 [7,7,7]\n\
                 \n\
                 []\n\
                 [3]\n\
                 \n\
                 [[[]]]\n\
                 [[]]\n\
                 \n\
                 [1,[2,[3,[4,[5,6,7]]]],8,9]\n\
                 [1,[2,[3,[4,[5,6,0]]]],8,9]"
            ),
            Ok(13)
        );
    }
}
