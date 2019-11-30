const INPUT_FILE: &str = include_str!("../input.txt");

fn part_1(file: &str) -> i32 {
    0
}

fn main() {
    let part1_result = part_1(INPUT_FILE);
    println!("Part 1: {}", part1_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_day08_part1_examples() {
        assert_eq!(part_1("10 players; last marble is worth 1618 points"), 8317);
        assert_eq!(part_1("13 players; last marble is worth 7999 points"), 146373);
        assert_eq!(part_1("17 players; last marble is worth 1104 points"), 2764);
        assert_eq!(part_1("21 players; last marble is worth 6111 points"), 54718);
        assert_eq!(part_1("30 players; last marble is worth 5807 points"), 37305);
    }
}
