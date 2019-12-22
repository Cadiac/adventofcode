extern crate regex;
use regex::Regex;
use std::collections::VecDeque;

const INPUT_FILE: &str = include_str!("../input.txt");

fn shuffle_deal_into_new_stack(deck: &mut VecDeque<u64>) {
    println!("shuffle_deal_into_new_stack");
    let end_swap_index = deck.len() / 2;

    for index in 0..end_swap_index {
        deck.swap(index, deck.len() - 1 - index);
    }
}

fn shuffle_cut_n(deck: &mut VecDeque<u64>, cuts: i64) {
    println!("shuffle_cut_n {}", cuts);
    if cuts > 0 {
        deck.rotate_left(cuts as usize);
    } else if cuts < 0 {
        deck.rotate_right(cuts.abs() as usize);
    }
}

fn shuffle_deal_with_increment_n(deck: &mut VecDeque<u64>, increment: usize) {
    println!("shuffle_deal_with_increment_n {}", increment);
    let deck_cards = deck.len();
    let mut new_deck = VecDeque::from(vec![0; deck_cards]);

    for index in 0..deck_cards {
        let card = deck.pop_front().unwrap();
        let new_index = (index * increment) % deck_cards;
        new_deck[new_index] = card;
    }

    deck.append(&mut new_deck);
}

fn part_1(input: String) -> usize {
    let mut deck: VecDeque<u64> = (0u64..10007).map(u64::from).collect();;

    let cut_regex = Regex::new(r"^cut (.+)").unwrap();
    let deal_with_increment_regex = Regex::new(r"^deal with increment (.+)$").unwrap();
    let deal_into_new_stacks_regex = Regex::new(r"^deal into new stack$").unwrap();

    for line in input.lines() {
        if let Some(capture) = cut_regex.captures(line) {
            shuffle_cut_n(&mut deck, capture[1].parse::<i64>().unwrap());
        }
        if let Some(capture) = deal_with_increment_regex.captures(line) {
            shuffle_deal_with_increment_n(&mut deck, capture[1].parse::<usize>().unwrap());
        }
        if let Some(_capture) = deal_into_new_stacks_regex.captures(line) {
            shuffle_deal_into_new_stack(&mut deck);
        }
    }

   return deck.iter().position(|card| *card == 2019).unwrap();
}

fn part_2(input: String) -> u64 {
    let mut deck: VecDeque<u64> = (0..119315717514047u64).map(u64::from).collect();;

    let cut_regex = Regex::new(r"^cut (.+)").unwrap();
    let deal_with_increment_regex = Regex::new(r"^deal with increment (.+)$").unwrap();
    let deal_into_new_stacks_regex = Regex::new(r"^deal into new stack$").unwrap();

    for _repeat in 0..101741582076661usize {
        for line in input.lines() {
            if let Some(capture) = cut_regex.captures(line) {
                shuffle_cut_n(&mut deck, capture[1].parse::<i64>().unwrap());
            }
            if let Some(capture) = deal_with_increment_regex.captures(line) {
                shuffle_deal_with_increment_n(&mut deck, capture[1].parse::<usize>().unwrap());
            }
            if let Some(_capture) = deal_into_new_stacks_regex.captures(line) {
                shuffle_deal_into_new_stack(&mut deck);
            }
        }
    }

   return deck[2020];
}

fn main() -> () {
    let position = part_1(String::from(INPUT_FILE));
    // let card = part_2(String::from(INPUT_FILE));
    
    println!("[INFO]: Part 1: {:?}", position);
    // println!("[INFO]: Part 2: {:?}", card);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_shuffles_deal_into_new_stack() {
        let mut deck = VecDeque::from(vec![0,1,2,3,4,5,6,7,8,9]);
        shuffle_deal_into_new_stack(&mut deck);

        assert_eq!(deck, VecDeque::from(vec![9,8,7,6,5,4,3,2,1,0]));
    }


    #[test]
    fn it_shuffles_cut_3() {
        let mut deck = VecDeque::from(vec![0,1,2,3,4,5,6,7,8,9]);
        shuffle_cut_n(&mut deck, 3);

        assert_eq!(deck, VecDeque::from(vec![3,4,5,6,7,8,9,0,1,2]));
    }

    #[test]
    fn it_shuffles_cut_negative_4() {
        let mut deck = VecDeque::from(vec![0,1,2,3,4,5,6,7,8,9]);
        shuffle_cut_n(&mut deck, -4);

        assert_eq!(deck, VecDeque::from(vec![6,7,8,9,0,1,2,3,4,5]));
    }

    #[test]
    fn it_shuffles_deal_with_increment_3() {
        let mut deck = VecDeque::from(vec![0,1,2,3,4,5,6,7,8,9]);
        shuffle_deal_with_increment_n(&mut deck, 3);

        assert_eq!(deck, VecDeque::from(vec![0,7,4,1,8,5,2,9,6,3]));
    }
}
