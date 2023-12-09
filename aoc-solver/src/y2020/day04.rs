use nom::{
    branch::alt, branch::permutation, bytes::complete::tag, bytes::complete::take_while_m_n,
    character::complete::digit1, character::complete::multispace0, combinator::map_res,
    combinator::opt, sequence::delimited, sequence::pair, sequence::preceded, IResult,
};

use crate::solution::{AocError, Solution};

pub struct Day04;

#[derive(Debug, PartialEq)]
struct Passport {
    byr: u32,
    iyr: u32,
    eyr: u32,
    hgt: (u32, String),
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

fn not_whitespace(input: &str) -> IResult<&str, &str> {
    nom::bytes::complete::is_not(" \t\n")(input)
}

fn birth_year(input: &str) -> IResult<&str, u32> {
    map_res(
        delimited(multispace0, preceded(tag("byr:"), digit1), multispace0),
        |i: &str| i.parse::<u32>(),
    )(input)
}

fn issue_year(input: &str) -> IResult<&str, u32> {
    map_res(
        delimited(multispace0, preceded(tag("iyr:"), digit1), multispace0),
        |i: &str| i.parse::<u32>(),
    )(input)
}

fn expiration_year(input: &str) -> IResult<&str, u32> {
    map_res(
        delimited(multispace0, preceded(tag("eyr:"), digit1), multispace0),
        |i: &str| i.parse::<u32>(),
    )(input)
}

fn parse_centimeters(input: &str) -> IResult<&str, (u32, &str)> {
    pair(
        map_res(digit1, |i: &str| i.parse::<u32>()),
        alt((tag("cm"), tag("in"))),
    )(input)
}

fn height(input: &str) -> IResult<&str, (u32, &str)> {
    delimited(
        multispace0,
        preceded(tag("hgt:"), parse_centimeters),
        multispace0,
    )(input)
}

fn parse_hex_color(input: &str) -> IResult<&str, &str> {
    preceded(
        tag("#"),
        take_while_m_n(6, 6, |c: char| c.is_ascii_hexdigit()),
    )(input)
}

fn hair_color(input: &str) -> IResult<&str, &str> {
    delimited(
        multispace0,
        preceded(tag("hcl:"), parse_hex_color),
        multispace0,
    )(input)
}

fn parse_eye_color(input: &str) -> IResult<&str, &str> {
    alt((
        tag("amb"),
        tag("blu"),
        tag("brn"),
        tag("gry"),
        tag("grn"),
        tag("hzl"),
        tag("oth"),
    ))(input)
}

fn eye_color(input: &str) -> IResult<&str, &str> {
    delimited(
        multispace0,
        preceded(tag("ecl:"), parse_eye_color),
        multispace0,
    )(input)
}

fn parse_passport_number(input: &str) -> IResult<&str, &str> {
    take_while_m_n(9, 9, |c: char| c.is_ascii_digit())(input)
}

fn passport_id(input: &str) -> IResult<&str, &str> {
    delimited(
        multispace0,
        preceded(tag("pid:"), parse_passport_number),
        multispace0,
    )(input)
}

fn country_id(input: &str) -> IResult<&str, Option<String>> {
    let (unhandled, parsed) = delimited(
        multispace0,
        opt(preceded(tag("cid:"), not_whitespace)),
        multispace0,
    )(input)?;

    let cid_string = parsed.map(|c| c.to_string());

    Ok((unhandled, cid_string))
}

fn parse_passport_part1(input: &str) -> IResult<&str, ()> {
    let required_field = |field: &'static str| {
        delimited(
            multispace0,
            preceded(tag(field), not_whitespace),
            multispace0,
        )
    };

    let optional_field = |field: &'static str| {
        delimited(
            multispace0,
            opt(preceded(tag(field), not_whitespace)),
            multispace0,
        )
    };

    let (unhandled, _parsed) = permutation((
        required_field("byr:"),
        required_field("iyr:"),
        required_field("eyr:"),
        required_field("hgt:"),
        required_field("hcl:"),
        required_field("ecl:"),
        required_field("pid:"),
        optional_field("cid:"),
    ))(input)?;

    // we only care about the amount of parsed
    Ok((unhandled, ()))
}

fn parse_passport_part2(input: &str) -> IResult<&str, Passport> {
    let (unhandled, (byr, iyr, eyr, hgt, hcl, ecl, pid, cid)) = permutation((
        birth_year,
        issue_year,
        expiration_year,
        height,
        hair_color,
        eye_color,
        passport_id,
        country_id,
    ))(input)?;

    let passport = Passport {
        byr,
        iyr,
        eyr,
        hgt: (hgt.0, String::from(hgt.1)),
        hcl: String::from(hcl),
        ecl: String::from(ecl),
        pid: String::from(pid),
        cid,
    };

    Ok((unhandled, passport))
}

impl Solution for Day04 {
    type A = usize;
    type B = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2020/day04.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let valid_passports = input
            .split("\n\n")
            .map(parse_passport_part1)
            .filter(|parsed| parsed.is_ok())
            .count();

        Ok(valid_passports)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let valid_passports = input
            .split("\n\n")
            .map(parse_passport_part2)
            .filter_map(|passport| {
                if passport.is_err() {
                    return None;
                }

                let (_unhandled, valid) = passport.unwrap();

                if valid.byr < 1920 || valid.byr > 2002 {
                    return None;
                }

                if valid.iyr < 2010 || valid.iyr > 2020 {
                    return None;
                }

                if valid.eyr < 2020 || valid.eyr > 2030 {
                    return None;
                }

                if valid.hgt.1 == "cm" && (valid.hgt.0 < 150 || valid.hgt.0 > 193) {
                    return None;
                }

                if valid.hgt.1 == "in" && (valid.hgt.0 < 59 || valid.hgt.0 > 76) {
                    return None;
                }

                Some(valid)
            })
            .count();

        Ok(valid_passports)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day04.part_1(
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
            Ok(2)
        );
    }

    #[test]
    fn it_solves_part2_examples() {
        assert_eq!(
            Day04.part_2(
                "eyr:1972 cid:100\n\
                 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\
                 \n\
                 iyr:2019\n\
                 hcl:#602927 eyr:1967 hgt:170cm\n\
                 ecl:grn pid:012533040 byr:1946\n\
                 \n\
                 hcl:dab227 iyr:2012\n\
                 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\
                 \n\
                 hgt:59cm ecl:zzz\n\
                 eyr:2038 hcl:74454a iyr:2023\n\
                 pid:3556412378 byr:2007"
            ),
            Ok(0)
        );

        assert_eq!(
            Day04.part_2(
                "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n\
                 hcl:#623a2f\n
                 \n\
                 eyr:2029 ecl:blu cid:129 byr:1989\n\
                 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\
                 \n\
                 hcl:#888785\n\
                 hgt:164cm byr:2001 iyr:2015 cid:88\n\
                 pid:545766238 ecl:hzl\n\
                 eyr:2022\n\
                 \n\
                 iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
            ),
            Ok(4)
        );
    }
}
