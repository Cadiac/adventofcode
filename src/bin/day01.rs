const INPUT_FILE: &str = include_str!("../../inputs/day01.txt");

fn parse(input: &str) -> Vec<u32> {
    input
        .split('\n')
        .map(|depth| depth.parse::<u32>().unwrap())
        .collect()
}

fn part_1(input: &str) -> usize {
    parse(input)
        .windows(2)
        .filter(|depths| depths[0] < depths[1])
        .count()
}

fn part_2(input: &str) -> usize {
    parse(input)
        .windows(3)
        .map(|depths| depths.iter().sum())
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|depths| depths[0] < depths[1])
        .count()
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
    fn it_solves_part1_example() {
        assert_eq!(
            part_1(
                "199\n\
                    200\n\
                    208\n\
                    210\n\
                    200\n\
                    207\n\
                    240\n\
                    269\n\
                    260\n\
                    263"
            ),
            7
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            part_2(
                "199\n\
                    200\n\
                    208\n\
                    210\n\
                    200\n\
                    207\n\
                    240\n\
                    269\n\
                    260\n\
                    263"
            ),
            5
        );
    }
}
