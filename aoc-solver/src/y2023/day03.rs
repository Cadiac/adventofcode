use std::collections::HashMap;

use crate::{
    solution::{AocError, Solution},
    utils::Coords,
};

pub struct Day03;

#[rustfmt::skip]
const NEIGHBOUR_OFFSETS: [(i32, i32); 8] = [
    (-1,-1), (0,-1), (1,-1),
    (-1, 0),         (1, 0),
    (-1, 1), (0, 1), (1, 1),
];

type Symbols = HashMap<Coords<usize>, char>;
type Numbers = HashMap<(Coords<usize>, Coords<usize>), u32>;

fn parse(input: &str) -> Result<(Symbols, Numbers), AocError> {
    let mut symbols: Symbols = HashMap::new();
    let mut numbers: Numbers = HashMap::new();

    for (y, line) in input.trim().lines().enumerate() {
        let mut current_number = vec![];

        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_alphanumeric() {
                current_number.push(c);
            } else if c != '.' {
                symbols.insert(Coords { x, y }, c);
            }

            if !current_number.is_empty() && (!c.is_ascii_alphanumeric() || x == line.len() - 1) {
                let start = x - current_number.len();
                let end = x - 1;

                let number = current_number
                    .into_iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .map_err(|err| AocError::parse(line, err))?;

                numbers.insert((Coords { x: start, y }, Coords { x: end, y }), number);

                current_number = vec![];
            }
        }
    }
    Ok((symbols, numbers))
}

impl Solution for Day03 {
    type A = u32;
    type B = u32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day03.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let (symbols, numbers) = parse(input)?;

        let sum = numbers
            .iter()
            .filter(|((start, end), _)| {
                for y in start.y.saturating_sub(1)..=(end.y + 1) {
                    for x in start.x.saturating_sub(1)..=(end.x + 1) {
                        if symbols.contains_key(&Coords { x, y }) {
                            return true;
                        }
                    }
                }

                false
            })
            .map(|(_, number)| number)
            .sum();

        Ok(sum)
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        let (symbols, numbers) = parse(input)?;

        let sum = symbols
            .into_iter()
            .filter_map(|(coords, symbol)| {
                if symbol != '*' {
                    return None;
                }

                let x = coords.x as i32;
                let y = coords.y as i32;

                let adjacent_numbers: Vec<u32> = numbers
                    .iter()
                    .filter_map(|((start, end), number)| {
                        for (dx, dy) in NEIGHBOUR_OFFSETS {
                            let has_adjacent_number = y + dy == start.y as i32
                                && x + dx >= start.x as i32
                                && x + dx <= end.x as i32;

                            if has_adjacent_number {
                                return Some(*number);
                            }
                        }

                        None
                    })
                    .collect();

                if adjacent_numbers.len() != 2 {
                    return None;
                }

                Some(adjacent_numbers.iter().product::<u32>())
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
    fn it_solves_part1_example_special_cases() {
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
