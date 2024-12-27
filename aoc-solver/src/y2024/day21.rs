use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use itertools::Itertools;

use crate::solution::{AocError, Solution};

type Coords = (isize, isize);
type Keyboard = HashMap<Coords, char>;
type Sequence = Vec<char>;

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

fn parse(input: &str) -> Result<Vec<(Sequence, u64)>, AocError> {
    let codes = input
        .lines()
        .map(|line| {
            let numeric_part = &line[0..3];
            let numeric: u64 = numeric_part
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

fn dijkstra(keys: &Keyboard, start: &Coords, end: &Coords) -> Vec<Vec<Direction>> {
    let mut distances: HashMap<Coords, u32> = HashMap::new();
    let mut heap: BinaryHeap<Search> = BinaryHeap::new();

    heap.push(Search {
        distance: 0,
        position: *start,
        inputs: Vec::new(),
    });

    let mut shortest_sequences = vec![];

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
            shortest_sequences.push(inputs);
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
        }
    }

    shortest_sequences
}

fn numerical_keypad() -> (Keyboard, Coords) {
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

fn directional_keypad() -> (Keyboard, Coords) {
    #[rustfmt::skip]
    let keypad = HashMap::from([
                       ((1, 0), '^'), ((2, 0), 'A'),
        ((0, 1), '<'), ((1, 1), 'v'), ((2, 1), '>'),
    ]);
    let start = (2, 0);

    (keypad, start)
}

// Used to convert the initial numeric code to directional inputs.
// Original part 1 was based on this, still used on before the recursive
// directional inputs search.
fn find_code_inputs(
    code: &[char],
    keys: &HashMap<Coords, char>,
    start: &Coords,
) -> Vec<Vec<Sequence>> {
    let mut parts = vec![];
    let mut current = start;

    for button in code {
        let (target, _) = keys.iter().find(|(_, key)| *key == button).unwrap();

        let shortest_paths = dijkstra(keys, current, target);

        let part = shortest_paths
            .iter()
            .map(|shortest| convert_to_directional(shortest))
            .collect();

        parts.push(part);

        current = target;
    }

    parts
}

fn find_shortest_recursive(
    sequence: &[char],
    keys: &HashMap<Coords, char>,
    start: &Coords,
    remaining_robots: u32,
    cache: &mut HashMap<(String, u32), u64>,
) -> u64 {
    let sequence_key: String = sequence.iter().collect();
    if let Some(cached) = cache.get(&(sequence_key.clone(), remaining_robots)) {
        return *cached;
    }

    let mut current = start;
    let mut count = 0;

    for button in sequence {
        let (target, _) = keys.iter().find(|(_, key)| *key == button).unwrap();
        let shortest_paths = dijkstra(keys, current, target);

        let possible = shortest_paths
            .iter()
            .map(|shortest| convert_to_directional(shortest))
            .collect::<Vec<_>>();

        count += possible
            .iter()
            .map(|part| {
                if remaining_robots > 1 {
                    find_shortest_recursive(part, keys, start, remaining_robots - 1, cache)
                } else {
                    part.len() as u64
                }
            })
            .min()
            .unwrap();

        current = target;
    }

    cache.insert((sequence_key, remaining_robots), count);

    count
}

fn convert_to_directional(shortest: &[Direction]) -> Sequence {
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

fn shortest_sequence(code: &[char], robots: u32, cache: &mut HashMap<(String, u32), u64>) -> u64 {
    let (numpad, numpad_start) = numerical_keypad();
    let (keypad, keypad_start) = directional_keypad();

    let code_inputs = find_code_inputs(code, &numpad, &numpad_start);

    let shortest = code_inputs
        .iter()
        .map(|digit_possible_shortest_sequences| {
            digit_possible_shortest_sequences
                .iter()
                .map(|sequence| {
                    find_shortest_recursive(sequence, &keypad, &keypad_start, robots, cache)
                })
                .min()
                .unwrap()
        })
        .sum();

    shortest
}

pub struct Day21;
impl Solution for Day21 {
    type Part1 = u64;
    type Part2 = u64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day21.txt")
    }

    fn part_1(&self, input: &str) -> Result<u64, AocError> {
        let codes = parse(input)?;
        let mut cache = HashMap::new();

        let complexities = codes
            .into_iter()
            .map(|(code, numeric_part)| shortest_sequence(&code, 2, &mut cache) * numeric_part)
            .sum();

        Ok(complexities)
    }

    fn part_2(&self, input: &str) -> Result<u64, AocError> {
        let codes = parse(input)?;
        let mut cache = HashMap::new();

        let complexities = codes
            .into_iter()
            .map(|(code, numeric_part)| shortest_sequence(&code, 25, &mut cache) * numeric_part)
            .sum();

        Ok(complexities)
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
    fn it_solves_part2_real() {
        assert_eq!(Day21.part_2(Day21.default_input()), Ok(167538833832712));
    }

    #[test]
    fn it_finds_shortest_numpad() {
        let (numpad, start) = numerical_keypad();

        assert_eq!(
            find_code_inputs(&['0', '2', '9', 'A'], &numpad, &start),
            vec![
                vec![vec!['<', 'A']], // A - 0
                vec![vec!['^', 'A']], // 0 - 2
                vec![
                    // 2 - 9
                    vec!['^', '>', '^', 'A'],
                    vec!['^', '^', '>', 'A'],
                    vec!['>', '^', '^', 'A']
                ],
                vec![vec!['v', 'v', 'v', 'A']] // 9 - A
            ]
        );
    }

    #[test]
    fn it_finds_shortest_keypad() {
        let (keypad, start) = directional_keypad();

        assert_eq!(
            find_code_inputs(&['<', 'A'], &keypad, &start),
            vec![
                vec![vec!['v', '<', '<', 'A'], vec!['<', 'v', '<', 'A']], // A - <
                vec![vec!['>', '^', '>', 'A'], vec!['>', '>', '^', 'A']]  // < - A
            ]
        );
    }

    #[test]
    fn it_finds_shortest_keypad_2() {
        let (keypad, start) = directional_keypad();

        assert_eq!(
            find_code_inputs(&['^', '>', '^', 'A'], &keypad, &start),
            vec![
                vec![vec!['<', 'A']],                           // A - ^
                vec![vec!['>', 'v', 'A'], vec!['v', '>', 'A']], // ^ - >
                vec![vec!['^', '<', 'A'], vec!['<', '^', 'A']], // > - ^
                vec![vec!['>', 'A']]                            // ^ - A
            ]
        );
    }

    #[test]
    fn it_finds_shortest_keypad_3() {
        let (keypad, start) = directional_keypad();
        assert_eq!(
            find_code_inputs(&['^', '^', '>', 'A'], &keypad, &start),
            vec![
                vec![vec!['<', 'A']],                           // A - ^
                vec![vec!['A']],                                // ^ - ^
                vec![vec!['>', 'v', 'A'], vec!['v', '>', 'A']], // ^ - >
                vec![vec!['^', 'A']]                            // > - A
            ]
        );
    }

    #[test]
    fn it_finds_min_inputs_length_1() {
        assert_eq!(
            shortest_sequence(&['0', '2', '9', 'A'], 2, &mut HashMap::new()),
            68
        );
    }

    #[test]
    fn it_finds_min_inputs_length_2() {
        assert_eq!(
            shortest_sequence(&['9', '8', '0', 'A'], 2, &mut HashMap::new()),
            60
        );
    }

    #[test]
    fn it_finds_min_inputs_length_3() {
        assert_eq!(
            shortest_sequence(&['1', '7', '9', 'A'], 2, &mut HashMap::new()),
            68
        );
    }

    #[test]
    fn it_finds_min_inputs_length_4() {
        assert_eq!(
            shortest_sequence(&['4', '5', '6', 'A'], 2, &mut HashMap::new()),
            64
        );
    }

    #[test]
    fn it_finds_min_inputs_length_5() {
        assert_eq!(
            shortest_sequence(&['3', '7', '9', 'A'], 2, &mut HashMap::new()),
            64
        );
    }
}
