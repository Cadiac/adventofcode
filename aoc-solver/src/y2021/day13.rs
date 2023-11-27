use std::collections::HashSet;

use crate::solution::{AocError, Solution};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

pub struct Day13;

#[derive(Debug)]
enum Fold {
    X(i32),
    Y(i32),
}

fn parse(input: &str) -> (HashSet<Coords>, Vec<Fold>) {
    let mut dots: HashSet<Coords> = HashSet::new();
    let mut folds: Vec<Fold> = Vec::new();

    let input_parts: Vec<&str> = input.split("\n\n").collect();

    assert_eq!(input_parts.len(), 2);

    let coords_input = input_parts[0];
    let folds_input = input_parts[1];

    for line in coords_input.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        assert_eq!(parts.len(), 2);

        dots.insert(Coords {
            x: parts[0].parse::<i32>().unwrap(),
            y: parts[1].parse::<i32>().unwrap(),
        });
    }

    for line in folds_input.lines() {
        let parts: Vec<&str> = line.split("fold along ").collect();
        assert_eq!(parts.len(), 2);

        let fold_parts: Vec<&str> = parts[1].split('=').collect();
        assert_eq!(fold_parts.len(), 2);

        let fold = fold_parts[1].parse::<i32>().unwrap();

        if fold_parts[0] == "x" {
            folds.push(Fold::X(fold));
        } else if fold_parts[0] == "y" {
            folds.push(Fold::Y(fold));
        }
    }

    (dots, folds)
}

fn fold_paper(dots: &mut HashSet<Coords>, fold: Fold) {
    let new_dots: Vec<Coords> = match fold {
        Fold::X(fold_x) => {
            let folded_new = dots
                .iter()
                .filter(|dot| dot.x > fold_x)
                .map(|dot| {
                    let distance_to_fold = dot.x - fold_x;
                    Coords {
                        x: fold_x - distance_to_fold,
                        y: dot.y,
                    }
                })
                .collect();

            // Drop any dots at the fold or after it
            dots.retain(|dot| dot.x < fold_x);

            folded_new
        }
        Fold::Y(fold_y) => {
            let folded_new = dots
                .iter()
                .filter(|dot| dot.y > fold_y)
                .map(|dot| {
                    let distance_to_fold = dot.y - fold_y;
                    Coords {
                        x: dot.x,
                        y: fold_y - distance_to_fold,
                    }
                })
                .collect();

            // Drop any dots at the fold or after it
            dots.retain(|dot| dot.y < fold_y);

            folded_new
        }
    };

    for dot in new_dots {
        dots.insert(dot);
    }
}

impl Solution for Day13 {
    type F = usize;
    type S = String;

    fn meta(&self) -> (u32, u32) {
        (13, 2021)
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2021/day13.txt")
    }

    fn part_1(&self, input: &str) -> Result<Self::F, AocError> {
        let (mut dots, folds) = parse(input);

        folds
            .into_iter()
            .take(1)
            .for_each(|fold| fold_paper(&mut dots, fold));

        Ok(dots.len())
    }

    fn part_2(&self, input: &str) -> Result<Self::S, AocError> {
        let (mut dots, folds) = parse(input);

        folds
            .into_iter()
            .for_each(|fold| fold_paper(&mut dots, fold));

        let mut output: Vec<char> = vec!['\n'];

        for y in 0..6 {
            for x in 0..40 {
                if dots.contains(&Coords { x, y }) {
                    output.push('█');
                } else {
                    output.push(' ');
                }
            }
            output.push('\n');
        }

        Ok(output.iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example1() {
        assert_eq!(
            Day13.part_1(
                "6,10\n\
                 0,14\n\
                 9,10\n\
                 0,3\n\
                 10,4\n\
                 4,11\n\
                 6,0\n\
                 6,12\n\
                 4,1\n\
                 0,13\n\
                 10,12\n\
                 3,4\n\
                 3,0\n\
                 8,4\n\
                 1,10\n\
                 2,14\n\
                 8,10\n\
                 9,0\n\n\
                 fold along y=7\n\
                 fold along x=5",
            ),
            Ok(17)
        );
    }
}
