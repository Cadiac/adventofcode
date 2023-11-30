use std::cmp::{max, min};

use crate::solution::{AocError, Solution};

pub struct Day22;

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

    fn volume(&self) -> usize {
        (self.x_max - self.x_min) as usize
            * (self.y_max - self.y_min) as usize
            * (self.z_max - self.z_min) as usize
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

    fn remove_intersection(&self, cuboid: &Cuboid) -> Vec<Cuboid> {
        if !self.intersects_cuboid(cuboid) {
            return vec![*self];
        }

        if cuboid.contains_cuboid(self) {
            return vec![];
        }

        let intersection = self.intersection(cuboid);

        // Now construct the 3x3x3 cuboids excluding the intersection
        let mut cuboids: Vec<Cuboid> = Vec::with_capacity(26);

        for (x_min, x_max) in [
            (self.x_min, intersection.x_min),
            (intersection.x_min, intersection.x_max),
            (intersection.x_max, self.x_max),
        ] {
            for (y_min, y_max) in [
                (self.y_min, intersection.y_min),
                (intersection.y_min, intersection.y_max),
                (intersection.y_max, self.y_max),
            ] {
                for (z_min, z_max) in [
                    (self.z_min, intersection.z_min),
                    (intersection.z_min, intersection.z_max),
                    (intersection.z_max, self.z_max),
                ] {
                    // Skip the intersection
                    if x_min == intersection.x_min
                        && x_max == intersection.x_max
                        && y_min == intersection.y_min
                        && y_max == intersection.y_max
                        && z_min == intersection.z_min
                        && z_max == intersection.z_max
                    {
                        continue;
                    }

                    let new_cuboid = Cuboid::new(x_min, x_max, y_min, y_max, z_min, z_max);
                    if new_cuboid.volume() > 0 {
                        cuboids.push(new_cuboid);
                    }
                }
            }
        }

        cuboids
    }
}

fn reboot_reactor(input: &str, initialization_procedure: bool) -> usize {
    let mut cuboids: Vec<Cuboid> = Vec::new();

    for line in input.lines() {
        let (state, (x_min, x_max, y_min, y_max, z_min, z_max)): (&str, _) =
            serde_scan::scan!("{} x={}..{},y={}..{},z={}..{}" <- line).unwrap();

        let cuboid = if initialization_procedure {
            if x_min > 50 || x_max < -50 || y_min > 50 || y_max < -50 || z_min > 50 || z_max < -50 {
                continue;
            }

            Cuboid::new(
                max(x_min, -50),
                min(x_max, 50) + 1,
                max(y_min, -50),
                min(y_max, 50) + 1,
                max(z_min, -50),
                min(z_max, 50) + 1,
            )
        } else {
            Cuboid::new(x_min, x_max + 1, y_min, y_max + 1, z_min, z_max + 1)
        };

        // First remove the intersection of this cuboid from any discovered cuboids
        cuboids = cuboids
            .iter()
            .flat_map(|c| c.remove_intersection(&cuboid))
            .collect();

        if state == "on" {
            // Then if the state is on add the cuboid into discovered cuboids
            cuboids.push(cuboid);
        }
    }

    cuboids.iter().map(|c| c.volume()).sum::<usize>()
}

impl Solution for Day22 {
    type F = usize;
    type S = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2021/day22.txt")
    }

    fn part_1(&self, input: &str) -> Result<Self::F, AocError> {
        Ok(reboot_reactor(input, true))
    }

    fn part_2(&self, input: &str) -> Result<Self::S, AocError> {
        Ok(reboot_reactor(input, false))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_small_example() {
        assert_eq!(
            reboot_reactor(
                "on x=10..12,y=10..12,z=10..12\n\
                 on x=11..13,y=11..13,z=11..13\n\
                 off x=9..11,y=9..11,z=9..11\n\
                 on x=10..10,y=10..10,z=10..10",
                true
            ),
            39
        );
    }

    #[test]
    fn it_solves_part1_large_example() {
        assert_eq!(
            reboot_reactor(
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
                true
            ),
            590784
        );
    }

    #[test]
    fn it_solves_part2_example_1() {
        assert_eq!(
            reboot_reactor(
                "on x=-5..47,y=-31..22,z=-19..33\n\
                 on x=-44..5,y=-27..21,z=-14..35\n\
                 on x=-49..-1,y=-11..42,z=-10..38\n\
                 on x=-20..34,y=-40..6,z=-44..1\n\
                 off x=26..39,y=40..50,z=-2..11\n\
                 on x=-41..5,y=-41..6,z=-36..8\n\
                 off x=-43..-33,y=-45..-28,z=7..25\n\
                 on x=-33..15,y=-32..19,z=-34..11\n\
                 off x=35..47,y=-46..-34,z=-11..5\n\
                 on x=-14..36,y=-6..44,z=-16..29\n\
                 on x=-57795..-6158,y=29564..72030,z=20435..90618\n\
                 on x=36731..105352,y=-21140..28532,z=16094..90401\n\
                 on x=30999..107136,y=-53464..15513,z=8553..71215\n\
                 on x=13528..83982,y=-99403..-27377,z=-24141..23996\n\
                 on x=-72682..-12347,y=18159..111354,z=7391..80950\n\
                 on x=-1060..80757,y=-65301..-20884,z=-103788..-16709\n\
                 on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856\n\
                 on x=-52752..22273,y=-49450..9096,z=54442..119054\n\
                 on x=-29982..40483,y=-108474..-28371,z=-24328..38471\n\
                 on x=-4958..62750,y=40422..118853,z=-7672..65583\n\
                 on x=55694..108686,y=-43367..46958,z=-26781..48729\n\
                 on x=-98497..-18186,y=-63569..3412,z=1232..88485\n\
                 on x=-726..56291,y=-62629..13224,z=18033..85226\n\
                 on x=-110886..-34664,y=-81338..-8658,z=8914..63723\n\
                 on x=-55829..24974,y=-16897..54165,z=-121762..-28058\n\
                 on x=-65152..-11147,y=22489..91432,z=-58782..1780\n\
                 on x=-120100..-32970,y=-46592..27473,z=-11695..61039\n\
                 on x=-18631..37533,y=-124565..-50804,z=-35667..28308\n\
                 on x=-57817..18248,y=49321..117703,z=5745..55881\n\
                 on x=14781..98692,y=-1341..70827,z=15753..70151\n\
                 on x=-34419..55919,y=-19626..40991,z=39015..114138\n\
                 on x=-60785..11593,y=-56135..2999,z=-95368..-26915\n\
                 on x=-32178..58085,y=17647..101866,z=-91405..-8878\n\
                 on x=-53655..12091,y=50097..105568,z=-75335..-4862\n\
                 on x=-111166..-40997,y=-71714..2688,z=5609..50954\n\
                 on x=-16602..70118,y=-98693..-44401,z=5197..76897\n\
                 on x=16383..101554,y=4615..83635,z=-44907..18747\n\
                 off x=-95822..-15171,y=-19987..48940,z=10804..104439\n\
                 on x=-89813..-14614,y=16069..88491,z=-3297..45228\n\
                 on x=41075..99376,y=-20427..49978,z=-52012..13762\n\
                 on x=-21330..50085,y=-17944..62733,z=-112280..-30197\n\
                 on x=-16478..35915,y=36008..118594,z=-7885..47086\n\
                 off x=-98156..-27851,y=-49952..43171,z=-99005..-8456\n\
                 off x=2032..69770,y=-71013..4824,z=7471..94418\n\
                 on x=43670..120875,y=-42068..12382,z=-24787..38892\n\
                 off x=37514..111226,y=-45862..25743,z=-16714..54663\n\
                 off x=25699..97951,y=-30668..59918,z=-15349..69697\n\
                 off x=-44271..17935,y=-9516..60759,z=49131..112598\n\
                 on x=-61695..-5813,y=40978..94975,z=8655..80240\n\
                 off x=-101086..-9439,y=-7088..67543,z=33935..83858\n\
                 off x=18020..114017,y=-48931..32606,z=21474..89843\n\
                 off x=-77139..10506,y=-89994..-18797,z=-80..59318\n\
                 off x=8476..79288,y=-75520..11602,z=-96624..-24783\n\
                 on x=-47488..-1262,y=24338..100707,z=16292..72967\n\
                 off x=-84341..13987,y=2429..92914,z=-90671..-1318\n\
                 off x=-37810..49457,y=-71013..-7894,z=-105357..-13188\n\
                 off x=-27365..46395,y=31009..98017,z=15428..76570\n\
                 off x=-70369..-16548,y=22648..78696,z=-1892..86821\n\
                 on x=-53470..21291,y=-120233..-33476,z=-44150..38147\n\
                 off x=-93533..-4276,y=-16170..68771,z=-104985..-24507",
                true
            ),
            474140
        );
    }

    #[test]
    fn it_solves_part2_example_2() {
        assert_eq!(
            reboot_reactor(
                "on x=-5..47,y=-31..22,z=-19..33\n\
                 on x=-44..5,y=-27..21,z=-14..35\n\
                 on x=-49..-1,y=-11..42,z=-10..38\n\
                 on x=-20..34,y=-40..6,z=-44..1\n\
                 off x=26..39,y=40..50,z=-2..11\n\
                 on x=-41..5,y=-41..6,z=-36..8\n\
                 off x=-43..-33,y=-45..-28,z=7..25\n\
                 on x=-33..15,y=-32..19,z=-34..11\n\
                 off x=35..47,y=-46..-34,z=-11..5\n\
                 on x=-14..36,y=-6..44,z=-16..29\n\
                 on x=-57795..-6158,y=29564..72030,z=20435..90618\n\
                 on x=36731..105352,y=-21140..28532,z=16094..90401\n\
                 on x=30999..107136,y=-53464..15513,z=8553..71215\n\
                 on x=13528..83982,y=-99403..-27377,z=-24141..23996\n\
                 on x=-72682..-12347,y=18159..111354,z=7391..80950\n\
                 on x=-1060..80757,y=-65301..-20884,z=-103788..-16709\n\
                 on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856\n\
                 on x=-52752..22273,y=-49450..9096,z=54442..119054\n\
                 on x=-29982..40483,y=-108474..-28371,z=-24328..38471\n\
                 on x=-4958..62750,y=40422..118853,z=-7672..65583\n\
                 on x=55694..108686,y=-43367..46958,z=-26781..48729\n\
                 on x=-98497..-18186,y=-63569..3412,z=1232..88485\n\
                 on x=-726..56291,y=-62629..13224,z=18033..85226\n\
                 on x=-110886..-34664,y=-81338..-8658,z=8914..63723\n\
                 on x=-55829..24974,y=-16897..54165,z=-121762..-28058\n\
                 on x=-65152..-11147,y=22489..91432,z=-58782..1780\n\
                 on x=-120100..-32970,y=-46592..27473,z=-11695..61039\n\
                 on x=-18631..37533,y=-124565..-50804,z=-35667..28308\n\
                 on x=-57817..18248,y=49321..117703,z=5745..55881\n\
                 on x=14781..98692,y=-1341..70827,z=15753..70151\n\
                 on x=-34419..55919,y=-19626..40991,z=39015..114138\n\
                 on x=-60785..11593,y=-56135..2999,z=-95368..-26915\n\
                 on x=-32178..58085,y=17647..101866,z=-91405..-8878\n\
                 on x=-53655..12091,y=50097..105568,z=-75335..-4862\n\
                 on x=-111166..-40997,y=-71714..2688,z=5609..50954\n\
                 on x=-16602..70118,y=-98693..-44401,z=5197..76897\n\
                 on x=16383..101554,y=4615..83635,z=-44907..18747\n\
                 off x=-95822..-15171,y=-19987..48940,z=10804..104439\n\
                 on x=-89813..-14614,y=16069..88491,z=-3297..45228\n\
                 on x=41075..99376,y=-20427..49978,z=-52012..13762\n\
                 on x=-21330..50085,y=-17944..62733,z=-112280..-30197\n\
                 on x=-16478..35915,y=36008..118594,z=-7885..47086\n\
                 off x=-98156..-27851,y=-49952..43171,z=-99005..-8456\n\
                 off x=2032..69770,y=-71013..4824,z=7471..94418\n\
                 on x=43670..120875,y=-42068..12382,z=-24787..38892\n\
                 off x=37514..111226,y=-45862..25743,z=-16714..54663\n\
                 off x=25699..97951,y=-30668..59918,z=-15349..69697\n\
                 off x=-44271..17935,y=-9516..60759,z=49131..112598\n\
                 on x=-61695..-5813,y=40978..94975,z=8655..80240\n\
                 off x=-101086..-9439,y=-7088..67543,z=33935..83858\n\
                 off x=18020..114017,y=-48931..32606,z=21474..89843\n\
                 off x=-77139..10506,y=-89994..-18797,z=-80..59318\n\
                 off x=8476..79288,y=-75520..11602,z=-96624..-24783\n\
                 on x=-47488..-1262,y=24338..100707,z=16292..72967\n\
                 off x=-84341..13987,y=2429..92914,z=-90671..-1318\n\
                 off x=-37810..49457,y=-71013..-7894,z=-105357..-13188\n\
                 off x=-27365..46395,y=31009..98017,z=15428..76570\n\
                 off x=-70369..-16548,y=22648..78696,z=-1892..86821\n\
                 on x=-53470..21291,y=-120233..-33476,z=-44150..38147\n\
                 off x=-93533..-4276,y=-16170..68771,z=-104985..-24507",
                false
            ),
            2758514936282235
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
    fn it_removes_cuboid_intersection() {
        // Removes an intersection from one corner
        //       .+------+
        //     .' |    .'|
        //    +---+--+'  |
        //    |   |  |   |.+------+
        //    |  .+--+-XXX |    .'|
        //    |.'    | XX--+--+'  |
        //    +------+'|   |  |   |
        //             |  .+--+---+
        //             |.'    | .'
        //             +------+'
        assert_eq!(
            Cuboid::new(0, 10, 0, 10, 0, 10)
                .remove_intersection(&Cuboid::new(5, 15, 5, 15, 5, 15))
                .len(),
            7
        );

        // Removes an intersection on one edge
        //       .+------+
        //     .' |    .'|
        //    +---+-XX'  |
        //    |   | XX   |
        //    |  .+-XX---+
        //    |.'   XX .'
        //    +-----XX'
        assert_eq!(
            Cuboid::new(0, 10, 0, 10, 0, 10)
                .remove_intersection(&Cuboid::new(5, 15, 5, 15, 0, 10))
                .len(),
            3
        );

        // Removes an intersection on one face
        //       .+------+
        //     .' |    .'|
        //    +======+'  |
        //    |XXXXXX|   |
        //    |XXXXXX+---+
        //    |XXXXXX| .'
        //    +======+'
        assert_eq!(
            Cuboid::new(0, 10, 0, 10, 0, 10)
                .remove_intersection(&Cuboid::new(5, 15, 0, 10, 0, 10))
                .len(),
            1
        );

        // Removes an intersection through the cuboid
        //       .+--XX--+
        //     .' |XX  .'|
        //    +--XX--+'  |
        //    |  XX  |   |
        //    |  XX--+---+
        //    |.'XX  | .'
        //    +--XX--+'
        assert_eq!(
            Cuboid::new(0, 10, 0, 10, 0, 10)
                .remove_intersection(&Cuboid::new(5, 6, 5, 6, 5, 6))
                .len(),
            26
        );

        // Removes an intersection piercing through the cuboid
        //       .+------+
        //     .' |    .'|
        //    +---+--+'  |
        //    | XXXXX|XX |
        //    |  .+--+---+
        //    |.'    | .'
        //    +------+'
        assert_eq!(
            Cuboid::new(0, 10, 0, 10, 0, 10)
                .remove_intersection(&Cuboid::new(5, 6, 5, 6, -5, 15))
                .len(),
            8
        );

        // Removes an intersection piercing into the cuboid, but not piercing through
        //       .+------+
        //     .' |    .'|
        //    +---+--+'  |
        //    |   | X|XX |
        //    |  .+--+---+
        //    |.'    | .'
        //    +------+'
        assert_eq!(
            Cuboid::new(0, 10, 0, 10, 0, 10)
                .remove_intersection(&Cuboid::new(5, 6, 5, 6, -6, 5))
                .len(),
            17
        );
    }
}
