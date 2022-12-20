use std::{hash::Hash};

use cached::proc_macro::cached;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Blueprint {
    id: i32,
    ore: i32,
    clay: i32,
    obsidian: (i32, i32),
    geode: (i32, i32,)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Ores {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

#[cached(size=100000000)]
fn run(blueprint: Blueprint, production: Ores, inventory: Ores, minute: u32) -> i32 {
    if minute > 24 {
        return inventory.geode;
    }

    let mut best = 0;

    // println!("== Minute {} ==", minute + 1);

    let actions = Day19::valid_actions(&blueprint, inventory);

    // Consider each action
    for action in actions {
        let mut next_inventory = inventory;
        let mut next_production = production;

        match action {
            Action::BuildGeode => {
                next_inventory.ore -= blueprint.geode.0;
                next_inventory.obsidian -= blueprint.geode.1;
            },
            Action::BuildObsidian => {
                next_inventory.ore -= blueprint.obsidian.0;
                next_inventory.clay -= blueprint.obsidian.1;
            },
            Action::BuildClay => {
                next_inventory.ore -= blueprint.clay;
            },
            Action::BuildOre => {
                next_inventory.ore -= blueprint.ore;
            },
            Action::Noop => {}
        };

        // Production phase
        next_inventory.ore += production.ore;
        next_inventory.clay += production.clay;
        next_inventory.obsidian += production.obsidian;
        next_inventory.geode += production.geode;

        // println!("Collected {production:?}; you now have {inventory:?}.");

        // Finish building whatever we were building
        match action {
            Action::BuildGeode => next_production.geode += 1,
            Action::BuildObsidian => next_production.obsidian += 1,
            Action::BuildClay => next_production.clay += 1,
            Action::BuildOre => next_production.ore += 1,
            Action::Noop => {}
        };

        let total_geodes = run(blueprint, next_production, next_inventory, minute + 1);
        if total_geodes > best {
            best = total_geodes;
        }
    }

    best
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
                ore: ore_robot,
                clay: clay_robot,
                obsidian: (obsidian_robot_ore, obsidian_robot_clay),
                geode: (geode_robot_ore, geode_robot_obsidian),
            });
        }

        Ok(blueprints)
    }

    fn valid_actions(blueprint: &Blueprint, inventory: Ores) -> Vec<Action> {
        let mut actions = Vec::new();

        let can_produce_geode_robot = inventory.ore >= blueprint.geode.0 && inventory.obsidian >= blueprint.geode.1;
        if can_produce_geode_robot {
            actions.push(Action::BuildGeode);
            return actions;
        }

        let can_produce_obsidian_robot = inventory.ore >= blueprint.obsidian.0 && inventory.clay >= blueprint.obsidian.1;
        if can_produce_obsidian_robot {
            actions.push(Action::BuildObsidian);
        }

        let can_produce_clay_robot = inventory.ore >= blueprint.clay;
        if can_produce_clay_robot {
            actions.push(Action::BuildClay);
        }

        let can_produce_ore_robot = inventory.ore >= blueprint.ore;
        if can_produce_ore_robot {
            actions.push(Action::BuildOre);
        }

        actions.push(Action::Noop);

        actions
    }

    fn simulate(blueprint: Blueprint) -> i32 {
        run(blueprint, Ores { ore: 1, clay: 0, obsidian: 0, geode: 0 }, Ores { ore: 0, clay: 0, obsidian: 0, geode: 0 }, 1)
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
            let geodes = Day19::simulate(blueprint);
            println!("Blueprint {}: found {geodes} geodes", id);

            geodes * id
        }).sum())
    }

    fn part_2(&self, input: &str) -> Result<i32, AocError> {
        unimplemented!();
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

    #[test]
    fn it_solves_part1_first() {
        assert_eq!(Day19.part_1(INPUT.lines().nth(0).unwrap()), Ok(9));
    }

    #[test]
    fn it_solves_part1_second() {
        assert_eq!(Day19.part_1(INPUT.lines().nth(1).unwrap()), Ok(24));
    }

    #[test]
    fn it_solves_part1_full() {
        assert_eq!(Day19.part_1(INPUT), Ok(33));
    }

    #[test]
    fn it_optimizes_part1_first() {
        let geodes = Day19::simulate(Day19::parse(INPUT).unwrap()[0].clone());
        assert_eq!(geodes, 9);
    }

    #[test]
    fn it_optimizes_part1_second() {
        let geodes = Day19::simulate(Day19::parse(INPUT).unwrap()[1].clone());
        assert_eq!(geodes, 12);
    }
}
