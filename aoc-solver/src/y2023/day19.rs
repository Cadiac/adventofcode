use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline},
    combinator::{map, map_res},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    Finish, IResult,
};

use crate::solution::{AocError, Solution};

pub struct Day19;

#[derive(Clone, PartialEq, Eq, Hash)]
enum Category {
    ExtremelyCoolLooking,
    Musical, // it makes a noise when you hit it!
    Aerodynamic,
    Shiny,
}

enum Operator {
    LessThan,
    GreaterThan,
}

#[derive(Clone)]
enum Target {
    Accepted,
    Rejected,
    Name(String),
}

enum Step {
    Comparison((Category, Operator, u32, Target)),
    Name(Target),
}

type Workflow = (String, Vec<Step>);
type Workflows = HashMap<String, Vec<Step>>;
type Part = HashMap<Category, u32>;

#[derive(Clone, Default)]
struct Range {
    min: u32,
    max: u32,
}

fn parse_comparison(input: &str) -> IResult<&str, Step> {
    let (unhandled, (category, operator, operand, target)) = tuple((
        parse_category,
        alt((
            map(tag("<"), |_| Operator::LessThan),
            map(tag(">"), |_| Operator::GreaterThan),
        )),
        map_res(digit1, |i: &str| i.parse::<u32>()),
        preceded(tag(":"), parse_target),
    ))(input)?;

    Ok((
        unhandled,
        Step::Comparison((category, operator, operand, target)),
    ))
}

fn parse_target(input: &str) -> IResult<&str, Target> {
    alt((
        map(tag("A"), |_| Target::Accepted),
        map(tag("R"), |_| Target::Rejected),
        map(alpha1, |name: &str| Target::Name(name.to_owned())),
    ))(input)
}

fn parse_category(input: &str) -> IResult<&str, Category> {
    alt((
        map(tag("x"), |_| Category::ExtremelyCoolLooking),
        map(tag("m"), |_| Category::Musical),
        map(tag("a"), |_| Category::Aerodynamic),
        map(tag("s"), |_| Category::Shiny),
    ))(input)
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    map(
        tuple((
            alpha1,
            delimited(
                tag("{"),
                separated_list1(
                    tag(","),
                    alt((parse_comparison, map(parse_target, Step::Name))),
                ),
                tag("}"),
            ),
        )),
        |(name, steps)| (name.to_owned(), steps),
    )(input)
}

fn parse_workflows(input: &str) -> IResult<&str, Workflows> {
    let (unhandled, workflows) = many1(terminated(parse_workflow, newline))(input)?;
    Ok((unhandled, workflows.into_iter().collect()))
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let (unhandled, categories) = delimited(
        tag("{"),
        separated_list1(
            tag(","),
            separated_pair(
                parse_category,
                tag("="),
                map_res(digit1, |i: &str| i.parse::<u32>()),
            ),
        ),
        tag("}"),
    )(input)?;

    Ok((unhandled, categories.into_iter().collect()))
}

fn parse_parts(input: &str) -> IResult<&str, Vec<Part>> {
    many1(terminated(parse_part, newline))(input)
}

fn parse(input: &str) -> Result<(Workflows, Vec<Part>), AocError> {
    let (unhandled, (workflows, parts)) =
        separated_pair(parse_workflows, newline, parse_parts)(input)
            .finish()
            .map_err(|err| AocError::parse("nom", err))?;

    if !unhandled.is_empty() {
        return Err(AocError::parse(unhandled, "Input wasn't fully parsed"));
    }

    Ok((workflows.into_iter().collect(), parts))
}

impl Solution for Day19 {
    type A = u32;
    type B = u64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day19.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let (workflows, parts) = parse(input)?;

        let sum = parts
            .iter()
            .filter_map(|part| {
                let mut current = "in";

                while let Some(steps) = workflows.get(current) {
                    for step in steps {
                        let (continue_next, target) = match step {
                            Step::Comparison((category, operator, operand, target)) => {
                                let is_fulfilled = match operator {
                                    Operator::GreaterThan => part[&category] > *operand,
                                    Operator::LessThan => part[&category] < *operand,
                                };

                                (is_fulfilled, target)
                            }
                            Step::Name(target) => (true, target),
                        };

                        if continue_next {
                            match target {
                                Target::Accepted => return Some(part.values().sum::<u32>()),
                                Target::Rejected => return None,
                                Target::Name(name) => {
                                    current = &name;
                                    break;
                                }
                            }
                        }
                    }
                }

                None
            })
            .sum();

        Ok(sum)
    }

    fn part_2(&self, input: &str) -> Result<u64, AocError> {
        let (workflows, _) = parse(input)?;

        let mut combinations = 0;
        let mut stack = vec![(
            "in",
            HashMap::from([
                (Category::ExtremelyCoolLooking, Range { min: 1, max: 4000 }),
                (Category::Musical, Range { min: 1, max: 4000 }),
                (Category::Aerodynamic, Range { min: 1, max: 4000 }),
                (Category::Shiny, Range { min: 1, max: 4000 }),
            ]),
        )];

        while let Some((current, mut ranges)) = stack.pop() {
            for step in &workflows[current] {
                // `ranges` continues the steps of the current workflow, never fulfilling the condition
                // `next` fulfills the condition and enters the target, leaving the current workflow.
                match step {
                    Step::Comparison((category, operator, operand, target)) => {
                        let mut next = ranges.clone();

                        match operator {
                            Operator::GreaterThan => {
                                ranges.entry(category.clone()).or_default().max = *operand;
                                next.entry(category.clone()).or_default().min = *operand + 1;
                            }
                            Operator::LessThan => {
                                ranges.entry(category.clone()).or_default().min = *operand;
                                next.entry(category.clone()).or_default().max = *operand - 1;
                            }
                        };

                        // Branch `next` like it met the condition and enter its target
                        match target {
                            Target::Rejected => {}
                            Target::Accepted => {
                                combinations += valid_combinations(&next);
                            }
                            Target::Name(name) => {
                                stack.push((name, next.clone()));
                            }
                        };
                    }
                    Step::Name(target) => {
                        match target {
                            Target::Rejected => {}
                            Target::Accepted => {
                                combinations += valid_combinations(&ranges);
                            }
                            Target::Name(name) => {
                                // Jump to the next workflow
                                stack.push((name, ranges.clone()));
                            }
                        };
                        break;
                    }
                };
            }
        }

        Ok(combinations)
    }
}

fn valid_combinations(ranges: &HashMap<Category, Range>) -> u64 {
    ranges
        .values()
        .map(|Range { min, max }| *max as u64 - *min as u64 + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const EXAMPLE_INPUT: &str =
       "px{a<2006:qkq,m>2090:A,rfg}\n\
        pv{a>1716:R,A}\n\
        lnx{m>1548:A,A}\n\
        rfg{s<537:gd,x>2440:R,A}\n\
        qs{s>3448:A,lnx}\n\
        qkq{x<1416:A,crn}\n\
        crn{x>2662:A,R}\n\
        in{s<1351:px,qqz}\n\
        qqz{s>2770:qs,m<1801:hdj,R}\n\
        gd{a>3333:R,R}\n\
        hdj{m>838:A,pv}\n\
        \n\
        {x=787,m=2655,a=1222,s=2876}\n\
        {x=1679,m=44,a=2067,s=496}\n\
        {x=2036,m=264,a=79,s=2244}\n\
        {x=2461,m=1339,a=466,s=291}\n\
        {x=2127,m=1623,a=2188,s=1013}\n";

    #[test]
    fn it_parses_example() {
        let result = parse(EXAMPLE_INPUT);
        assert!(result.is_ok());
        let (workflows, parts) = result.unwrap();
        assert_eq!(workflows.len(), 11);
        assert_eq!(parts.len(), 5);
    }

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(Day19.part_1(EXAMPLE_INPUT), Ok(19114));
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(Day19.part_2(EXAMPLE_INPUT), Ok(167409079868000));
    }
}
