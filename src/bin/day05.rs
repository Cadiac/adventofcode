const INPUT_FILE: &str = include_str!("../../inputs/day05.txt");

#[derive(Debug, PartialEq)]
struct BoardingPass {
    row: u8,
    column: u8,
    id: usize
}

fn parse_boarding_pass(input: &str) -> BoardingPass {
    let binary_str: String = input
        .chars()
        .map(|c| match c {
            'B' | 'R' => '1',
            'F' | 'L' => '0',
            unknown => unknown,
        })
        .collect();

    let row = u8::from_str_radix(&binary_str[0..7], 2).unwrap();
    let column = u8::from_str_radix(&binary_str[7..10], 2).unwrap();

    BoardingPass {
        row: row,
        column: column,
        id: 8 * row as usize + column as usize
    }
}

fn part_1(input: &str) -> usize {
    let highest_seat = input
        .lines()
        .map(parse_boarding_pass)
        .max_by(|a, b| a.id.cmp(&b.id))
        .unwrap();

    highest_seat.id
}

fn part_2(input: &str) -> usize {
    let mut seats: Vec<BoardingPass> = input
        .lines()
        .map(parse_boarding_pass)
        .collect();

    seats.sort_by(|a, b| a.id.cmp(&b.id));
    let first_id = seats[0].id;

    let (_, seat_higher_than_me) = seats
        .iter()
        .enumerate()
        .find(|(index, seat)| seat.id - first_id != *index)
        .unwrap();

    seat_higher_than_me.id - 1
}

fn part_2_arithmetic_sum(input: &str) -> usize {
    let seats: Vec<BoardingPass> = input
        .lines()
        .map(parse_boarding_pass)
        .collect();

    let highest_seat_by_id = seats
        .iter()
        .max_by(|a, b| a.id.cmp(&b.id))
        .unwrap();

    let lowest_seat_by_id = seats
        .iter()
        .min_by(|a, b| a.id.cmp(&b.id))
        .unwrap();

    // Arithmetic sum from n to m can be calculated with formula `n * (a_n + a_m) / 2`
    // The missing seat ID will be the number we're missing from the sum if we sum up our seat IDs
    let correct_sum = (seats.len() + 1) * (lowest_seat_by_id.id + highest_seat_by_id.id) / 2;
    let actual_sum = seats.iter().fold(0, |acc, seat| acc + seat.id);

    correct_sum - actual_sum
}

fn main() -> () {
    let part_1_result = part_1(INPUT_FILE);
    let part_2_result = part_2(INPUT_FILE);
    let part_2_result_extra = part_2_arithmetic_sum(INPUT_FILE);

    println!("[INFO]: Part 1: {:?}", part_1_result);
    println!("[INFO]: Part 2: {:?}", part_2_result);
    println!("[INFO]: Part 2+: {:?}", part_2_result_extra);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_part1_examples() {
        assert_eq!(parse_boarding_pass("FBFBBFFRLR"), BoardingPass{ row: 44, column: 5, id: 357 });
        assert_eq!(parse_boarding_pass("BFFFBBFRRR"), BoardingPass{ row: 70, column: 7, id: 567 });
        assert_eq!(parse_boarding_pass("FFFBBBFRRR"), BoardingPass{ row: 14, column: 7, id: 119 });
        assert_eq!(parse_boarding_pass("BBFFBBFRLL"), BoardingPass{ row: 102, column: 4, id: 820 });
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
