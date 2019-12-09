use std::collections::VecDeque;

const INPUT_FILE: &str = include_str!("../input.txt");
const MEMORY_SIZE: usize = 65536;

#[derive(Debug)]
struct ProgramState {
    mem: Vec<i64>,
    program_counter: usize,
    input_buffer: VecDeque<i64>,
    output_buffer: VecDeque<i64>,
    mode: (i64, i64, i64),
    relative_base: i64,
    halt: bool
}

fn read_param(state: &ProgramState, param: i64, mode: i64) -> i64 {
    match mode {
        0 => state.mem[param as usize],
        1 => param as i64,
        2 => state.mem[(state.relative_base + param) as usize],
        _ => panic!("unknown mode")
    }
}

// Opcode 1 adds together numbers read from two positions and stores the result in a third position.
fn add(state: &mut ProgramState)
    -> Result<(&mut ProgramState), &'static str> {
    let param_0 = state.mem[state.program_counter + 1];
    let param_1 = state.mem[state.program_counter + 2];
    let param_2 = state.mem[state.program_counter + 3] as usize;

    state.mem[param_2] = read_param(state, param_0, state.mode.0) + read_param(state, param_1, state.mode.1);
    state.program_counter += 4;

    return Ok(state);
}

// Opcode 2 works exactly like opcode 1, except it multiplies the two inputs instead of adding them.
fn mul(state: &mut ProgramState)
    -> Result<(&mut ProgramState), &'static str> {
    let param_0 = state.mem[state.program_counter + 1];
    let param_1 = state.mem[state.program_counter + 2];
    let param_2 = state.mem[state.program_counter + 3];

    state.mem[param_2] = read_param(state, param_0, state.mode.0) * read_param(state, param_1, state.mode.1);
    state.program_counter += 4;

    return Ok(state);
}

// Opcode 3 takes a single integer as input and saves it to the position given by its only parameter. 
// For example, the instruction 3,50 would take an input value and store it at address 50.
fn load_in(state: &mut ProgramState)
    -> Result<(&mut ProgramState), &'static str> {
    if state.input_buffer.len() == 0 {
        return Err("input required");
    }

    let param_0 = state.mem[state.program_counter + 1];
    let mem_addr = read_param(state, param_0, state.mode.0) as usize;

    state.mem[mem_addr] = state.input_buffer.pop_front().expect("not enough input values in buffer");
    state.program_counter += 2;

    return Ok(state);
}

// Opcode 4 outputs the value of its only parameter.
// For example, the instruction 4,50 would output the value at address 50.
fn load_out(state: &mut ProgramState)
    -> Result<(&mut ProgramState), &'static str> {
    let param_0 = state.mem[state.program_counter + 1];
    state.output_buffer.push_back(read_param(state, param_0, state.mode.0));
    println!("Set output to {:?}", state.output_buffer);
    state.program_counter += 2;

    return Ok(state);
}

// Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
fn jump_if_true(state: &mut ProgramState)
    -> Result<(&mut ProgramState), &'static str> {
    let param_0 = state.mem[state.program_counter + 1];
    let param_1 = state.mem[state.program_counter + 2];

    if read_param(state, param_0, state.mode.0) != 0 {
        state.program_counter = read_param(state, param_1, state.mode.1) as usize;
    } else {
        state.program_counter += 3;
    }

    return Ok(state);
}

// Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
fn jump_if_false(state: &mut ProgramState)
    -> Result<(&mut ProgramState), &'static str> {
    let param_0 = state.mem[state.program_counter + 1];
    let param_1 = state.mem[state.program_counter + 2];

    if read_param(state, param_0, state.mode.0) == 0 {
        state.program_counter = read_param(state, param_1, state.mode.1) as usize;
    } else {
        state.program_counter += 3;
    }

    return Ok(state);
}

// Opcode 7 is less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
fn cmp_less_than(state: &mut ProgramState)
    -> Result<(&mut ProgramState), &'static str> {
    let param_0 = state.mem[state.program_counter + 1];
    let param_1 = state.mem[state.program_counter + 2];
    let param_2 = state.mem[state.program_counter + 3] as usize;

    if read_param(state, param_0, state.mode.0) < read_param(state, param_1, state.mode.1) {
        state.mem[param_2] = 1;
    } else {
        state.mem[param_2] = 0;
    }

    state.program_counter += 4;

    return Ok(state);
}

// Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
fn cmp_equals(state: &mut ProgramState)
    -> Result<(&mut ProgramState), &'static str> {
    let param_0 = state.mem[state.program_counter + 1];
    let param_1 = state.mem[state.program_counter + 2];
    let param_2 = state.mem[state.program_counter + 3] as usize;

    if read_param(state, param_0, state.mode.0) == read_param(state, param_1, state.mode.1) {
        state.mem[param_2] = 1;
    } else {
        state.mem[param_2] = 0;
    }

    state.program_counter += 4;

    return Ok(state);
}

// Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
fn adj_rel_base(state: &mut ProgramState)
    -> Result<(&mut ProgramState), &'static str> {
    let param_0 = state.mem[state.program_counter + 1];

    state.relative_base += read_param(state, param_0, state.mode.0);
    state.program_counter += 2;

    return Ok(state);
}


fn run_program(state: &mut ProgramState) -> &mut ProgramState {
    loop {
        let instruction = state.mem[state.program_counter];
        let opcode = instruction % 100;
        let a_mode = (instruction % 1000) / 100;
        let b_mode = (instruction % 10000) / 1000;
        let c_mode = (instruction % 100000) / 10000;
    
        state.mode = (a_mode, b_mode, c_mode);

        // println!("[DEBUG]: state: {:?}", state);
        
        // TODO: Refactor to return enums
        let return_flag = match opcode {
            1 => add(state),
            2 => mul(state),
            3 => load_in(state),
            4 => load_out(state),
            5 => jump_if_true(state),
            6 => jump_if_false(state),
            7 => cmp_less_than(state),
            8 => cmp_equals(state),
            9 => adj_rel_base(state),
            99 => break,
            _ => {
                println!("[ERROR]: state: {:?}", state);
                panic!("unknown opcode");
            }
        };

        match return_flag {
            Ok(_) => {
                // Still running
            },
            Err(_err) => {
                state.input_buffer = VecDeque::new();
                return state;
            }
        }
    }

    state.halt = true;

    println!("[HALT]: state: {:?}", state);

    return state;
}

fn part_1(program: Vec<i64>) -> ProgramState {
    let mut memory = program.clone();
    memory.resize(MEMORY_SIZE, 0);

    let mut state = ProgramState{
        mem: memory,
        program_counter: 0,
        input_buffer: VecDeque::from(vec![1]),
        mode: (0, 0, 0),
        output_buffer: VecDeque::new(),
        relative_base: 0,
        halt: false
    };

    run_program(&mut state);

    return state;
}

fn main() -> () {
    let program: Vec<i64> = INPUT_FILE.split(',').map(|register| register.parse::<i64>().expect("Parse fail")).collect();

    let halt_state_part1 = part_1(program);
    println!("Part 1: {:?}", halt_state_part1.output_buffer);

    // let (part2_max_output, phase_settings) = part_2(&mem);
    // println!("Part 2: {}, settings: {:?}", part2_max_output, phase_settings);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_runs_part1_example_programs() {
        let mut state = ProgramState{
            mem: vec![1,9,10,3,2,3,11,0,99,30,40,50],
            program_counter: 0,
            input_buffer: VecDeque::new(),
            mode: (0, 0, 0),
            output_buffer: VecDeque::new(),
            relative_base: 0,
            halt: false
        };

        let halt_state = run_program(&mut state);
        assert_eq!(halt_state.mem, (vec![3500,9,10,70,2,3,11,0,99,30,40,50]));
        
        // let (mem, _, _, _, _) = run_program(vec![1,0,0,0,99], VecDeque::new(), 0);
        // assert_eq!(mem, vec![2,0,0,0,99]);

        // let (mem, _, _, _, _) = run_program(vec![2,3,0,3,99], VecDeque::new(), 0);
        // assert_eq!(mem, vec![2,3,0,6,99]);

        // let (mem, _, _, _, _) = run_program(vec![2,4,4,5,99,0], VecDeque::new(), 0);
        // assert_eq!(mem, vec![2,4,4,5,99,9801]);

        // let (mem, _, _, _, _) = run_program(vec![1,1,1,4,99,5,6,0,99], VecDeque::new(), 0);
        // assert_eq!(mem, vec![30,1,1,4,2,5,6,0,99]);
    }

    // #[test]
    // fn it_solves_input_output_example() {
    //     let (mem, output, _, _, _) = run_program(vec![3,0,4,0,99], VecDeque::from(vec![123]), 0);
    //     assert_eq!(mem, vec![123,0,4,0,99]);
    //     assert_eq!(output, 123);
    // }

    // #[test]
    // fn it_solves_parameter_mode_examples() {
    //     let (mem, _, _, _, _) = run_program(vec![1002,4,3,4,33], VecDeque::new(), 0);
    //     assert_eq!(mem, vec![1002,4,3,4,99]);
    // }

    // #[test]
    // fn it_solves_negative_examples() {
    //     let (mem, _, _, _, _) = run_program(vec![1101,100,-1,4,0], VecDeque::new(), 0);
    //     assert_eq!(mem, vec![1101,100,-1,4,99]);
    // }

    // #[test]
    // fn it_solves_day5_part2_compare_examples() {
    //     // Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
    //     let (_, output, _, input_remaining, _) = run_program(vec![3,9,8,9,10,9,4,9,99,-1,8], VecDeque::from(vec![8]), 0);
    //     assert_eq!(output, 1);
    //     assert!(input_remaining.is_empty());

    //     let (_, output, _, input_remaining, _) = run_program(vec![3,9,8,9,10,9,4,9,99,-1,8], VecDeque::from(vec![9]), 0);
    //     assert_eq!(output, 0);
    //     assert!(input_remaining.is_empty());

    //     // Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
    //     let (_, output, _, input_remaining, _)= run_program(vec![3,9,7,9,10,9,4,9,99,-1,8], VecDeque::from(vec![7]), 0);
    //     assert_eq!(output, 1);
    //     assert!(input_remaining.is_empty());

    //     let (_, output, _, input_remaining, _) = run_program(vec![3,9,8,9,10,9,4,9,99,-1,8], VecDeque::from(vec![9]), 0);
    //     assert_eq!(output, 0);
    //     assert!(input_remaining.is_empty());

    //     // Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
    //     let (_, output, _, input_remaining, _) = run_program(vec![3,3,1108,-1,8,3,4,3,99], VecDeque::from(vec![8]), 0);
    //     assert_eq!(output, 1);
    //     assert!(input_remaining.is_empty());

    //     let (_, output, _, input_remaining, _) = run_program(vec![3,3,1108,-1,8,3,4,3,99], VecDeque::from(vec![9]), 0);
    //     assert_eq!(output, 0);
    //     assert!(input_remaining.is_empty());

    //     // Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not)
    //     let (_, output, _, input_remaining, _) = run_program(vec![3,3,1107,-1,8,3,4,3,99], VecDeque::from(vec![7]), 0);
    //     assert_eq!(output, 1);
    //     assert!(input_remaining.is_empty());

    //     let (_, output, _, input_remaining, _) = run_program(vec![3,3,1107,-1,8,3,4,3,99], VecDeque::from(vec![9]), 0);
    //     assert_eq!(output, 0);
    //     assert!(input_remaining.is_empty());
    // }

    // #[test]
    // fn it_solves_day5_part2_jump_tests() {
    //     // Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:

    //     // using position mode
    //     let (_, output, _, _, _) = run_program(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], VecDeque::from(vec![0]), 0);
    //     assert_eq!(output, 0);
    //     let (_, output, _, _, _) = run_program(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], VecDeque::from(vec![1]), 0);
    //     assert_eq!(output, 1);

    //     // // using immediate mode
    //     let (_, output, _, _, _) = run_program(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], VecDeque::from(vec![0]), 0);
    //     assert_eq!(output, 0);
    //     let (_, output, _, _, _) = run_program(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], VecDeque::from(vec![123]), 0);
    //     assert_eq!(output, 1);
    // }

    // #[test]
    // fn it_solves_day5_part2_large_example() {
    //     // The mem uses an input instruction to ask for a single number.
    //     // The mem will then output 999 if the input value is below 8
    //     let (_, output, _, _, _) = run_program(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
    //         1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
    //         999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], VecDeque::from(vec![7]), 0);
    //     assert_eq!(output, 999);

    //     // output 1000 if the input value is equal to 8
    //     let (_, output, _, _, _) = run_program(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
    //         1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
    //         999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], VecDeque::from(vec![8]), 0);
    //     assert_eq!(output, 1000);

    //     // or output 1001 if the input value is greater than 8.
    //     let (_, output, _, _, _) = run_program(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
    //         1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
    //         999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], VecDeque::from(vec![9]), 0);
    //     assert_eq!(output, 1001);
    // }

    // #[test]
    // fn it_solves_max_thruster_signal_examples() {
    //     // assert_eq!(part_1(&vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]), (43210, vec![4,3,2,1,0]));
    //     // assert_eq!(part_1(&vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0]), (54321, vec![0,1,2,3,4]));
    //     // assert_eq!(part_1(&vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0]), (65210, vec![1,0,4,3,2]));
    // }

    // #[test]
    // fn it_handles_input_buffers() {
    //     let input = VecDeque::from(vec![123, 124, 125]);
    //     let (mem, output, _, input_buffer, _) = run_program(vec![3,0,4,0,99], input, 0);
    //     assert_eq!(mem, vec![123,0,4,0,99]);
    //     assert_eq!(output, 123);
    //     assert!(input_buffer.len() == 2);
    // }

    // #[test]
    // fn it_suspends_if_input_required() {
    //     let (mem, _, pc, _, halt) = run_program(vec![3,0,4,0,99], VecDeque::from(vec![]), 0);
    //     assert_eq!(mem, vec![3,0,4,0,99]);
    //     assert_eq!(halt, false);
    //     assert_eq!(pc, 0);
    // }

    // #[test]
    // fn it_returns_pc_correctly() {
    //     let (_, _, pc, _, halt) = run_program(vec![3,0,4,0,99], VecDeque::from(vec![123]), 0);
    //     assert_eq!(pc, 4);
    //     assert_eq!(halt, true);
    // }

    // #[test]
    // fn it_can_start_from_pc_offset() {
    //     let (_, _, pc, _, halt) = run_program(vec![99,99,99,3,0,4,0,99], VecDeque::from(vec![123]), 3);
    //     assert_eq!(pc, 7);
    //     assert_eq!(halt, true);
    // }

    // #[test]
    // fn it_solves_loop_max_thruster_signal_examples() {
    //     assert_eq!(part_2(&vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5]), (139629729, vec![9,8,7,6,5]));
    //     assert_eq!(part_2(&vec![
    //         3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,
    //         53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10]), (18216, vec![9,7,8,5,6]));
    // }

    #[test]
    fn it_runs_day09_part1_example_program_1() {
        let mut program = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        program.resize(128, 0);
        let mut state = ProgramState{
            mem: program,
            program_counter: 0,
            input_buffer: VecDeque::new(),
            output_buffer: VecDeque::new(),
            mode: (0, 0, 0),
            relative_base: 0,
            halt: false
        };

        let halt_state = run_program(&mut state);
        assert_eq!(halt_state.output_buffer, VecDeque::from(vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]));
    }

    #[test]
    fn it_runs_day09_part1_example_program_2() {
        let mut program = vec![1102,34915192,34915192,7,4,7,99,0];
        program.resize(128, 0);
        let mut state = ProgramState{
            mem: program,
            program_counter: 0,
            input_buffer: VecDeque::new(),
            output_buffer: VecDeque::new(),
            mode: (0, 0, 0),
            relative_base: 0,
            halt: false
        };

        let halt_state = run_program(&mut state);
        assert_eq!(halt_state.output_buffer, VecDeque::from(vec![1219070632396864]));
    }

    #[test]
    fn it_runs_day09_part1_example_program_3() {
        let mut program = vec![104,1125899906842624,99];
        program.resize(128, 0);
        let mut state = ProgramState{
            mem: program,
            program_counter: 0,
            input_buffer: VecDeque::new(),
            output_buffer: VecDeque::new(),
            mode: (0, 0, 0),
            relative_base: 0,
            halt: false
        };

        let halt_state = run_program(&mut state);
        assert_eq!(halt_state.output_buffer, VecDeque::from(vec![1125899906842624]));
    }
}
