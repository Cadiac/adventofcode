const INPUT_FILE: &str = include_str!("../../inputs/day05.txt");

fn parse_boarding_pass(input: &str) -> (usize, usize, usize) {
    let binary_str: String = input
        .chars()
        .map(|c| match c {
            'B' | 'R' => '1',
            'F' | 'L' => '0',
            unknown => unknown,
        })
        .collect();

    let row = usize::from_str_radix(&binary_str[0..7], 2).unwrap();
    let column = usize::from_str_radix(&binary_str[7..10], 2).unwrap();

    (row, column, 8 * row + column)
}

fn part_1(input: &str) -> usize {
    let highest_seat = input
        .lines()
        .map(parse_boarding_pass)
        .max_by(|a, b| a.2.cmp(&b.2))
        .unwrap();

    highest_seat.2
}

fn part_2(input: &str) -> usize {
    let mut seats: Vec<(usize, usize, usize)> = input
        .lines()
        .map(parse_boarding_pass)
        .collect();

    seats.sort_by(|a, b| a.2.cmp(&b.2));
    let first_id = seats[0].2;

    let (_, seat_higher_than_me) = seats
        .iter()
        .enumerate()
        .find(|(index, seat)| seat.2 - first_id != *index)
        .unwrap();

    seat_higher_than_me.2 - 1
}

fn main() -> () {
    let part_1_result = part_1(INPUT_FILE);
    let part_2_result = part_2(INPUT_FILE);

    println!("[INFO]: Part 1: {:?}", part_1_result);
    println!("[INFO]: Part 2: {:?}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_part1_examples() {
        assert_eq!(parse_boarding_pass("FBFBBFFRLR"), (44, 5, 357));
        assert_eq!(parse_boarding_pass("BFFFBBFRRR"), (70, 7, 567));
        assert_eq!(parse_boarding_pass("FFFBBBFRRR"), (14, 7, 119));
        assert_eq!(parse_boarding_pass("BBFFBBFRLL"), (102, 4, 820));
    }

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            part_1(
                "FBFBBFFRLR\n\
                 BFFFBBFRRR\n\
                 FFFBBBFRRR\n\
                 BBFFBBFRLL"
            ),
            820
        );
    }
}
