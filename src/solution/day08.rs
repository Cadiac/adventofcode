use crate::solution::{AocError, Solution};

pub struct Day08;

fn rotate_left<T>(data: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let size = data.len();

    let mut line_iters: Vec<_> = data
        .into_iter()
        .map(|line| line.into_iter().rev())
        .collect();

    (0..size)
        .map(|_| {
            line_iters
                .iter_mut()
                .map(|line| line.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn parse(input: &str) -> Vec<Vec<(i32, bool)>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|tree| (tree.to_digit(10).unwrap() as i32, false))
                .collect()
        })
        .collect()
}

impl Solution for Day08 {
    type F = i32;
    type S = i32;

    fn name(&self) -> &'static str {
        "Day 08"
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/day08.txt")
    }

    fn part_1(&self, input: &str) -> Result<i32, AocError> {
        let mut trees = parse(input);
        let mut seen = 0;

        for _ in 0..4 {
            for column in trees.iter_mut() {
                let mut tallest = -1;
                for (tree_height, already_seen) in column.iter_mut() {
                    if *tree_height > tallest {
                        tallest = *tree_height;

                        if *already_seen == false {
                            seen += 1;
                            *already_seen = true;
                        }
                    }
                }
            }

            trees = rotate_left(trees);
        }

        Ok(seen)
    }

    fn part_2(&self, input: &str) -> Result<i32, AocError> {
        let mut trees = parse(input);
        let mut best_score = 0;

        for y in 0..trees.len() {
            for x in 0..trees[y].len() {
                let starting_point = trees[y][x].0;

                let mut scores = Vec::new();

                // Offset translations of x and y in all rotations
                let offsets = [
                    (x, y),
                    (y, trees[y].len() - 1 - x),
                    (trees[y].len() - 1 - x, trees.len() - 1 - y),
                    (trees.len() - 1 - y, x)
                ];

                for (offset_x, offset_y) in offsets {

                    let mut seen = 0;

                    for (tree_height, _) in trees[offset_y].iter_mut().skip(offset_x + 1) {
                        seen += 1;
                        if *tree_height >= starting_point {
                            break;
                        }
                    }
        
                    scores.push(seen);
        
                    trees = rotate_left(trees);
                }

                let total_score = scores.into_iter().reduce(|acc, cur| acc * cur).unwrap();

                if total_score > best_score {
                    best_score = total_score;
                }
            }
        }

        Ok(best_score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day08.part_1(
                "30373\n\
                25512\n\
                65332\n\
                33549\n\
                35390"
            ),
            Ok(21)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day08.part_2(
                "30373\n\
                25512\n\
                65332\n\
                33549\n\
                35390"
            ),
            Ok(8)
        );
    }
}
