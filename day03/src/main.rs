extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry;

const INPUT_FILE: &str = include_str!("../input.txt");

struct Rectangle {
    id: String,
    x: u32,
    y: u32,
    h: u32,
    w: u32
}

fn init_rectangles(file: &str) -> Vec<Rectangle> {
    let mut rectangles: Vec<Rectangle> = Vec::with_capacity(file.lines().count());

    for line in file.lines() {
        let input_regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();

        for capture in input_regex.captures_iter(line) {
            let rect = Rectangle{
                id: String::from(&capture[1]),
                x: capture[2].parse::<u32>().unwrap(),
                y: capture[3].parse::<u32>().unwrap(),
                w: capture[4].parse::<u32>().unwrap(),
                h: capture[5].parse::<u32>().unwrap(),
            };

            rectangles.push(rect);
        }
    }

    rectangles
}

fn part_1(file: &str) -> usize {
    let rectangles = init_rectangles(file);

    let mut fabric = HashMap::new();

    for rect in &rectangles {
        for x in rect.x..rect.x+rect.w {
            for y in rect.y..rect.y+rect.h {
                *fabric.entry((x,y)).or_insert(0) += 1;
            }
        }
    }

    fabric.values().filter(|f| **f >= 2).count()
}

fn part_2(file: &str) -> String {
    let rectangles = init_rectangles(file);

    let mut all_rectangles: HashSet<String> = HashSet::new();
    let mut intersecting_rectangles: HashSet<String> = HashSet::new();
    let mut claimed_coordinates: HashMap<(u32, u32), String> = HashMap::new();

    for rect in &rectangles {
        all_rectangles.insert(rect.id.clone());

        for x in rect.x..rect.x+rect.w {
            for y in rect.y..rect.y+rect.h {
                match claimed_coordinates.entry((x,y)) {
                    Entry::Vacant(entry) => { entry.insert(rect.id.clone()); }, // Mark owned
                    Entry::Occupied(mut entry) => {
                        // Already owned by entry, save both as intersecting
                        intersecting_rectangles.insert(entry.get().clone());
                        intersecting_rectangles.insert(rect.id.clone());
                    }
                }
            }
        }
    }

    all_rectangles
        .difference(&intersecting_rectangles)
        .next() // There should be only one result
        .expect("No results")
        .clone()
}

fn main() {
    let part1_result = part_1(INPUT_FILE);
    let part2_result = part_2(INPUT_FILE);

    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_FILE: &str = include_str!("../test/example.txt");

    #[test]
    fn it_solves_day02_part1_example() {
        assert_eq!(part_1(TEST_FILE), 4);
    }

    #[test]
    fn it_solves_day02_part2_example() {
        assert_eq!(part_2(TEST_FILE), String::from("3"));
    }
}
