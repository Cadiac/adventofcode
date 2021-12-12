use std::collections::HashMap;

const INPUT_FILE: &str = include_str!("../../inputs/day12.txt");

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut caves: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split('-').collect();
        let name = parts[0];
        let neighbour = parts[1];

        caves.entry(name).or_insert(vec![]).push(neighbour);
        caves.entry(neighbour).or_insert(vec![]).push(name);
    }

    caves
}

fn search_paths<'a>(
    caves: &HashMap<&str, Vec<&str>>,
    current: &'a str,
    mut path: Vec<&'a str>,
) -> usize {
    path.push(current);

    if current == "end" {
        return 1;
    }

    let neighbours = caves.get(&current).unwrap();

    let mut paths_to_end = 0;
    for neighbour in neighbours.iter() {
        let neighbour_is_small = neighbour.chars().any(|c| c.is_ascii_lowercase());

        if neighbour_is_small && path.contains(neighbour) {
            continue;
        }

        paths_to_end += search_paths(caves, neighbour.clone(), path.clone());
    }

    paths_to_end
}

fn search_paths_part2<'a>(
    caves: &HashMap<&str, Vec<&str>>,
    current: &'a str,
    mut path: Vec<&'a str>,
    is_some_small_visited_twice: bool,
) -> usize {
    path.push(current);

    if current == "end" {
        return 1;
    }

    let neighbours = caves.get(&current).unwrap();

    let mut paths_to_end = 0;
    for neighbour in neighbours.iter() {
        if *neighbour == "start" {
            continue;
        }

        let is_neighbour_small = neighbour.chars().any(|c| c.is_ascii_lowercase());
        if is_neighbour_small && path.contains(neighbour) {
            if is_some_small_visited_twice {
                continue;
            } else {
                paths_to_end += search_paths_part2(
                    caves,
                    neighbour.clone(),
                    path.clone(),
                    true,
                );
            }
        } else {
            paths_to_end += search_paths_part2(
                caves,
                neighbour.clone(),
                path.clone(),
                is_some_small_visited_twice,
            );
        }
    }

    paths_to_end
}

fn part_1(input: &str) -> usize {
    let caves = parse(input);
    search_paths(&caves, "start", vec![])
}

fn part_2(input: &str) -> usize {
    let caves = parse(input);
    search_paths_part2(&caves, "start", vec![], false)
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
    fn it_solves_part1_example1() {
        assert_eq!(
            part_1(
                "start-A\n\
                 start-b\n\
                 A-c\n\
                 A-b\n\
                 b-d\n\
                 A-end\n\
                 b-end",
            ),
            10
        );
    }

    #[test]
    fn it_solves_part1_example2() {
        assert_eq!(
            part_1(
                "dc-end\n\
                 HN-start\n\
                 start-kj\n\
                 dc-start\n\
                 dc-HN\n\
                 LN-dc\n\
                 HN-end\n\
                 kj-sa\n\
                 kj-HN\n\
                 kj-dc",
            ),
            19
        );
    }

    #[test]
    fn it_solves_part1_example3() {
        assert_eq!(
            part_1(
                "fs-end\n\
                 he-DX\n\
                 fs-he\n\
                 start-DX\n\
                 pj-DX\n\
                 end-zg\n\
                 zg-sl\n\
                 zg-pj\n\
                 pj-he\n\
                 RW-he\n\
                 fs-DX\n\
                 pj-RW\n\
                 zg-RW\n\
                 start-pj\n\
                 he-WI\n\
                 zg-he\n\
                 pj-fs\n\
                 start-RW",
            ),
            226
        );
    }

    #[test]
    fn it_solves_part2_example1() {
        assert_eq!(
            part_2(
                "start-A\n\
                 start-b\n\
                 A-c\n\
                 A-b\n\
                 b-d\n\
                 A-end\n\
                 b-end",
            ),
            36
        );
    }

    #[test]
    fn it_solves_part2_example2() {
        assert_eq!(
            part_2(
                "dc-end\n\
                 HN-start\n\
                 start-kj\n\
                 dc-start\n\
                 dc-HN\n\
                 LN-dc\n\
                 HN-end\n\
                 kj-sa\n\
                 kj-HN\n\
                 kj-dc",
            ),
            103
        );
    }

    #[test]
    fn it_solves_part2_example3() {
        assert_eq!(
            part_2(
                "fs-end\n\
                 he-DX\n\
                 fs-he\n\
                 start-DX\n\
                 pj-DX\n\
                 end-zg\n\
                 zg-sl\n\
                 zg-pj\n\
                 pj-he\n\
                 RW-he\n\
                 fs-DX\n\
                 pj-RW\n\
                 zg-RW\n\
                 start-pj\n\
                 he-WI\n\
                 zg-he\n\
                 pj-fs\n\
                 start-RW",
            ),
            3509
        );
    }
}
