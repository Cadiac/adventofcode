use itertools::Itertools;

use crate::solution::{AocError, Solution};

#[derive(Clone, Eq, PartialEq)]
struct Computer {
    a: u64,
    b: u64,
    c: u64,
    instruction_pointer: usize,
    program: Vec<u8>,
}

fn parse(input: &str) -> Result<Computer, AocError> {
    let (registers, program) = input
        .split_once("\n\n")
        .ok_or_else(|| AocError::parse(input, "Missing register or program sections"))?;

    let (a, b, c) = registers
        .lines()
        .collect_tuple()
        .ok_or(AocError::parse(registers, "Missing position"))?;

    let a = a
        .strip_prefix("Register A: ")
        .and_then(|a| a.parse::<u64>().ok())
        .ok_or_else(|| AocError::parse(a, "A"))?;

    let b = b
        .strip_prefix("Register B: ")
        .and_then(|b| b.parse::<u64>().ok())
        .ok_or_else(|| AocError::parse(b, "B"))?;
    let c = c
        .strip_prefix("Register C: ")
        .and_then(|c| c.parse::<u64>().ok())
        .ok_or_else(|| AocError::parse(c, "C"))?;

    let program = program
        .strip_prefix("Program: ")
        .ok_or_else(|| AocError::parse(program, "Program"))?;

    let program = program
        .split(",")
        .map(|instruction| {
            instruction
                .parse::<u8>()
                .map_err(|err| AocError::parse(instruction, err))
        })
        .try_collect()?;

    Ok(Computer {
        a,
        b,
        c,
        program,
        instruction_pointer: 0,
    })
}

fn resolve_combo(operand: &u8, computer: &Computer) -> u64 {
    match operand {
        0..=3 => *operand as u64,
        4 => computer.a,
        5 => computer.b,
        6 => computer.c,
        _ => unreachable!(),
    }
}

fn run_program(mut computer: Computer) -> Vec<u8> {
    let mut output = vec![];

    while let (Some(opcode), Some(operand)) = (
        computer.program.get(computer.instruction_pointer),
        computer.program.get(computer.instruction_pointer + 1),
    ) {
        computer.instruction_pointer += 2;

        match opcode {
            // `adv`, performs division of A register by two to the power of combo operand.
            0 => computer.a >>= resolve_combo(operand, &computer),
            // `bxl`, bitwise XOR of register B and the instruction's literal operand.
            1 => computer.b ^= *operand as u64,
            // `bst`, combo operand modulo 8 stored to B.
            2 => computer.b = resolve_combo(operand, &computer) % 8,
            // `jnz`, jumps instruction pointer if A is not zero to literal operand.
            3 if computer.a != 0 => computer.instruction_pointer = *operand as usize,
            // `bxc`, bitwise XOR of register B and register C to register B.
            4 => computer.b ^= computer.c,
            // `out`, combo operand modulo 8, then outputs that value.
            5 => output.push((resolve_combo(operand, &computer) & 0b111) as u8),
            // `bdv`, performs division of A register by two to the power of combo operand to register B.
            6 => computer.b = computer.a >> resolve_combo(operand, &computer),
            // `cdv`, performs division of A register by two to the power of combo operand to register C.
            7 => computer.c = computer.a >> resolve_combo(operand, &computer),
            _ => {}
        }
    }

    output
}

pub struct Day17;
impl Solution for Day17 {
    type Part1 = String;
    type Part2 = u64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day17.txt")
    }

    fn part_1(&self, input: &str) -> Result<String, AocError> {
        let computer = parse(input)?;
        let output = run_program(computer);

        Ok(output.into_iter().join(","))
    }

    fn part_2(&self, input: &str) -> Result<u64, AocError> {
        let Computer { program, .. } = parse(input)?;

        // This approach works on my (and the example) input, which has "0,3" as the
        // only instruction affecting A's value, dividing A by 8 and "3,0" as the last
        // instruction to halt the program when A equals zero, otherwise looping back to beginning.
        // This is the only loop in the instructions, and also conveniently B and C values are never
        // persisted between these loops.
        //
        // Knowing this, since we're expected to output 16 digits we can estimate that the loop has to run through
        // 16 times and A's initial value is somewhere between 8^15 and 8^16,
        // but those are too large numbers for efficient brute forcing.
        //
        // If we look for the expected instructions one by one starting from the end of the list,
        // at the time the last input is being output A has been divided by 8 (A >> 3)
        // so many times that there are only last three bits left. If we try all possible
        // values for these last bits (0b000 ..= 0b111) and see which ones produce the expected
        // instruction output, save all those, shift A left by three bits (A << 3) and try all of the
        // next three bit combinations to see which produce the second instruction we should be able to
        // eventually construct the full initial A value.

        let mut possible = vec![0];

        for instruction in program.iter().rev() {
            let mut next_possible = Vec::new();

            for prev_a in possible {
                for last_3_bits in 0b000..=0b111 {
                    let a = (prev_a << 3) + last_3_bits;
                    let output = run_program(Computer {
                        a,
                        b: 0,
                        c: 0,
                        instruction_pointer: 0,
                        program: program.clone(),
                    });

                    if let Some(produced) = output.first() {
                        if produced == instruction {
                            next_possible.push(a);
                        }
                    }
                }
            }

            possible = next_possible;
        }

        let initial_a = possible
            .into_iter()
            .min()
            .ok_or(AocError::logic("No solution"))?;

        Ok(initial_a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example_1() {
        assert_eq!(
            Day17.part_1(
                "Register A: 729\n\
                 Register B: 0\n\
                 Register C: 0\n\
                 \n\
                 Program: 0,1,5,4,3,0"
            ),
            Ok(String::from("4,6,3,5,6,3,5,2,1,0"))
        );
    }

    #[test]
    fn it_solves_part2_example_1() {
        assert_eq!(
            Day17.part_2(
                "Register A: 2024\n\
                 Register B: 0\n\
                 Register C: 0\n\
                 \n\
                 Program: 0,3,5,4,3,0"
            ),
            Ok(117440)
        );
    }
}
