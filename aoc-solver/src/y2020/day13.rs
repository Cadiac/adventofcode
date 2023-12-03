use crate::solution::{AocError, Solution};

pub struct Day13;

fn find_first_after(freq: i64, earliest: i64) -> i64 {
    let n = earliest / freq;
    freq * (n + 1)
}

// Modular inverse, taken from
// https://rosettacode.org/wiki/Modular_inverse
fn mod_inv(a: i64, module: i64) -> i64 {
    let mut mn = (module, a);
    let mut xy = (0, 1);
    while mn.1 != 0 {
        xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
        mn = (mn.1, mn.0 % mn.1);
    }
    while xy.0 < 0 {
        xy.0 += module;
    }
    xy.0
}

impl Solution for Day13 {
    type F = i64;
    type S = i64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2020/day13.txt")
    }

    fn part_1(&self, input: &str) -> Result<i64, AocError> {
        let time_of_leave = input.lines().nth(0).unwrap().parse::<i64>().unwrap();

        let schedules: Vec<i64> = input
            .lines()
            .nth(1)
            .unwrap()
            .split(',')
            .flat_map(|bus| {
                if bus == "x" {
                    return None;
                }
                Some(bus.parse::<i64>().unwrap())
            })
            .collect();

        let earliest_bus = schedules
            .iter()
            .min_by(|x, y| {
                find_first_after(**x, time_of_leave).cmp(&find_first_after(**y, time_of_leave))
            })
            .ok_or(AocError::logic("No earliest bus"))?;

        let result =
            earliest_bus * (find_first_after(*earliest_bus, time_of_leave) - time_of_leave);

        Ok(result)
    }

    fn part_2(&self, input: &str) -> Result<i64, AocError> {
        let schedules: Vec<(i64, i64)> = input
            .lines()
            .nth(1)
            .unwrap()
            .split(',')
            .enumerate()
            .flat_map(|(offset, freq)| {
                if freq == "x" {
                    return None;
                }
                Some((freq.parse::<i64>().unwrap(), offset as i64))
            })
            .collect();

        // This problem is https://en.wikipedia.org/wiki/Chinese_remainder_theorem.
        // I initially used an online solver https://www.dcode.fr/chinese-remainder to
        // solve my input using the remainders and modulos I just printed out here.
        // Afterwards I implemented the solver following an example from
        // https://www.geeksforgeeks.org/chinese-remainder-theorem-set-2-implementation/
        let prod: i64 = schedules.iter().map(|(freq, _offset)| freq).product();
        let result: i64 = schedules
            .iter()
            .map(|(freq, offset)| {
                let remainder = (freq - offset) % freq;
                let pp = prod / freq;

                remainder * mod_inv(pp, *freq) * pp
            })
            .sum();

        Ok(result % prod)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day13.part_1(
                "939\n\
                 7,13,x,x,59,x,31,19"
            ),
            Ok(295)
        )
    }

    #[test]
    fn it_solves_part2_examples() {
        assert_eq!(Day13.part_2("\n7,13,x,x,59,x,31,19"), Ok(1068781));
        assert_eq!(Day13.part_2("\n17,x,13,19"), Ok(3417));
        assert_eq!(Day13.part_2("\n67,7,59,61"), Ok(754018));
        assert_eq!(Day13.part_2("\n67,x,7,59,61"), Ok(779210));
        assert_eq!(Day13.part_2("\n67,7,x,59,61"), Ok(1261476));
        assert_eq!(Day13.part_2("\n1789,37,47,1889"), Ok(1202161486));
    }
}
