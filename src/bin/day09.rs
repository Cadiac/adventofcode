use itertools::Itertools;
use std::collections::HashSet;

const INPUT_FILE: &str = include_str!("../../inputs/day09.txt");

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|height| height.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn find_low_spots(heightmap: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut low_spots = vec![];

    for y in 0..heightmap.len() {
        for x in 0..heightmap[y].len() {
            let height = heightmap[y][x];

            if (y == 0 || heightmap[y - 1][x] > height)
                && (x == 0 || heightmap[y][x - 1] > height)
                && (y + 1 >= heightmap.len() || heightmap[y + 1][x] > height)
                && (x + 1 >= heightmap[y].len() || heightmap[y][x + 1] > height)
            {
                // Found a low spot
                low_spots.push((x, y));
            }
        }
    }

    low_spots
}

fn find_basin_recursive(
    (x, y): (usize, usize),
    heightmap: &[Vec<u32>],
    mut basin: HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let height = heightmap[y][x];

    // Locations of height 9 do not count as being in any basin,
    // and all other locations will always be part of exactly one basin.
    // This means that we really only care about the size of an area bounded by height 9 locations.
    if height == 9 {
        return basin;
    }

    if !basin.insert((x, y)) {
        // These coordinates were already part of the basin, don't traverse this basin any further
        return basin;
    }

    // Otherwise traverse the neighbours unless we hit the edges
    if y > 0 {
        basin = find_basin_recursive((x, (y - 1)), heightmap, basin);
    }

    if x > 0 {
        basin = find_basin_recursive((x - 1, y), heightmap, basin);
    }

    if y + 1 < heightmap.len() {
        basin = find_basin_recursive((x, y + 1), heightmap, basin);
    }

    if x + 1 < heightmap[y].len() {
        basin = find_basin_recursive((x + 1, y), heightmap, basin);
    }

    basin
}

fn part_1(input: &str) -> u32 {
    let heightmap = parse(input);

    find_low_spots(&heightmap)
        .into_iter()
        .map(|(x, y)| heightmap[y][x] + 1)
        .sum()
}

fn part_2(input: &str) -> usize {
    let heightmap = parse(input);

    let low_spots = find_low_spots(&heightmap);

    low_spots
        .into_iter()
        .map(|low_spot| find_basin_recursive(low_spot, &heightmap, HashSet::new()).len())
        .sorted_by(|a, b| Ord::cmp(&b, &a))
        .take(3)
        .product()
}

fn main() {
    let part_1_result = part_1(INPUT_FILE);
    println!("[INFO]: Part 1: {:?}", part_1_result);

    let part_2_result = part_2(INPUT_FILE);
    println!("[INFO]: Part 2: {:?}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            part_1(
                "2199943210\n\
                 3987894921\n\
                 9856789892\n\
                 8767896789\n\
                 9899965678"
            ),
            15
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            part_2(
                "2199943210\n\
                 3987894921\n\
                 9856789892\n\
                 8767896789\n\
                 9899965678"
            ),
            1134
        );
    }
}
