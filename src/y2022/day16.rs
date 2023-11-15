use std::collections::{HashMap, HashSet};
use std::{cmp::Ordering, collections::BinaryHeap};

use crate::solution::{AocError, Solution};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: u32,
    tunnels: HashMap<&'a str, u32>,
    distances: HashMap<&'a str, u32>,
}

#[derive(Clone, Eq, PartialEq)]
struct Search<'a> {
    valve: &'a str,
    distance: u32,
}

impl<'a> Ord for Search<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<'a> PartialOrd for Search<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Use dijkstra to determine the distances between valves
fn dijkstra(valves: &HashMap<&str, Valve>, source: &str, target: &str) -> Option<u32> {
    let mut dist: HashMap<&str, u32> = HashMap::new();
    let mut heap: BinaryHeap<Search> = BinaryHeap::new();

    *dist.entry(source).or_insert(0) = 0;

    heap.push(Search {
        distance: 0,
        valve: source,
    });

    while let Some(Search { valve, distance }) = heap.pop() {
        if valve == target {
            return Some(dist[valve]);
        }

        // we've already found a better way
        if distance > *dist.get(valve).unwrap_or(&u32::MAX) {
            continue;
        }

        for (tunnel, length) in valves[valve].tunnels.iter() {
            let next = Search {
                distance: distance + length,
                valve: tunnel,
            };

            if next.distance < *dist.get(tunnel).unwrap_or(&u32::MAX) {
                *dist.entry(tunnel).or_insert(0) = next.distance;
                heap.push(next);
            }
        }
    }

    None
}

pub struct Day16;

impl Day16 {
    fn parse(input: &str) -> Result<Vec<Valve>, AocError> {
        let mut valves = HashMap::new();

        for line in input.lines() {
            let (name, flow_rate, tunnels): (&str, u32, &str) =
                serde_scan::scan!("Valve {} has flow rate={}; tunnels lead to valves {}" <- line)
                    .map_err(|err| AocError::parse(line, err))?;

            let valve = Valve {
                name,
                flow_rate,
                tunnels: tunnels.split(", ").map(|s| (s, 1)).collect(),
                distances: HashMap::new(),
            };

            valves.insert(name, valve);
        }

        let v = valves.clone();
        for (target, valve) in valves.iter_mut() {
            for source in v.clone().into_keys() {
                if let Some(distance) = dijkstra(&v, source, target) {
                    if distance != 0 {
                        valve.distances.insert(source, distance);
                    }
                }
            }
        }

        // Simplify the graph - get rid of any where the rate is zero
        while let Some(to_remove) = valves
            .keys()
            .find(|name| valves[*name].flow_rate == 0)
            .cloned()
        {
            if let Some(removed) = valves.remove(&to_remove) {
                for new_source in removed.tunnels.keys() {
                    if let Some(source) = valves.get_mut(new_source) {
                        let length_to_removed = source.tunnels.remove(&removed.name).unwrap_or(1);
                        for (target, length) in removed.tunnels.iter() {
                            if new_source != target {
                                *source.tunnels.entry(target).or_insert(0) +=
                                    length + length_to_removed;
                            }
                        }
                    }
                }
            }
        }

        Ok(valves.into_values().collect())
    }

    fn find_best_actions(
        activated: &mut HashSet<usize>,
        current: &str,
        remaining: u32,
        pressure_released: u32,
        valves: &[&Valve],
    ) -> u32 {
        if activated.len() == valves.len() || remaining == 0 {
            return pressure_released;
        }

        let mut best = 0;

        // Consider each unactivated valve as the next destination
        for next in 0..valves.len() {
            if !activated.contains(&next) {
                activated.insert(next);

                // Moving costs time
                let target = valves[next];
                let distance = target.distances.get(current).unwrap();

                // Spend one minute per step moving + 1 minute on arrival to open the valve
                let next_remaining = remaining.saturating_sub(distance + 1);

                // The valve will now release pressure for the remaining time
                let next_pressure_released = pressure_released + next_remaining * target.flow_rate;

                let pressure_released = Day16::find_best_actions(
                    activated,
                    valves[next].name,
                    next_remaining,
                    next_pressure_released,
                    valves,
                );

                best = best.max(pressure_released);
                activated.remove(&next);
            }
        }
        best
    }

    fn find_with_elephant(valves: &[Valve]) -> u32 {
        // My original solution would run for many hours, finding the best solution after ~2 hours and I got the star.
        // This solution was a bruteforce TSP based on the part 1.

        // This faster approach is based on an observation by /u/nirgle on Reddit:
        // Consider the pressure released by the elephant separately, visiting all the
        // valves that the protagonist doesn't visit. To do this efficiently,
        // precalculate the 26min scores for each combination of valves opened first,
        // and use each combination and its opposite to find the max pressure released.

        let valve_count = valves.len();
        let valve_opening_combinations = 1 << valve_count;

        let mut pressure_by_combination = Vec::with_capacity(valve_opening_combinations as usize);

        for combination in 0..valve_opening_combinations {
            let mut combination_valves = Vec::with_capacity(valve_count);

            for (valve_bit, valve) in valves.iter().enumerate() {
                if (combination & (1 << valve_bit)) != 0 {
                    combination_valves.push(valve);
                }
            }

            let pressure_released =
                Day16::find_best_actions(&mut HashSet::new(), "AA", 26, 0, &combination_valves);

            pressure_by_combination.push(pressure_released);
        }

        let mut best = 0;

        for combination in 0..valve_opening_combinations / 2 {
            let mask = (1 << valve_count) - 1;
            let inverse = !combination & mask;

            let protagonist_released = pressure_by_combination[combination as usize];
            let elephant_released = pressure_by_combination[inverse as usize];

            let total = protagonist_released + elephant_released;

            best = best.max(total)
        }

        best
    }
}

impl Solution for Day16 {
    type F = u32;
    type S = u32;

    fn meta(&self) -> (u32, u32) {
        (16, 2022)
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/2022/day16.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let valves = Day16::parse(input)?;
        let valves: Vec<&Valve> = valves.iter().collect();

        let pressure_released = Day16::find_best_actions(&mut HashSet::new(), "AA", 30, 0, &valves);

        Ok(pressure_released)
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        let valves = Day16::parse(input)?;

        let pressure_released = Day16::find_with_elephant(&valves);

        Ok(pressure_released)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\n\
        Valve BB has flow rate=13; tunnels lead to valves CC, AA\n\
        Valve CC has flow rate=2; tunnels lead to valves DD, BB\n\
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE\n\
        Valve EE has flow rate=3; tunnels lead to valves FF, DD\n\
        Valve FF has flow rate=0; tunnels lead to valves EE, GG\n\
        Valve GG has flow rate=0; tunnels lead to valves FF, HH\n\
        Valve HH has flow rate=22; tunnels lead to valves GG\n\
        Valve II has flow rate=0; tunnels lead to valves AA, JJ\n\
        Valve JJ has flow rate=21; tunnels lead to valves II";

    #[test]
    fn it_solves_part1() {
        assert_eq!(Day16.part_1(INPUT), Ok(1651));
    }

    #[test]
    fn it_solves_part2() {
        assert_eq!(Day16.part_2(INPUT), Ok(1707));
    }
}
