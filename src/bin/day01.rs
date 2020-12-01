const INPUT_FILE: &str = include_str!("../../inputs/day01.txt");

fn part_1(file: String) -> usize {
    let input: Vec<usize> = file
        .split("\n")
        .map(|num| num.parse::<usize>().expect("parsing"))
        .collect();

    for first_idx in 0..(input.len() - 1) {
        for second_idx in (first_idx + 1)..input.len() {
            if input[first_idx] + input[second_idx] == 2020 {
                return input[first_idx] * input[second_idx];
            }
        }
    }

    return 0;
}

fn part_2(file: String) -> usize {
    let input: Vec<usize> = file
        .split("\n")
        .map(|num| num.parse::<usize>().expect("parsing"))
        .collect();

    for first_idx in 0..(input.len() - 2) {
        for second_idx in (first_idx + 1)..(input.len() - 1) {
            for third_idx in (second_idx + 1)..input.len() {
                if input[first_idx] + input[second_idx] + input[third_idx] == 2020 {
                    return input[first_idx] * input[second_idx] * input[third_idx];
                }
            }
        }
    }

    return 0;
}

fn main() -> () {
    let part1_sum = part_1(String::from(INPUT_FILE));
    let part2_sum = part_2(String::from(INPUT_FILE));

    println!("[INFO]: Part 1: {:?}", part1_sum);
    println!("[INFO]: Part 1: {:?}", part2_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            part_1(String::from("1721\n979\n366\n299\n675\n1456")),
            514579
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            part_2(String::from("1721\n979\n366\n299\n675\n1456")),
            241861950
        );
    }
}
