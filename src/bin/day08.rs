extern crate nom;

use std::collections::HashSet;

use nom::{branch::alt, bytes::complete::tag, character::complete::space0, IResult};

const INPUT_FILE: &str = include_str!("../../inputs/day08.txt");

#[derive(PartialEq, Debug)]
pub enum Flag {
    Halted,
    Running,
    InfiniteLoop,
    Exception,
}

#[derive(Debug, Default, Clone)]
pub struct Instruction {
    operation: String,
    arg: i32,
}

#[derive(Debug, Default)]
pub struct GameConsole {
    pub mem: Vec<Instruction>,
    pub program_counter: usize,
    pub accumulator: i32,
}

impl GameConsole {
    // `acc` increases or decreases a single global value called the accumulator by the value given in the argument.
    #[inline]
    fn acc(&mut self) -> Flag {
        self.accumulator += self.mem[self.program_counter].arg;
        self.program_counter += 1;
        return Flag::Running;
    }

    // `jmp` jumps to a new instruction relative to itself.
    // The next instruction to execute is found using the argument as an offset from the jmp instruction.
    #[inline]
    fn jmp(&mut self) -> Flag {
        self.program_counter =
            (self.program_counter as i32 + self.mem[self.program_counter].arg) as usize;
        return Flag::Running;
    }

    // `nop` stands for No OPeration - it does nothing. The instruction immediately below it is executed next.
    #[inline]
    fn nop(&mut self) -> Flag {
        self.program_counter += 1;
        return Flag::Running;
    }

    pub fn run_program(&mut self) -> Flag {
        let mut executed: HashSet<usize> = HashSet::new();
        loop {
            if !executed.insert(self.program_counter) {
                return Flag::InfiniteLoop;
            }

            if self.program_counter >= self.mem.len() {
                return Flag::Halted;
            }

            match self.mem[self.program_counter].operation.as_str() {
                "acc" => self.acc(),
                "jmp" => self.jmp(),
                "nop" => self.nop(),
                _ => panic!(),
            };
        }
    }
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (i, operation) = alt((tag("acc"), tag("jmp"), tag("nop")))(input)?;
    let (i, _ws) = space0(i)?;
    let arg = i32::from_str_radix(i, 10).unwrap();

    let instruction = Instruction {
        operation: operation.to_string(),
        arg: arg,
    };

    return Ok(("", instruction));
}

fn part_1(input: &str) -> i32 {
    let instructions: Vec<Instruction> = input
        .lines()
        .filter_map(|line| parse_instruction(line).ok())
        .map(|(_, instruction)| instruction)
        .collect();

    let mut console = GameConsole {
        mem: instructions,
        ..Default::default()
    };

    let _flag = console.run_program();

    console.accumulator
}

fn part_2(input: &str) -> i32 {
    let corrupted_instructions: Vec<Instruction> = input
        .lines()
        .filter_map(|line| parse_instruction(line).ok())
        .map(|(_, instruction)| instruction)
        .collect();

    for (i, instruction) in corrupted_instructions.iter().enumerate() {
        if instruction.operation == "acc" {
            continue;
        }

        let mut fixed_instructions = corrupted_instructions.clone();

        match instruction.operation.as_str() {
            "nop" => fixed_instructions[i].operation = String::from("jmp"),
            "jmp" => fixed_instructions[i].operation = String::from("nop"),
            _ => continue,
        }

        let mut console = GameConsole {
            mem: fixed_instructions,
            ..Default::default()
        };
    
        let flag = console.run_program();

        if flag == Flag::Halted {
            return console.accumulator;
        }
    }

    return -1;
}

fn main() -> () {
    let part_1_result = part_1(INPUT_FILE);
    let part_2_result = part_2(INPUT_FILE);

    println!("[INFO]: Part 1: {:?}", part_1_result);
    println!("[INFO]: Part 2: {:?}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            part_1(
                "nop +0\n\
                 acc +1\n\
                 jmp +4\n\
                 acc +3\n\
                 jmp -3\n\
                 acc -99\n\
                 acc +1\n\
                 jmp -4\n\
                 acc +6"
            ),
            5
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            part_2(
                "nop +0\n\
                 acc +1\n\
                 jmp +4\n\
                 acc +3\n\
                 jmp -3\n\
                 acc -99\n\
                 acc +1\n\
                 jmp -4\n\
                 acc +6"
            ),
            8
        );
    }
}