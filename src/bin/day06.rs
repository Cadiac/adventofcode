const INPUT_FILE: &str = include_str!("../../inputs/day06.txt");

fn parse(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|initial_timer| initial_timer.parse::<usize>().unwrap())
        .collect()
}

fn solve(input: &str, days: usize) -> i64 {
    let mut fish_after_days_by_initial_timer: [i64; 9] = [0; 9];

    for initial_timer in 0..=8 {
        let mut count_by_timer: [i64; 9] = [0; 9];
        count_by_timer[initial_timer] = 1;

        for _day in 0..days { 
            let mut next_count_by_timer = count_by_timer.clone();

            for timer in 0..count_by_timer.len() {
                let count = count_by_timer[timer];

                if timer == 0 {
                    next_count_by_timer[8] += count;
                    next_count_by_timer[6] += count;
                } else {
                    next_count_by_timer[timer - 1] += count;
                }

                next_count_by_timer[timer] -= count;
            }

            count_by_timer = next_count_by_timer;
        }

        fish_after_days_by_initial_timer[initial_timer] = count_by_timer.iter().sum();
    }

    parse(input).into_iter()
        .map(|initial_timer| fish_after_days_by_initial_timer[initial_timer])
        .sum()
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
