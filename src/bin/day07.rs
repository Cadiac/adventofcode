extern crate permutohedron;
use permutohedron::Heap;
use std::collections::VecDeque;

const INPUT_FILE: &str = include_str!("../../inputs/day07.txt");

fn read_param(program: &Vec<i32>, param: usize, mode: i32) -> i32 {
    match mode {
        0 => program[param],
        1 => param as i32,
        _ => panic!("unknown mode")
    }
}

// Opcode 1 adds together numbers read from two positions and stores the result in a third position.
fn add(program: &mut Vec<i32>, mut program_counter: usize, input_buffer: VecDeque<i32>, output: i32, mode: (i32, i32, i32))
    -> Result<(usize, i32, VecDeque<i32>), &'static str> {
    let param_0 = program[program_counter + 1] as usize;
    let param_1 = program[program_counter + 2] as usize;
    let param_2 = program[program_counter + 3] as usize;

    program[param_2] = read_param(&program, param_0, mode.0) + read_param(&program, param_1, mode.1);
    program_counter += 4;

    return Ok((program_counter, output, input_buffer));
}

// Opcode 2 works exactly like opcode 1, except it multiplies the two inputs instead of adding them.
fn mul(program: &mut Vec<i32>, mut program_counter: usize, input_buffer: VecDeque<i32>, output: i32, mode: (i32, i32, i32))
    -> Result<(usize, i32, VecDeque<i32>), &'static str> {
    let param_0 = program[program_counter + 1] as usize;
    let param_1 = program[program_counter + 2] as usize;
    let param_2 = program[program_counter + 3] as usize;

    program[param_2] = read_param(&program, param_0, mode.0) * read_param(&program, param_1, mode.1);
    program_counter += 4;

    return Ok((program_counter, output, input_buffer));
}

// Opcode 3 takes a single integer as input and saves it to the position given by its only parameter. 
// For example, the instruction 3,50 would take an input value and store it at address 50.
fn load_in(program: &mut Vec<i32>, mut program_counter: usize, mut input_buffer: VecDeque<i32>, output: i32, _mode: (i32, i32, i32))
    -> Result<(usize, i32, VecDeque<i32>), &'static str> {
    let param_0 = program[program_counter + 1] as usize;
    if input_buffer.len() == 0 {
        return Err("input required");
    }
    program[param_0] = input_buffer.pop_front().expect("not enough input values in buffer");
    program_counter += 2;

    return Ok((program_counter, output, input_buffer));
}

// Opcode 4 outputs the value of its only parameter.
// For example, the instruction 4,50 would output the value at address 50.
fn load_out(program: &mut Vec<i32>, mut program_counter: usize, input_buffer: VecDeque<i32>, _output: i32, mode: (i32, i32, i32))
    -> Result<(usize, i32, VecDeque<i32>), &'static str> {
    let param_0 = program[program_counter + 1] as usize;
    let output = read_param(&program, param_0, mode.0);
    program_counter += 2;

    return Ok((program_counter, output, input_buffer));
}

// Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
fn jump_if_true(program: &mut Vec<i32>, mut program_counter: usize, input_buffer: VecDeque<i32>, output: i32, mode: (i32, i32, i32))
    -> Result<(usize, i32, VecDeque<i32>), &'static str> {
    let param_0 = program[program_counter + 1] as usize;
    let param_1 = program[program_counter + 2] as usize;

    if read_param(&program, param_0, mode.0) != 0 {
        program_counter = read_param(&program, param_1, mode.1) as usize;
    } else {
        program_counter += 3;
    }

    return Ok((program_counter, output, input_buffer));
}

// Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
fn jump_if_false(program: &mut Vec<i32>, mut program_counter: usize, input_buffer: VecDeque<i32>, output: i32, mode: (i32, i32, i32))
    -> Result<(usize, i32, VecDeque<i32>), &'static str> {
    let param_0 = program[program_counter + 1] as usize;
    let param_1 = program[program_counter + 2] as usize;

    if read_param(&program, param_0, mode.0) == 0 {
        program_counter = read_param(&program, param_1, mode.1) as usize;
    } else {
        program_counter += 3;
    }

    return Ok((program_counter, output, input_buffer));
}

// Opcode 7 is less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
fn cmp_less_than(program: &mut Vec<i32>, mut program_counter: usize, input_buffer: VecDeque<i32>, output: i32, mode: (i32, i32, i32))
    -> Result<(usize, i32, VecDeque<i32>), &'static str> {
    let param_0 = program[program_counter + 1] as usize;
    let param_1 = program[program_counter + 2] as usize;
    let param_2 = program[program_counter + 3] as usize;

    if read_param(&program, param_0, mode.0) < read_param(&program, param_1, mode.1) {
        program[param_2] = 1;
    } else {
        program[param_2] = 0;
    }

    program_counter += 4;

    return Ok((program_counter, output, input_buffer));
}

// Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
fn cmp_equals(program: &mut Vec<i32>, mut program_counter: usize, input_buffer: VecDeque<i32>, output: i32, mode: (i32, i32, i32))
    -> Result<(usize, i32, VecDeque<i32>), &'static str> {
    let param_0 = program[program_counter + 1] as usize;
    let param_1 = program[program_counter + 2] as usize;
    let param_2 = program[program_counter + 3] as usize;

    if read_param(&program, param_0, mode.0) == read_param(&program, param_1, mode.1) {
        program[param_2] = 1;
    } else {
        program[param_2] = 0;
    }

    program_counter += 4;

    return Ok((program_counter, output, input_buffer));
}

fn run_program(mut program: Vec<i32>, mut input_buffer: VecDeque<i32>, program_counter_start: usize) -> (Vec<i32>, i32, usize, VecDeque<i32>, bool) {
    // println!("[DEBUG]: input: {:?}", input_buffer);
    // println!("[DEBUG]: program: {:?}", program);

    let mut output = 0;
    let mut program_counter = program_counter_start;

    loop {
        let instruction = program[program_counter];
        let opcode = instruction % 100;
        let a_mode = (instruction % 1000) / 100;
        let b_mode = (instruction % 10000) / 1000;
        let c_mode = (instruction % 100000) / 10000;
    
        let mode = (a_mode, b_mode, c_mode);

        // println!("[DEBUG]: pc: {}, opcode: {}, mode: {:?}", program_counter, opcode, mode);
        
        let state = match opcode {
            1 => add(&mut program, program_counter, input_buffer, output, mode),
            2 => mul(&mut program, program_counter, input_buffer, output, mode),
            3 => load_in(&mut program, program_counter, input_buffer, output, mode),
            4 => load_out(&mut program, program_counter, input_buffer, output, mode),
            5 => jump_if_true(&mut program, program_counter, input_buffer, output, mode),
            6 => jump_if_false(&mut program, program_counter, input_buffer, output, mode),
            7 => cmp_less_than(&mut program, program_counter, input_buffer, output, mode),
            8 => cmp_equals(&mut program, program_counter, input_buffer, output, mode),
            99 => break,
            _ => {
                println!("[ERROR]: pc: {}, opcode: {}, program: {:?}", program_counter, program[program_counter], program);
                panic!("unknown opcode");
            }
        };

        match state {
            Ok((new_program_counter, new_output, new_input_buffer)) => {
                program_counter = new_program_counter;
                output = new_output;
                input_buffer = new_input_buffer;
            },
            Err(_err) => {
                return (program, output, program_counter, VecDeque::new(), false)
            }
        }
    }

    // println!("[HALT]: output: {:?}, pc: {}, opcode: {}, program: {:?}",
    //     output, program_counter, program[program_counter], program);

    return (program, output, program_counter, input_buffer, true);
}

fn part_1(main_program: &Vec<i32>) -> (i32, Vec<i32>) {
    let mut phase_settings = vec![0, 1, 2, 3, 4];
    let heap = Heap::new(&mut phase_settings);

    let mut permutations = Vec::new();
    for data in heap {
        permutations.push(data.clone());
    }

    let mut max_output_signal = 0;
    let mut max_phase_settings = Vec::new();

    for permutation in permutations {
        let (_halt_state, output, _pc, _input_buffer, _halt) = run_program(main_program.clone(), VecDeque::from(vec![permutation[0], 0]), 0);
        let (_halt_state, output, _pc, _input_buffer, _halt) = run_program(main_program.clone(), VecDeque::from(vec![permutation[1], output]), 0);
        let (_halt_state, output, _pc, _input_buffer, _halt) = run_program(main_program.clone(), VecDeque::from(vec![permutation[2], output]), 0);
        let (_halt_state, output, _pc, _input_buffer, _halt) = run_program(main_program.clone(), VecDeque::from(vec![permutation[3], output]), 0);
        let (_halt_state, output, _pc, _input_buffer, _halt) = run_program(main_program.clone(), VecDeque::from(vec![permutation[4], output]), 0);
    
        if output > max_output_signal {
            max_output_signal = output;
            max_phase_settings = permutation;
        }
    }

    return (max_output_signal, max_phase_settings);
}

fn part_2(main_program: &Vec<i32>) -> (i32, Vec<i32>) {
    let mut phase_settings = vec![5, 6, 7, 8, 9];
    let heap = Heap::new(&mut phase_settings);

    let mut permutations = Vec::new();
    for data in heap {
        permutations.push(data.clone());
    }

    let mut max_output_signal = 0;
    let mut max_phase_settings = Vec::new();

    for permutation in permutations {
        // println!("[DEBUG]: permutation: {:?}", permutation);

        let mut amp_index = 0;

        // Initialize memory and inputs
        let mut amps = vec![
            (main_program.clone(), VecDeque::from(vec![permutation[0], 0]), 0),
            (main_program.clone(), VecDeque::from(vec![permutation[1]]), 0),
            (main_program.clone(), VecDeque::from(vec![permutation[2]]), 0),
            (main_program.clone(), VecDeque::from(vec![permutation[3]]), 0),
            (main_program.clone(), VecDeque::from(vec![permutation[4]]), 0)
        ];

        loop {
            let state = amps[amp_index].clone();

            // println!("[AMP{}]: input: {:?}, pc: {}", amp_index, state.1, state.2);

            let (suspend_state, output, pc, input, halt) = run_program(state.0, state.1, state.2);

            // println!("[AMP{}]: halt: {}, output: {}", amp_index, halt, output);

            amps[amp_index] = (suspend_state, input, pc);
            amps[(amp_index + 1) % 5].1.push_back(output);

            if amp_index == 4 && halt {
                // println!("[HALT]: output: {}, max_output_signal: {}, permutation: {:?}", output, max_output_signal, permutation);

                if output > max_output_signal {
                    max_output_signal = output;
                    max_phase_settings = permutation;    
                }

                break;
            }

            amp_index = (amp_index + 1) % 5;
        }
    }
    
    return (max_output_signal, max_phase_settings);
}

fn main() -> () {
    let program: Vec<i32> = INPUT_FILE.split(',').map(|register| register.parse::<i32>().expect("Parse fail")).collect();

    let (part1_max_output, phase_settings) = part_1(&program);
    println!("Part 1: {}, settings: {:?}", part1_max_output, phase_settings);

    let (part2_max_output, phase_settings) = part_2(&program);
    println!("Part 2: {}, settings: {:?}", part2_max_output, phase_settings);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_runs_part1_example_programs() {
        let (program, _, _, _, _) = run_program(vec![1,9,10,3,2,3,11,0,99,30,40,50], VecDeque::new(), 0);
        assert_eq!(program, (vec![3500,9,10,70,2,3,11,0,99,30,40,50]));
        
        let (program, _, _, _, _) = run_program(vec![1,0,0,0,99], VecDeque::new(), 0);
        assert_eq!(program, vec![2,0,0,0,99]);

        let (program, _, _, _, _) = run_program(vec![2,3,0,3,99], VecDeque::new(), 0);
        assert_eq!(program, vec![2,3,0,6,99]);

        let (program, _, _, _, _) = run_program(vec![2,4,4,5,99,0], VecDeque::new(), 0);
        assert_eq!(program, vec![2,4,4,5,99,9801]);

        let (program, _, _, _, _) = run_program(vec![1,1,1,4,99,5,6,0,99], VecDeque::new(), 0);
        assert_eq!(program, vec![30,1,1,4,2,5,6,0,99]);
    }

    #[test]
    fn it_solves_input_output_example() {
        let (program, output, _, _, _) = run_program(vec![3,0,4,0,99], VecDeque::from(vec![123]), 0);
        assert_eq!(program, vec![123,0,4,0,99]);
        assert_eq!(output, 123);
    }

    #[test]
    fn it_solves_parameter_mode_examples() {
        let (program, _, _, _, _) = run_program(vec![1002,4,3,4,33], VecDeque::new(), 0);
        assert_eq!(program, vec![1002,4,3,4,99]);
    }

    #[test]
    fn it_solves_negative_examples() {
        let (program, _, _, _, _) = run_program(vec![1101,100,-1,4,0], VecDeque::new(), 0);
        assert_eq!(program, vec![1101,100,-1,4,99]);
    }

    #[test]
    fn it_solves_day5_part2_compare_examples() {
        // Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        let (_, output, _, input_remaining, _) = run_program(vec![3,9,8,9,10,9,4,9,99,-1,8], VecDeque::from(vec![8]), 0);
        assert_eq!(output, 1);
        assert!(input_remaining.is_empty());

        let (_, output, _, input_remaining, _) = run_program(vec![3,9,8,9,10,9,4,9,99,-1,8], VecDeque::from(vec![9]), 0);
        assert_eq!(output, 0);
        assert!(input_remaining.is_empty());

        // Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        let (_, output, _, input_remaining, _)= run_program(vec![3,9,7,9,10,9,4,9,99,-1,8], VecDeque::from(vec![7]), 0);
        assert_eq!(output, 1);
        assert!(input_remaining.is_empty());

        let (_, output, _, input_remaining, _) = run_program(vec![3,9,8,9,10,9,4,9,99,-1,8], VecDeque::from(vec![9]), 0);
        assert_eq!(output, 0);
        assert!(input_remaining.is_empty());

        // Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        let (_, output, _, input_remaining, _) = run_program(vec![3,3,1108,-1,8,3,4,3,99], VecDeque::from(vec![8]), 0);
        assert_eq!(output, 1);
        assert!(input_remaining.is_empty());

        let (_, output, _, input_remaining, _) = run_program(vec![3,3,1108,-1,8,3,4,3,99], VecDeque::from(vec![9]), 0);
        assert_eq!(output, 0);
        assert!(input_remaining.is_empty());

        // Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not)
        let (_, output, _, input_remaining, _) = run_program(vec![3,3,1107,-1,8,3,4,3,99], VecDeque::from(vec![7]), 0);
        assert_eq!(output, 1);
        assert!(input_remaining.is_empty());

        let (_, output, _, input_remaining, _) = run_program(vec![3,3,1107,-1,8,3,4,3,99], VecDeque::from(vec![9]), 0);
        assert_eq!(output, 0);
        assert!(input_remaining.is_empty());
    }

    #[test]
    fn it_solves_day5_part2_jump_tests() {
        // Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:

        // using position mode
        let (_, output, _, _, _) = run_program(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], VecDeque::from(vec![0]), 0);
        assert_eq!(output, 0);
        let (_, output, _, _, _) = run_program(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], VecDeque::from(vec![1]), 0);
        assert_eq!(output, 1);

        // // using immediate mode
        let (_, output, _, _, _) = run_program(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], VecDeque::from(vec![0]), 0);
        assert_eq!(output, 0);
        let (_, output, _, _, _) = run_program(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], VecDeque::from(vec![123]), 0);
        assert_eq!(output, 1);
    }

    #[test]
    fn it_solves_day5_part2_large_example() {
        // The program uses an input instruction to ask for a single number.
        // The program will then output 999 if the input value is below 8
        let (_, output, _, _, _) = run_program(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], VecDeque::from(vec![7]), 0);
        assert_eq!(output, 999);

        // output 1000 if the input value is equal to 8
        let (_, output, _, _, _) = run_program(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], VecDeque::from(vec![8]), 0);
        assert_eq!(output, 1000);

        // or output 1001 if the input value is greater than 8.
        let (_, output, _, _, _) = run_program(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], VecDeque::from(vec![9]), 0);
        assert_eq!(output, 1001);
    }

    #[test]
    fn it_solves_max_thruster_signal_examples() {
        // assert_eq!(part_1(&vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]), (43210, vec![4,3,2,1,0]));
        // assert_eq!(part_1(&vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0]), (54321, vec![0,1,2,3,4]));
        // assert_eq!(part_1(&vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0]), (65210, vec![1,0,4,3,2]));
    }

    #[test]
    fn it_handles_input_buffers() {
        let input = VecDeque::from(vec![123, 124, 125]);
        let (program, output, _, input_buffer, _) = run_program(vec![3,0,4,0,99], input, 0);
        assert_eq!(program, vec![123,0,4,0,99]);
        assert_eq!(output, 123);
        assert!(input_buffer.len() == 2);
    }

    #[test]
    fn it_suspends_if_input_required() {
        let (program, _, pc, _, halt) = run_program(vec![3,0,4,0,99], VecDeque::from(vec![]), 0);
        assert_eq!(program, vec![3,0,4,0,99]);
        assert_eq!(halt, false);
        assert_eq!(pc, 0);
    }

    #[test]
    fn it_returns_pc_correctly() {
        let (_, _, pc, _, halt) = run_program(vec![3,0,4,0,99], VecDeque::from(vec![123]), 0);
        assert_eq!(pc, 4);
        assert_eq!(halt, true);
    }

    #[test]
    fn it_can_start_from_pc_offset() {
        let (_, _, pc, _, halt) = run_program(vec![99,99,99,3,0,4,0,99], VecDeque::from(vec![123]), 3);
        assert_eq!(pc, 7);
        assert_eq!(halt, true);
    }

    #[test]
    fn it_solves_loop_max_thruster_signal_examples() {
        assert_eq!(part_2(&vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5]), (139629729, vec![9,8,7,6,5]));
        assert_eq!(part_2(&vec![
            3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,
            53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10]), (18216, vec![9,7,8,5,6]));
    }
}
