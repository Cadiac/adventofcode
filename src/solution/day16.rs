use std::collections::{HashMap, HashSet, BTreeSet};
use std::{cmp::Ordering, collections::BinaryHeap};
use std::hash::{Hash, Hasher};

use crate::solution::{AocError, Solution};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Valve {
    name: String,
    flow_rate: u32,
    tunnels: HashMap<String, u32>,
    distances: HashMap<String, u32>,
}

impl Hash for Valve {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Search {
    valve: String,
    distance: u32,
}

impl Ord for Search {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Search {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Use dijkstra to determine the distances between valves
fn dijkstra(valves: HashMap<String, Valve>, source: String, target: String) -> Option<u32> {
    let mut dist: HashMap<String, u32> = HashMap::new();
    let mut heap: BinaryHeap<Search> = BinaryHeap::new();

    *dist.entry(source.clone()).or_insert(0) = 0;

    heap.push(Search {
        distance: 0,
        valve: source,
    });

    while let Some(Search { valve, distance }) = heap.pop() {
        if valve == target {
            return Some(dist[&valve]);
        }

        // we've already found a better way
        if distance > *dist.get(&valve).unwrap_or(&u32::MAX) {
            continue;
        }

        for (tunnel, length) in valves[&valve].tunnels.iter() {
            let next = Search {
                distance: distance + length,
                valve: tunnel.clone(),
            };

            if next.distance < *dist.get(tunnel).unwrap_or(&u32::MAX) {
                *dist.entry(tunnel.clone()).or_insert(0) = next.distance;
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
            let (name, flow_rate, tunnels): (String, u32, String) =
                // TODO: parsing fuckery, tunnel vs tunnels in the real input
                serde_scan::scan!("Valve {} has flow rate={}; tunnels lead to valves {}" <- line)
                    .map_err(|err| AocError::parse(line, err))?;

            let valve = Valve {
                name: name.clone(),
                flow_rate,
                tunnels: tunnels.split(", ").map(|s| (s.to_owned(), 1)).collect(),
                distances: HashMap::new(),
            };

            valves.insert(name, valve);
        }

        // Simplify the graph - get rid of any nodes except 'AA' where the rate is zero
        while let Some(to_remove) = valves
            .keys()
            .find(|name| *name != "AA" && valves[*name].flow_rate == 0)
            .cloned()
        {
            if let Some(removed) = valves.remove(&to_remove) {
                for new_source in removed.tunnels.keys() {
                    if let Some(source) = valves.get_mut(new_source) {
                        let length_to_removed = source.tunnels.remove(&removed.name).unwrap_or(1);
                        for (target, length) in removed.tunnels.iter() {
                            if new_source != target {
                                *source.tunnels.entry(target.clone()).or_insert(0) +=
                                    length + length_to_removed;
                            }
                        }
                    }
                }
            }
        }

        // Calculate distances from every node to every node in graph
        let v = valves.clone();
        for (source, valve) in valves.iter_mut() {
            for target in v.clone().into_keys() {
                if let Some(distance) = dijkstra(v.clone(), source.clone(), target.clone()) {
                    if distance != 0 {
                        valve.distances.insert(target, distance);
                    }
                }
            }
        }

        // Force "AA" to the beginning as the first node
        let mut valves: Vec<_> = valves.into_values().collect();
        valves.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(valves)
    }

    fn find_path(
        path: &mut Vec<usize>,
        activated: &mut HashSet<usize>,
        current: usize,
        minute: u32,
        pressure_released: u32,
        valves: &[Valve],
    ) -> u32 {
        if activated.len() == valves.len() || minute >= 30 {
            return pressure_released;
        }

        let mut best = 0;

        // Consider each unactivated valve as the next destination
        for next in 0..valves.len() {
            if !activated.contains(&next) {
                path.push(next);
                activated.insert(next);

                // Moving costs time
                let target = valves[next].clone();
                let distance = valves[current].distances.get(&target.name).unwrap();

                // Spend one minute per step moving + 1 minute on arrival to open the valve
                let next_minute = u32::min(minute + distance + 1, 30);

                // The valve will now release pressure for the remaining time
                let next_pressure_released =
                    pressure_released + (30 - next_minute) * target.flow_rate;

                let pressure_released = Day16::find_path(
                    path,
                    activated,
                    next,
                    next_minute,
                    next_pressure_released,
                    valves,
                );
                if pressure_released > best {
                    best = pressure_released;
                }
                activated.remove(&next);
                path.pop();
            }
        }
        best
    }

    fn tsp(valves: Vec<Valve>) -> (Vec<usize>, u32) {
        // Initialize the path and activated set
        let mut path = vec![0];
        let mut activated = HashSet::new();
        activated.insert(0);

        // Call the recursive function to find path to release most pressure
        let best_pressure_released = Day16::find_path(&mut path, &mut activated, 0, 0, 0, &valves);
        (path, best_pressure_released)
    }

    fn find_with_elephant(
        activated: &mut BTreeSet<usize>,
        current: (usize, usize),
        mut cooldown: (u32, u32),
        minute: u32,
        pressure_released: u32,
        valves: &[Valve],
    ) -> u32 {
        let time_limit = 26;
    
        if activated.len() == valves.len() || minute >= time_limit {
            return pressure_released;
        }
    
        let mut best = 0;        
    
        // advance time by a minute
        // minute += 1;
    
        if cooldown.0 > 0 {
            cooldown.0 -= 1;
        }
    
        if cooldown.1 > 0 {
            cooldown.1 -= 1;
        }
    
        if cooldown.0 > 0 && cooldown.1 > 0 {
            // Both still just moving or opening the valve ("on cooldown"), no new choices
            return Day16::find_with_elephant(activated, current, cooldown, minute + 1, pressure_released, valves);
        }
    
        if cooldown.0 == 0 && cooldown.1 > 0 {
            // One is ready to make a choice
            // Consider each unactivated valve as the next destination
            for next in 0..valves.len() {
                if !activated.contains(&next) {
                    activated.insert(next);
    
                    // Moving costs time
                    let target = valves[next].clone();
                    let distance = valves[current.0].distances.get(&target.name).unwrap();
    
                    // Spend one minute per step moving + 1 minute on arrival to open the valve
                    let duration_to_open = distance + 1;
    
                    // The valve will now release pressure for the remaining time
                    let next_pressure_released =
                        pressure_released + u32::max(time_limit - minute - duration_to_open, 0) * target.flow_rate;
    
                    let pressure_released = Day16::find_with_elephant(
                        activated,
                        (next, current.1),
                        (duration_to_open, cooldown.1),
                        minute + 1,
                        next_pressure_released,
                        valves,
                    );
                    if pressure_released > best {
                        best = pressure_released;
                    }
                    activated.remove(&next);
                }
            }
    
        } else if cooldown.0 > 0 && cooldown.1 == 0 {
            // Second is ready to make a choice
            // Consider each unactivated valve as the next destination
            for next in 0..valves.len() {
                if !activated.contains(&next) {
                    activated.insert(next);
    
                    // Moving costs time
                    let target = valves[next].clone();
                    let distance = valves[current.1].distances.get(&target.name).unwrap();
    
                    // Spend one minute per step moving + 1 minute on arrival to open the valve
                    let duration_to_open = distance + 1;
    
                    // The valve will now release pressure for the remaining time
                    let next_pressure_released =
                        pressure_released + u32::max(time_limit - minute - duration_to_open, 0) * target.flow_rate;
    
                    let pressure_released = Day16::find_with_elephant(
                        activated,
                        (current.0, next),
                        (cooldown.0, duration_to_open),
                        minute + 1,
                        next_pressure_released,
                        valves,
                    );
                    if pressure_released > best {
                        best = pressure_released;
                    }
                    activated.remove(&next);
                }
            }
    
        } else if cooldown.0 == 0 && cooldown.1 == 0 {
            // Both are ready to make choices
            for next_1 in 0..valves.len() {
                if !activated.contains(&next_1) {
                    activated.insert(next_1);
    
                    for next_2 in 0..valves.len() {
                        if !activated.contains(&next_2) {
                            activated.insert(next_2);
    
                            // TODO: be smart about this, move with the one closer to the target
    
                            // Moving costs time
                            let target_1 = valves[next_1].clone();
                            let distance_1 = valves[current.0].distances.get(&target_1.name).unwrap();
    
                            let target_2 = valves[next_2].clone();
                            let distance_2 = valves[current.1].distances.get(&target_2.name).unwrap();
    
                            // Spend one minute per step moving + 1 minute on arrival to open the valve
                            let duration_to_open_1 = distance_1 + 1;
                            let duration_to_open_2 = distance_2 + 1;
    
                            // The valve will now release pressure for the remaining time
                            let next_pressure_released =
                                pressure_released + u32::max(time_limit - minute - duration_to_open_1, 0) * target_1.flow_rate
                                    + u32::max(time_limit - minute - duration_to_open_2, 0) * target_2.flow_rate;
    
                            let pressure_released = Day16::find_with_elephant(
                                activated,
                                (next_1, next_2),
                                (duration_to_open_1, duration_to_open_2),
                                minute + 1,
                                next_pressure_released,
                                valves,
                            );
                            if pressure_released > best {
                                best = pressure_released;
                            }
                            activated.remove(&next_2);
                        }
                    }
    
                    activated.remove(&next_1);
                }
            }
    
            // There's only one more choice to be made.
            // One will just sit idle.
            // Figure out which one needs to make this choice
            if valves.len() - activated.len() == 1 {
                for next in 0..valves.len() {
                    if !activated.contains(&next) {
                        activated.insert(next);
    
                        // Moving costs time
                        let target = valves[next].clone();
                        let distance = valves[current.0].distances.get(&target.name).unwrap();
        
    
                        // Spend one minute per step moving + 1 minute on arrival to open the valve
                        let duration_to_open = distance + 1;
    
                        // The valve will now release pressure for the remaining time
                        let next_pressure_released =
                            pressure_released + u32::max(time_limit - minute - duration_to_open, 0) * target.flow_rate;
    
                        let pressure_released = Day16::find_with_elephant(
                            activated,
                            (next, current.1),
                            (duration_to_open, cooldown.1),
                            minute + 1,
                            next_pressure_released,
                            valves,
                        );
                        if pressure_released > best {
                            best = pressure_released;
                        }
                        activated.remove(&next);
                    }
                }
    
                for next in 0..valves.len() {
                    if !activated.contains(&next) {
                        activated.insert(next);
    
                        // Moving costs time
                        let target = valves[next].clone();
                        let distance = valves[current.1].distances.get(&target.name).unwrap();
    
                        // Spend one minute per step moving + 1 minute on arrival to open the valve
                        let duration_to_open = distance + 1;
    
                        // The valve will now release pressure for the remaining time
                        let next_pressure_released =
                            pressure_released + u32::max(time_limit - minute - duration_to_open, 0) * target.flow_rate;
    
                        let pressure_released = Day16::find_with_elephant(
                            activated,
                            (current.0, next),
                            (cooldown.0, duration_to_open),
                            minute + 1,
                            next_pressure_released,
                            valves,
                        );
                        if pressure_released > best {
                            best = pressure_released;
                        }
                        activated.remove(&next);
                    }
                }
            }
        }
    
        best
    }    

    fn tsp_with_elephant(valves: Vec<Valve>) -> u32 {
        let mut activated = BTreeSet::new();
        activated.insert(0);

        // Call the recursive function to find path to release most pressure
        let best_pressure_released = Day16::find_with_elephant(&mut activated, (0, 0), (0, 0), 0, 0, &valves);
        best_pressure_released
    }
}

impl Solution for Day16 {
    type F = u32;
    type S = u32;

    fn name(&self) -> &'static str {
        "Day 16"
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/day16.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let valves = Day16::parse(input)?;

        let (route, released) = Day16::tsp(valves);

        Ok(released)
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        let valves = Day16::parse(input)?;

        let released = Day16::tsp_with_elephant(valves);

        Ok(released)
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

    #[ignore]
    #[test]
    fn it_solves_part2() {
        assert_eq!(Day16.part_2(INPUT), Ok(1707));
    }
}