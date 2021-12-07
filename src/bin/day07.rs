const INPUT_FILE: &str = include_str!("../../inputs/day07.txt");

fn parse(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|coord| coord.parse::<usize>().unwrap())
        .collect()
}

fn part_1(input: &str) -> usize {
    let depths = parse(input);

    let max_depth = *depths.iter().max().unwrap_or(&0);

    let mut costs = vec![vec![0; max_depth + 1]; max_depth + 1];

    for (index, initial) in depths.into_iter().enumerate() {
        for (target, cost) in costs.iter_mut().enumerate().take(max_depth + 1) {
            cost[index] = if target > initial {
                target - initial
            } else {
                initial - target
            };
        }
    }

    costs
        .iter()
        .map(|cost| cost.iter().sum())
        .min()
        .unwrap_or(0)
}

fn part_2(input: &str) -> usize {
    let depths = parse(input);

    let max_depth = *depths.iter().max().unwrap_or(&0);

    let mut costs = vec![vec![0; max_depth + 1]; max_depth + 1];

    for (index, initial) in depths.into_iter().enumerate() {
        for (target, cost) in costs.iter_mut().enumerate().take(max_depth + 1) {
            let n = if target > initial {
                target - initial
            } else {
                initial - target
            };
            // https://en.wikipedia.org/wiki/Triangular_number
            cost[index] = n * (n + 1) / 2;
        }
    }

    costs
        .iter()
        .map(|cost| cost.iter().sum())
        .min()
        .unwrap_or(0)
}

fn main() {
    let part_1_result = part_1(INPUT_FILE);
    println!("[INFO]: Part 1: {:?}", part_1_result);

    let part_2_result = part_2(INPUT_FILE);
    println!("[INFO]: Part 2: {:?}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example2() {
        assert_eq!(part_1("16,1,2,0,4,2,7,1,2,14"), 37);
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(part_2("16,1,2,0,4,2,7,1,2,14"), 168);
    }
}
