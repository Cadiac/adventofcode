use crate::solution::{AocError, Solution};

#[derive(Debug, PartialEq, Eq)]
enum Block {
    Free,
    File(u64, usize),
}

fn parse(input: &str) -> Result<Vec<Block>, AocError> {
    let mut is_parsing_file = true;
    let mut id = 0;
    let mut blocks = Vec::new();

    for c in input.chars() {
        let size = c
            .to_digit(10)
            .ok_or_else(|| AocError::parse(c.to_string(), "Unexpected character"))?
            as usize;

        if is_parsing_file {
            blocks.extend((0..size).map(|_| Block::File(id, size)));
            id += 1;
        } else {
            blocks.extend((0..size).map(|_| Block::Free));
        }

        is_parsing_file = !is_parsing_file;
    }

    Ok(blocks)
}

fn compact_by_block(blocks: &mut [Block]) {
    let mut head = 0;
    let mut tail = blocks.len() - 1;

    loop {
        while let Some(Block::File(_, _)) = blocks.get(head) {
            head += 1
        }

        while let Some(Block::Free) = blocks.get(tail) {
            tail -= 1
        }

        if head > tail {
            return;
        }

        blocks.swap(head, tail);
    }
}

fn compact_by_file(blocks: &mut [Block]) {
    let mut tail = blocks.len().saturating_sub(1);

    while let Some(space_required) = next_file_from_tail(blocks, &mut tail) {
        if !move_file_if_space(blocks, &mut tail, space_required) {
            if space_required > tail {
                return;
            }

            tail -= space_required;
        }
    }
}

fn next_file_from_tail(blocks: &[Block], tail: &mut usize) -> Option<usize> {
    while *tail > 0 && matches!(blocks[*tail], Block::Free) {
        *tail -= 1;
    }

    match blocks[*tail] {
        Block::File(_, size) => Some(size),
        _ => None,
    }
}

fn move_file_if_space(blocks: &mut [Block], tail: &mut usize, space_required: usize) -> bool {
    let mut head = 0;
    let mut continuous_space = 0;

    while head < *tail {
        match blocks[head] {
            Block::Free => {
                continuous_space += 1;
                head += 1;

                if continuous_space >= space_required {
                    let start = head - continuous_space;

                    for i in 0..space_required {
                        blocks.swap(start + i, *tail - i);
                    }

                    *tail -= space_required;
                    return true;
                }
            }
            Block::File(_, size) => {
                head += size;
                continuous_space = 0;
            }
        }
    }

    false
}

fn checksum(blocks: &[Block]) -> u64 {
    blocks
        .iter()
        .enumerate()
        .flat_map(|(index, block)| match block {
            Block::File(id, _size) => Some(index as u64 * id),
            Block::Free => None,
        })
        .sum()
}

pub struct Day09;
impl Solution for Day09 {
    type Part1 = u64;
    type Part2 = u64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day09.txt")
    }

    fn part_1(&self, input: &str) -> Result<u64, AocError> {
        let mut disk = parse(input)?;

        compact_by_block(&mut disk);

        Ok(checksum(&disk))
    }

    fn part_2(&self, input: &str) -> Result<u64, AocError> {
        let mut disk = parse(input)?;

        compact_by_file(&mut disk);

        Ok(checksum(&disk))
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
    fn it_solves_part2_example() {
        assert_eq!(Day09.part_2("2333133121414131402"), Ok(2858));
    }

    #[test]
    fn it_should_parse() {
        assert_eq!(
            parse("12345"),
            Ok(vec![
                Block::File(0, 1),
                Block::Free,
                Block::Free,
                Block::File(1, 3),
                Block::File(1, 3),
                Block::File(1, 3),
                Block::Free,
                Block::Free,
                Block::Free,
                Block::Free,
                Block::File(2, 5),
                Block::File(2, 5),
                Block::File(2, 5),
                Block::File(2, 5),
                Block::File(2, 5),
            ])
        );
    }

    #[test]
    fn it_should_compact_by_block() {
        let mut disk = [
            Block::File(0, 1),
            Block::Free,
            Block::Free,
            Block::File(1, 3),
            Block::File(1, 3),
            Block::File(1, 3),
            Block::Free,
            Block::Free,
            Block::Free,
            Block::Free,
            Block::File(2, 5),
            Block::File(2, 5),
            Block::File(2, 5),
            Block::File(2, 5),
            Block::File(2, 5),
        ];

        compact_by_block(&mut disk);

        assert_eq!(
            disk,
            [
                Block::File(0, 1),
                Block::File(2, 5),
                Block::File(2, 5),
                Block::File(1, 3),
                Block::File(1, 3),
                Block::File(1, 3),
                Block::File(2, 5),
                Block::File(2, 5),
                Block::File(2, 5),
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
    fn it_should_compact_by_file() {
        let mut disk = [
            Block::File(0, 1),
            Block::Free,
            Block::Free,
            Block::Free,
            Block::File(1, 2),
            Block::File(1, 2),
            Block::Free,
            Block::Free,
            Block::Free,
            Block::Free,
            Block::Free,
            Block::File(2, 4),
            Block::File(2, 4),
            Block::File(2, 4),
            Block::File(2, 4),
        ];

        compact_by_file(&mut disk);

        assert_eq!(
            disk,
            [
                Block::File(0, 1),
                Block::File(1, 2),
                Block::File(1, 2),
                Block::Free,
                Block::Free,
                Block::Free,
                Block::File(2, 4),
                Block::File(2, 4),
                Block::File(2, 4),
                Block::File(2, 4),
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
            Block::File(0, 1),
            Block::File(2, 5),
            Block::File(2, 5),
            Block::File(1, 3),
            Block::File(1, 3),
            Block::File(1, 3),
            Block::File(2, 5),
            Block::File(2, 5),
            Block::File(2, 5),
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
