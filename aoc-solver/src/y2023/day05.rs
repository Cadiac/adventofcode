use std::collections::HashMap;

use itertools::Itertools;

use crate::solution::{AocError, Solution};

pub struct Day05;

#[derive(Debug, Clone)]
struct MappingRange {
    destination_start: i64,
    source_start: i64,
    range_length: i64,
}

struct Range {
    start: i64,
    end: i64,
}

#[derive(Debug, Clone)]
struct Mapping {
    destination: String,
    ranges: Vec<MappingRange>,
}

fn parse(input: &str) -> Result<(Vec<i64>, HashMap<String, Mapping>), AocError> {
    let mut chunks = input.trim().split("\n\n");

    let seeds: Vec<i64> = chunks
        .next()
        .ok_or_else(|| AocError::parse(input, "Missing seeds chunk"))?
        .strip_prefix("seeds: ")
        .ok_or_else(|| AocError::parse(input, "Missing seeds prefix"))?
        .split_ascii_whitespace()
        .map(parse_number)
        .collect::<Result<_, _>>()?;

    let mappings = chunks
        .map(|mapping| {
            let mut lines = mapping.lines();

            let header = lines
                .next()
                .ok_or(AocError::parse(mapping, "Missing header"))?;

            let (source, destination) = header
                .split_once("-to-")
                .ok_or(AocError::parse(header, "Missing -to- delimiter"))?;

            let destination = destination
                .strip_suffix(" map:")
                .ok_or(AocError::parse(destination, "Missing map suffix"))?
                .to_owned();

            let ranges = lines
                .map(|line| {
                    let (destination_start, source_start, range_length) = line
                        .splitn(3, ' ')
                        .collect_tuple()
                        .ok_or_else(|| AocError::parse(line, "Invalid mapping"))?;

                    Ok(MappingRange {
                        destination_start: parse_number(destination_start)?,
                        source_start: parse_number(source_start)?,
                        range_length: parse_number(range_length)?,
                    })
                })
                .collect::<Result<_, _>>()?;

            let mapping = Mapping {
                destination,
                ranges,
            };

            Ok((source.to_owned(), mapping))
        })
        .collect::<Result<_, _>>()?;

    Ok((seeds, mappings))
}

fn parse_number(number: &str) -> Result<i64, AocError> {
    number
        .parse()
        .map_err(|_| AocError::parse(number, "Error parsing number"))
}

impl Solution for Day05 {
    type A = i64;
    type B = i64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day05.txt")
    }

    fn part_1(&self, input: &str) -> Result<i64, AocError> {
        let (seeds, mappings) = parse(input)?;

        let mut numbers = vec![];

        for seed in seeds {
            let mut category = "seed";
            let mut value = seed;

            while let Some(mapping) = mappings.get(category) {
                let range = mapping.ranges.iter().find(|range| {
                    value >= range.source_start && value < range.source_start + range.range_length
                });

                if let Some(range) = range {
                    let range_offset = value - range.source_start;
                    value = range.destination_start + range_offset;
                }

                category = mapping.destination.as_str();
            }

            numbers.push(value);
        }

        numbers
            .into_iter()
            .min()
            .ok_or(AocError::logic("No solution"))
    }

    fn part_2(&self, input: &str) -> Result<i64, AocError> {
        let (seeds, mappings) = parse(input)?;

        let seed_ranges = seeds.chunks_exact(2).map(|range| Range {
            start: range[0],
            end: range[0] + range[1] - 1,
        });

        let lowest = seed_ranges
            .into_iter()
            .map(|seed_range| {
                let mut category = "seed";
                let mut unmapped = vec![(seed_range.start, seed_range.end)];

                while let Some(mapping) = mappings.get(category) {
                    let mut mapped = vec![];

                    while let Some((range_start, range_end)) = unmapped.pop() {
                        let mut is_mapped = false;

                        for mapping_range in mapping.ranges.iter() {
                            // Only one of the mappings should be applied to each range
                            if is_mapped {
                                break;
                            }

                            let mapping_start = mapping_range.source_start;
                            let mapping_end =
                                mapping_range.source_start + mapping_range.range_length - 1;
                            let offset =
                                mapping_range.destination_start - mapping_range.source_start;

                            if mapping_start <= range_start && mapping_end >= range_end {
                                // Full overlap, map the entire range
                                //  567890
                                // ^^^^^^^^
                                mapped.push((range_start + offset, range_end + offset));
                                is_mapped = true;
                            } else if mapping_start <= range_start && mapping_end >= range_start {
                                // Left end of the range overlaps
                                //  567890
                                // ^^^^^
                                mapped.push((range_start + offset, mapping_end + offset));
                                unmapped.push((range_end + 1, range_end));
                                is_mapped = true;
                            } else if mapping_end >= range_end && mapping_start <= range_end {
                                // Right end of the range overlaps
                                //  567890
                                //     ^^^^^
                                unmapped.push((range_start, mapping_start - 1));
                                mapped.push((mapping_start + offset, range_end + offset));
                                is_mapped = true;
                            } else if mapping_start > range_start && mapping_end < range_end {
                                // Overlap in the middle
                                //  567890
                                //   ^^^
                                unmapped.push((range_start, mapping_start - 1));
                                mapped.push((mapping_start + offset, mapping_end + offset));
                                unmapped.push((mapping_end + 1, range_end));
                                is_mapped = true;
                            }
                        }

                        if !is_mapped {
                            // All mappings miss the range, include it unmapped
                            //  567890
                            //         ^^^
                            mapped.push((range_start, range_end));
                        }
                    }

                    category = mapping.destination.as_str();
                    unmapped = mapped;
                }

                unmapped
                    .into_iter()
                    .map(|(start, _)| start)
                    .min()
                    .ok_or(AocError::logic("No ranges"))
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .min()
            .ok_or(AocError::logic("No solution"))?;

        Ok(lowest)
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
    fn it_solves_part2_real_input() {
        assert_eq!(Day05.part_2(Day05.default_input()), Ok(26829166));
    }
}
