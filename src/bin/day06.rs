const INPUT_FILE: &str = include_str!("../../inputs/day06.txt");

use std::collections::HashMap;

fn parse(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|initial_timer| initial_timer.parse::<usize>().unwrap())
        .collect()
}

fn part_1(input: &str, days: usize) -> usize {
    let mut fish_after_days_by_initial_timer: HashMap<usize, usize> = HashMap::new();

    for initial_timer in 1..=5 {
        let mut fish: Vec<usize> = vec![initial_timer];

        for _day in 0..days {
            fish = fish
                .into_iter()
                .flat_map(|timer| {
                    if timer == 0usize {
                        vec![6, 8]
                    } else {
                        vec![timer - 1usize]
                    }
                })
                .collect();
        }

        fish_after_days_by_initial_timer.insert(initial_timer, fish.len());
    }

    parse(input).iter()
        .map(|initial_timer| fish_after_days_by_initial_timer.get(initial_timer).unwrap())
        .sum()
}

fn part_2(input: &str, days: usize) -> i64 {
    let mut fish_after_days_by_initial_timer: HashMap<usize, i64> = HashMap::new();

    for initial_timer in 1..=5 {
        let mut fish_by_timer: HashMap<usize, i64> = HashMap::new();
        fish_by_timer.insert(initial_timer, 1);

        for _day in 0..days { 
            let mut next_fish_by_timer = fish_by_timer.clone();

            for (timer, count) in fish_by_timer.into_iter() {
                if timer == 0 {
                    *next_fish_by_timer.entry(8).or_insert(0) += count;
                    *next_fish_by_timer.entry(6).or_insert(0) += count;
                } else {
                    *next_fish_by_timer.entry(timer - 1).or_insert(0) += count;
                }

                *next_fish_by_timer.entry(timer).or_insert(0) -= count;
            }

            fish_by_timer = next_fish_by_timer;
        }

        fish_after_days_by_initial_timer.insert(initial_timer, fish_by_timer.values().sum());
    }

    parse(input).iter()
        .map(|initial_timer| fish_after_days_by_initial_timer.get(initial_timer).unwrap())
        .sum()
}

fn main() {
    let part_1_result = part_1(INPUT_FILE, 80);
    println!("[INFO]: Part 1: {:?}", part_1_result);

    let part_2_result = part_2(INPUT_FILE, 256);
    println!("[INFO]: Part 2: {:?}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_examples() {
        assert_eq!(part_1("3,4,3,1,2", 18), 26);
        assert_eq!(part_1("3,4,3,1,2", 80), 5934);
    }

    #[test]
    fn it_solves_part2_examples() {
        assert_eq!(part_2("3,4,3,1,2", 256), 26984457539);
    }
}
