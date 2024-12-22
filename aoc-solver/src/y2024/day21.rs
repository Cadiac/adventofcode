use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use itertools::Itertools;

use crate::solution::{AocError, Solution};

type Coords = (isize, isize);
type Keyboard = HashMap<Coords, char>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

impl Direction {
    fn as_delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

fn parse(input: &str) -> Result<Vec<(Vec<char>, u32)>, AocError> {
    let codes = input
        .lines()
        .map(|line| {
            let numeric_part = &line[0..3];
            let numeric: u32 = numeric_part
                .parse()
                .map_err(|err| AocError::parse(numeric_part, err))?;
            let code = line.chars().collect();

            Ok((code, numeric))
        })
        .try_collect()?;

    Ok(codes)
}

#[derive(Clone, Eq, PartialEq)]
struct Search {
    distance: u32,
    position: Coords,
    inputs: Vec<Direction>,
}

impl Ord for Search {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Search {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Robot {
    keys: HashMap<Coords, char>,
    current: Coords,
}

fn create_numerical_keypad() -> (Keyboard, Coords) {
    #[rustfmt::skip]
    let keypad = HashMap::from([
        ((0, 0), '7'), ((1, 0), '8'), ((2, 0), '9'),
        ((0, 1), '4'), ((1, 1), '5'), ((2, 1), '6'),
        ((0, 2), '1'), ((1, 2), '2'), ((2, 2), '3'),
                       ((1, 3), '0'), ((2, 3), 'A'),
    ]);
    let start = (2, 3);

    (keypad, start)
}

fn create_directional_keypad() -> (Keyboard, Coords) {
    #[rustfmt::skip]
    let keypad = HashMap::from([
                       ((1, 0), '^'), ((2, 0), 'A'),
        ((0, 1), '<'), ((1, 1), 'v'), ((2, 1), '>'),
    ]);
    let start = (2, 0);

    (keypad, start)
}

fn find_shortest(keys: &Keyboard, start: &Coords, end: &Coords) -> Vec<Vec<Direction>> {
    let mut distances: HashMap<Coords, u32> = HashMap::new();
    let mut heap: BinaryHeap<Search> = BinaryHeap::new();

    heap.push(Search {
        distance: 0,
        position: *start,
        inputs: Vec::new(),
    });

    let mut shortest = vec![];

    while let Some(Search {
        position,
        distance,
        inputs,
    }) = heap.pop()
    {
        if distance > *distances.get(&position).unwrap_or(&u32::MAX) {
            continue;
        }

        if position == *end {
            shortest.push(inputs);
            continue;
        }

        for direction in DIRECTIONS {
            let (dx, dy) = direction.as_delta();
            let next_position = (position.0 + dx, position.1 + dy);

            if !keys.contains_key(&next_position) {
                continue;
            }

            let next_distance = distance + 1;
            let best_known = distances.entry(next_position).or_insert(u32::MAX);

            if next_distance <= *best_known {
                *best_known = next_distance;

                let mut next_inputs = inputs.clone();
                next_inputs.push(direction);

                heap.push(Search {
                    position: next_position,
                    distance: next_distance,
                    inputs: next_inputs,
                });
            }

            distances.insert(position, distance);

            if keys.contains_key(&next_position) && !distances.contains_key(&next_position) {}
        }
    }

    shortest
}

fn combine_parts(parts: &[Vec<Vec<char>>]) -> Vec<Vec<char>> {
    let mut combinations = vec![vec![]];

    for group in parts {
        let mut new_combinations = Vec::new();

        for combo_so_far in &combinations {
            for option in group {
                let mut combined = combo_so_far.clone();
                combined.extend_from_slice(option);
                new_combinations.push(combined);
            }
        }

        combinations = new_combinations;
    }

    combinations
}

fn complexity(shortest: u32, numeric_part: u32) -> u32 {
    shortest * numeric_part
}

impl Robot {
    fn new(keys: Keyboard, current: Coords) -> Self {
        Self { keys, current }
    }

    fn find_inputs_to_produce(&self, sequence: &[char]) -> Vec<Vec<char>> {
        let mut parts = vec![];
        let mut current = self.current;

        for button in sequence {
            let (target, _key) = self.keys.iter().find(|(_, key)| *key == button).unwrap();
            let shortest_paths = find_shortest(&self.keys, &current, target);

            let part = shortest_paths
                .iter()
                .map(|shortest| convert_to_directional(shortest))
                .collect();

            parts.push(part);

            current = *target;
        }

        combine_parts(&parts)
    }
}

fn convert_to_directional(shortest: &[Direction]) -> Vec<char> {
    let mut input = vec![];
    for direction in shortest {
        input.push(match direction {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        });
    }
    input.push('A');
    input
}

fn find_shortest_inputs(sequence: &[char]) -> Vec<char> {
    let (numpad, numpad_start) = create_numerical_keypad();
    let (keypad, keypad_start) = create_directional_keypad();

    let numpad_robot = Robot::new(numpad, numpad_start);
    let keypad_robot_1 = Robot::new(keypad.clone(), keypad_start);
    let keypad_robot_2 = Robot::new(keypad, keypad_start);

    let numpad_inputs = numpad_robot.find_inputs_to_produce(sequence);

    let shortest: Vec<_> = numpad_inputs
        .iter()
        .flat_map(|numpad_input| keypad_robot_1.find_inputs_to_produce(numpad_input))
        .flat_map(|keypad_input| keypad_robot_2.find_inputs_to_produce(&keypad_input))
        .min_by(|a, b| a.len().cmp(&b.len()))
        .unwrap_or(Vec::new());

    shortest
}

pub struct Day21;
impl Solution for Day21 {
    type A = u32;
    type B = u32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day21.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let codes = parse(input)?;

        let complexities = codes
            .into_iter()
            .map(|(code, numeric_part)| {
                let shortest = find_shortest_inputs(&code).len() as u32;
                complexity(shortest, numeric_part)
            })
            .sum();

        Ok(complexities)
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const INPUT: &str =
        "029A\n\
         980A\n\
         179A\n\
         456A\n\
         379A";

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(Day21.part_1(INPUT), Ok(126384));
    }

    #[test]
    fn it_finds_shortest_numpad() {
        let (keypad, a) = create_numerical_keypad();

        let robot = Robot::new(keypad, a);

        assert_eq!(
            robot.find_inputs_to_produce(&['0', '2', '9', 'A']),
            vec![
                "<A^A^>^AvvvA".chars().collect::<Vec<_>>(),
                "<A^A^^>AvvvA".chars().collect::<Vec<_>>(),
                "<A^A>^^AvvvA".chars().collect::<Vec<_>>(),
            ]
        );
    }

    #[test]
    fn it_finds_min_inputs_length_1() {
        assert_eq!(find_shortest_inputs(&['0', '2', '9', 'A']).len(), 68);
    }

    #[test]
    fn it_finds_min_inputs_length_2() {
        assert_eq!(find_shortest_inputs(&['9', '8', '0', 'A']).len(), 60);
    }

    #[test]
    fn it_finds_min_inputs_length_3() {
        assert_eq!(find_shortest_inputs(&['1', '7', '9', 'A']).len(), 68);
    }

    #[test]
    fn it_finds_min_inputs_length_4() {
        assert_eq!(find_shortest_inputs(&['4', '5', '6', 'A']).len(), 64);
    }

    #[test]
    fn it_finds_min_inputs_length_5() {
        assert_eq!(find_shortest_inputs(&['3', '7', '9', 'A']).len(), 64);
    }
}
