extern crate serde_scan;
use std::cmp::{max, min};
use std::collections::HashSet;

type Region = ((i32, i32), (i32, i32), (i32, i32));

const INPUT_FILE: &str = include_str!("../../inputs/day22.txt");

fn part_1(input: &str, bounds: Region) -> usize {
    let max_size =
        (bounds.0 .1 - bounds.0 .0) * (bounds.1 .1 - bounds.1 .0) * (bounds.2 .1 - bounds.2 .0);
    let mut cubes: HashSet<(i32, i32, i32)> = HashSet::with_capacity(max_size as usize);

    input.lines().for_each(|line| {
        let (state, cuboid): (&str, Region) =
            serde_scan::scan!("{} x={}..{},y={}..{},z={}..{}" <- line).unwrap();

        let x_min = max(cuboid.0 .0, bounds.0 .0);
        let x_max = min(cuboid.0 .1, bounds.0 .1);
        let y_min = max(cuboid.1 .0, bounds.1 .0);
        let y_max = min(cuboid.1 .1, bounds.1 .1);
        let z_min = max(cuboid.2 .0, bounds.2 .0);
        let z_max = min(cuboid.2 .1, bounds.2 .1);

        if state == "on" {
            for x in x_min..=x_max {
                for y in y_min..=y_max {
                    for z in z_min..=z_max {
                        cubes.insert((x, y, z));
                    }
                }
            }
        } else if state == "off" {
            for x in x_min..=x_max {
                for y in y_min..=y_max {
                    for z in z_min..=z_max {
                        cubes.remove(&(x, y, z));
                    }
                }
            }
        }
    });

    cubes.len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cuboid {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
}

impl Cuboid {
    fn new(x_min: i32, x_max: i32, y_min: i32, y_max: i32, z_min: i32, z_max: i32) -> Self {
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
        }
    }

    fn volume(&self) -> i32 {
        (self.x_max - self.x_min) * (self.y_max - self.y_min) * (self.z_max - self.z_min)
    }

    fn contains_cuboid(&self, cuboid: &Cuboid) -> bool {
        self.x_min <= cuboid.x_min
            && self.x_max >= cuboid.x_max
            && self.y_min <= cuboid.y_min
            && self.y_max >= cuboid.y_max
            && self.z_min <= cuboid.z_min
            && self.z_max >= cuboid.z_max
    }

    fn intersects_cuboid(&self, cuboid: &Cuboid) -> bool {
        // If cube is below or above the other, they don't intersect
        if self.y_max <= cuboid.y_min || self.y_min >= cuboid.y_max {
            return false;
        }

        // If one cube is to the left or right of the other, they don't intersect
        if self.x_max <= cuboid.x_min || self.x_min >= cuboid.x_max {
            return false;
        }

        // If one cube is to behind or in front of the other, they don't intersect
        if self.z_max <= cuboid.z_min || self.z_min >= cuboid.z_max {
            return false;
        }

        // Otherwise they intersect
        true
    }

    fn intersection(&self, cuboid: &Cuboid) -> Cuboid {
        assert!(self.intersects_cuboid(cuboid));

        Cuboid {
            x_min: max(self.x_min, cuboid.x_min),
            x_max: min(self.x_max, cuboid.x_max),
            y_min: max(self.y_min, cuboid.y_min),
            y_max: min(self.y_max, cuboid.y_max),
            z_min: max(self.z_min, cuboid.z_min),
            z_max: min(self.z_max, cuboid.z_max),
        }
    }

    fn join_subdivide(&self, cuboid: &Cuboid) -> Vec<Cuboid> {
        assert!(self.intersects_cuboid(cuboid));

        let intersection = self.intersection(cuboid);

        let bounds = Cuboid {
            x_min: min(self.x_min, cuboid.x_min),
            x_max: max(self.x_max, cuboid.x_max),
            y_min: min(self.y_min, cuboid.y_min),
            y_max: max(self.y_max, cuboid.y_max),
            z_min: min(self.z_min, cuboid.z_min),
            z_max: max(self.z_max, cuboid.z_max),
        };

        // Now construct the 3x3x3 cuboids around and including the intersection
        // Test if the original cuboids contain the new splits, if not discard them.
        let mut cuboids: Vec<Cuboid> = Vec::with_capacity(27);

        for (x_min, x_max) in [
            (bounds.x_min, intersection.x_min),
            (intersection.x_min, intersection.x_max),
            (intersection.x_max, bounds.x_max),
        ] {
            for (y_min, y_max) in [
                (bounds.y_min, intersection.y_min),
                (intersection.y_min, intersection.y_max),
                (intersection.y_max, bounds.y_max),
            ] {
                for (z_min, z_max) in [
                    (bounds.z_min, intersection.z_min),
                    (intersection.z_min, intersection.z_max),
                    (intersection.z_max, bounds.z_max),
                ] {
                    let new_cuboid = Cuboid::new(x_min, x_max, y_min, y_max, z_min, z_max);
                    if new_cuboid.volume() > 0
                        && (self.contains_cuboid(&new_cuboid)
                            || cuboid.contains_cuboid(&new_cuboid))
                    {
                        cuboids.push(new_cuboid);
                    }
                }
            }
        }

        cuboids
    }
}

// Yeah, not going to happen.
fn part_2(input: &str) -> usize {
    let mut cuboids_on: Vec<Cuboid> = vec![];
    let mut cuboids_off: Vec<Cuboid> = vec![];

    input.lines().for_each(|line| {
        println!("{}", line);

        let (state, ((x_min, x_max), (y_min, y_max), (z_min, z_max))): (&str, Region) =
            serde_scan::scan!("{} x={}..{},y={}..{},z={}..{}" <- line).unwrap();

        if state == "on" {
            cuboids_on.push(Cuboid {
                x_min,
                x_max,
                y_min,
                y_max,
                z_min,
                z_max,
            });
        } else if state == "off" {
            cuboids_off.push(Cuboid {
                x_min,
                x_max,
                y_min,
                y_max,
                z_min,
                z_max,
            });
        }
    });

    for cuboid in cuboids_on {
        // If the cuboid overlaps with any of the off cuboids,
        // split the cuboid and remove the off the overlapping cuboid.

        // If it overlaps another cuboid fully, ignore it.
    }

    unimplemented!();
}

fn main() {
    let part_1_result = part_1(INPUT_FILE, ((-50, 50), (-50, 50), (-50, 50)));
    println!("[INFO]: Part 1: {:?}", part_1_result);

    let part_2_result = part_2(INPUT_FILE);
    println!("[INFO]: Part 2: {:?}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_small_example() {
        assert_eq!(
            part_1(
                "on x=10..12,y=10..12,z=10..12\n\
                 on x=11..13,y=11..13,z=11..13\n\
                 off x=9..11,y=9..11,z=9..11\n\
                 on x=10..10,y=10..10,z=10..10",
                ((-50, 50), (-50, 50), (-50, 50))
            ),
            39
        );
    }

    #[test]
    fn it_solves_part1_large_example() {
        assert_eq!(
            part_1(
                "on x=-20..26,y=-36..17,z=-47..7\n\
                 on x=-20..33,y=-21..23,z=-26..28\n\
                 on x=-22..28,y=-29..23,z=-38..16\n\
                 on x=-46..7,y=-6..46,z=-50..-1\n\
                 on x=-49..1,y=-3..46,z=-24..28\n\
                 on x=2..47,y=-22..22,z=-23..27\n\
                 on x=-27..23,y=-28..26,z=-21..29\n\
                 on x=-39..5,y=-6..47,z=-3..44\n\
                 on x=-30..21,y=-8..43,z=-13..34\n\
                 on x=-22..26,y=-27..20,z=-29..19\n\
                 off x=-48..-32,y=26..41,z=-47..-37\n\
                 on x=-12..35,y=6..50,z=-50..-2\n\
                 off x=-48..-32,y=-32..-16,z=-15..-5\n\
                 on x=-18..26,y=-33..15,z=-7..46\n\
                 off x=-40..-22,y=-38..-28,z=23..41\n\
                 on x=-16..35,y=-41..10,z=-47..6\n\
                 off x=-32..-23,y=11..30,z=-14..3\n\
                 on x=-49..-5,y=-3..45,z=-29..18\n\
                 off x=18..30,y=-20..-8,z=-3..13\n\
                 on x=-41..9,y=-7..43,z=-33..15\n\
                 on x=-54112..-39298,y=-85059..-49293,z=-27449..7877\n\
                 on x=967..23432,y=45373..81175,z=27513..53682",
                ((-50, 50), (-50, 50), (-50, 50))
            ),
            590784
        );
    }

    #[test]
    fn it_checks_if_cuboid_contains_another() {
        assert_eq!(
            Cuboid::new(-10, 10, -10, 10, -10, 10)
                .contains_cuboid(&Cuboid::new(-5, 5, -5, 5, -5, 5)),
            true
        );
        assert_eq!(
            Cuboid::new(-10, 10, -10, 10, -10, 10)
                .contains_cuboid(&Cuboid::new(5, 15, -5, 5, -5, 5)),
            false
        );
        assert_eq!(
            Cuboid::new(-10, 10, -10, 10, -10, 10)
                .contains_cuboid(&Cuboid::new(5, 10, -5, 5, -5, 5)),
            true
        );
        assert_eq!(
            Cuboid::new(-10, 10, -10, 10, -10, 10)
                .contains_cuboid(&Cuboid::new(-20, 20, -20, 20, -20, 20)),
            false
        );
        assert_eq!(
            Cuboid::new(-10, 10, -10, 10, -10, 10)
                .contains_cuboid(&Cuboid::new(5, 10, 5, 10, 5, 10)),
            true
        );
        assert_eq!(
            Cuboid::new(-10, 10, -10, 10, -10, 10)
                .contains_cuboid(&Cuboid::new(11, 21, -10, 10, -10, 10)),
            false
        );
    }

    #[test]
    fn it_checks_if_cuboid_intersects_another() {
        assert_eq!(
            Cuboid::new(-10, 10, -10, 10, -10, 10)
                .intersects_cuboid(&Cuboid::new(-5, 5, -5, 5, -5, 5)),
            true
        );
        assert_eq!(
            Cuboid::new(-10, 10, -10, 10, -10, 10)
                .intersects_cuboid(&Cuboid::new(5, 15, -5, 5, -5, 5)),
            true
        );
        assert_eq!(
            Cuboid::new(-10, 10, -10, 10, -10, 10)
                .intersects_cuboid(&Cuboid::new(5, 10, -5, 5, -5, 5)),
            true
        );
        assert_eq!(
            Cuboid::new(-10, 10, -10, 10, -10, 10)
                .intersects_cuboid(&Cuboid::new(-20, 20, -20, 20, -20, 20)),
            true
        );
        assert_eq!(
            Cuboid::new(-10, 10, -10, 10, -10, 10)
                .contains_cuboid(&Cuboid::new(5, 10, 5, 10, 5, 10)),
            true
        );
        assert_eq!(
            Cuboid::new(-10, 10, -10, 10, -10, 10)
                .intersects_cuboid(&Cuboid::new(11, 21, -10, 10, -10, 10)),
            false
        );
    }

    #[test]
    #[should_panic]
    fn it_panics_if_intersection_doesnt_intersect() {
        Cuboid::new(-10, 10, -10, 10, -10, 10).intersection(&Cuboid::new(11, 21, -10, 10, -10, 10));
    }

    #[test]
    fn it_finds_intersection_of_cuboids() {
        assert_eq!(
            Cuboid::new(-10, 10, -10, 10, -10, 10).intersection(&Cuboid::new(-5, 5, -5, 5, -5, 5)),
            Cuboid::new(-5, 5, -5, 5, -5, 5)
        );
        assert_eq!(
            Cuboid::new(-10, 10, -10, 10, -10, 10).intersection(&Cuboid::new(0, 20, 0, 20, 0, 20)),
            Cuboid::new(0, 10, 0, 10, 0, 10)
        );
        assert_eq!(
            Cuboid::new(-10, 10, -10, 10, -10, 10)
                .intersection(&Cuboid::new(-20, 20, -5, 5, -5, 5)),
            Cuboid::new(-10, 10, -5, 5, -5, 5)
        );
    }

    #[test]
    fn it_joins_intersecting_cuboids() {
        // Intersects one corner
        assert_eq!(
            Cuboid::new(0, 10, 0, 10, 0, 10)
                .join_subdivide(&Cuboid::new(5, 15, 5, 15, 5, 15))
                .len(),
            15
        );

        // Intersects one edge, same height and width
        assert_eq!(
            Cuboid::new(0, 10, 0, 10, 0, 10)
                .join_subdivide(&Cuboid::new(5, 15, 5, 15, 0, 10))
                .len(),
            7
        );

        // Intersects one edge, greater height upwards
        assert_eq!(
            Cuboid::new(0, 10, 0, 10, 0, 10)
                .join_subdivide(&Cuboid::new(5, 15, 5, 15, 0, 15))
                .len(),
            11
        );

        // Intersects one edge, greater height downwards
        assert_eq!(
            Cuboid::new(0, 10, 0, 10, 0, 10)
                .join_subdivide(&Cuboid::new(5, 15, 5, 15, -5, 10))
                .len(),
            11
        );

        // Intersects one edge, greater height both directions
        assert_eq!(
            Cuboid::new(0, 10, 0, 10, 0, 10)
                .join_subdivide(&Cuboid::new(5, 15, 5, 15, -5, 15))
                .len(),
            15
        );

        // Intersects one face, same height and width
        assert_eq!(
            Cuboid::new(0, 10, 0, 10, 0, 10)
                .join_subdivide(&Cuboid::new(5, 15, 0, 10, 0, 10))
                .len(),
            3
        );

        // Intersects one face, greater height upwards
        assert_eq!(
            Cuboid::new(0, 10, 0, 10, 0, 10)
                .join_subdivide(&Cuboid::new(5, 15, 0, 10, 0, 15))
                .len(),
            5
        );

        // Intersects one face, greater height downwards
        assert_eq!(
            Cuboid::new(0, 10, 0, 10, 0, 10)
                .join_subdivide(&Cuboid::new(5, 15, 0, 10, -5, 10))
                .len(),
            5
        );

        // Intersects one face of the cuboid, with greater height in both directions
        assert_eq!(
            Cuboid::new(0, 10, 0, 10, 0, 10)
                .join_subdivide(&Cuboid::new(5, 15, 0, 10, -5, 15))
                .len(),
            7
        );

        // Intersects the cuboid in the middle, like a hamburger,
        // with same x, y sticking out towards negative
        assert_eq!(
            Cuboid::new(0, 10, 0, 10, 0, 10)
                .join_subdivide(&Cuboid::new(0, 10, -5, 10, 5, 6))
                .len(),
            4
        );

        // Intersects the cuboid in the middle, like a hamburger,
        // with same x, y sticking out towards negative
        assert_eq!(
            Cuboid::new(0, 10, 0, 10, 0, 10)
                .join_subdivide(&Cuboid::new(0, 10, 0, 15, 5, 6))
                .len(),
            4
        );

        // Intersects the cuboid in the middle, like a hamburger,
        // with same x, y sticking out towards positive
        assert_eq!(
            Cuboid::new(0, 10, 0, 10, 0, 10)
                .join_subdivide(&Cuboid::new(0, 10, -5, 10, 5, 6))
                .len(),
            4
        );

        // Intersects the cuboid in the middle, like a hamburger,
        // with same x, y sticking out towards both
        assert_eq!(
            Cuboid::new(0, 10, 0, 10, 0, 10)
                .join_subdivide(&Cuboid::new(0, 10, -5, 15, 5, 6))
                .len(),
            5
        );

        // Intersects the cuboid in the middle, like a hamburger,
        // sticking out to three directions
        assert_eq!(
            Cuboid::new(0, 10, 0, 10, 0, 10)
                .join_subdivide(&Cuboid::new(0, 15, -5, 15, 5, 6))
                .len(),
            8
        );

        // Intersects the cuboid in the middle, like a hamburger,
        // sticking out to all directions
        assert_eq!(
            Cuboid::new(0, 10, 0, 10, 0, 10)
                .join_subdivide(&Cuboid::new(-5, 15, -5, 15, 5, 6))
                .len(),
            11
        );

        // Pierces the cuboid in the middle, sticking out one side
        assert_eq!(
            Cuboid::new(0, 10, 0, 10, 0, 10)
                .join_subdivide(&Cuboid::new(5, 6, 5, 6, 0, 15))
                .len(),
            10
        );

        // Pierces the cuboid in the middle, sticking out both sides
        assert_eq!(
            Cuboid::new(0, 10, 0, 10, 0, 10)
                .join_subdivide(&Cuboid::new(5, 6, 5, 6, -5, 15))
                .len(),
            11
        );

        // Stabs the cuboid to the middle, not piercing, sticking out
        assert_eq!(
            Cuboid::new(0, 10, 0, 10, 0, 10)
                .join_subdivide(&Cuboid::new(5, 6, 5, 6, -6, 5))
                .len(),
            19
        );
        // The intersecting part is 1x1x5, the non-intersecting part is 1x1x6
        assert_eq!(
            Cuboid::new(0, 10, 0, 10, 0, 10)
                .join_subdivide(&Cuboid::new(5, 6, 5, 6, -6, 5))
                .iter()
                .map(|cuboid| cuboid.volume())
                .sum::<i32>()
                - Cuboid::new(0, 10, 0, 10, 0, 10).volume(),
            6
        );
    }
}
