use crate::solution::{AocError, Solution};
use std::collections::HashMap;

pub struct Day08;

fn rotate_left<T>(data: Vec<Vec<T>>) -> Result<Vec<Vec<T>>, AocError> {
    let size = data.len();

    let mut line_iters: Vec<_> = data
        .into_iter()
        .map(|line| line.into_iter().rev())
        .collect();

    (0..size)
        .map(|_| line_iters.iter_mut().map(|line| line.next()).collect())
        .collect::<Option<_>>()
        .ok_or_else(|| AocError::logic("rotation failed"))
}

fn parse(input: &str) -> Result<Vec<Vec<(i32, bool)>>, AocError> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|tree| match tree.to_digit(10) {
                    Some(digit) => Ok((digit as i32, false)),
                    None => Err(AocError::parse(tree, "failed to parse digit")),
                })
                .collect()
        })
        .collect()
}

impl Solution for Day08 {
    type F = i32;
    type S = i32;

    fn meta(&self) -> (u32, u32) {
        (8, 2022)
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/2022/day08.txt")
    }

    fn part_1(&self, input: &str) -> Result<i32, AocError> {
        let mut trees = parse(input)?;
        let mut seen = 0;

        for _ in 0..4 {
            for column in trees.iter_mut() {
                let mut tallest = -1;
                for (tree_height, already_seen) in column.iter_mut() {
                    if *tree_height > tallest {
                        tallest = *tree_height;

                        if !(*already_seen) {
                            *already_seen = true;
                            seen += 1;
                        }
                    }
                }
            }

            trees = rotate_left(trees)?;
        }

        Ok(seen)
    }

    fn part_2(&self, input: &str) -> Result<i32, AocError> {
        let mut trees = parse(input)?;

        // Assume square shaped world
        let width = trees
            .first()
            .ok_or_else(|| AocError::logic("empty world"))?
            .len();

        let mut scores: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

        // Move one by one in each direction, collecting how many trees are seen that way,
        // and store these on each coordinate towards every direction
        for i in 0..4 {
            for y in 0..trees.len() {
                for x in 0..trees[y].len() {
                    let mut t_x = x;
                    let mut t_y = y;

                    // Translate the starting point coordinates by rotation
                    for _ in 0..i {
                        let previous_x = t_x;
                        t_x = t_y;
                        t_y = width - 1 - previous_x;
                    }

                    let mut trees_iter = trees[t_y].iter().skip(t_x);
                    let (starting_point, _) = trees_iter
                        .next()
                        .ok_or_else(|| AocError::logic("no starting point"))?;

                    let mut seen = 0;
                    for (tree_height, _) in trees_iter {
                        seen += 1;
                        if tree_height >= starting_point {
                            break;
                        }
                    }

                    scores.entry((x, y)).or_default().push(seen);
                }
            }

            trees = rotate_left(trees)?;
        }

        scores
            .into_values()
            .map(|values| values.into_iter().product())
            .max()
            .ok_or_else(|| AocError::logic("no best score"))
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
