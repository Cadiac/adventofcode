use std::collections::HashMap;

use crate::solution::{AocError, Solution};

pub struct Day03;

#[rustfmt::skip]
const NEIGHBOUR_OFFSETS: [(i32, i32); 8] = [
    (-1,-1), (0,-1), (1,-1),
    (-1, 0),         (1, 0),
    (-1, 1), (0, 1), (1, 1),
];

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

type Symbols = HashMap<Coord, char>;
type Numbers = HashMap<(Coord, Coord), u32>;

fn parse(input: &str) -> Result<(Symbols, Numbers), AocError> {
    let mut symbols: Symbols = HashMap::new();
    let mut numbers: Numbers = HashMap::new();

    let mut current_number = vec![];
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_alphanumeric() {
                current_number.push(c);
            }

            if (!c.is_ascii_alphanumeric() || x == line.len() - 1) && !current_number.is_empty() {
                let start = (x - current_number.len()) as i32;
                let end = x as i32 - 1;

                let number = current_number
                    .into_iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();

                numbers.insert(
                    (
                        Coord {
                            x: start,
                            y: y as i32,
                        },
                        Coord {
                            x: end,
                            y: y as i32,
                        },
                    ),
                    number,
                );

                current_number = vec![];
            }

            if !c.is_ascii_alphanumeric() && c != '.' {
                symbols.insert(
                    Coord {
                        x: x as i32,
                        y: y as i32,
                    },
                    c,
                );
            }
        }
    }
    Ok((symbols, numbers))
}

impl Solution for Day03 {
    type F = u32;
    type S = u32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day03.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let (symbols, numbers) = parse(input)?;

        let sum = numbers
            .iter()
            .filter(|((start, end), _)| {
                (start.x..=end.x).any(|x| {
                    NEIGHBOUR_OFFSETS.iter().any(|(dx, dy)| {
                        symbols.contains_key(&Coord {
                            x: x as i32 + dx,
                            y: start.y + dy,
                        })
                    })
                })
            })
            .map(|(_, number)| number)
            .sum();

        Ok(sum)
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        let (symbols, numbers) = parse(input)?;

        let sum = symbols
            .into_iter()
            .filter(|(_, symbol)| *symbol == '*')
            .filter_map(|(coords, _)| {
                let adjacent_numbers: Vec<u32> = numbers
                    .iter()
                    .filter_map(|((start, end), number)| {
                        for (nx, ny) in NEIGHBOUR_OFFSETS {
                            let has_adjacent_number = coords.y + ny == start.y
                                && coords.x + nx >= start.x
                                && coords.x + nx <= end.x;

                            if has_adjacent_number {
                                return Some(*number);
                            }
                        }

                        None
                    })
                    .collect();

                if adjacent_numbers.len() == 2 {
                    return Some(adjacent_numbers.iter().product::<u32>());
                }

                None
            })
            .sum();

        Ok(sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day03.part_1(
                "467..114..\n\
                 ...*......\n\
                 ..35..633.\n\
                 ......#...\n\
                 617*......\n\
                 .....+.58.\n\
                 ..592.....\n\
                 ......755.\n\
                 ...$.*....\n\
                 .664.598.."
            ),
            Ok(4361)
        );
    }

    #[test]
    fn it_solves_part1_example_advanced() {
        assert_eq!(
            Day03.part_1(
                "467..114..\n\
                 ...*......\n\
                 ..35......\n\
                 .......#63\n\
                 617*12....\n\
                 .....+.58.\n\
                 ..592.....\n\
                 ......755.\n\
                 ...$.*....\n\
                 .664.598.."
            ),
            Ok(3803)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day03.part_2(
                "467..114..\n\
                 ...*......\n\
                 ..35..633.\n\
                 ......#...\n\
                 617*......\n\
                 .....+.58.\n\
                 ..592.....\n\
                 ......755.\n\
                 ...$.*....\n\
                 .664.598.."
            ),
            Ok(467835)
        );
    }
}
