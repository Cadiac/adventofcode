use crate::solution::{AocError, Solution};
use itertools::Itertools;
use std::collections::HashMap;

const SCORES_PART_1: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];
const SCORES_PART_2: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];
const HAND_SIZE: usize = 5;

pub struct Day07;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

type Cards = [u8; HAND_SIZE];

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: Cards,
    kind: HandType,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ord::cmp(self, other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.kind
            .cmp(&other.kind)
            .then(self.cards.cmp(&other.cards))
    }
}

fn parse(input: &str, scores: &[char], use_jokers: bool) -> Result<Vec<(Hand, u32)>, AocError> {
    let card_scores: HashMap<char, u8> = scores
        .iter()
        .enumerate()
        .map(|(score, card)| (*card, score as u8))
        .collect();

    let hands = input
        .trim()
        .lines()
        .map(|line| {
            let (hand_str, bid_str) = line
                .split_once(' ')
                .ok_or(AocError::parse(line, "Invalid line"))?;

            let cards = parse_cards(hand_str, &card_scores)?;
            let bid = parse_number(bid_str)?;
            let kind = hand_type(cards, use_jokers);

            Ok((Hand { cards, kind }, bid))
        })
        .collect::<Result<_, _>>()?;

    Ok(hands)
}

fn parse_cards(cards: &str, card_scores: &HashMap<char, u8>) -> Result<Cards, AocError> {
    let mut result = [0; HAND_SIZE];

    for (i, card) in cards.chars().enumerate().take(HAND_SIZE) {
        result[i] = *card_scores
            .get(&card)
            .ok_or_else(|| AocError::parse(card, "Unknown card"))?;
    }

    Ok(result)
}

fn parse_number(number: &str) -> Result<u32, AocError> {
    number
        .parse()
        .map_err(|_| AocError::parse(number, "Error parsing number"))
}

fn hand_type(cards: Cards, use_jokers: bool) -> HandType {
    let mut counts_by_card: HashMap<u8, u8> = HashMap::new();

    for card in cards {
        *counts_by_card.entry(card).or_default() += 1;
    }

    let jokers = if use_jokers {
        counts_by_card.remove(&0).unwrap_or(0)
    } else {
        0
    };

    let mut counts = counts_by_card.into_values().sorted().rev();

    // Find the two piles of cards with most copies
    let first = counts.next().unwrap_or(0) + jokers;
    let second = counts.next().unwrap_or(0);

    match (first, second) {
        (5, _) => HandType::FiveOfAKind,
        (4, _) => HandType::FourOfAKind,
        (3, 2) => HandType::FullHouse,
        (3, _) => HandType::ThreeOfAKind,
        (2, 2) => HandType::TwoPair,
        (2, _) => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn calculate_winnings(input: &str, scores: &[char], use_jokers: bool) -> Result<u32, AocError> {
    let total_winnings = parse(input, scores, use_jokers)?
        .into_iter()
        .sorted()
        .enumerate()
        .map(|(rank, (_, bid))| (rank as u32 + 1) * bid)
        .sum();

    Ok(total_winnings)
}

impl Solution for Day07 {
    type F = u32;
    type S = u32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day07.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        calculate_winnings(input, &SCORES_PART_1, false)
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        calculate_winnings(input, &SCORES_PART_2, true)
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
    fn it_solves_part2_example() {
        assert_eq!(
            Day07.part_2(
                "32T3K 765\n\
                 T55J5 684\n\
                 KK677 28\n\
                 KTJJT 220\n\
                 QQQJA 483\n"
            ),
            Ok(5905)
        );
    }

    #[test]
    fn it_finds_correct_hand_types() {
        assert_eq!(hand_type([11, 5, 2, 12, 10], true), HandType::HighCard);

        assert_eq!(hand_type([0, 6, 12, 8, 4], false), HandType::HighCard);
        assert_eq!(hand_type([0, 6, 12, 8, 4], true), HandType::OnePair);

        assert_eq!(hand_type([4, 1, 4, 10, 1], true), HandType::TwoPair);

        assert_eq!(hand_type([1, 5, 5, 0, 3], false), HandType::OnePair);
        assert_eq!(hand_type([1, 5, 5, 0, 3], true), HandType::ThreeOfAKind);

        assert_eq!(hand_type([10, 10, 7, 7, 0], false), HandType::TwoPair);
        assert_eq!(hand_type([10, 10, 7, 7, 0], true), HandType::FullHouse);

        assert_eq!(hand_type([12, 4, 4, 0, 4], false), HandType::ThreeOfAKind);
        assert_eq!(hand_type([12, 4, 4, 0, 4], true), HandType::FourOfAKind);

        assert_eq!(hand_type([12, 12, 0, 12, 0], false), HandType::FullHouse);
        assert_eq!(hand_type([12, 12, 0, 12, 0], true), HandType::FiveOfAKind);
    }
}
