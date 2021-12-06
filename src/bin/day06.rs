const INPUT_FILE: &str = include_str!("../../inputs/day06.txt");

fn parse(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|initial_timer| initial_timer.parse::<usize>().unwrap())
        .collect()
}

// Each fish produces offspring independent of other fish, so each fish
// with a certain initial timer end up producing the same number of total fish.
// Similarly, at any given day each fish with equal current timers will end up
// producing the same amount of total fish, so only the counts of fish with
// each different timer need to be considered.
fn solve(input: &str, days: usize) -> i64 {
    let mut fish_by_timer: [i64; 9] = [0; 9];

    for initial_timer in parse(input).into_iter() {
        fish_by_timer[initial_timer] += 1;
    }

    for _day in 0..days {
        let mut next_fish_by_timer = fish_by_timer;

        for timer in 0..fish_by_timer.len() {
            let count = fish_by_timer[timer];

            if timer == 0 {
                // The fish are ready to produce offspring -
                // after this day each fish with this internal timer resets to 6,
                // and they each create a new lanternfish with an internal timer of 8.
                next_fish_by_timer[6] += count;
                next_fish_by_timer[8] += count;
            } else {
                // Normally their internal timer just decreases by one
                next_fish_by_timer[timer - 1] += count;
            }

            next_fish_by_timer[timer] -= count;
        }

        fish_by_timer = next_fish_by_timer;
    }

    fish_by_timer.iter().sum()
}

fn main() {
    let part_1_result = solve(INPUT_FILE, 80);
    println!("[INFO]: Part 1: {:?}", part_1_result);

    let part_2_result = solve(INPUT_FILE, 256);
    println!("[INFO]: Part 2: {:?}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_examples() {
        assert_eq!(solve("3,4,3,1,2", 18), 26);
        assert_eq!(solve("3,4,3,1,2", 80), 5934);
    }

    #[test]
    fn it_solves_part2_examples() {
        assert_eq!(solve("3,4,3,1,2", 256), 26984457539);
    }
}
