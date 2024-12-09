use crate::solution::{AocError, Solution};

#[derive(Debug, PartialEq, Eq)]
enum Block {
    Free,
    File(u32),
}

fn parse(input: &str) -> Result<Vec<Block>, AocError> {
    let mut is_parsing_file = true;
    let mut id = 0;

    let mut blocks = Vec::new();
    for character in input.chars() {
        let size = character
            .to_digit(10)
            .ok_or_else(|| AocError::parse(character.to_string(), "Unexpected character"))?;

        if is_parsing_file {
            for _ in 0..size {
                blocks.push(Block::File(id));
            }
            id += 1;
            is_parsing_file = false;
        } else {
            for _ in 0..size {
                blocks.push(Block::Free);
            }
            is_parsing_file = true;
        }
    }

    Ok(blocks)
}

fn compact(blocks: &mut [Block]) {
    let mut head = 0;
    let mut tail = blocks.len() - 1;

    loop {
        while let Some(Block::File(_)) = blocks.get(head) {
            head += 1
        }

        while let Some(Block::Free) = blocks.get(tail) {
            tail -= 1
        }

        if head >= tail {
            return;
        }

        blocks.swap(head, tail);
    }
}

fn checksum(blocks: &[Block]) -> u32 {
    blocks
        .iter()
        .enumerate()
        .map_while(|(index, block)| match block {
            Block::File(id) => Some(index as u32 * id),
            Block::Free => None,
        })
        .sum()
}

pub struct Day09;
impl Solution for Day09 {
    type A = u32;
    type B = u32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day09.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let mut disk = parse(input)?;

        compact(&mut disk);

        Ok(checksum(&disk))
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(Day09.part_1("2333133121414131402"), Ok(1928));
    }

    #[test]
    fn it_should_parse() {
        assert_eq!(
            parse("12345"),
            Ok(vec![
                Block::File(0),
                Block::Free,
                Block::Free,
                Block::File(1),
                Block::File(1),
                Block::File(1),
                Block::Free,
                Block::Free,
                Block::Free,
                Block::Free,
                Block::File(2),
                Block::File(2),
                Block::File(2),
                Block::File(2),
                Block::File(2),
            ])
        );
    }

    #[test]
    fn it_should_compact() {
        let mut disk = [
            Block::File(0),
            Block::Free,
            Block::Free,
            Block::File(1),
            Block::File(1),
            Block::File(1),
            Block::Free,
            Block::Free,
            Block::Free,
            Block::Free,
            Block::File(2),
            Block::File(2),
            Block::File(2),
            Block::File(2),
            Block::File(2),
        ];

        compact(&mut disk);

        assert_eq!(
            disk,
            [
                Block::File(0),
                Block::File(2),
                Block::File(2),
                Block::File(1),
                Block::File(1),
                Block::File(1),
                Block::File(2),
                Block::File(2),
                Block::File(2),
                Block::Free,
                Block::Free,
                Block::Free,
                Block::Free,
                Block::Free,
                Block::Free,
            ]
        );
    }

    #[test]
    fn it_should_calculate_checkshum() {
        let disk = [
            Block::File(0),
            Block::File(2),
            Block::File(2),
            Block::File(1),
            Block::File(1),
            Block::File(1),
            Block::File(2),
            Block::File(2),
            Block::File(2),
            Block::Free,
            Block::Free,
            Block::Free,
            Block::Free,
            Block::Free,
            Block::Free,
        ];

        assert_eq!(checksum(&disk), 60);
    }
}
