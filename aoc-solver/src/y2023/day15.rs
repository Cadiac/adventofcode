use crate::solution::{AocError, Solution};

pub struct Day15;

fn hash(label: &str) -> usize {
    label
        .chars()
        .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

impl Solution for Day15 {
    type A = usize;
    type B = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day15.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let sum = input.trim().split(',').map(hash).sum();

        Ok(sum)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let initial_boxes: [Vec<(&str, u8)>; 256] = vec![Vec::new(); 256]
            .try_into()
            .map_err(|_| AocError::logic("Failed to initialize boxes"))?;

        let sum = input
            .trim()
            .split(',')
            .try_fold(initial_boxes, |mut boxes, step| {
                if let Some(label) = step.strip_suffix('-') {
                    let index = hash(label);

                    if let Some(lens) = boxes[index].iter().position(|(l, _)| *l == label) {
                        boxes[index].remove(lens);
                    }
                } else if let Some((label, focal_length)) = step.split_once('=') {
                    let index = hash(label);
                    let focal_length = focal_length
                        .parse()
                        .map_err(|err| AocError::parse(focal_length, err))?;

                    if let Some(lens) = boxes[index].iter().position(|(l, _)| *l == label) {
                        boxes[index][lens] = (label, focal_length)
                    } else {
                        boxes[index].push((label, focal_length));
                    }
                }

                Ok(boxes)
            })?
            .iter()
            .enumerate()
            .map(|(box_number, lens_box)| {
                lens_box
                    .iter()
                    .enumerate()
                    .map(|(slot_number, (_, focal_length))| {
                        (box_number + 1) * (slot_number + 1) * *focal_length as usize
                    })
                    .sum::<usize>()
            })
            .sum();

        Ok(sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day15.part_1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            Ok(1320)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day15.part_2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            Ok(145)
        );
    }
}
