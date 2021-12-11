use std::collections::HashMap;
use std::collections::HashSet;

const INPUT_FILE: &str = include_str!("../../inputs/day11.txt");

fn parse(input: &str) -> HashMap<(i32, i32), u32> {
    let mut octopuses = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, energy_level) in line.chars().enumerate() {
            if let Some(energy_level) = energy_level.to_digit(10) {
                octopuses.insert((x as i32, y as i32), energy_level);
            }
        }
    }
    octopuses
}

fn simulate_step(mut octopuses: HashMap<(i32, i32), u32>) -> (HashMap<(i32, i32), u32>, usize) {
    let mut flashed: HashSet<(i32, i32)> = HashSet::new();

    for energy_level in octopuses.values_mut() {
        *energy_level += 1;
    }

    loop {
        let mut some_octopus_flashed = false;

        for ((x, y), energy_level) in octopuses.clone() {
            // Then, any octopus with an energy level greater than 9 flashes.
            if energy_level > 9 {
                // An octopus can only flash at most once per step.
                if flashed.insert((x, y)) {
                    some_octopus_flashed = true;

                    // This increases the energy level of all adjacent octopuses by 1,
                    // including octopuses that are diagonally adjacent.
                    // If this causes an octopus to have an energy level greater than 9, it also flashes.
                    #[rustfmt::skip]
                    let offsets = [
                        (-1, 1), (0, 1), (1, 1),
                        (-1, 0),         (1, 0),
                        (-1,-1), (0,-1), (1,-1),
                    ];

                    for (offset_x, offset_y) in offsets {
                        if let Some(neighbour) = octopuses.get_mut(&(x + offset_x, y + offset_y)) {
                            *neighbour += 1;
                        }
                    }
                }
            }
        }

        // This process continues as long as new octopuses keep having their energy level increased beyond 9.
        if !some_octopus_flashed {
            break;
        }
    }

    // Finally, any octopus that flashed during this step has its energy level set to 0, as
    // it used all of its energy to flash.
    for coords in flashed.iter() {
        if let Some(octopus) = octopuses.get_mut(coords) {
            *octopus = 0;
        }
    }

    (octopuses, flashed.len())
}

fn part_1(input: &str, steps: usize) -> usize {
    let mut octopuses = parse(input);

    let mut total_flashes = 0;
    for _ in 0..steps {
        let (new_octopuses, flashes) = simulate_step(std::mem::take(&mut octopuses));
        octopuses = new_octopuses;
        total_flashes += flashes;
    }

    total_flashes
}

fn part_2(input: &str) -> usize {
    let mut octopuses = parse(input);

    let mut steps = 1;

    loop {
        let (new_octopuses, flashes) = simulate_step(std::mem::take(&mut octopuses));
        if flashes == new_octopuses.len() {
            return steps;
        }

        octopuses = new_octopuses;
        steps += 1;
    }
}

fn main() {
    let part_1_result = part_1(INPUT_FILE, 100);
    println!("[INFO]: Part 1: {:?}", part_1_result);

    let part_2_result = part_2(INPUT_FILE);
    println!("[INFO]: Part 2: {:?}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_small_example() {
        assert_eq!(
            part_1(
                "11111\n\
                 19991\n\
                 19191\n\
                 19991\n\
                 11111",
                2
            ),
            9
        );
    }

    #[test]
    fn it_solves_part1_large_example() {
        assert_eq!(
            part_1(
                "5483143223\n\
                 2745854711\n\
                 5264556173\n\
                 6141336146\n\
                 6357385478\n\
                 4167524645\n\
                 2176841721\n\
                 6882881134\n\
                 4846848554\n\
                 5283751526",
                100
            ),
            1656
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            part_2(
                "5483143223\n\
                 2745854711\n\
                 5264556173\n\
                 6141336146\n\
                 6357385478\n\
                 4167524645\n\
                 2176841721\n\
                 6882881134\n\
                 4846848554\n\
                 5283751526"
            ),
            195
        );
    }
}
