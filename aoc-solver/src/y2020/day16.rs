use regex::Regex;
use std::collections::HashSet;

use crate::solution::{AocError, Solution};

pub struct Day16;

#[derive(Debug, Clone)]
struct Field {
    name: String,
    valid_1_min: u64,
    valid_1_max: u64,
    valid_2_min: u64,
    valid_2_max: u64,
}

fn parse_fields(input: &str) -> Vec<Field> {
    let fields_regex = Regex::new(r"(.+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();

    fields_regex
        .captures_iter(input)
        .map(|capture| {
            let name = capture.get(1).unwrap().as_str().to_string();
            let valid_1_min = capture.get(2).unwrap().as_str().parse::<u64>().unwrap();
            let valid_1_max = capture.get(3).unwrap().as_str().parse::<u64>().unwrap();

            let valid_2_min = capture.get(4).unwrap().as_str().parse::<u64>().unwrap();
            let valid_2_max = capture.get(5).unwrap().as_str().parse::<u64>().unwrap();

            Field {
                name,
                valid_1_min,
                valid_1_max,
                valid_2_min,
                valid_2_max,
            }
        })
        .collect()
}

fn parse_ticket(input: &str, fields: &[Field]) -> Option<Vec<u64>> {
    let ticket = input
        .split(',')
        .map(|value| value.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    if !is_valid_ticket(&ticket, fields) {
        return None;
    }

    Some(ticket)
}

fn is_valid_ticket(ticket: &[u64], fields: &[Field]) -> bool {
    ticket.iter().all(|value| {
        fields.iter().any(|rule| {
            (*value >= rule.valid_1_min && *value <= rule.valid_1_max)
                || (*value >= rule.valid_2_min && *value <= rule.valid_2_max)
        })
    })
}

fn find_possible_fields(
    tickets: &[Vec<u64>],
    fields: &[Field],
    fields_count: usize,
) -> Vec<(usize, String)> {
    let mut possible_fields: Vec<HashSet<String>> = vec![HashSet::new(); fields_count];

    for ticket in tickets {
        for (i, value) in ticket.iter().enumerate() {
            let mut possible: HashSet<String> = fields
                .iter()
                .filter(|field| {
                    (*value >= field.valid_1_min && *value <= field.valid_1_max)
                        || (*value >= field.valid_2_min && *value <= field.valid_2_max)
                })
                .map(|field| field.name.clone())
                .collect();

            if possible_fields[i].is_empty() {
                possible_fields[i] = possible;
            } else {
                possible_fields[i] = possible_fields[i]
                    .iter()
                    .filter_map(|f| possible.take(f))
                    .collect();
            }
        }
    }

    let mut final_fields: Vec<(usize, String)> = Vec::new();

    // We've now found something like [["a"], ["a", "b"], ["b", "c"]]
    // Now move the uniquely resolved fields based on these to final vector
    // one by one, always removing them from each indices possible values.
    while possible_fields.iter().any(|possible| !possible.is_empty()) {
        let (i, unique) = possible_fields
            .iter()
            .enumerate()
            .find(|(_i, possible)| possible.len() == 1)
            .unwrap();
        let unique_name = unique.iter().next().unwrap().clone();

        for field in &mut possible_fields {
            *field = field
                .clone()
                .into_iter()
                .filter(|f| f != &unique_name)
                .collect();
        }

        final_fields.push((i, unique_name));
    }

    final_fields
}

type Ticket = Vec<u64>;
type TicketField = (usize, String);

fn parse_part_2(input: &str) -> Option<(Ticket, Vec<TicketField>)> {
    let mut inputs_iter = input.split("\n\n");

    let fields = parse_fields(inputs_iter.next()?);

    let my_ticket_input = inputs_iter.next()?.lines().nth(1)?;
    let my_ticket = parse_ticket(my_ticket_input, &fields)?;
    let fields_count = my_ticket.len();

    let mut tickets: Vec<Vec<u64>> = inputs_iter
        .next()?
        .lines()
        .skip(1)
        .flat_map(|ticket_i| parse_ticket(ticket_i, &fields))
        .collect();

    tickets.push(my_ticket.clone());

    let possible_fields = find_possible_fields(&tickets, &fields, fields_count);

    Some((my_ticket, possible_fields))
}

impl Solution for Day16 {
    type A = u64;
    type B = u64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2020/day16.txt")
    }

    fn part_1(&self, input: &str) -> Result<u64, AocError> {
        let mut inputs_iter = input.split("\n\n");

        let fields = parse_fields(inputs_iter.next().unwrap());

        let sum = inputs_iter
            .nth(1) // skip my ticket
            .unwrap()
            .lines()
            .skip(1)
            .flat_map(|ticket| {
                ticket
                    .split(',')
                    .map(|value| value.parse::<u64>().unwrap())
                    .filter(|value| {
                        !fields.iter().any(|field| {
                            (*value >= field.valid_1_min && *value <= field.valid_1_max)
                                || (*value >= field.valid_2_min && *value <= field.valid_2_max)
                        })
                    })
                    .collect::<Vec<u64>>()
            })
            .sum();

        Ok(sum)
    }

    fn part_2(&self, input: &str) -> Result<u64, AocError> {
        let (my_ticket, possible_fields) = parse_part_2(input).unwrap();

        let departure_values: Vec<u64> = possible_fields
            .into_iter()
            .filter(|(_i, field)| field.starts_with("departure"))
            .map(|(i, _field)| my_ticket[i])
            .collect();

        let product = departure_values.iter().product();

        Ok(product)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_examples() {
        assert_eq!(
            Day16.part_1(
                "class: 1-3 or 5-7\n\
                 row: 6-11 or 33-44\n\
                 seat: 13-40 or 45-50\n\
                 \n\
                 your ticket:\n\
                 7,1,14\n\
                 \n\
                 nearby tickets:\n\
                 7,3,47\n\
                 40,4,50\n\
                 55,2,20\n\
                 38,6,12"
            ),
            Ok(71)
        );
    }

    #[test]
    fn it_solves_part2_examples() {
        assert_eq!(
            parse_part_2(
                "class: 0-1 or 4-19\n\
                 row: 0-5 or 8-19\n\
                 seat: 0-13 or 16-19\n\
                 \n\
                 your ticket:\n\
                 11,12,13\n\
                 \n\
                 nearby tickets:\n\
                 3,9,18\n\
                 15,1,5\n\
                 5,14,9"
            ),
            Some((
                vec![11, 12, 13],
                vec![
                    (0, String::from("row")),
                    (1, String::from("class")),
                    (2, String::from("seat"))
                ]
            ))
        );
    }
}
