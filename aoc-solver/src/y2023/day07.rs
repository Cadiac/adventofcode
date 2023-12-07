use std::collections::HashMap;

use itertools::Itertools;

use crate::solution::{AocError, Solution};

const SCORES_PART_1: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];
const SCORES_PART_2: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

pub struct Day07;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

type Cards = [u8; 5];

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

fn parse(input: &str) -> Result<Vec<(Hand, u32)>, AocError> {
    let card_scores: HashMap<char, u8> = SCORES_PART_1
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

            let cards = parse_cards(hand_str, &card_scores)?;
            let bid = parse_number(bid_str)?;
            let kind = hand_type(cards);
            let hand = Hand { cards, kind };

            Ok((hand, bid))
        })
        .collect::<Result<_, _>>()?;

    Ok(hands)
}

fn parse2(input: &str) -> Result<Vec<(Hand, u32)>, AocError> {
    let card_scores: HashMap<char, u8> = SCORES_PART_2
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

            let cards = parse_cards(hand_str, &card_scores)?;
            let bid = parse_number(bid_str)?;
            let kind = hand_type_jokers(cards);
            let hand = Hand { cards, kind };

            Ok((hand, bid))
        })
        .collect::<Result<_, _>>()?;

    Ok(hands)
}

fn parse_cards(cards: &str, card_scores: &HashMap<char, u8>) -> Result<Cards, AocError> {
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

fn hand_type(cards: Cards) -> HandType {
    let mut counts_by_card: HashMap<u8, u8> = HashMap::new();

    for card in cards {
        *counts_by_card.entry(card).or_default() += 1;
    }

    if counts_by_card.values().any(|count| *count == 5) {
        return HandType::FiveOfAKind;
    }

    if counts_by_card.values().any(|count| *count == 4) {
        return HandType::FourOfAKind;
    }

    if counts_by_card.values().any(|count| *count == 3)
        && counts_by_card.values().any(|count| *count == 2)
    {
        return HandType::FullHouse;
    }

    if counts_by_card.values().any(|count| *count == 3) {
        return HandType::ThreeOfAKind;
    }

    let mut two_pairs = 0;
    for count in counts_by_card.values() {
        if *count == 2 {
            two_pairs += 1
        }
    }

    if two_pairs == 2 {
        return HandType::TwoPair;
    }

    if counts_by_card.values().any(|count| *count == 2) {
        return HandType::OnePair;
    }

    HandType::HighCard
}

fn hand_type_jokers(cards: Cards) -> HandType {
    let mut counts_by_card: HashMap<u8, u8> = HashMap::new();

    for card in cards {
        *counts_by_card.entry(card).or_default() += 1;
    }

    let jokers = counts_by_card.remove(&0).unwrap_or(0);
    let mut others = counts_by_card.values().sorted().rev();

    let first = others.next().unwrap_or(&0) + jokers;
    let second = others.next().unwrap_or(&0);

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

impl Solution for Day07 {
    type F = u32;
    type S = u32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day07.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let total_winnings = parse(input)?
            .into_iter()
            .sorted()
            .enumerate()
            .map(|(rank, (_, bid))| (rank as u32 + 1) * bid)
            .sum();

        Ok(total_winnings)
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        let total_winnings = parse2(input)?
            .into_iter()
            .sorted()
            .enumerate()
            .map(|(rank, (_, bid))| (rank as u32 + 1) * bid)
            .sum();

        Ok(total_winnings)
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
        assert_eq!(hand_type_jokers([11, 5, 2, 12, 10]), HandType::HighCard);
        assert_eq!(hand_type_jokers([0, 6, 12, 8, 4]), HandType::OnePair);
        assert_eq!(hand_type_jokers([4, 1, 4, 10, 1]), HandType::TwoPair);
        assert_eq!(hand_type_jokers([1, 5, 5, 0, 3]), HandType::ThreeOfAKind);
        assert_eq!(hand_type_jokers([10, 10, 7, 7, 0]), HandType::FullHouse);
        assert_eq!(hand_type_jokers([12, 4, 4, 0, 4]), HandType::FourOfAKind);
        assert_eq!(hand_type_jokers([12, 12, 0, 12, 0]), HandType::FiveOfAKind);
    }
}
