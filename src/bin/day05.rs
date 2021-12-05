const INPUT_FILE: &str = include_str!("../../inputs/day05.txt");

use std::cmp::Ordering;
use std::collections::HashMap;

use aoc::Coords;

fn parse(input: &str) -> Vec<(Coords<usize>, Coords<usize>)> {
    input
        .lines()
        .map(|line| {
            let mut coords = line.split(" -> ");
            let mut start = coords.next().unwrap().split(',');
            let start_x = start.next().unwrap().parse::<usize>().unwrap();
            let start_y = start.next().unwrap().parse::<usize>().unwrap();

            let mut end = coords.next().unwrap().split(',');
            let end_x = end.next().unwrap().parse::<usize>().unwrap();
            let end_y = end.next().unwrap().parse::<usize>().unwrap();

            (
                Coords { x: start_x, y: start_y },
                Coords { x: end_x, y: end_y },
            )
        })
        .collect()
}

fn solve(input: &str, is_allow_diagonal: bool) -> usize {
    let mut hydrothermal_vents: HashMap<(usize, usize), usize> = HashMap::new();

    for (start, end) in parse(input) {
        if !is_allow_diagonal && start.x != end.x && start.y != end.y {
            continue;
        }

        let range_x: Box<dyn Iterator<Item = usize>> =
            match start.x.cmp(&end.x) {
                Ordering::Greater => Box::new((end.x..=start.x).rev()),
                Ordering::Less => Box::new(start.x..=end.x),
                Ordering::Equal => Box::new(std::iter::repeat(start.x)),
            };

        let range_y: Box<dyn Iterator<Item = usize>> =
            match start.y.cmp(&end.y) {
                Ordering::Greater => Box::new((end.y..=start.y).rev()),
                Ordering::Less => Box::new(start.y..=end.y),
                Ordering::Equal => Box::new(std::iter::repeat(start.y)),
            };

        for coords in range_x.zip(range_y) {
            let count = hydrothermal_vents.entry(coords).or_insert(0);
            *count += 1;
        }
    }

    hydrothermal_vents
        .into_values()
        .filter(|count| *count > 1)
        .count()
}

fn main() {
    let part_1_result = solve(INPUT_FILE, false);
    println!("[INFO]: Part 1: {:?}", part_1_result);

    let part_2_result = solve(INPUT_FILE, true);
    println!("[INFO]: Part 2: {:?}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            solve(
                "0,9 -> 5,9\n\
                8,0 -> 0,8\n\
                9,4 -> 3,4\n\
                2,2 -> 2,1\n\
                7,0 -> 7,4\n\
                6,4 -> 2,0\n\
                0,9 -> 2,9\n\
                3,4 -> 1,4\n\
                0,0 -> 8,8\n\
                5,5 -> 8,2",
                false
            ),
            5
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            solve(
                "0,9 -> 5,9\n\
                8,0 -> 0,8\n\
                9,4 -> 3,4\n\
                2,2 -> 2,1\n\
                7,0 -> 7,4\n\
                6,4 -> 2,0\n\
                0,9 -> 2,9\n\
                3,4 -> 1,4\n\
                0,0 -> 8,8\n\
                5,5 -> 8,2",
                true
            ),
            12
        );
    }
}
