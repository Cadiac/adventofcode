use std::collections::HashMap;

use itertools::Itertools;

use crate::solution::{AocError, Solution};

pub struct Day05;

#[derive(Debug, Clone)]
struct MappingRange {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

#[derive(Debug, Clone)]
struct Mapping {
    destination: String,
    ranges: Vec<MappingRange>,
}

fn parse(input: &str) -> Result<(Vec<u64>, HashMap<String, Mapping>), AocError> {
    let mut chunks = input.trim().split("\n\n");

    let seeds: Vec<u64> = chunks
        .next()
        .ok_or_else(|| AocError::parse(input, "Missing seeds chunk"))?
        .strip_prefix("seeds: ")
        .ok_or_else(|| AocError::parse(input, "Missing seeds prefix"))?
        .split_ascii_whitespace()
        .map(|number| number.parse::<u64>().unwrap())
        .collect();

    let mappings = chunks
        .map(|mapping| {
            let mut lines = mapping.lines();

            let header = lines.next().unwrap();

            let (source, mut destination) = header.split_once("-to-").unwrap();
            destination = destination.strip_suffix(" map:").unwrap();

            let mut mapping = Mapping {
                destination: destination.to_owned(),
                ranges: Vec::new(),
            };

            for line in lines {
                match line.splitn(3, ' ').collect_tuple() {
                    Some((destination_range_start, source_range_start, range_length)) => {
                        let d = destination_range_start.parse::<u64>().unwrap();
                        let s = source_range_start.parse::<u64>().unwrap();
                        let l = range_length.parse::<u64>().unwrap();

                        mapping.ranges.push(MappingRange {
                            destination_range_start: d,
                            source_range_start: s,
                            range_length: l,
                        });
                    }
                    None => unreachable!("invalid mapping"),
                }
            }

            (source.to_owned(), mapping)
        })
        .collect();

    Ok((seeds, mappings))
}

impl Solution for Day05 {
    type F = u64;
    type S = u64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day05.txt")
    }

    fn part_1(&self, input: &str) -> Result<u64, AocError> {
        let (seeds, mappings) = parse(input)?;

        let mut numbers = vec![];

        for seed in seeds {
            let mut current = "seed";
            let mut value = seed;

            while current != "location" {
                let mapping = mappings.get(current).unwrap();

                let range = mapping.ranges.iter().find(|range| {
                    value >= range.source_range_start
                        && value < range.source_range_start + range.range_length
                });

                if let Some(range) = range {
                    let range_offset = value - range.source_range_start;
                    value = range.destination_range_start + range_offset;
                }

                current = mapping.destination.as_str();
            }

            numbers.push(value);
        }

        let lowest = numbers.iter().min().unwrap_or(&0);

        Ok(*lowest)
    }

    fn part_2(&self, input: &str) -> Result<u64, AocError> {
        let (seeds, mappings) = parse(input)?;

        let seeds = seeds
            .chunks_exact(2)
            .flat_map(|range| (range[0]..(range[0] + range[1])).collect::<Vec<u64>>());

        let count = seeds.clone().count();
        // print!("We have {count} seeds");

        let mut numbers = vec![];

        let mut progress = 0;

        for seed in seeds {
            if progress % 100000 == 0 {
                // println!("{progress}/{count}")
            }
            let mut current = "seed";
            let mut value = seed;

            while current != "location" {
                let mapping = mappings.get(current).unwrap();

                let range = mapping.ranges.iter().find(|range| {
                    value >= range.source_range_start
                        && value < range.source_range_start + range.range_length
                });

                if let Some(range) = range {
                    let range_offset = value - range.source_range_start;
                    value = range.destination_range_start + range_offset;
                }

                current = mapping.destination.as_str();
            }

            numbers.push(value);
            progress += 1;
        }

        let lowest = numbers.iter().min().unwrap_or(&0);

        Ok(*lowest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day05.part_1(
                "seeds: 79 14 55 13\n\
                 \n\
                 seed-to-soil map:\n\
                 50 98 2\n\
                 52 50 48\n\
                 \n\
                 soil-to-fertilizer map:\n\
                 0 15 37\n\
                 37 52 2\n\
                 39 0 15\n\
                 \n\
                 fertilizer-to-water map:\n\
                 49 53 8\n\
                 0 11 42\n\
                 42 0 7\n\
                 57 7 4\n\
                 \n\
                 water-to-light map:\n\
                 88 18 7\n\
                 18 25 70\n\
                 \n\
                 light-to-temperature map:\n\
                 45 77 23\n\
                 81 45 19\n\
                 68 64 13\n\
                 \n\
                 temperature-to-humidity map:\n\
                 0 69 1\n\
                 1 0 69\n\
                 \n\
                 humidity-to-location map:\n\
                 60 56 37\n\
                 56 93 4\n"
            ),
            Ok(35)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day05.part_2(
                "seeds: 79 14 55 13\n\
                 \n\
                 seed-to-soil map:\n\
                 50 98 2\n\
                 52 50 48\n\
                 \n\
                 soil-to-fertilizer map:\n\
                 0 15 37\n\
                 37 52 2\n\
                 39 0 15\n\
                 \n\
                 fertilizer-to-water map:\n\
                 49 53 8\n\
                 0 11 42\n\
                 42 0 7\n\
                 57 7 4\n\
                 \n\
                 water-to-light map:\n\
                 88 18 7\n\
                 18 25 70\n\
                 \n\
                 light-to-temperature map:\n\
                 45 77 23\n\
                 81 45 19\n\
                 68 64 13\n\
                 \n\
                 temperature-to-humidity map:\n\
                 0 69 1\n\
                 1 0 69\n\
                 \n\
                 humidity-to-location map:\n\
                 60 56 37\n\
                 56 93 4\n"
            ),
            Ok(46)
        );
    }

    #[test]
    fn it_solves_part2_real() {
        Day05.part_2(include_str!("../../../inputs/2023/day05.txt"));
    }
}
