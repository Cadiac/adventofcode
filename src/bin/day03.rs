use std::collections::HashSet;

const INPUT_FILE: &str = include_str!("../../inputs/day03.txt");

fn count_trees(
    trees: &HashSet<(usize, usize)>,
    columns: usize,
    rows: usize,
    right: usize,
    down: usize,
) -> usize {
    let mut current: (usize, usize) = (0, 0);
    let mut hits = 0;

    while current.1 < rows {
        if trees.contains(&current) {
            hits = hits + 1;
        }

        current.0 = (current.0 + right) % columns;
        current.1 = current.1 + down;
    }

    return hits;
}

fn parse_input(input: &str) -> (HashSet<(usize, usize)>, usize, usize) {
    let mut trees: HashSet<(usize, usize)> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, tile) in line.chars().enumerate() {
            if tile == '#' {
                trees.insert((x, y));
            }
        }
    }

    let rows = input.lines().count();
    let columns = input.lines().next().unwrap().len();

    return (trees, columns, rows);
}

fn main() -> () {
    let (trees, columns, rows) = parse_input(INPUT_FILE);
    let part_1_result = count_trees(&trees, columns, rows, 3, 1);
    let part_2_result = count_trees(&trees, columns, rows, 1, 1)
        * count_trees(&trees, columns, rows, 3, 1)
        * count_trees(&trees, columns, rows, 5, 1)
        * count_trees(&trees, columns, rows, 7, 1)
        * count_trees(&trees, columns, rows, 1, 2);

    println!("[INFO]: Part 1: {:?}", part_1_result);
    println!("[INFO]: Part 2: {:?}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        let (trees, rows, columns) = parse_input(
            "..##.......\n\
             #...#...#..\n\
             .#....#..#.\n\
             ..#.#...#.#\n\
             .#...##..#.\n\
             ..#.##.....\n\
             .#.#.#....#\n\
             .#........#\n\
             #.##...#...\n\
             #...##....#\n\
             .#..#...#.#",
        );

        assert_eq!(columns, 11);
        assert_eq!(rows, 11);
        assert_eq!(trees.len(), 37);

        assert_eq!(count_trees(&trees, columns, rows, 3, 1), 7);
    }

    #[test]
    fn it_solves_part2_example() {
        let (trees, rows, columns) = parse_input(
            "..##.......\n\
             #...#...#..\n\
             .#....#..#.\n\
             ..#.#...#.#\n\
             .#...##..#.\n\
             ..#.##.....\n\
             .#.#.#....#\n\
             .#........#\n\
             #.##...#...\n\
             #...##....#\n\
             .#..#...#.#",
        );

        assert_eq!(count_trees(&trees, columns, rows, 1, 1), 2);
        assert_eq!(count_trees(&trees, columns, rows, 3, 1), 7);
        assert_eq!(count_trees(&trees, columns, rows, 5, 1), 3);
        assert_eq!(count_trees(&trees, columns, rows, 7, 1), 4);
        assert_eq!(count_trees(&trees, columns, rows, 1, 2), 2);
    }
}
