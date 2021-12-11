use std::collections::HashSet;

const INPUT_FILE: &str = include_str!("../../inputs/day11.txt");

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|energy_level| energy_level.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn simulate_step(mut octopuses: Vec<Vec<u32>>) -> (Vec<Vec<u32>>, usize) {
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();

    for row in &mut octopuses {
        for octopus in row {
            // First, the energy level of each octopus increases by 1.
            *octopus += 1;
        }
    }

    loop {
        let mut some_octopus_flashed = false;

        for y in 0..octopuses.len() {
            for x in 0..octopuses[y].len() {
                // Then, any octopus with an energy level greater than 9 flashes.
                if octopuses[y][x] > 9 {
                    // An octopus can only flash at most once per step.
                    if flashed.insert((x, y)) {
                        some_octopus_flashed = true;

                        // This increases the energy level of all adjacent octopuses by 1,
                        // including octopuses that are diagonally adjacent.
                        // If this causes an octopus to have an energy level greater than 9, it also flashes.

                        if y > 0 {
                            octopuses[y - 1][x] += 1;
                            if x > 0 {
                                octopuses[y - 1][x - 1] += 1;
                            }
                            if x + 1 < octopuses[y - 1].len() {
                                octopuses[y - 1][x + 1] += 1;
                            }
                        }

                        if y + 1 < octopuses.len() {
                            octopuses[y + 1][x] += 1;
                            if x > 0 {
                                octopuses[y + 1][x - 1] += 1;
                            }
                            if x + 1 < octopuses[y + 1].len() {
                                octopuses[y + 1][x + 1] += 1;
                            }
                        }

                        if x > 0 {
                            octopuses[y][x - 1] += 1;
                        }

                        if x + 1 < octopuses[y].len() {
                            octopuses[y][x + 1] += 1;
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
    let count = flashed.len();

    for (x, y) in flashed.into_iter() {
        octopuses[y][x] = 0;
    }

    (octopuses, count)
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

    let total_octopuses_count = octopuses.iter().map(|row| row.len()).sum();

    let mut steps = 1;

    loop {
        let (new_octopuses, flashes) = simulate_step(std::mem::take(&mut octopuses));
        if flashes == total_octopuses_count {
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
