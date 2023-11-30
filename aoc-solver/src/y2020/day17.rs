use std::collections::HashMap;

use crate::solution::{AocError, Solution};

pub struct Day17;

#[derive(Debug, Default)]
pub struct GameOfCubes {
    pub cubes: HashMap<(i32, i32, i32, i32), bool>,
    pub step: u32,
}

impl GameOfCubes {
    #[inline]
    fn new(input: &str) -> GameOfCubes {
        let cubes: HashMap<(i32, i32, i32, i32), bool> = input
            .lines()
            .enumerate()
            .flat_map(|(y, xs)| -> HashMap<(i32, i32, i32, i32), bool> {
                xs.chars()
                    .enumerate()
                    .map(|(x, state)| ((x as i32, y as i32, 0, 0), state == '#'))
                    .collect()
            })
            .collect();

        GameOfCubes {
            cubes: cubes,
            step: 0,
        }
    }

    #[allow(dead_code)]
    fn print_3d(&self) {
        println!("Step {:?}:", self.step);
        let x_min = self.cubes.iter().min_by_key(|cube| cube.0 .0).unwrap().0 .0;
        let x_max = self.cubes.iter().max_by_key(|cube| cube.0 .0).unwrap().0 .0;

        let y_min = self.cubes.iter().min_by_key(|cube| cube.0 .1).unwrap().0 .1;
        let y_max = self.cubes.iter().max_by_key(|cube| cube.0 .1).unwrap().0 .1;

        let z_min = self.cubes.iter().min_by_key(|cube| cube.0 .2).unwrap().0 .2;
        let z_max = self.cubes.iter().max_by_key(|cube| cube.0 .2).unwrap().0 .2;

        for z in z_min..=z_max {
            println!("z={}", z);
            for y in y_min..=y_max {
                for x in x_min..=x_max {
                    let state = match self.is_active((x as i32, y as i32, z as i32, 0)) {
                        true => '#',
                        false => '.',
                    };

                    print!("{}", state);
                }
                print!("\n");
            }
            print!("\n");
        }
    }

    #[allow(dead_code)]
    fn print_4d(&self) {
        println!("Step {:?}:", self.step);
        let x_min = self.cubes.iter().min_by_key(|cube| cube.0 .0).unwrap().0 .0;
        let x_max = self.cubes.iter().max_by_key(|cube| cube.0 .0).unwrap().0 .0;

        let y_min = self.cubes.iter().min_by_key(|cube| cube.0 .1).unwrap().0 .1;
        let y_max = self.cubes.iter().max_by_key(|cube| cube.0 .1).unwrap().0 .1;

        let z_min = self.cubes.iter().min_by_key(|cube| cube.0 .2).unwrap().0 .2;
        let z_max = self.cubes.iter().max_by_key(|cube| cube.0 .2).unwrap().0 .2;

        let w_min = self.cubes.iter().min_by_key(|cube| cube.0 .3).unwrap().0 .3;
        let w_max = self.cubes.iter().max_by_key(|cube| cube.0 .3).unwrap().0 .3;

        for w in w_min..=w_max {
            for z in z_min..=z_max {
                println!("z={}, w={}", z, w);
                for y in y_min..=y_max {
                    for x in x_min..=x_max {
                        let state = match self.is_active((x as i32, y as i32, z as i32, w as i32)) {
                            true => '#',
                            false => '.',
                        };
                        print!("{}", state);
                    }
                    print!("\n");
                }
                print!("\n");
            }
        }
    }

    fn active_cubes_count(&self) -> usize {
        self.cubes.iter().filter(|(_coords, state)| **state).count()
    }

    fn is_active(&self, coords: (i32, i32, i32, i32)) -> bool {
        match self.cubes.get(&coords) {
            Some(state) => *state == true,
            _ => false,
        }
    }

    fn adjacent_active_count(&self, coords: &(i32, i32, i32, i32)) -> usize {
        let mut count = 0;
        for w in [-1, 0, 1].iter() {
            for z in [-1, 0, 1].iter() {
                for y in [-1, 0, 1].iter() {
                    for x in [-1, 0, 1].iter() {
                        if *x == 0 && *y == 0 && *z == 0 && *w == 0 {
                            continue;
                        }
                        if self.is_active((coords.0 + x, coords.1 + y, coords.2 + z, coords.3 + w))
                        {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    fn simulate_3d(&mut self, steps: u32) -> usize {
        // self.print();

        while self.step < steps {
            let mut new_cubes = self.cubes.clone();
            self.step += 1;

            // The only meaningful cubes that can turn active are the ones within
            // distance of max one from now active cubes
            for (coords, _s) in self.cubes.iter() {
                for z in [-1, 0, 1].iter() {
                    for y in [-1, 0, 1].iter() {
                        for x in [-1, 0, 1].iter() {
                            let coords = (coords.0 + x, coords.1 + y, coords.2 + z, 0);
                            let state = self.is_active(coords);
                            let adjacent_count = self.adjacent_active_count(&coords);

                            if state == true && adjacent_count != 2 && adjacent_count != 3 {
                                new_cubes.insert(coords, false);
                            }
                            // If a cube is inactive but exactly 3 of its neighbors are active,
                            // the cube becomes active. Otherwise, the cube remains inactive.
                            else if state == false && adjacent_count == 3 {
                                new_cubes.insert(coords, true);
                            }
                        }
                    }
                }
            }

            self.cubes = new_cubes;

            // self.print();
        }

        self.active_cubes_count()
    }

    fn simulate_4d(&mut self, steps: u32) -> usize {
        // self.print();

        while self.step < steps {
            let mut new_cubes = self.cubes.clone();
            self.step += 1;

            // The only meaningful cubes that can turn active are the ones within
            // distance of max one from now active cubes
            for (coords, _s) in self.cubes.iter() {
                for w in [-1, 0, 1].iter() {
                    for z in [-1, 0, 1].iter() {
                        for y in [-1, 0, 1].iter() {
                            for x in [-1, 0, 1].iter() {
                                let coords =
                                    (coords.0 + x, coords.1 + y, coords.2 + z, coords.3 + w);
                                let state = self.is_active(coords);
                                let adjacent_count = self.adjacent_active_count(&coords);

                                if state == true && adjacent_count != 2 && adjacent_count != 3 {
                                    new_cubes.insert(coords, false);
                                }
                                // If a cube is inactive but exactly 3 of its neighbors are active,
                                // the cube becomes active. Otherwise, the cube remains inactive.
                                else if state == false && adjacent_count == 3 {
                                    new_cubes.insert(coords, true);
                                }
                            }
                        }
                    }
                }
            }

            self.cubes = new_cubes;

            // self.print();
        }

        self.active_cubes_count()
    }
}

impl Solution for Day17 {
    type F = usize;
    type S = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2020/day17.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let result = GameOfCubes::new(input).simulate_3d(6);

        Ok(result)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let result = GameOfCubes::new(input).simulate_4d(6);

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_active_neighbors() {
        let game_of_cubes = GameOfCubes::new(
            "###\n\
             ###\n\
             ###",
        );

        assert_eq!(game_of_cubes.adjacent_active_count(&(1, 1, 0, 0)), 8);
        assert_eq!(game_of_cubes.adjacent_active_count(&(1, 1, 1, 0)), 9);
        assert_eq!(game_of_cubes.adjacent_active_count(&(0, 0, 0, 0)), 3);
        assert_eq!(game_of_cubes.adjacent_active_count(&(0, 0, -1, 0)), 4);
        assert_eq!(game_of_cubes.adjacent_active_count(&(-1, -1, -1, 0)), 1);
    }

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            GameOfCubes::new(
                ".#.\n\
                 ..#\n\
                 ###"
            )
            .simulate_3d(6),
            112
        )
    }

    #[test]
    #[ignore]
    fn it_solves_part2_example() {
        assert_eq!(
            GameOfCubes::new(
                ".#.\n\
                 ..#\n\
                 ###"
            )
            .simulate_4d(6),
            848
        )
    }
}
