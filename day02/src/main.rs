use std::fs::File;
use std::io::prelude::*;

fn run_program(mut program: Vec<usize>) -> Vec<usize> {
    let mut program_counter = 0;

    loop {
        if program.len() > 0 {
            match program[program_counter] {
                1 => {
                    // Opcode 1 adds together numbers read from two positions and stores the result in a third position.
                    // println!("pc: {}, opcode: 1, program: {:?}", program_counter, program);
                    if program.len() - program_counter < 4 {
                        panic!("program_counter out of range");
                    }
                    let a_addr = program[program_counter + 1];
                    let b_addr = program[program_counter + 2];
                    let mem_addr = program[program_counter + 3];
                    if program.len() < mem_addr && program.len() < a_addr && program.len() < b_addr {
                        panic!("register out of range");
                    }
                    program[mem_addr] = program[a_addr] + program[b_addr];
                    program_counter += 4;
                },
                2 => {
                    // Opcode 2 works exactly like opcode 1, except it multiplies the two inputs instead of adding them.
                    // println!("pc: {}, opcode: 2, program: {:?}", program_counter, program);
                    if program.len() - program_counter < 4 {
                        panic!("program_counter out of range");
                    }
                    let a_addr = program[program_counter + 1];
                    let b_addr = program[program_counter + 2];
                    let mem_addr = program[program_counter + 3];
                    if program.len() < mem_addr && program.len() < a_addr && program.len() < b_addr {
                        panic!("register out of range");
                    }
                    program[mem_addr] = program[a_addr] * program[b_addr];
                    program_counter += 4;
                },
                99 => {
                    // 99 means that the program is finished and should immediately halt
                    // println!("pc: {}, opcode: 99, program: {:?}", program_counter, program);
                    break;
                },
                _ => {
                    println!("pc: {}, opcode: {}, program: {:?}", program_counter, program[program_counter], program);
                    panic!("unknown opcode")
                }
            }
        }
    }

    return program;
}

fn part_1(file_name: String) -> usize {
    let mut file = File::open(file_name).expect("failed to open file");
    let mut program_buffer = String::new();
    file.read_to_string(&mut program_buffer).expect("failed to read file to string");

    let mut program: Vec<usize> = program_buffer.split(',').map(|register| register.parse::<usize>().expect("Parse fail")).collect();

    // To do this, before running the program, replace position 1 with the value 12 and replace position 2 with the value 2.
    program[1] = 12;
    program[2] = 2;

    let halt_state = run_program(program);

    // What value is left at position 0 after the program halts?
    return halt_state[0];
}

fn part_2(file_name: String) -> usize {
    let mut file = File::open(file_name).expect("failed to open file");
    let mut program_buffer = String::new();
    file.read_to_string(&mut program_buffer).expect("failed to read file to string");

    let main_program: Vec<usize> = program_buffer.split(',').map(|register| register.parse::<usize>().expect("Parse fail")).collect();

    for noun in 0..99 {
        for verb in 0..99 {
            let mut program = main_program.clone();

            program[1] = noun;
            program[2] = verb;

            let halt_state = run_program(program);
            let output = halt_state[0];

            if output == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    panic!("no results");
}

fn main() -> () {
    let part1_program_result = part_1(String::from("./input.txt"));
    let part2_program_result = part_2(String::from("./input.txt"));

    println!("Part 1: {}", part1_program_result);
    println!("Part 2: {}", part2_program_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_runs_part1_example_programs() {
        assert_eq!(run_program(vec![1,9,10,3,2,3,11,0,99,30,40,50]), vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
        assert_eq!(run_program(vec![1,0,0,0,99]), vec![2,0,0,0,99]);
        assert_eq!(run_program(vec![2,3,0,3,99]), vec![2,3,0,6,99]);
        assert_eq!(run_program(vec![2,4,4,5,99,0]), vec![2,4,4,5,99,9801]);
        assert_eq!(run_program(vec![1,1,1,4,99,5,6,0,99]), vec![30,1,1,4,2,5,6,0,99]);
    }

    #[test]
    fn it_solves_part2() {
        assert_eq!(part_2(String::from("./input.txt")), 8444);
    }
}
