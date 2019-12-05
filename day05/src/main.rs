const INPUT_FILE: &str = include_str!("../input.txt");

fn read_param(program: &Vec<i32>, parameter: usize, mode: i32) -> i32 {
    match mode {
        0 => program[parameter],
        1 => parameter as i32,
        _ => panic!("unknown mode")
    }
}

// Opcode 1 adds together numbers read from two positions and stores the result in a third position.
fn add(mut program: Vec<i32>, mut program_counter: usize, registers: (i32, i32), mode: (i32, i32, i32)) -> (Vec<i32>, usize, (i32, i32)) {
    let a_addr = program[program_counter + 1] as usize;
    let b_addr = program[program_counter + 2] as usize;
    let mem_addr = program[program_counter + 3] as usize;

    program[mem_addr] = read_param(&program, a_addr, mode.0) + read_param(&program, b_addr, mode.1);
    program_counter += 4;

    return (program, program_counter, registers);
}

// Opcode 2 works exactly like opcode 1, except it multiplies the two inputs instead of adding them.
fn mul(mut program: Vec<i32>, mut program_counter: usize, registers: (i32, i32), mode: (i32, i32, i32)) -> (Vec<i32>, usize, (i32, i32)) {
    let a_addr = program[program_counter + 1] as usize;
    let b_addr = program[program_counter + 2] as usize;
    let mem_addr = program[program_counter + 3] as usize;

    program[mem_addr] = read_param(&program, a_addr, mode.0) * read_param(&program, b_addr, mode.1);
    program_counter += 4;

    return (program, program_counter, registers);
}

// Opcode 3 takes a single integer as input and saves it to the position given by its only parameter. 
// For example, the instruction 3,50 would take an input value and store it at address 50.
fn load_in(mut program: Vec<i32>, mut program_counter: usize, registers: (i32, i32), _mode: (i32, i32, i32)) -> (Vec<i32>, usize, (i32, i32)) {
    let mem_addr = program[program_counter + 1] as usize;
    program[mem_addr] = registers.0;
    program_counter += 2;

    return (program, program_counter, registers);
}

// Opcode 4 outputs the value of its only parameter.
// For example, the instruction 4,50 would output the value at address 50.
fn load_out(program: Vec<i32>, mut program_counter: usize, mut registers: (i32, i32), mode: (i32, i32, i32)) -> (Vec<i32>, usize, (i32, i32)) {
    let mem_addr = program[program_counter + 1] as usize;
    registers.1 = read_param(&program, mem_addr, mode.0);
    program_counter += 2;

    return (program, program_counter, registers);
}

fn run_program(mut program: Vec<i32>, input: i32) -> (Vec<i32>, (i32, i32)) {
    let mut program_counter = 0;
    let mut registers = (input, 0);

    loop {
        let instruction = program[program_counter];

        let opcode = instruction % 100;
        let a_mode = (instruction % 1000) / 100;
        let b_mode = (instruction % 10000) / 1000;
        let c_mode = (instruction % 100000) / 10000;

        let mode = (a_mode, b_mode, c_mode);

        println!("[DEBUG]: pc: {}, opcode: {}, mode: {:?}", program_counter, opcode, mode);
        println!("[DEBUG]: program: {:?}", program);

        let (new_program, new_program_counter, new_registers) = match opcode {
            1 => add(program, program_counter, registers, mode),
            2 => mul(program, program_counter, registers, mode),
            3 => load_in(program, program_counter, registers, mode),
            4 => load_out(program, program_counter, registers, mode),
            99 => break,
            _ => {
                println!("[ERROR]: pc: {}, opcode: {}, program: {:?}", program_counter, program[program_counter], program);
                panic!("unknown opcode");
            }
        };
        program = new_program;
        program_counter = new_program_counter;
        registers = new_registers;
    }

    println!("[HALT]: registers: {:?}, pc: {}, opcode: {}, program: {:?}",
        registers, program_counter, program[program_counter], program);

    return (program, registers);
}

fn part_1(main_program: &Vec<i32>) -> i32 {
    let program = main_program.clone();

    let (_halt_state, registers) = run_program(program, 1);

    return registers.1;
}

fn main() -> () {
    let program: Vec<i32> = INPUT_FILE.split(',').map(|register| register.parse::<i32>()    .expect("Parse fail")).collect();
    let part1_program_result = part_1(&program);

    println!("Part 1: {}", part1_program_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_runs_part1_example_programs() {
        assert_eq!(run_program(vec![1,9,10,3,2,3,11,0,99,30,40,50], 0), (vec![3500,9,10,70,2,3,11,0,99,30,40,50], (0, 0)));
        assert_eq!(run_program(vec![1,0,0,0,99], 0), (vec![2,0,0,0,99], (0, 0)));
        assert_eq!(run_program(vec![2,3,0,3,99], 0), (vec![2,3,0,6,99], (0, 0)));
        assert_eq!(run_program(vec![2,4,4,5,99,0], 0), (vec![2,4,4,5,99,9801], (0, 0)));
        assert_eq!(run_program(vec![1,1,1,4,99,5,6,0,99], 0), (vec![30,1,1,4,2,5,6,0,99], (0, 0)));
    }

    #[test]
    fn it_solves_input_output_example() {
        assert_eq!(run_program(vec![3,0,4,0,99], 123), (vec![123,0,4,0,99], (123, 123)));
    }

    #[test]
    fn it_solves_parameter_mode_examples() {
        assert_eq!(run_program(vec![1002,4,3,4,33], 0), (vec![1002,4,3,4,99], (0, 0)));
    }

    #[test]
    fn it_solves_negative_examples() {
        assert_eq!(run_program(vec![1101,100,-1,4,0], 0), (vec![1101,100,-1,4,99], (0, 0)));
    }

    // #[test]
    // fn it_solves_part2() {
    //     let program: Vec<i32> = INPUT_FILE.split(',').map(|register| register.parse::<i32>().expect("Parse fail")).collect();
    //     assert_eq!(part_2(&program), 8444);
    // }
}
