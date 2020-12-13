const INPUT_FILE: &str = include_str!("../../inputs/day13.txt");

fn find_first_after(freq: u64, earliest: u64) -> u64 {
    let n = earliest / freq;
    return freq * (n + 1);
}

fn part_1(input: &str) -> u64 {
    let time_of_leave = input.lines().nth(0).unwrap().parse::<u64>().unwrap();

    let schedules: Vec<u64> = input
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .flat_map(|bus| {
            if bus == "x" {
                return None;
            }
            return Some(bus.parse::<u64>().unwrap());
        })
        .collect();

    let earliest_bus = schedules
        .iter()
        .min_by(|x, y| {
            find_first_after(**x, time_of_leave).cmp(&find_first_after(**y, time_of_leave))
        })
        .unwrap();

    return earliest_bus * (find_first_after(*earliest_bus, time_of_leave) - time_of_leave);
}

fn part_2(input: &str) -> u64 {
    let mut timestamp: u64 = 100000000000000;
    // let mut timestamp: u64 = 0;

    let schedules: Vec<(u64, u64)> = input
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .enumerate()
        .flat_map(|(offset, freq)| {
            if freq == "x" {
                return None;
            }
            return Some((freq.parse::<u64>().unwrap(), offset as u64));
        })
        .collect();

    // println!("[DEBUG] Found schedules {:?}", schedules);

    'outer: loop {
        // println!("[DEBUG] Timestamp {:?}", timestamp);

        for (freq, offset) in schedules.iter() {
            let t = timestamp + *offset;
            // println!("[DEBUG] Timestamp {:?}, t {:?}", timestamp, t);
            let is_scheduled_stop = t % freq == 0;

            if !is_scheduled_stop {
                // We can skip ahead to to next even - offset
                let skip = find_first_after(*freq, t) - offset;
                timestamp = skip;
                // println!("[DEBUG] Not a scheduled stop for bus {:?}, skipping for {:?} to {:?}", freq, skip, timestamp);
                continue 'outer;
            }
        }
        return timestamp;
    }
}

fn main() -> () {
    let part_1_result = part_1(INPUT_FILE);
    println!("[INFO]: Part 1: {:?}", part_1_result);

    let part_2_result = part_2(INPUT_FILE);
    println!("[INFO]: Part 2: {:?}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            part_1(
                "939\n\
                 7,13,x,x,59,x,31,19"
            ),
            295
        )
    }

    #[test]
    fn it_solves_part2_examples() {
        assert_eq!(part_2("\n7,13,x,x,59,x,31,19"), 1068781);
        assert_eq!(part_2("\n17,x,13,19"), 3417);
        assert_eq!(part_2("\n67,7,59,61"), 754018);
        assert_eq!(part_2("\n67,x,7,59,61"), 779210);
        assert_eq!(part_2("\n67,7,x,59,61"), 1261476);
        assert_eq!(part_2("\n1789,37,47,1889"), 1202161486);
    }
}
