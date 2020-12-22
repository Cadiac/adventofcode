use std::collections::VecDeque;
use std::collections::HashSet;

const INPUT_FILE: &str = include_str!("../../inputs/day22.txt");

#[derive(PartialEq)]
enum Winner {
    Player1,
    Player2,
}

fn parse_deck(input: &str) -> VecDeque<usize> {
    input
        .lines()
        .skip(1)
        .map(|card| card.parse().unwrap())
        .collect()
}

fn calculate_score(deck: VecDeque<usize>) -> usize {
    deck.iter().rev().enumerate().map(|(value, card)| (value + 1) * card).sum()
}

fn find_winner(deck_1: VecDeque<usize>, deck_2: VecDeque<usize>) -> (Winner, VecDeque<usize>) {
    if deck_2.len() == 0 {
        return (Winner::Player1, deck_1);
    } else {
        return (Winner::Player2, deck_2);
    }
}

fn part_1(input: &str) -> usize {
    let mut iter = input.split("\n\n");

    let mut deck_1 = parse_deck(iter.next().unwrap());
    let mut deck_2 = parse_deck(iter.next().unwrap());

    while !deck_1.is_empty() && !deck_2.is_empty() {
        // Draw cards
        let card_1 = deck_1.pop_front().unwrap();
        let card_2 = deck_2.pop_front().unwrap();

        if card_1 > card_2 {
            deck_1.push_back(card_1);
            deck_1.push_back(card_2);
        } else {
            deck_2.push_back(card_2);
            deck_2.push_back(card_1);
        }
    }

    let (_winner, winning_deck) = find_winner(deck_1, deck_2);
    
    calculate_score(winning_deck)
}

fn play_recursive_game(mut deck_1: VecDeque<usize>, mut deck_2: VecDeque<usize>) -> (Winner, VecDeque<usize>) {
    let mut previous_rounds: HashSet<(VecDeque<usize>, VecDeque<usize>)> = HashSet::new();

    while !deck_1.is_empty() && !deck_2.is_empty() {
        // Before either player deals a card, if there was a previous round in this game
        // that had exactly the same cards in the same order in the same players' decks,
        // the game instantly ends in a win for player 1. Previous rounds from other games are not considered.
        // (This prevents infinite games of Recursive Combat, which everyone agrees is a bad idea.)
        if !previous_rounds.insert((deck_1.clone(), deck_2.clone())) {
            return (Winner::Player1, deck_1);
        }

        // Otherwise, this round's cards must be in a new configuration;
        // the players begin the round by each drawing the top card of their deck as normal.
        let card_1 = deck_1.pop_front().unwrap();
        let card_2 = deck_2.pop_front().unwrap();

        // If both players have at least as many cards remaining in their deck as the value of the card
        // they just drew, the winner of the round is determined by playing a new game of Recursive Combat
        if deck_1.len() as usize >= card_1 && deck_2.len() as usize >= card_2 {
            let mut sub_deck_1 = deck_1.clone();
            sub_deck_1.truncate(card_1);
            let mut sub_deck_2 = deck_2.clone();
            sub_deck_2.truncate(card_2);

            let (round_winner, _winning_deck) = play_recursive_game(sub_deck_1, sub_deck_2);

            if round_winner == Winner::Player1 {
                deck_1.push_back(card_1);
                deck_1.push_back(card_2);
            } else {
                deck_2.push_back(card_2);
                deck_2.push_back(card_1);
            }
        } else {
            // Otherwise, at least one player must not have enough cards left in their deck to recurse;
            // the winner of the round is the player with the higher-value card.
            if card_1 > card_2 {
                deck_1.push_back(card_1);
                deck_1.push_back(card_2);
            } else {
                deck_2.push_back(card_2);
                deck_2.push_back(card_1);
            }
        }
    }

    find_winner(deck_1, deck_2)
}

fn part_2(input: &str) -> usize {
    let mut iter = input.split("\n\n");

    let deck_1 = parse_deck(iter.next().unwrap());
    let deck_2 = parse_deck(iter.next().unwrap());

    let (_winner, winning_deck) = play_recursive_game(deck_1, deck_2);
    
    calculate_score(winning_deck)
}

fn main() -> () {
    let part_1_result = part_1(INPUT_FILE);
    let part_2_result = part_2(INPUT_FILE);

    println!("[INFO]: Part 1: {:?}", part_1_result);
    println!("[INFO]: Part 2: {:?}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            part_1(
                "Player 1:\n\
                 9\n\
                 2\n\
                 6\n\
                 3\n\
                 1\n\
                 \n\
                 Player 2:\n\
                 5\n\
                 8\n\
                 4\n\
                 7\n\
                 10"
            ),
            306
        );
    }

    #[test]
    fn it_solves_part2_inf_example() {
        assert_eq!(
            part_2(
                "Player 1:\n\
                 43\n\
                 19\n\
                 \n\
                 Player 2:\n\
                 2\n\
                 29\n\
                 14"
            ),
            105
        );
    }

    #[test]
    fn it_solves_part2_longer_example() {
        assert_eq!(
            part_2(
                "Player 1:\n\
                 9\n\
                 2\n\
                 6\n\
                 3\n\
                 1\n\
                 \n\
                 Player 2:\n\
                 5\n\
                 8\n\
                 4\n\
                 7\n\
                 10"
            ),
            291
        );
    }
}
