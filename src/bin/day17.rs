use std::collections::HashMap;

const INPUT_FILE: &str = include_str!("../../inputs/day17.txt");

#[derive(Debug, Default)]
pub struct GameOfCubes {
    pub cubes: HashMap<(i32, i32, i32), bool>,
    pub step: u32,
}

impl GameOfCubes {
    #[inline]
    fn new(input: &str) -> GameOfCubes {
        let cubes: HashMap<(i32, i32, i32), bool> = input
            .lines()
            .enumerate()
            .flat_map(|(y, xs)| -> HashMap<(i32, i32, i32), bool> {
                xs.chars()
                    .enumerate()
                    .map(|(x, state)| ((x as i32, y as i32, 0), state == '#'))
                    .collect()
            })
            .collect();

        GameOfCubes {
            cubes: cubes,
            step: 0,
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
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
                    let state = match self.is_active((x as i32, y as i32, z as i32)) {
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

    fn active_cubes_count(&self) -> usize {
        self.cubes.iter().filter(|(_coords, state)| **state).count()
    }

    fn is_active(&self, coords: (i32, i32, i32)) -> bool {
        match self.cubes.get(&coords) {
            Some(state) => *state == true,
            _ => false,
        }
    }

    fn adjacent_active_count(&self, coords: &(i32, i32, i32)) -> usize {
        let mut count = 0;
        for z in [-1, 0, 1].iter() {
            for y in [-1, 0, 1].iter() {
                for x in [-1, 0, 1].iter() {
                    if *x == 0 && *y == 0 && *z == 0 {
                        continue;
                    }
                    if self.is_active((coords.0 + x, coords.1 + y, coords.2 + z)) {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn simulate(&mut self, steps: u32) -> usize {
        // self.print();

        while self.step < steps {
            let mut new_cubes = self.cubes.clone();
            self.step += 1;

            // Loop through the bounding box around max and min active coordinates +-1 for each axis
            let active_cubes: Vec<(i32, i32, i32)> = self
                .cubes
                .iter()
                .filter(|(_, state)| **state)
                .map(|(coords, _s)| *coords)
                .collect();

            let x_min = active_cubes.iter().min_by_key(|cube| cube.0).unwrap().0 - 1;
            let x_max = active_cubes.iter().max_by_key(|cube| cube.0).unwrap().0 + 1;
            let y_min = active_cubes.iter().min_by_key(|cube| cube.1).unwrap().1 - 1;
            let y_max = active_cubes.iter().max_by_key(|cube| cube.1).unwrap().1 + 1;
            let z_min = active_cubes.iter().min_by_key(|cube| cube.2).unwrap().2 - 1;
            let z_max = active_cubes.iter().max_by_key(|cube| cube.2).unwrap().2 + 1;

            for z in z_min..=z_max {
                for y in y_min..=y_max {
                    for x in x_min..=x_max {
                        let coords = (x, y, z);
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

            self.cubes = new_cubes;

            // self.print();
        }

        self.active_cubes_count()
    }
}

#[derive(Debug, Default)]
pub struct GameOfHyperCubes {
    pub cubes: HashMap<(i32, i32, i32, i32), bool>,
    pub step: u32,
}

impl GameOfHyperCubes {
    #[inline]
    fn new(input: &str) -> GameOfHyperCubes {
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

        GameOfHyperCubes {
            cubes: cubes,
            step: 0,
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
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
                        if self.is_active((coords.0 + x, coords.1 + y, coords.2 + z, coords.3 + w)) {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    fn simulate(&mut self, steps: u32) -> usize {
        // self.print();

        while self.step < steps {
            let mut new_cubes = self.cubes.clone();
            self.step += 1;

            // Loop through the bounding box around max and min active coordinates +-1 for each axis
            let active_cubes: Vec<(i32, i32, i32, i32)> = self
                .cubes
                .iter()
                .filter(|(_, state)| **state)
                .map(|(coords, _s)| *coords)
                .collect();

            let x_min = active_cubes.iter().min_by_key(|cube| cube.0).unwrap().0 - 1;
            let x_max = active_cubes.iter().max_by_key(|cube| cube.0).unwrap().0 + 1;
            let y_min = active_cubes.iter().min_by_key(|cube| cube.1).unwrap().1 - 1;
            let y_max = active_cubes.iter().max_by_key(|cube| cube.1).unwrap().1 + 1;
            let z_min = active_cubes.iter().min_by_key(|cube| cube.2).unwrap().2 - 1;
            let z_max = active_cubes.iter().max_by_key(|cube| cube.2).unwrap().2 + 1;
            let w_min = active_cubes.iter().min_by_key(|cube| cube.3).unwrap().3 - 1;
            let w_max = active_cubes.iter().max_by_key(|cube| cube.3).unwrap().3 + 1;

            for w in w_min..=w_max {
                for z in z_min..=z_max {
                    for y in y_min..=y_max {
                        for x in x_min..=x_max {
                            let coords = (x, y, z, w);
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
}

fn main() -> () {
    let part_1_result = GameOfCubes::new(INPUT_FILE).simulate(6);
    let part_2_result = GameOfHyperCubes::new(INPUT_FILE).simulate(6);

    println!("[INFO]: Part 1: {:?}", part_1_result);
    println!("[INFO]: Part 2: {:?}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_active_neighbors() {
        assert_eq!(
            GameOfCubes::new(
                "###\n\
                 ###\n\
                 ###"
            )
            .adjacent_active_count(&(1, 1, 0)),
            8
        );

        assert_eq!(
            GameOfCubes::new(
                "###\n\
                 ###\n\
                 ###"
            )
            .adjacent_active_count(&(1, 1, 1)),
            9
        );

        assert_eq!(
            GameOfCubes::new(
                "###\n\
                 ###\n\
                 ###"
            )
            .adjacent_active_count(&(0, 0, 0)),
            3
        );

        assert_eq!(
            GameOfCubes::new(
                "###\n\
                 ###\n\
                 ###"
            )
            .adjacent_active_count(&(0, 0, -1)),
            4
        );

        assert_eq!(
            GameOfCubes::new(
                "###\n\
                 ###\n\
                 ###"
            )
            .adjacent_active_count(&(-1, -1, -1)),
            1
        );
    }

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            GameOfCubes::new(
                ".#.\n\
                 ..#\n\
                 ###"
            )
            .simulate(6),
            112
        )
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            GameOfHyperCubes::new(
                ".#.\n\
                 ..#\n\
                 ###"
            )
            .simulate(6),
            848
        )
    }
}
