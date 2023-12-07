use std::{collections::HashMap, cmp::Ordering};

use itertools::Itertools;
use log::info;

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
    let mut counts_by_card: HashMap<u8, u8> = HashMap::new();

    for card in hand {
        *counts_by_card.entry(card).or_default() += 1;
    }

    if counts_by_card.values().any(|count| *count == 5) {
        return 0;
    }

    if counts_by_card.values().any(|count| *count == 4) {
        return 1;
    }

    if counts_by_card.values().any(|count| *count == 3) && counts_by_card.values().any(|count| *count == 2) {
        return 2;
    }

    if counts_by_card.values().any(|count| *count == 3) {
        return 4;
    }

    if counts_by_card.values().any(|count| *count == 2) && counts_by_card.values().any(|count| *count == 2) {
        return 3;
    }

    if counts_by_card.values().any(|count| *count == 2) {
        return 5;
    }

    return 6;
}

fn sort_hands(a: &(Hand, u32, u32), b: &(Hand, u32, u32)) -> Ordering {
    match a.1.partial_cmp(&b.1) {
        Some(Ordering::Equal) | None => (),
        Some(ordering) => return ordering
    }

    for i in 0..5 {
        match a.0[i].partial_cmp(&b.0[i]) {
            Some(Ordering::Equal) | None => (),
            Some(ordering) => return ordering
        }
    }

    Ordering::Equal
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
            .map(|(hand, bid)| (hand, score_hand(hand), bid))
            .sorted_by(sort_hands)
            .collect::<Vec<_>>();

        let _ = total_winnings.iter().enumerate().map(|(rank, (hand, score, bid))| {
            info!("{rank}: {hand:?} {score} {bid}")
        });

        let total_winnings = total_winnings.iter()
            .enumerate()
            .map(|(rank, (_, _, bid))| (rank as u32 + 1) * bid)
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

    #[test]
    fn it_solves_part1_real() {
        assert_eq!(
            Day07.part_1(Day07.default_input()),
            Ok(6440)
        );
    }
}
