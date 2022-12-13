use std::cmp::Ordering;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, newline, space0},
    combinator::{cut, map},
    error::context,
    multi::separated_list0,
    sequence::{preceded, separated_pair, terminated},
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

fn parse_pair(input: &str) -> IResult<&str, Pair> {
    context(
        "pair",
        map(
            separated_pair(parse_packet, newline, parse_packet),
            |(left, right)| Pair { left, right },
        ),
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Pair>> {
    context("pairs", separated_list0(tag("\n\n"), parse_pair))(input)
}

pub struct Day13;

fn compare(left: &Packet, right: &Packet) -> Ordering {
    if let Packet::Value(lval) = left {
        if let Packet::Value(rval) = right {
            return lval.cmp(rval);
        } else if let Packet::Array(rval) = right {
            let lval = vec![left.clone()];

            for i in 0..usize::min(lval.len(), rval.len()) {
                let cmp = compare(&lval[i], &rval[i]);
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }

            return lval.len().cmp(&rval.len())
        }
    }

    if let Packet::Array(lval) = left {
        let rval = match right {
            Packet::Array(rval) => rval.clone(),
            Packet::Value(_) => {
                vec![right.clone()]
            }
        };

        for i in 0..usize::min(lval.len(), rval.len()) {
            let cmp = compare(&lval[i], &rval[i]);
            if cmp != Ordering::Equal {
                return cmp;
            }
        }

        return lval.len().cmp(&rval.len())
    }

    Ordering::Equal
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
        let (_, pairs) = parse(input).map_err(|err| AocError::parse("", err))?;

        let correct = pairs
            .iter()
            .enumerate()
            .filter(|(_, pair)| compare(&pair.left, &pair.right) == Ordering::Less)
            .map(|(i, _)| i + 1)
            .collect::<Vec<_>>();

        Ok(correct.iter().sum())
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let (_, pairs) = parse(input).map_err(|err| AocError::parse("", err))?;

        let mut packets = pairs
            .into_iter()
            .flat_map(|pair| vec!(pair.left, pair.right))
            .collect::<Vec<_>>();

        let key_2 = Packet::Array(vec![Packet::Value(2)]);
        let key_6 = Packet::Array(vec![Packet::Value(6)]);

        packets.push(key_2.clone());
        packets.push(key_6.clone());

        packets.sort_by(compare);

        Ok(packets
            .into_iter()
            .enumerate()
            .filter(|(_, key)| *key == key_2 || *key == key_6)
            .map(|(index, _)| index + 1)
            .product())
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

    #[test]
    fn it_solves_part2() {
        assert_eq!(
            Day13.part_2(
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
            Ok(140)
        );
    }
}
