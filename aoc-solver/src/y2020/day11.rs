use std::collections::HashMap;

use crate::solution::{AocError, Solution};

pub struct Day11;

const ADJACENT_TILES: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[derive(Debug, Default)]
pub struct GameOfSeats {
    pub seats: HashMap<(i32, i32), char>,
    pub step: u32,
    pub dimensions: (usize, usize),
}

impl GameOfSeats {
    #[inline]
    fn new(input: &str) -> GameOfSeats {
        let seats: HashMap<(i32, i32), char> = input
            .lines()
            .enumerate()
            .flat_map(|(row, columns)| -> HashMap<(i32, i32), char> {
                columns
                    .chars()
                    .enumerate()
                    .map(|(column, seat)| ((column as i32, row as i32), seat))
                    .collect()
            })
            .collect();

        let height = input.lines().count();
        let width = input.lines().next().unwrap().chars().count();

        GameOfSeats {
            seats: seats,
            step: 0,
            dimensions: (width, height),
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("Step {:?}:", self.step);
        for y in 0..self.dimensions.1 {
            for x in 0..self.dimensions.0 {
                print!("{}", self.seats.get(&(x as i32, y as i32)).unwrap());
            }
            print!("\n");
        }
        print!("\n");
    }

    fn occupied_seats_count(&self) -> usize {
        self.seats
            .iter()
            .filter(|(_coords, seat)| *seat == &'#')
            .count()
    }

    fn is_occupied(&self, seat: (i32, i32)) -> bool {
        match self.seats.get(&seat) {
            Some('#') => true,
            _ => false,
        }
    }

    fn adjacent_occupied_count(&self, seat: (i32, i32)) -> usize {
        ADJACENT_TILES
            .iter()
            .filter(|(x, y)| self.is_occupied((seat.0 + x, seat.1 + y)))
            .count()
    }

    fn part_1(&mut self) -> usize {
        // self.print();

        let mut is_stabilized = false;

        while !is_stabilized {
            let mut new_seats = self.seats.clone();
            self.step += 1;

            for (seat, state) in self.seats.clone() {
                // If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
                if state == 'L' && self.adjacent_occupied_count(seat) == 0 {
                    *new_seats.entry(seat).or_insert('#') = '#';
                }
                // If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
                else if state == '#' && self.adjacent_occupied_count(seat) >= 4 {
                    *new_seats.entry(seat).or_insert('L') = 'L';
                }
            }

            is_stabilized = new_seats == self.seats;
            self.seats = new_seats;

            // self.print();
        }

        self.occupied_seats_count()
    }

    fn occupied_at_direction(&self, seat: (i32, i32), direction: (i32, i32)) -> bool {
        let current = (seat.0 + direction.0, seat.1 + direction.1);

        match self.seats.get(&current) {
            Some('#') => true,
            Some('L') => false,
            Some('.') => self.occupied_at_direction(current, direction),
            _ => false,
        }
    }

    fn visible_occupied_count(&self, seat: (i32, i32)) -> usize {
        ADJACENT_TILES
            .iter()
            .filter(|direction| self.occupied_at_direction(seat, **direction))
            .count()
    }

    fn part_2(&mut self) -> usize {
        // self.print();

        let mut is_stabilized = false;

        while !is_stabilized {
            let mut new_seats = self.seats.clone();
            self.step += 1;

            for (coords, seat) in self.seats.clone() {
                // If a seat is empty (L) and there are no occupied seats in any of the
                // visible directions, the seat becomes occupied.
                if seat == 'L' && self.visible_occupied_count(coords) == 0 {
                    *new_seats.entry(coords).or_insert('#') = '#';
                }
                // If a seat is occupied (#) and five or more seats in any of the visible
                // directions to it are also occupied, the seat becomes empty.
                else if seat == '#' && self.visible_occupied_count(coords) >= 5 {
                    *new_seats.entry(coords).or_insert('L') = 'L';
                }
            }

            is_stabilized = new_seats == self.seats;
            self.seats = new_seats;

            // self.print();
        }

        self.occupied_seats_count()
    }
}

impl Solution for Day11 {
    type F = usize;
    type S = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2020/day11.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let result = GameOfSeats::new(input).part_1();

        Ok(result)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let result = GameOfSeats::new(input).part_2();

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            GameOfSeats::new(
                "L.LL.LL.LL\n\
                 LLLLLLL.LL\n\
                 L.L.L..L..\n\
                 LLLL.LL.LL\n\
                 L.LL.LL.LL\n\
                 L.LLLLL.LL\n\
                 ..L.L.....\n\
                 LLLLLLLLLL\n\
                 L.LLLLLL.L\n\
                 L.LLLLL.LL"
            )
            .part_1(),
            37
        )
    }

    #[test]
    fn it_calculates_part2_example() {
        assert_eq!(
            GameOfSeats::new(
                "L.LL.LL.LL\n\
                 LLLLLLL.LL\n\
                 L.L.L..L..\n\
                 LLLL.LL.LL\n\
                 L.LL.LL.LL\n\
                 L.LLLLL.LL\n\
                 ..L.L.....\n\
                 LLLLLLLLLL\n\
                 L.LLLLLL.L\n\
                 L.LLLLL.LL"
            )
            .part_2(),
            26
        )
    }

    #[test]
    fn it_finds_visible_occupied_counts() {
        assert_eq!(
            GameOfSeats::new(
                ".......#.\n\
                 ...#.....\n\
                 .#.......\n\
                 .........\n\
                 ..#L....#\n\
                 ....#....\n\
                 .........\n\
                 #........\n\
                 ...#....."
            )
            .visible_occupied_count((3, 4)),
            8
        );

        assert_eq!(
            GameOfSeats::new(
                ".............\n\
                 .L.L.#.#.#.#.\n\
                 ............."
            )
            .visible_occupied_count((1, 1)),
            0
        );

        assert_eq!(
            GameOfSeats::new(
                ".##.##.\n\
                 #.#.#.#\n\
                 ##...##\n\
                 ...L...\n\
                 ##...##\n\
                 #.#.#.#\n\
                 .##.##."
            )
            .visible_occupied_count((3, 3)),
            0
        );
    }
}
