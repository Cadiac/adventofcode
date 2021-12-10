extern crate nom;

use nom::{
    branch::alt,
    character::complete::char,
    combinator::{cut, opt},
    error::{convert_error, VerboseError, VerboseErrorKind::Char},
    multi::many1,
    sequence::preceded,
    Err, IResult,
};

const INPUT_FILE: &str = include_str!("../../inputs/day10.txt");

enum ParseResult {
    Ok,
    Incomplete { expected: char },
    Corrupted { illegal: char },
}

fn parse(input: &str) -> ParseResult {
    match chunks(input) {
        Err(Err::Incomplete(_needed)) => unimplemented!(),
        Err(Err::Error(e)) | Err(Err::Failure(e)) => {
            // println!("{}", convert_error(input, e.clone()));
            let (unhandled, error_kind) = e.errors[0].clone();
            let expected = match error_kind {
                Char(expected_char) => expected_char,
                _ => unimplemented!(),
            };
            if let Some(illegal) = unhandled.chars().next() {
                ParseResult::Corrupted { illegal }
            } else {
                ParseResult::Incomplete { expected }
            }
        }
        Ok(_) => ParseResult::Ok,
    }
}

fn chunks(input: &str) -> IResult<&str, (), VerboseError<&str>> {
    let (unhandled, _parsed) = many1(alt((
        preceded(char('('), cut(preceded(opt(chunks), char(')')))),
        preceded(char('['), cut(preceded(opt(chunks), char(']')))),
        preceded(char('{'), cut(preceded(opt(chunks), char('}')))),
        preceded(char('<'), cut(preceded(opt(chunks), char('>')))),
    )))(input)?;

    // We're only checking what is left unhandled
    Ok((unhandled, ()))
}

fn score_syntax_error(found_illegal: char) -> usize {
    match found_illegal {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn score_autocomplete(completion: Vec<char>) -> usize {
    completion.iter().fold(0, |mut acc, c| {
        // Start with a total score of 0.
        // Then, for each character, multiply the total score by 5
        acc *= 5;

        // and then increase the total score by the point value given
        // for the character in the following table:
        acc += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        };

        acc
    })
}

fn complete_input(input: &str) -> Option<Vec<char>> {
    let mut completion: Vec<char> = Vec::new();
    let mut completed_input = String::from(input);

    loop {
        match parse(completed_input.as_str()) {
            ParseResult::Incomplete { expected, .. } => {
                completed_input.push(expected);
                completion.push(expected);
            }
            ParseResult::Ok => {
                return Some(completion);
            }
            ParseResult::Corrupted { .. } => {
                return None;
            }
        };
    }
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(parse)
        .map(|result| match result {
            ParseResult::Corrupted { illegal } => score_syntax_error(illegal),
            _ => 0,
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let mut scores: Vec<usize> = input
        .lines()
        .flat_map(complete_input)
        .map(score_autocomplete)
        .collect();

    scores.sort();
    scores[scores.len() / 2]
}

fn main() -> () {
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
                "[({(<(())[]>[[{[]{<()<>>\n\
                 [(()[<>])]({[<{<<[]>>(\n\
                 {([(<{}[<>[]}>{[]{[(<()>\n\
                 (((({<>}<{<{<>}{[]{[]{}\n\
                 [[<[([]))<([[{}[[()]]]\n\
                 [{[{({}]{}}([{[{{{}}([]\n\
                 {<[[]]>}<{[{[{[]{()[[[]\n\
                 [<(<(<(<{}))><([]([]()\n\
                 <{([([[(<>()){}]>(<<{{\n\
                 <{([{{}}[<[[[<>{}]]]>[]]"
            ),
            26397
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            part_2(
                "[({(<(())[]>[[{[]{<()<>>\n\
                 [(()[<>])]({[<{<<[]>>(\n\
                 {([(<{}[<>[]}>{[]{[(<()>\n\
                 (((({<>}<{<{<>}{[]{[]{}\n\
                 [[<[([]))<([[{}[[()]]]\n\
                 [{[{({}]{}}([{[{{{}}([]\n\
                 {<[[]]>}<{[{[{[]{()[[[]\n\
                 [<(<(<(<{}))><([]([]()\n\
                 <{([([[(<>()){}]>(<<{{\n\
                 <{([{{}}[<[[[<>{}]]]>[]]"
            ),
            288957
        );
    }
}
