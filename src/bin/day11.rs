use std::collections::HashMap;

const INPUT_FILE: &str = include_str!("../../inputs/day11.txt");

const ADJACENT_TILES: [(i32, i32); 8] = [
    (-1,-1), (0,-1), (1,-1),
    (-1, 0),         (1, 0),
    (-1, 1), (0, 1), (1, 1),
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

    fn is_occupied(&self, seat: (i32, i32)) -> bool {
        match self.seats.get(&seat) {
            Some(s) => *s == '#',
            None => false,
        }
    }

    fn part_1(&mut self) -> usize {
        self.print();

        let mut is_stabilized = false;

        while !is_stabilized {
            let mut new_seats = self.seats.clone();
            self.step += 1;

            for (coords, seat) in self.seats.clone() {
                // If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
                if seat == 'L'
                    && ADJACENT_TILES
                        .iter()
                        .all(|(x, y)| !self.is_occupied((coords.0 + x, coords.1 + y)))
                {
                    *new_seats.entry(coords).or_insert('#') = '#';
                }
                // If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
                else if seat == '#'
                    && ADJACENT_TILES
                        .iter()
                        .filter(|(x, y)| self.is_occupied((coords.0 + x, coords.1 + y)))
                        .count()
                        >= 4
                {
                    *new_seats.entry(coords).or_insert('L') = 'L';
                }
            }

            is_stabilized = new_seats == self.seats;
            self.seats = new_seats;

            self.print();
        }

        return self
            .seats
            .iter()
            .filter(|(_coords, seat)| *seat == &'#')
            .count();
    }
}

fn main() -> () {
    let part_1_result = GameOfSeats::new(INPUT_FILE).part_1();

    println!("[INFO]: Part 1: {:?}", part_1_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example_1() {
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
}
