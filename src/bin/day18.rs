use std::collections::HashMap;
use std::collections::HashSet;

const INPUT_FILE: &str = include_str!("../../inputs/day18.txt");

fn part_1(input: String) -> (usize, Vec<char>) {
    let mut walkable: HashSet<(i64, i64)> = HashSet::new();
    let mut you: (i64, i64) = (0, 0);
    let mut keys: HashMap<(i64, i64), char> = HashMap::new();
    let mut doors: HashMap<(i64, i64), char> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character == '#' || character >= 'a' && character <= 'z' {
                walkable.insert((x as i64, y as i64));
            }
            if character == '@' {
                you = (x as i64, y as i64);
            }
            if character >= 'a' && character <= 'z' {
                keys.insert((x as i64, y as i64), character);
            }
            if character >= 'A' && character <= 'Z' {
                doors.insert((x as i64, y as i64), character);
            }
        }
    }

    println!("you: {:?}, keys: {:?}, doors: {:?}", you, keys, doors);

    return (0, Vec::new());
}

fn main() -> () {
    let sum = part_1(String::from(INPUT_FILE));
    
    println!("[INFO]: Part 1: {:?}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn it_solves_part1_example_simple() {
        assert_eq!(part_1(String::from(
            "#########
             #b.A.@.a#
             #########")), (8, vec!['a', 'b']));
    }

    #[ignore]
    #[test]
    fn it_solves_part1_example_2() {
        assert_eq!(part_1(String::from(
            "########################
             #f.D.E.e.C.b.A.@.a.B.c.#
             ######################.#
             #d.....................#
             ########################")), (86, vec!['a', 'b', 'c', 'd', 'e', 'f']));
    }

    #[ignore]
    #[test]
    fn it_solves_part1_example_3() {
        assert_eq!(part_1(String::from(
            "########################
             #...............b.C.D.f#
             #.######################
             #.....@.a.B.c.d.A.e.F.g#
             ########################")), (132, vec!['b', 'a', 'c', 'd', 'f', 'e', 'g']));
    }

    #[ignore]
    #[test]
    fn it_solves_part1_example_4() {
        assert_eq!(part_1(String::from(
            "#################
             #i.G..c...e..H.p#
             ########.########
             #j.A..b...f..D.o#
             ########@########
             #k.E..a...g..B.n#
             ########.########
             #l.F..d...h..C.m#
             #################")), (136, vec!['a', 'f', 'b', 'j', 'g', 'n', 'h', 'd', 'l', 'o', 'e', 'p', 'c', 'i', 'k', 'm']));
    }

    #[ignore]
    #[test]
    fn it_solves_part1_example_5() {
        assert_eq!(part_1(String::from(
            "########################
             #@..............ac.GI.b#
             ###d#e#f################
             ###A#B#C################
             ###g#h#i################
             ########################")), (136, vec!['a', 'c', 'f', 'i', 'd', 'g', 'b', 'e', 'h']));
    }
}
