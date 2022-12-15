use std::collections::{HashMap, HashSet};

use crate::solution::{AocError, Solution};

struct Sensor {
    pos: (i64, i64),
    beacon: (i64, i64),
    distance: i64,
}

impl Sensor {
    fn width(&self, y: i64) -> i64 {
        self.distance - i64::abs(self.pos.1 - y)
    }

    fn range(&self, y: i64) -> Option<(i64, i64)> {
        let width = self.width(y);
        if width < 0 {
            return None;
        }

        let min = self.pos.0 - width;
        let max = self.pos.0 + width;

        Some((min, max))
    }
}

fn distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    i64::abs(a.0 - b.0) + i64::abs(a.1 - b.1)
}

pub struct Day15;

impl Day15 {
    fn parse(input: &str) -> Result<(Vec<Sensor>, HashSet<(i64, i64)>), AocError> {
        let mut sensors = Vec::new();
        let mut beacons = HashSet::new();

        for line in input.lines() {
            let (x, y, b_x, b_y) =
                serde_scan::scan!("Sensor at x={}, y={}: closest beacon is at x={}, y={}" <- line)
                    .map_err(|err| AocError::parse(line, err))?;

            let sensor = Sensor {
                pos: (x, y),
                beacon: (b_x, b_y),
                distance: distance((x, y), (b_x, b_y)),
            };

            sensors.push(sensor);
            beacons.insert((b_x, b_y));
        }

        Ok((sensors, beacons))
    }

    fn count_impossible(y: i64, sensors: &Vec<Sensor>, beacons: &HashSet<(i64, i64)>) -> i64 {
        let mut known_impossible = sensors.iter()
            .flat_map(|sensor| sensor.range(y))
            .collect::<Vec<_>>();

        // Sort by the range starts
        known_impossible.sort_by(|a, b| a.0.cmp(&b.0));

        let mut impossible_count = 0;

        let mut previous_end = i64::MIN;

        for (start, end) in known_impossible {
            let overlap = 
                if start > previous_end {
                    // New section begins
                    0
                } else {
                    i64::max(previous_end - start + 1, 0)
                };

            let total_width = end - start + 1;

            println!("Width {total_width}, start: {start}, end: {end}, overlap: {overlap}");

            impossible_count += i64::max(total_width - overlap, 0);
            previous_end = i64::max(previous_end, end);

            println!("Impossible: {impossible_count}, previous_end: {previous_end}")
        }

        let known_beacons = beacons.iter().filter(|beacon| beacon.1 == y).count() as i64;
        impossible_count -= known_beacons;

        return impossible_count;
    }
}

impl Solution for Day15 {
    type F = i64;
    type S = i64;

    fn name(&self) -> &'static str {
        "Day 15"
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/day15.txt")
    }

    fn part_1(&self, input: &str) -> Result<i64, AocError> {
        let (sensors, beacons) = Day15::parse(input)?;

        let count = Day15::count_impossible(2000000, &sensors, &beacons);

        Ok(count)
    }

    fn part_2(&self, input: &str) -> Result<i64, AocError> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1() {
        let input =
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n\
             Sensor at x=9, y=16: closest beacon is at x=10, y=16\n\
             Sensor at x=13, y=2: closest beacon is at x=15, y=3\n\
             Sensor at x=12, y=14: closest beacon is at x=10, y=16\n\
             Sensor at x=10, y=20: closest beacon is at x=10, y=16\n\
             Sensor at x=14, y=17: closest beacon is at x=10, y=16\n\
             Sensor at x=8, y=7: closest beacon is at x=2, y=10\n\
             Sensor at x=2, y=0: closest beacon is at x=2, y=10\n\
             Sensor at x=0, y=11: closest beacon is at x=2, y=10\n\
             Sensor at x=20, y=14: closest beacon is at x=25, y=17\n\
             Sensor at x=17, y=20: closest beacon is at x=21, y=22\n\
             Sensor at x=16, y=7: closest beacon is at x=15, y=3\n\
             Sensor at x=14, y=3: closest beacon is at x=15, y=3\n\
             Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        let (sensors, beacons) = Day15::parse(input).unwrap();

        assert_eq!(
            Day15::count_impossible(10, &sensors, &beacons),
            26
        );
    }

    #[test]
    fn it_solves_part1_real() {
        assert_eq!(Day15.part_1(Day15.default_input()), Ok(5181556));
    }
}
