extern crate nom;

use nom::{
    branch::permutation, bytes::complete::tag, character::complete::multispace0, combinator::opt,
    sequence::delimited, sequence::preceded, IResult,
};

const INPUT_FILE: &str = include_str!("../../inputs/day04.txt");

#[derive(Debug, PartialEq)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

fn not_whitespace(input: &str) -> IResult<&str, &str> {
    nom::bytes::complete::is_not(" \t\n")(input)
}

fn parse_byr(input: &str) -> IResult<&str, &str> {
    delimited(
        multispace0,
        preceded(tag("byr:"), not_whitespace),
        multispace0,
    )(input)
}

fn parse_iyr(input: &str) -> IResult<&str, &str> {
    delimited(
        multispace0,
        preceded(tag("iyr:"), not_whitespace),
        multispace0,
    )(input)
}

fn parse_eyr(input: &str) -> IResult<&str, &str> {
    delimited(
        multispace0,
        preceded(tag("eyr:"), not_whitespace),
        multispace0,
    )(input)
}

fn parse_hgt(input: &str) -> IResult<&str, &str> {
    delimited(
        multispace0,
        preceded(tag("hgt:"), not_whitespace),
        multispace0,
    )(input)
}

fn parse_hcl(input: &str) -> IResult<&str, &str> {
    delimited(
        multispace0,
        preceded(tag("hcl:"), not_whitespace),
        multispace0,
    )(input)
}

fn parse_ecl(input: &str) -> IResult<&str, &str> {
    delimited(
        multispace0,
        preceded(tag("ecl:"), not_whitespace),
        multispace0,
    )(input)
}

fn parse_pid(input: &str) -> IResult<&str, &str> {
    delimited(
        multispace0,
        preceded(tag("pid:"), not_whitespace),
        multispace0,
    )(input)
}

fn parse_cid(input: &str) -> IResult<&str, Option<&str>> {
    delimited(
        multispace0,
        opt(preceded(tag("cid:"), not_whitespace)),
        multispace0,
    )(input)
}

fn parse_passport_part1(input: &str) -> IResult<&str, Passport> {
    let (unhandled, (byr, iyr, eyr, hgt, hcl, ecl, pid, cid)) = permutation((
        parse_byr, parse_iyr, parse_eyr, parse_hgt, parse_hcl, parse_ecl, parse_pid, parse_cid,
    ))(input)?;

    let cid_str = match cid {
        Some(c) => Some(c.to_string()),
        None => None,
    };

    let passport = Passport {
        byr: String::from(byr),
        iyr: String::from(iyr),
        eyr: String::from(eyr),
        hgt: String::from(hgt),
        hcl: String::from(hcl),
        ecl: String::from(ecl),
        pid: String::from(pid),
        cid: cid_str,
    };
    return Ok((unhandled, passport));
}

fn parse_passport_part2(input: &str) -> IResult<&str, Passport> {
    let (unhandled, (byr, iyr, eyr, hgt, hcl, ecl, pid, cid)) = permutation((
        parse_byr, parse_iyr, parse_eyr, parse_hgt, parse_hcl, parse_ecl, parse_pid, parse_cid,
    ))(input)?;

    let cid_str = match cid {
        Some(c) => Some(c.to_string()),
        None => None,
    };

    let passport = Passport {
        byr: String::from(byr),
        iyr: String::from(iyr),
        eyr: String::from(eyr),
        hgt: String::from(hgt),
        hcl: String::from(hcl),
        ecl: String::from(ecl),
        pid: String::from(pid),
        cid: cid_str,
    };
    return Ok((unhandled, passport));
}

fn part_1(input: &str) -> usize {
    let valid_passports = input
        .split("\n\n")
        .map(parse_passport_part1)
        .filter(|parsed| parsed.is_ok())
        .count();

    valid_passports
}

fn main() -> () {
    let part_1_result = part_1(INPUT_FILE);

    println!("[INFO]: Part 1: {:?}", part_1_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_passports() {
        assert_eq!(
            parse_passport_part1(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
                 byr:1937 iyr:2017 cid:147 hgt:183cm"
            ),
            Ok((
                "",
                Passport {
                    ecl: String::from("gry"),
                    pid: String::from("860033327"),
                    eyr: String::from("2020"),
                    hcl: String::from("#fffffd"),
                    byr: String::from("1937"),
                    iyr: String::from("2017"),
                    cid: Some(String::from("147")),
                    hgt: String::from("183cm")
                }
            ))
        );

        assert_eq!(
            parse_passport_part1(
                "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
                 hcl:#cfa07d byr:1929"
            )
            .is_ok(),
            false
        );

        assert_eq!(
            parse_passport_part1(
                "hcl:#ae17e1 iyr:2013\n\
                 eyr:2024\n\
                 ecl:brn pid:760753108 byr:1931\n\
                 hgt:179cm"
            ),
            Ok((
                "",
                Passport {
                    hcl: String::from("#ae17e1"),
                    iyr: String::from("2013"),
                    eyr: String::from("2024"),
                    ecl: String::from("brn"),
                    pid: String::from("760753108"),
                    byr: String::from("1931"),
                    cid: None,
                    hgt: String::from("179cm")
                }
            ))
        );

        assert_eq!(
            parse_passport_part1(
                "hcl:#cfa07d eyr:2025 pid:166559648\n\
                 iyr:2011 ecl:brn hgt:59in"
            )
            .is_ok(),
            false
        );
    }

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            part_1(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
                 byr:1937 iyr:2017 cid:147 hgt:183cm\n\
                 \n\
                 iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
                 hcl:#cfa07d byr:1929\n\
                 \n\
                 hcl:#ae17e1 iyr:2013\n\
                 eyr:2024\n\
                 ecl:brn pid:760753108 byr:1931\n\
                 hgt:179cm\n\
                 \n\
                 hcl:#cfa07d eyr:2025 pid:166559648\n\
                 iyr:2011 ecl:brn hgt:59in"
            ),
            2
        );
    }
}
