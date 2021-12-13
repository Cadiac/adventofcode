use std::collections::HashSet;

const INPUT_FILE: &str = include_str!("../../inputs/day13.txt");

use aoc::Coords;

#[derive(Debug)]
enum Fold {
    X(i32),
    Y(i32),
}

fn parse(input: &str) -> (HashSet<Coords<i32>>, Vec<Fold>) {
    let mut dots: HashSet<Coords<i32>> = HashSet::new();
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

fn fold_paper(dots: &mut HashSet<Coords<i32>>, fold: Fold) {
    let new_dots: Vec<Coords<i32>> = match fold {
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

fn part_1(input: &str) -> usize {
    let (mut dots, folds) = parse(input);

    folds
        .into_iter()
        .take(1)
        .for_each(|fold| fold_paper(&mut dots, fold));

    dots.len()
}

fn part_2(input: &str) {
    let (mut dots, folds) = parse(input);

    folds
        .into_iter()
        .for_each(|fold| fold_paper(&mut dots, fold));

    println!("[INFO]: Part 2:");
    for y in 0..6 {
        for x in 0..40 {
            if dots.contains(&Coords { x, y }) {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    let part_1_result = part_1(INPUT_FILE);
    println!("[INFO]: Part 1: {:?}", part_1_result);

    part_2(INPUT_FILE);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example1() {
        assert_eq!(
            part_1(
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
            17
        );
    }
}
