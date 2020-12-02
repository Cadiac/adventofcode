extern crate regex;

use regex::Regex;

const INPUT_FILE: &str = include_str!("../../inputs/day02.txt");

struct DatabaseRow {
    password: String,
    check_char: char,
    min: usize,
    max: usize,
}

fn validate_part1(db_row: &DatabaseRow) -> bool {
    let occurences = db_row.password.chars().filter(|c| c == &db_row.check_char).count();
    return occurences >= db_row.min && occurences <= db_row.max;
}

fn validate_part2(db_row: &DatabaseRow) -> bool {
    let first_match = db_row.password.chars().nth(db_row.min - 1).unwrap() == db_row.check_char;
    let second_match = db_row.password.chars().nth(db_row.max - 1).unwrap() == db_row.check_char;
    return first_match ^ second_match;
}

fn parse_input(input: &str) -> Vec<DatabaseRow> {
    let input_regex = Regex::new(r"^(\d+)-(\d+) (\S): (\S+)$").unwrap();
    let rows: Vec<DatabaseRow> = input
        .lines()
        .map(|line| {
            let capture = input_regex.captures(line).unwrap();

            return DatabaseRow {
                min: capture[1].parse::<usize>().unwrap(),
                max: capture[2].parse::<usize>().unwrap(),
                password: capture[4].parse::<String>().unwrap(),
                check_char: capture[3].parse::<char>().unwrap()
            };
        })
        .collect();

    return rows;
}

fn part_1(input: &str) -> usize {
    let parsed_input = parse_input(input);
    return parsed_input.iter().filter(|row| validate_part1(row)).count();
}

fn part_2(input: &str) -> usize {
    let parsed_input = parse_input(input);
    return parsed_input.iter().filter(|row| validate_part2(row)).count();
}

fn main() -> () {
    let part_1_result = part_1(INPUT_FILE);
    let part_2_result = part_2(INPUT_FILE);

    println!("[INFO]: Part 1: {:?}", part_1_result);
    println!("[INFO]: Part 2: {:?}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_validates_part1_pws_correctly() {
        assert_eq!(
            validate_part1(&DatabaseRow {
                password: String::from("abcde"),
                check_char: 'a',
                min: 1,
                max: 3
            }),
            true
        );
        assert_eq!(
            validate_part1(&DatabaseRow {
                password: String::from("cdefg"),
                check_char: 'b',
                min: 1,
                max: 3
            }),
            false
        );
        assert_eq!(
            validate_part1(&DatabaseRow {
                password: String::from("ccccccccc"),
                check_char: 'c',
                min: 2,
                max: 9
            }),
            true
        )
    }

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(part_1("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc"), 2);
    }

    #[test]
    fn it_validates_part2_pws_correctly() {
        assert_eq!(
            validate_part2(&DatabaseRow {
                password: String::from("abcde"),
                check_char: 'a',
                min: 1,
                max: 3
            }),
            true
        );
        assert_eq!(
            validate_part2(&DatabaseRow {
                password: String::from("cdefg"),
                check_char: 'b',
                min: 1,
                max: 3
            }),
            false
        );
        assert_eq!(
            validate_part2(&DatabaseRow {
                password: String::from("ccccccccc"),
                check_char: 'c',
                min: 2,
                max: 9
            }),
            false
        )
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(part_2("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc"), 1);
    }

}
