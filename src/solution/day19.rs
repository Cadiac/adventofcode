use std::{hash::Hash, collections::HashSet};

use crate::solution::{AocError, Solution};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl From<&str> for Resource {
    fn from(input: &str) -> Resource {
        match input {
            "ore" => Resource::Ore,
            "clay" => Resource::Clay,
            "obsidian" => Resource::Obsidian,
            "geode" => Resource::Geode,
            _ => unimplemented!(),
        }
    }
}

enum Action {
    BuildOre,
    BuildClay,
    BuildObsidian,
    BuildGeode,
    Noop,
}

#[derive(PartialEq, Eq, Hash)]
enum Plan {
    OreRobot,
    ClayRobot,
    ObsidianRobot,
    GeodeRobot,
    Idle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Blueprint {
    id: i32,
    ore: Ores,
    clay: Ores,
    obsidian: Ores,
    geode: Ores,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Ores {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

pub struct Day19;

impl Day19 {
    fn parse(input: &str) -> Result<Vec<Blueprint>, AocError> {
        let mut blueprints = Vec::new();

        for line in input.lines() {
            let (id, ore_robot, clay_robot, obsidian_robot_ore, obsidian_robot_clay, geode_robot_ore, geode_robot_obsidian): (i32, i32, i32, i32, i32, i32, i32) = 
                serde_scan::scan!("Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian." <- line)
                    .map_err(|err| AocError::parse("input", err))?;

            blueprints.push(Blueprint {
                id,
                ore: Ores { ore: ore_robot, ..Ores::default() },
                clay: Ores { ore: clay_robot, ..Ores::default() },
                obsidian: Ores { ore: obsidian_robot_ore, clay: obsidian_robot_clay, ..Ores::default() },
                geode: Ores { ore: geode_robot_ore, obsidian: geode_robot_obsidian, ..Ores::default() },
            });
        }

        Ok(blueprints)
    }

    fn max_resource_spend(blueprint: &Blueprint) -> Ores {
        let max_ore = *[blueprint.clay.ore, blueprint.obsidian.ore, blueprint.geode.ore].iter().max().unwrap_or(&0);
        let max_clay = blueprint.obsidian.clay;
        let max_obsidian = blueprint.geode.obsidian;

        Ores { ore: max_ore, clay: max_clay, obsidian: max_obsidian, geode: 0 }
    }

    fn plan_next(blueprint: &Blueprint, inventory: Ores, production: Ores, max_spend: Ores, minute: u32, time_limit: u32) -> HashSet<(Plan, u32)> {
        let mut plans: HashSet<(Plan, u32)> = HashSet::new();

        // TODO: Consider if the current inventory is enough to run the production until time runs out, if so don't expand

        if production.obsidian > 0 {
            // Consider how long does it take to produce resources for this and skip to that time
            let required_ore = i32::max(blueprint.geode.ore - inventory.ore, 0);
            let required_obsidian = i32::max(blueprint.geode.obsidian - inventory.obsidian, 0);

            let minutes_to_produce = i32::max((required_ore + production.ore - 1) / production.ore, (required_obsidian + production.obsidian - 1) / production.obsidian);

            if minutes_to_produce == 0 {
                plans.insert((Plan::GeodeRobot, 0));
            } else {
                if minute + (minutes_to_produce as u32) < time_limit {
                    plans.insert((Plan::GeodeRobot, minutes_to_produce as u32));
                } else {
                    // assert!(time_limit > minute, "{}, {}", time_limit, minute);
                    plans.insert((Plan::Idle, time_limit - minute));
                }
            }
        }

        if production.ore < max_spend.ore {
            // Consider how long does it take to produce resources for this and skip to that time
            let required_resources = i32::max(blueprint.ore.ore - inventory.ore, 0);
            let minutes_to_produce = (required_resources + production.ore - 1) / production.ore;

            if minutes_to_produce == 0 {
                plans.insert((Plan::OreRobot, 0));
            } else {
                if minute + (minutes_to_produce as u32) < time_limit {
                    plans.insert((Plan::OreRobot, minutes_to_produce as u32));
                } else {
                    plans.insert((Plan::Idle, time_limit - minute));
                }
            }
        }

        if production.clay < max_spend.clay {
            // Consider how long does it take to produce resources for this and skip to that time
            let required_resources = i32::max(blueprint.clay.ore - inventory.ore, 0);
            let minutes_to_produce = (required_resources + production.ore - 1) / production.ore;

            if minutes_to_produce == 0 {
                plans.insert((Plan::ClayRobot, 0));
            } else {
                if minute + (minutes_to_produce as u32) < time_limit {
                    plans.insert((Plan::ClayRobot, minutes_to_produce as u32));
                } else {
                    plans.insert((Plan::Idle, time_limit - minute));
                }
            }
        }

        if production.clay > 0 && production.obsidian < max_spend.obsidian {
            // Consider how long does it take to produce resources for this and skip to that time
            let required_ore = i32::max(blueprint.obsidian.ore - inventory.ore, 0);
            let required_clay = i32::max(blueprint.obsidian.clay - inventory.clay, 0);

            let minutes_to_produce = i32::max((required_ore + production.ore - 1) / production.ore, (required_clay + production.clay - 1) / production.clay);

            if minutes_to_produce == 0 {
                plans.insert((Plan::ObsidianRobot, 0));
            } else {
                if minute + (minutes_to_produce as u32) < time_limit {
                    plans.insert((Plan::ObsidianRobot, minutes_to_produce as u32));
                } else {
                    plans.insert((Plan::Idle, time_limit - minute));
                }
            }
        }

        plans
    }

    fn simulate(blueprint: Blueprint, time_limit: u32) -> i32 {
        // Determine how much resources can be spent in a minute at max. Building more doesn't help
        let max_spend = Day19::max_resource_spend(&blueprint);

        let production = Ores { ore: 1, clay: 0, obsidian: 0, geode: 0 };
        let inventory = Ores { ore: 0, clay: 0, obsidian: 0, geode: 0 };

        Day19::run(blueprint, max_spend, production, inventory, time_limit, 1)
    }

    fn run(blueprint: Blueprint, max_spend: Ores, production: Ores, inventory: Ores, time_limit: u32, minute: u32) -> i32 {
        if minute > time_limit {
            return inventory.geode;
        }
    
        let mut best = 0;
       
        let plans = Day19::plan_next(&blueprint, inventory, production, max_spend, minute, time_limit);

        // Consider each plan
        for (plan, skipped_minutes) in plans {
            let mut next_inventory = inventory;
            let mut next_production = production;
            let mut next_minute = minute;
            
            if skipped_minutes <= 0 {
                // TOOD: why
                next_minute += skipped_minutes;
            } else {
                next_minute += skipped_minutes;
            }

            // Resources produced during the skipped minutes
            next_inventory.ore += production.ore * skipped_minutes as i32;
            next_inventory.clay += production.clay * skipped_minutes as i32;
            next_inventory.obsidian += production.obsidian * skipped_minutes as i32;
            next_inventory.geode += production.geode * skipped_minutes as i32;
    
            // Pay the costs
            match plan {
                Plan::GeodeRobot => {
                    next_inventory.ore -= blueprint.geode.ore;
                    next_inventory.obsidian -= blueprint.geode.obsidian;
                },
                Plan::ObsidianRobot => {
                    next_inventory.ore -= blueprint.obsidian.ore;
                    next_inventory.clay -= blueprint.obsidian.clay;
                },
                Plan::ClayRobot => {
                    next_inventory.ore -= blueprint.clay.ore;
                },
                Plan::OreRobot => {
                    next_inventory.ore -= blueprint.ore.ore;
                },
                Plan::Idle => ()
            };
    
            // Production phase between build and it finishing
            // TODO: Combine this to the skipped production and just skip to this point
            next_inventory.ore += production.ore;
            next_inventory.clay += production.clay;
            next_inventory.obsidian += production.obsidian;
            next_inventory.geode += production.geode;
    
            // println!("Collected {production:?}; you now have {inventory:?}.");
    
            // Finish building whatever we were building
            match plan {
                Plan::GeodeRobot => next_production.geode += 1,
                Plan::ObsidianRobot => next_production.obsidian += 1,
                Plan::ClayRobot => next_production.clay += 1,
                Plan::OreRobot => next_production.ore += 1,
                Plan::Idle => ()
            };
    
            let total_geodes = Day19::run(blueprint, max_spend, next_production, next_inventory, time_limit, next_minute + 1);
            if total_geodes > best {
                best = total_geodes;
            }
        }

        best
    }
}

impl Solution for Day19 {
    type F = i32;
    type S = i32;

    fn name(&self) -> &'static str {
        "Day 19"
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/day19.txt")
    }

    fn part_1(&self, input: &str) -> Result<i32, AocError> {
        let blueprints = Day19::parse(input)?;
        Ok(blueprints.into_iter().map(|blueprint| {
            let id = blueprint.id;
            let geodes = Day19::simulate(blueprint, 24);
            println!("Blueprint {}: found {geodes} geodes", id);

            geodes * id
        }).sum())
    }

    fn part_2(&self, input: &str) -> Result<i32, AocError> {
        let blueprints = Day19::parse(input)?;
        Ok(blueprints.into_iter().take(3).map(|blueprint| {
            let id = blueprint.id;
            let geodes = Day19::simulate(blueprint, 32);
            println!("Blueprint {}: found {geodes} geodes", id);

            geodes * id
        }).product())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Blueprint 1: \
            Each ore robot costs 4 ore. \
            Each clay robot costs 2 ore. \
            Each obsidian robot costs 3 ore and 14 clay. \
            Each geode robot costs 2 ore and 7 obsidian.\n\
        Blueprint 2: \
            Each ore robot costs 2 ore. \
            Each clay robot costs 3 ore. \
            Each obsidian robot costs 3 ore and 8 clay. \
            Each geode robot costs 3 ore and 12 obsidian.";

    #[ignore]
    #[test]
    fn it_solves_part1_first() {
        assert_eq!(Day19.part_1(INPUT.lines().nth(0).unwrap()), Ok(9));
    }

    #[ignore]
    #[test]
    fn it_solves_part1_second() {
        assert_eq!(Day19.part_1(INPUT.lines().nth(1).unwrap()), Ok(24));
    }

    #[ignore]
    #[test]
    fn it_solves_part1_full() {
        assert_eq!(Day19.part_1(INPUT), Ok(33));
    }

    #[ignore]
    #[test]
    fn it_optimizes_part1_first() {
        let geodes = Day19::simulate(Day19::parse(INPUT).unwrap()[0].clone(), 24);
        assert_eq!(geodes, 9);
    }

    #[ignore]
    #[test]
    fn it_optimizes_part1_second() {
        let geodes = Day19::simulate(Day19::parse(INPUT).unwrap()[1].clone(), 24);
        assert_eq!(geodes, 12);
    }

    #[ignore]
    #[test]
    fn it_optimizes_part2_first() {
        let geodes = Day19::simulate(Day19::parse(INPUT).unwrap()[0].clone(), 32);
        assert_eq!(geodes, 56);
    }

    #[test]
    fn it_optimizes_part2_second() {
        let geodes = Day19::simulate(Day19::parse(INPUT).unwrap()[1].clone(), 32);
        assert_eq!(geodes, 62);
    }
}
