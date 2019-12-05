const INPUT_FILE: &str = include_str!("../input.txt");

fn read_param(program: &Vec<i32>, param: usize, mode: i32) -> i32 {
    match mode {
        0 => program[param],
        1 => param as i32,
        _ => panic!("unknown mode")
    }
}

// Opcode 1 adds together numbers read from two positions and stores the result in a third position.
fn add(mut program: Vec<i32>, mut program_counter: usize, registers: (i32, i32), mode: (i32, i32, i32)) -> (Vec<i32>, usize, (i32, i32)) {
    let param_0 = program[program_counter + 1] as usize;
    let param_1 = program[program_counter + 2] as usize;
    let param_2 = program[program_counter + 3] as usize;

    program[param_2] = read_param(&program, param_0, mode.0) + read_param(&program, param_1, mode.1);
    program_counter += 4;

    return (program, program_counter, registers);
}

// Opcode 2 works exactly like opcode 1, except it multiplies the two inputs instead of adding them.
fn mul(mut program: Vec<i32>, mut program_counter: usize, registers: (i32, i32), mode: (i32, i32, i32)) -> (Vec<i32>, usize, (i32, i32)) {
    let param_0 = program[program_counter + 1] as usize;
    let param_1 = program[program_counter + 2] as usize;
    let param_2 = program[program_counter + 3] as usize;

    program[param_2] = read_param(&program, param_0, mode.0) * read_param(&program, param_1, mode.1);
    program_counter += 4;

    return (program, program_counter, registers);
}

// Opcode 3 takes a single integer as input and saves it to the position given by its only parameter. 
// For example, the instruction 3,50 would take an input value and store it at address 50.
fn load_in(mut program: Vec<i32>, mut program_counter: usize, registers: (i32, i32), _mode: (i32, i32, i32)) -> (Vec<i32>, usize, (i32, i32)) {
    let param_0 = program[program_counter + 1] as usize;
    program[param_0] = registers.0;
    program_counter += 2;

    return (program, program_counter, registers);
}

// Opcode 4 outputs the value of its only parameter.
// For example, the instruction 4,50 would output the value at address 50.
fn load_out(program: Vec<i32>, mut program_counter: usize, mut registers: (i32, i32), mode: (i32, i32, i32)) -> (Vec<i32>, usize, (i32, i32)) {
    let param_0 = program[program_counter + 1] as usize;
    registers.1 = read_param(&program, param_0, mode.0);
    program_counter += 2;

    return (program, program_counter, registers);
}

// Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
fn jump_if_true(program: Vec<i32>, mut program_counter: usize, registers: (i32, i32), mode: (i32, i32, i32)) -> (Vec<i32>, usize, (i32, i32)) {
    let param_0 = program[program_counter + 1] as usize;
    let param_1 = program[program_counter + 2] as usize;

    if read_param(&program, param_0, mode.0) != 0 {
        program_counter = read_param(&program, param_1, mode.1) as usize;
    } else {
        program_counter += 3;
    }

    return (program, program_counter, registers);
}

// Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
fn jump_if_false(program: Vec<i32>, mut program_counter: usize, registers: (i32, i32), mode: (i32, i32, i32)) -> (Vec<i32>, usize, (i32, i32)) {
    let param_0 = program[program_counter + 1] as usize;
    let param_1 = program[program_counter + 2] as usize;

    if read_param(&program, param_0, mode.0) == 0 {
        program_counter = read_param(&program, param_1, mode.1) as usize;
    } else {
        program_counter += 3;
    }

    return (program, program_counter, registers);
}

// Opcode 7 is less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
fn less_than(mut program: Vec<i32>, mut program_counter: usize, registers: (i32, i32), mode: (i32, i32, i32)) -> (Vec<i32>, usize, (i32, i32)) {
    let param_0 = program[program_counter + 1] as usize;
    let param_1 = program[program_counter + 2] as usize;
    let param_2 = program[program_counter + 3] as usize;

    if read_param(&program, param_0, mode.0) < read_param(&program, param_1, mode.1) {
        program[param_2] = 1;
    } else {
        program[param_2] = 0;
    }

    program_counter += 4;

    return (program, program_counter, registers);
}

// Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
fn equals(mut program: Vec<i32>, mut program_counter: usize, registers: (i32, i32), mode: (i32, i32, i32)) -> (Vec<i32>, usize, (i32, i32)) {
    let param_0 = program[program_counter + 1] as usize;
    let param_1 = program[program_counter + 2] as usize;
    let param_2 = program[program_counter + 3] as usize;

    if read_param(&program, param_0, mode.0) == read_param(&program, param_1, mode.1) {
        program[param_2] = 1;
    } else {
        program[param_2] = 0;
    }

    program_counter += 4;

    return (program, program_counter, registers);
}

fn run_program(mut program: Vec<i32>, input: i32) -> (Vec<i32>, (i32, i32)) {
    let mut program_counter = 0;
    let mut registers = (input, 0);
    
    println!("[DEBUG]: input: {}", input);
    println!("[DEBUG]: program: {:?}", program);

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
            5 => jump_if_true(program, program_counter, registers, mode),
            6 => jump_if_false(program, program_counter, registers, mode),
            7 => less_than(program, program_counter, registers, mode),
            8 => equals(program, program_counter, registers, mode),
            99 => break,
            _ => {
                println!("[ERROR]: pc: {}, opcode: {}, program: {:?}", program_counter, program[program_counter], program);
                panic!("unknown opcode");
            }
        };
        program = new_program;
        program_counter = new_program_counter;
        registers = new_registers;

        println!("[DEBUG]: program: {:?}", program);
    }

    println!("[HALT]: registers: {:?}, pc: {}, opcode: {}, program: {:?}",
        registers, program_counter, program[program_counter], program);

    println!("[DEBUG]: output: {}", registers.1);

    return (program, registers);
}

fn part_1(main_program: &Vec<i32>) -> i32 {
    let program = main_program.clone();

    let (_halt_state, registers) = run_program(program, 1);

    return registers.1;
}

fn part_2(main_program: &Vec<i32>) -> i32 {
    let program = main_program.clone();

    let (_halt_state, registers) = run_program(program, 5);

    return registers.1;
}

fn main() -> () {
    let program: Vec<i32> = INPUT_FILE.split(',').map(|register| register.parse::<i32>()    .expect("Parse fail")).collect();
    let part1_program_result = part_1(&program);
    let part2_program_result = part_2(&program);

    println!("Part 1: {}", part1_program_result);
    println!("Part 2: {}", part2_program_result);
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

    #[test]
    fn it_solves_part2_compare_examples() {
        // Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        let (_program, output) = run_program(vec![3,9,8,9,10,9,4,9,99,-1,8], 8);
        assert_eq!(output.1, 1);
        let (_program, output) = run_program(vec![3,9,8,9,10,9,4,9,99,-1,8], 9);
        assert_eq!(output.1, 0);

        // Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        let (_program, output) = run_program(vec![3,9,7,9,10,9,4,9,99,-1,8], 7);
        assert_eq!(output.1, 1);
        let (_program, output) = run_program(vec![3,9,8,9,10,9,4,9,99,-1,8], 9);
        assert_eq!(output.1, 0);

        // Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        let (_program, output) = run_program(vec![3,3,1108,-1,8,3,4,3,99], 8);
        assert_eq!(output.1, 1);
        let (_program, output) = run_program(vec![3,3,1108,-1,8,3,4,3,99], 9);
        assert_eq!(output.1, 0);

        // Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not)
        let (_program, output) = run_program(vec![3,3,1107,-1,8,3,4,3,99], 7);
        assert_eq!(output.1, 1);
        let (_program, output) = run_program(vec![3,3,1107,-1,8,3,4,3,99], 9);
        assert_eq!(output.1, 0);
    }

    #[test]
    fn it_solves_part2_jump_tests() {
        // Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:

        // using position mode
        let (_program, output) = run_program(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 0);
        assert_eq!(output.1, 0);
        let (_program, output) = run_program(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 1);
        assert_eq!(output.1, 1);

        // // using immediate mode
        let (_program, output) = run_program(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], 0);
        assert_eq!(output.1, 0);
        let (_program, output) = run_program(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], 123);
        assert_eq!(output.1, 1);
    }

    #[test]
    fn it_solves_part2_large_example() {
        // The program uses an input instruction to ask for a single number.
        // The program will then output 999 if the input value is below 8
        let (_program, output) = run_program(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 7);
        assert_eq!(output.1, 999);

        // output 1000 if the input value is equal to 8
        let (_program, output) = run_program(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 8);
        assert_eq!(output.1, 1000);

        // or output 1001 if the input value is greater than 8.
        let (_program, output) = run_program(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 9);
        assert_eq!(output.1, 1001);
    }
}
