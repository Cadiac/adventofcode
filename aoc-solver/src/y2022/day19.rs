use crate::solution::{AocError, Solution};

type Resources = [u32; 4];
type Resource = usize;

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Blueprint {
    id: u32,
    costs: [[u32; 3]; 4],
}

pub struct Day19;

impl Day19 {
    fn parse(input: &str) -> Result<Vec<Blueprint>, AocError> {
        let mut blueprints = Vec::new();

        for line in input.lines() {
            let (id, ore_ore, clay_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian): (
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            ) = serde_scan::scan!("Blueprint {}: \
                    Each ore robot costs {} ore. \
                    Each clay robot costs {} ore. \
                    Each obsidian robot costs {} ore and {} clay. \
                    Each geode robot costs {} ore and {} obsidian." <- line)
            .map_err(|err| AocError::parse("input", err))?;

            let costs = [
                [ore_ore, 0, 0],
                [clay_ore, 0, 0],
                [obsidian_ore, obsidian_clay, 0],
                [geode_ore, 0, geode_obsidian],
            ];

            blueprints.push(Blueprint { id, costs });
        }

        Ok(blueprints)
    }

    fn max_resource_spend(blueprint: &Blueprint) -> Resources {
        let max_ore_spend = *[
            blueprint.costs[CLAY][ORE],
            blueprint.costs[OBSIDIAN][ORE],
            blueprint.costs[GEODE][ORE],
        ]
        .iter()
        .max()
        .unwrap_or(&0);

        let max_clay_spend = blueprint.costs[OBSIDIAN][CLAY];
        let max_obsidian_spend = blueprint.costs[GEODE][OBSIDIAN];

        [max_ore_spend, max_clay_spend, max_obsidian_spend, 1000]
    }

    fn plan_next(
        blueprint: &Blueprint,
        inventory: &Resources,
        production: &Resources,
        max_spend: &Resources,
        minute: u32,
        time_limit: u32,
    ) -> Vec<(Option<Resource>, u32)> {
        let mut plans: Vec<(Option<Resource>, u32)> = Vec::new();
        let remaining_time = time_limit - minute;

        for resource in ORE..=GEODE {
            // Don't consider producing more if the production meets max spending
            if production[resource] >= max_spend[resource] {
                continue;
            }

            // If the current inventory is enough to keep expanding until time runs out don't expand
            let production_shortage = max_spend[resource] - production[resource];
            if inventory[resource] >= production_shortage * remaining_time {
                continue;
            }

            // Consider how long does it take to produce resources for this robot and skip to that time
            let durations: Vec<u32> = blueprint.costs[resource]
                .iter()
                .enumerate()
                .flat_map(|(ingredient, cost)| {
                    if *cost <= inventory[ingredient] {
                        return Some(0);
                    }

                    if production[ingredient] == 0 {
                        return None;
                    }

                    let required = cost.saturating_sub(inventory[ingredient]);
                    Some((required + production[ingredient] - 1) / production[ingredient])
                })
                .collect();

            let can_produce_ingredients = durations.len() == 3;

            if can_produce_ingredients {
                let minutes_to_produce = durations.iter().max().unwrap();

                if minute + minutes_to_produce < time_limit {
                    plans.push((Some(resource), *minutes_to_produce));
                }
            }
        }

        // If nothing else just idle to the end
        if plans.is_empty() {
            plans.push((None, remaining_time));
        }

        plans
    }

    fn simulate(blueprint: &Blueprint, time_limit: u32) -> u32 {
        // Determine how much resources can be spent in a minute at max. Building more doesn't help
        let max_spend = Day19::max_resource_spend(blueprint);

        let production = [1, 0, 0, 0];
        let inventory = [0, 0, 0, 0];

        Day19::run(
            blueprint,
            &max_spend,
            &production,
            &inventory,
            time_limit,
            1,
        )
    }

    fn run(
        blueprint: &Blueprint,
        max_spend: &Resources,
        production: &Resources,
        inventory: &Resources,
        time_limit: u32,
        minute: u32,
    ) -> u32 {
        if minute > time_limit {
            return inventory[GEODE];
        }

        let mut most_geodes = 0;

        let plans = Day19::plan_next(
            blueprint, inventory, production, max_spend, minute, time_limit,
        );

        // Consider each plan
        for (plan, skipped_minutes) in plans {
            let mut next_inventory = *inventory;
            let mut next_production = *production;
            let next_minute = minute + skipped_minutes;

            // Resources produced during the skipped minutes and this minute
            for resource in ORE..=GEODE {
                next_inventory[resource] += production[resource] * (skipped_minutes + 1);
            }

            // Pay the costs and finish building whatever we were building if any
            if let Some(planned) = plan {
                for (resource, robot_cost) in blueprint.costs[planned].iter().enumerate() {
                    next_inventory[resource] -= robot_cost;
                }
                next_production[planned] += 1
            }

            // DFS
            let total_geodes = Day19::run(
                blueprint,
                max_spend,
                &next_production,
                &next_inventory,
                time_limit,
                next_minute + 1,
            );
            if total_geodes > most_geodes {
                most_geodes = total_geodes;
            }
        }

        most_geodes
    }
}

impl Solution for Day19 {
    type A = u32;
    type B = u32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2022/day19.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let blueprints = Day19::parse(input)?;

        Ok(blueprints
            .into_iter()
            .map(|blueprint| Day19::simulate(&blueprint, 24) * blueprint.id)
            .sum())
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        let blueprints = Day19::parse(input)?;
        Ok(blueprints
            .into_iter()
            .take(3)
            .map(|blueprint| Day19::simulate(&blueprint, 32))
            .product())
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
        assert_eq!(Day19.part_1(INPUT.lines().next().unwrap()), Ok(9));
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
        let blueprint = Day19::parse(INPUT).unwrap()[0].clone();
        let geodes = Day19::simulate(&blueprint, 24);
        assert_eq!(geodes, 9);
    }

    #[test]
    fn it_optimizes_part1_second() {
        let blueprint = Day19::parse(INPUT).unwrap()[1].clone();
        let geodes = Day19::simulate(&blueprint, 24);
        assert_eq!(geodes, 12);
    }

    #[test]
    fn it_optimizes_part2_first() {
        let blueprint = Day19::parse(INPUT).unwrap()[0].clone();
        let geodes = Day19::simulate(&blueprint, 32);
        assert_eq!(geodes, 56);
    }

    #[test]
    fn it_optimizes_part2_second() {
        let blueprint = Day19::parse(INPUT).unwrap()[1].clone();
        let geodes = Day19::simulate(&blueprint, 32);
        assert_eq!(geodes, 62);
    }
}
