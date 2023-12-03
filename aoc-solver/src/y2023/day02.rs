use crate::solution::{AocError, Solution};

pub struct Day02;

struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    hands: Vec<Hand>,
}

fn parse(input: &str) -> Result<Game, AocError> {
    let mut iter = input.split(": ");

    let id = iter
        .next()
        .and_then(|s| s.strip_prefix("Game "))
        .and_then(|s| s.parse::<u32>().ok())
        .ok_or(AocError::parse(input, "invalid game id header"))?;

    let hands = iter
        .next()
        .ok_or(AocError::parse(input, "missing hands"))?
        .split("; ")
        .map(|hand_str: &str| {
            let mut hand = Hand {
                red: 0,
                green: 0,
                blue: 0,
            };

            for shown_hand in hand_str.split(", ").map(|color| color.split_once(' ')) {
                match shown_hand {
                    Some((amount, color)) => {
                        let amount = amount
                            .parse::<u32>()
                            .map_err(|err| AocError::parse(amount, err))?;

                        match color {
                            "red" => hand.red = amount,
                            "green" => hand.green = amount,
                            "blue" => hand.blue = amount,
                            _ => return Err(AocError::parse(color, "invalid color")),
                        }
                    }
                    None => return Err(AocError::parse(hand_str, "invalid hand")),
                }
            }

            Ok(hand)
        })
        .collect::<Result<_, _>>()?;

    Ok(Game { id, hands })
}

fn max_color_seen(hands: &[Hand], color_extractor: impl Fn(&Hand) -> u32) -> u32 {
    hands.iter().map(&color_extractor).max().unwrap_or(0)
}

impl Solution for Day02 {
    type F = u32;
    type S = u32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day02.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let sum_of_ids = input
            .trim()
            .lines()
            .map(parse)
            .try_fold(0, |acc, parse_result| {
                parse_result.map(|game| {
                    let is_possible = game
                        .hands
                        .iter()
                        .all(|hand| hand.red <= 12 && hand.green <= 13 && hand.blue <= 14);

                    if is_possible {
                        acc + game.id
                    } else {
                        acc
                    }
                })
            })?;

        Ok(sum_of_ids)
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        let sum_of_powers = input
            .trim()
            .lines()
            .map(parse)
            .try_fold(0, |acc, parse_result| {
                parse_result.map(|game| {
                    let red = max_color_seen(&game.hands, |hand| hand.red);
                    let blue = max_color_seen(&game.hands, |hand| hand.blue);
                    let green = max_color_seen(&game.hands, |hand| hand.green);

                    acc + red * blue * green
                })
            })?;

        Ok(sum_of_powers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day02.part_1(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
                Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
                Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
                Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
                Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            Ok(8)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day02.part_2(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
                Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
                Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
                Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
                Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            Ok(2286)
        );
    }
}
