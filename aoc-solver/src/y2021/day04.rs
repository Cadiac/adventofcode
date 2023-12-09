use std::collections::HashMap;
use std::collections::HashSet;

use crate::solution::{AocError, Solution};

pub struct Day04;

type Board = HashMap<usize, (usize, usize)>;

fn is_winner(board: &Board, drawn: &HashSet<usize>, last_drawn: usize) -> bool {
    if let Some((last_x, last_y)) = board.get(&last_drawn) {
        let drawn_coords = drawn.iter().flat_map(|number| board.get(number));

        return drawn_coords.clone().filter(|(x, _y)| x == last_x).count() == 5
            || drawn_coords.filter(|(_x, y)| y == last_y).count() == 5;
    }

    false
}

fn find_unmarked_numbers(board: &Board, drawn: &HashSet<usize>) -> Vec<usize> {
    board
        .keys()
        .filter(|number| !drawn.contains(number))
        .cloned()
        .collect()
}

fn parse(input: &str) -> (Vec<usize>, Vec<Board>) {
    let mut input_chunks = input.split("\n\n");
    let numbers_input: Vec<usize> = input_chunks
        .next()
        .unwrap()
        .split(',')
        .map(|number| number.parse::<usize>().unwrap())
        .collect();

    let boards: Vec<Board> = input_chunks
        .map(|board_input| {
            let mut board: Board = HashMap::new();

            for (y, line) in board_input.lines().enumerate() {
                for (x, number) in line.split_whitespace().enumerate() {
                    let parsed_num = number.parse::<usize>().unwrap();

                    board.insert(parsed_num, (x, y));
                }
            }

            board
        })
        .collect();

    (numbers_input, boards)
}

impl Solution for Day04 {
    type A = usize;
    type B = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2021/day04.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let (numbers_input, boards) = parse(input);
        let mut drawn: HashSet<usize> = HashSet::new();

        for number in numbers_input {
            drawn.insert(number);

            for board in boards.iter() {
                if is_winner(board, &drawn, number) {
                    let unmarked = find_unmarked_numbers(board, &drawn);
                    return Ok(unmarked.iter().sum::<usize>() * number);
                }
            }
        }

        Err(AocError::logic("No winners!"))
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let (numbers_input, mut boards) = parse(input);
        let mut drawn: HashSet<usize> = HashSet::new();

        for number in numbers_input {
            drawn.insert(number);

            let (winners, remaining): (Vec<Board>, Vec<Board>) = boards
                .into_iter()
                .partition(|board| is_winner(board, &drawn, number));

            if remaining.is_empty() {
                // No more boards playing, the last board just won.
                // Assume that there was exactly one winner
                let winner = winners.first().expect("at least one winner");
                let unmarked = find_unmarked_numbers(winner, &drawn);
                return Ok(unmarked.iter().sum::<usize>() * number);
            }

            boards = remaining;
        }

        Err(AocError::logic("No winners!"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day04.part_1(
                "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\
                \n\
                22 13 17 11  0\n\
                 8  2 23  4 24\n\
                21  9 14 16  7\n\
                 6 10  3 18  5\n\
                 1 12 20 15 19\n\
                \n\
                 3 15  0  2 22\n\
                 9 18 13 17  5\n\
                19  8  7 25 23\n\
                20 11 10 24  4\n\
                14 21 16 12  6\n\
                \n\
                14 21 17 24  4\n\
                10 16 15  9 19\n\
                18  8 23 26 20\n\
                22 11 13  6  5\n\
                 2  0 12  3  7"
            ),
            Ok(4512)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day04.part_2(
                "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\
                \n\
                22 13 17 11  0\n\
                 8  2 23  4 24\n\
                21  9 14 16  7\n\
                 6 10  3 18  5\n\
                 1 12 20 15 19\n\
                \n\
                 3 15  0  2 22\n\
                 9 18 13 17  5\n\
                19  8  7 25 23\n\
                20 11 10 24  4\n\
                14 21 16 12  6\n\
                \n\
                14 21 17 24  4\n\
                10 16 15  9 19\n\
                18  8 23 26 20\n\
                22 11 13  6  5\n\
                 2  0 12  3  7"
            ),
            Ok(1924)
        );
    }
}
