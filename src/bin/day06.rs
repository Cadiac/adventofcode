use std::collections::HashSet;

const INPUT_FILE: &str = include_str!("../../inputs/day06.txt");

fn parse_answers_1(input: &str) -> HashSet::<char> {
    let answers: HashSet::<char> = input.lines()
        .flat_map(|line| line.chars())
        .collect();

    answers
}

fn parse_answers_2(input: &str) -> HashSet::<char> {
    let mut common_answers: HashSet::<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    
    for line in input.lines() {
        let single_answers: HashSet::<char> = line.chars().collect();
        common_answers = common_answers.intersection(&single_answers).copied().collect();
    }

    common_answers
}

fn part_1(input: &str) -> usize {
    let sum = input
        .split("\n\n")
        .map(|i| parse_answers_1(i).len())
        .sum();

    sum
}

fn part_2(input: &str) -> usize {
    let sum = input
        .split("\n\n")
        .map(|i| parse_answers_2(i).len())
        .sum();

    sum
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
    fn it_parses_part1_examples() {
        assert_eq!(parse_answers_1("abc"), vec!['a', 'b', 'c'].into_iter().collect());
        assert_eq!(parse_answers_1("a\nb\nc"), vec!['a', 'b', 'c'].into_iter().collect());
        assert_eq!(parse_answers_1("ab\nac"), vec!['a', 'b', 'c'].into_iter().collect());
        assert_eq!(parse_answers_1("a\na\na\na"), vec!['a'].into_iter().collect());
        assert_eq!(parse_answers_1("b"), vec!['b'].into_iter().collect());
    }

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            part_1(
                "abc\n\
                 \n\
                 a\n\
                 b\n\
                 c\n\
                 \n\
                 ab\n\
                 ac\n\
                 \n\
                 a\n\
                 a\n\
                 a\n\
                 a\n\
                 \n\
                 b"
            ),
            11
        );
    }

    #[test]
    fn it_parses_part2_examples() {
        assert_eq!(parse_answers_2("abc"), vec!['a', 'b', 'c'].into_iter().collect());
        assert_eq!(parse_answers_2("a\nb\nc"), vec![].into_iter().collect());
        assert_eq!(parse_answers_2("ab\nac"), vec!['a'].into_iter().collect());
        assert_eq!(parse_answers_2("a\na\na\na"), vec!['a'].into_iter().collect());
        assert_eq!(parse_answers_2("b"), vec!['b'].into_iter().collect());
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            part_2(
                "abc\n\
                 \n\
                 a\n\
                 b\n\
                 c\n\
                 \n\
                 ab\n\
                 ac\n\
                 \n\
                 a\n\
                 a\n\
                 a\n\
                 a\n\
                 \n\
                 b"
            ),
            6
        );
    }
}
