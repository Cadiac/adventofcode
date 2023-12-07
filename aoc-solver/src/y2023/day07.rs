use std::collections::HashMap;

use crate::solution::{AocError, Solution};

pub struct Day07;

#[derive(Debug, Clone)]
struct Card {}

type Hand = [u8; 5];

fn parse(input: &str) -> Result<Vec<(Hand, u32)>, AocError> {
    let card_scores: HashMap<char, u8> = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ]
    .into_iter()
    .rev()
    .enumerate()
    .map(|(score, card)| (card, score as u8))
    .collect();

    let hands = input
        .trim()
        .lines()
        .map(|line| {
            let (hand_str, bid_str) = line
                .split_once(' ')
                .ok_or(AocError::parse(line, "Invalid line"))?;

            let hand = parse_cards(hand_str, &card_scores)?;
            let bid = parse_number(bid_str)?;

            Ok((hand, bid))
        })
        .collect::<Result<_, _>>()?;

    Ok(hands)
}

fn parse_cards(cards: &str, card_scores: &HashMap<char, u8>) -> Result<Hand, AocError> {
    cards
        .chars()
        .take(5)
        .map(|card| {
            card_scores
                .get(&card)
                .copied()
                .ok_or(AocError::parse(card, "Unknown card"))
        })
        .collect::<Result<Vec<_>, AocError>>()?
        .try_into()
        .map_err(|_| AocError::parse(cards, "Wrong number of cards"))
}

fn parse_number(number: &str) -> Result<u32, AocError> {
    number
        .parse()
        .map_err(|_| AocError::parse(number, "Error parsing number"))
}

fn score_hand(hand: Hand) -> u32 {
    todo!()
}

impl Solution for Day07 {
    type F = u32;
    type S = u32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day07.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let total_winnings = parse(input)?
            .into_iter()
            .map(|(hand, bid)| score_hand(hand) * bid)
            .sum();

        Ok(total_winnings)
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day07.part_1(
                "32T3K 765\n\
                 T55J5 684\n\
                 KK677 28\n\
                 KTJJT 220\n\
                 QQQJA 483\n"
            ),
            Ok(6440)
        );
    }
}
