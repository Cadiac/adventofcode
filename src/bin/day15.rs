extern crate pancurses;

use std::collections::VecDeque;
use std::collections::HashMap;
use std::io;
use pancurses::{initscr, endwin};

const INPUT_FILE: &str = include_str!("../../inputs/day15.txt");
const MEMORY_SIZE: usize = 2048;

#[derive(Debug, Default)]
struct ProgramState {
    mem: Vec<i64>,
    program_counter: usize,
    input_buffer: VecDeque<i64>,
    output_buffer: VecDeque<i64>,
    mode: (i64, i64, i64),
    relative_base: i64,
    halt: bool
}

#[derive(PartialEq)]
enum Flag {
    InputRequired,
    Halted,
    Running,
    Exception
}

fn read_param(state: &ProgramState, param: i64, mode: i64) -> i64 {
    match mode {
        0 => state.mem[param as usize],
        1 => param as i64,
        2 => state.mem[(state.relative_base + param) as usize],
        _ => panic!("unknown mode")
    }
}

fn write_param(state: &ProgramState, param: i64, mode: i64) -> usize {
    match mode {
        0 => param as usize,
        1 => param as usize,
        2 => (state.relative_base + param) as usize,
        _ => panic!("unknown mode")
    }
}

// Opcode 1 adds together numbers read from two positions and stores the result in a third position.
fn add(state: &mut ProgramState) -> Flag {
    let param_0 = state.mem[state.program_counter + 1];
    let param_1 = state.mem[state.program_counter + 2];
    let param_2 = state.mem[state.program_counter + 3];

    let write_addr = write_param(state, param_2, state.mode.2);

    state.mem[write_addr] = read_param(state, param_0, state.mode.0) + read_param(state, param_1, state.mode.1);
    state.program_counter += 4;

    return Flag::Running;
}

// Opcode 2 works exactly like opcode 1, except it multiplies the two inputs instead of adding them.
fn mul(state: &mut ProgramState) -> Flag {
    let param_0 = state.mem[state.program_counter + 1];
    let param_1 = state.mem[state.program_counter + 2];
    let param_2 = state.mem[state.program_counter + 3];

    let write_addr = write_param(state, param_2, state.mode.2);

    state.mem[write_addr] = read_param(state, param_0, state.mode.0) * read_param(state, param_1, state.mode.1);
    state.program_counter += 4;

    return Flag::Running;
}

// Opcode 3 takes a single integer as input and saves it to the position given by its only parameter. 
// For example, the instruction 3,50 would take an input value and store it at address 50.
fn load_in(state: &mut ProgramState) -> Flag {
    if state.input_buffer.len() == 0 {
        return Flag::InputRequired;
    }

    let param_0 = state.mem[state.program_counter + 1];
    let write_addr = write_param(state, param_0, state.mode.0);

    state.mem[write_addr] = state.input_buffer.pop_front().expect("not enough input values in buffer");
    state.program_counter += 2;

    return Flag::Running;
}

// Opcode 4 outputs the value of its only parameter.
// For example, the instruction 4,50 would output the value at address 50.
fn load_out(state: &mut ProgramState) -> Flag {
    let param_0 = state.mem[state.program_counter + 1];
    state.output_buffer.push_back(read_param(state, param_0, state.mode.0));
    state.program_counter += 2;

    return Flag::Running;
}

// Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
fn jump_if_true(state: &mut ProgramState) -> Flag {
    let param_0 = state.mem[state.program_counter + 1];
    let param_1 = state.mem[state.program_counter + 2];

    if read_param(state, param_0, state.mode.0) != 0 {
        state.program_counter = read_param(state, param_1, state.mode.1) as usize;
    } else {
        state.program_counter += 3;
    }

    return Flag::Running;
}

// Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
fn jump_if_false(state: &mut ProgramState)
    -> Flag {
    let param_0 = state.mem[state.program_counter + 1];
    let param_1 = state.mem[state.program_counter + 2];

    if read_param(state, param_0, state.mode.0) == 0 {
        state.program_counter = read_param(state, param_1, state.mode.1) as usize;
    } else {
        state.program_counter += 3;
    }

    return Flag::Running;
}

// Opcode 7 is less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
fn cmp_less_than(state: &mut ProgramState) -> Flag {
    let param_0 = state.mem[state.program_counter + 1];
    let param_1 = state.mem[state.program_counter + 2];
    let param_2 = state.mem[state.program_counter + 3];

    let write_addr = write_param(state, param_2, state.mode.2);

    if read_param(state, param_0, state.mode.0) < read_param(state, param_1, state.mode.1) {
        state.mem[write_addr] = 1;
    } else {
        state.mem[write_addr] = 0;
    }

    state.program_counter += 4;

    return Flag::Running;
}

// Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
fn cmp_equals(state: &mut ProgramState) -> Flag {
    let param_0 = state.mem[state.program_counter + 1];
    let param_1 = state.mem[state.program_counter + 2];
    let param_2 = state.mem[state.program_counter + 3];

    let write_addr = write_param(state, param_2, state.mode.2);

    if read_param(state, param_0, state.mode.0) == read_param(state, param_1, state.mode.1) {
        state.mem[write_addr] = 1;
    } else {
        state.mem[write_addr] = 0;
    }

    state.program_counter += 4;

    return Flag::Running;
}

// Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
fn adj_rel_base(state: &mut ProgramState) -> Flag {
    let param_0 = state.mem[state.program_counter + 1];

    state.relative_base += read_param(state, param_0, state.mode.0);
    state.program_counter += 2;

    return Flag::Running;
}


fn run_program(state: &mut ProgramState) -> Flag {
    loop {
        let instruction = state.mem[state.program_counter];
        let opcode = instruction % 100;
        let a_mode = (instruction % 1000) / 100;
        let b_mode = (instruction % 10000) / 1000;
        let c_mode = (instruction % 100000) / 10000;
    
        state.mode = (a_mode, b_mode, c_mode);

        // println!("[DEBUG]: state: {:?}", state);
        
        let exec_flag = match opcode {
            1 => add(state),
            2 => mul(state),
            3 => load_in(state),
            4 => load_out(state),
            5 => jump_if_true(state),
            6 => jump_if_false(state),
            7 => cmp_less_than(state),
            8 => cmp_equals(state),
            9 => adj_rel_base(state),
            99 => Flag::Halted,
            _ => Flag::Exception
        };

        match exec_flag {
            Flag::Running => continue,
            Flag::InputRequired => {
                // println!("[SUSPEND]: state: {:?}", state);
                return Flag::InputRequired;
            },
            Flag::Halted => {
                state.halt = true;
                // println!("[HALT]: state: {:?}", state);
                return Flag::Halted;
            },
            Flag::Exception => {
                state.halt = true;
                println!("[ERROR]: state: {:?}", state);
                return Flag::Exception;
            }
        }
    }
}

fn part_1(program: Vec<i64>) -> io::Result<()> {
    let mut memory = program.clone();
    memory.resize(MEMORY_SIZE, 0);

    let mut state = ProgramState{
        mem: memory,
        input_buffer: VecDeque::new(),
        ..Default::default()
    };

    let mut world: HashMap<(i64, i64), (i64, i64, i64)> = HashMap::new();

    let window = initscr();
    pancurses::curs_set(0);
    window.keypad(true);
    pancurses::start_color();

    for i in 10..30 {
        pancurses::init_color(i, 0, 1000 - (i-10)*40, 0);
        pancurses::init_pair(i, i, pancurses::COLOR_WHITE);
    }

    let mut facing = 3;

    let mut x = 0;
    let mut y = 0;

    let mut goal_found = false;
    let mut goal_x = 0;
    let mut goal_y = 0;

    let mut part_1 = -1;

    'main: loop {
        let direction;
        let part1_distance = world.entry((x, y)).or_insert((1, 0, 99999)).1;
        let part2_distance = world.entry((x, y)).or_insert((1, 0, 99999)).2;

        let mut target_x = x;
        let mut target_y = y;

        match facing {
            0 => { target_y -= 1; direction = 1; }, // Up
            1 => { target_x += 1; direction = 4; }, // Right
            2 => { target_y += 1; direction = 2; }, // Down
            3 => { target_x -= 1; direction = 3; }, // Left
            _ => unreachable!()
        }

        state.input_buffer.push_back(direction);

        let flag = run_program(&mut state);
        
        if Flag::Halted == flag {
            endwin();
            return Ok(());
        }

        // Explore the maze by following wall on the left
        'explore: while let Some(output) = state.output_buffer.pop_front() {
            if output == 0 {
                // hit a wall
                world.entry((target_x, target_y)).or_insert((output, 0, 0));

                // Move to the direction one step right from wall
                // so the wall stays on left
                facing = (facing + 1) % 4;
            } else if output == 1 && !goal_found {
                // moved
                x = target_x;
                y = target_y;

                world.entry((x, y)).or_insert((output, part1_distance + 1, 99999));

                // test is there a wall to left of us by moving there
                facing = (facing + 3) % 4;
            } else if output == 2 && !goal_found {
                // we found the goal - now start exploring from that point to everywhere,
                // mapping the distance
                goal_found = true;
                part_1 = part1_distance + 1;

                x = target_x;
                y = target_y;
                
                goal_x = x;
                goal_y = y;

                world.entry((x, y)).or_insert((output, part1_distance + 1, 0));
            } else if (output == 1 || output == 2) && goal_found {
                // moved
                x = target_x;
                y = target_y;

                if x == goal_x && y == goal_y {
                    // Back at goal, done
                    match window.getch() {
                        Some(_input) => (),
                        None => ()
                    };
                    window.refresh();
                    endwin();

                    return Ok(());
                }

                match world.get(&(x, y)) {
                    Some((_tile, _, known_distance)) => {
                        if *known_distance > part2_distance + 1 {
                            world.insert((x, y), (output, 0, part2_distance + 1));
                        }
                    },
                    None => {
                        world.insert((x, y), (output, 0, part2_distance + 1));
                    }
                }

                world.entry((x, y)).or_insert((output, 0, part2_distance + 1));

                // test is there a wall to left of us by moving there
                facing = (facing + 3) % 4;
            }
        }

        let (_, (_, _, part_2)) = world.iter().max_by(|(_, (_, _, a)), (_, (_, _, b))| a.cmp(b)).unwrap();

        window.mvprintw(0, 16, &format!("[PART_1]: {}", part_1));
        window.mvprintw(1, 16, &format!("[PART_2]: {}   ", part_2));
        window.mvprintw(2, 16, &format!("[OXYGEN]: ({}, {})", goal_x, goal_y));
        for (coords, (tile, _p1_dist, p2_dist)) in &world {
            if *tile == 0 {
                window.mvprintw((coords.1 + 25) as i32, (coords.0 + 25) as i32, "█");
            }
            if *tile == 1 {
                if *p2_dist != 99999 {
                    window.attron(pancurses::COLOR_PAIR((p2_dist / 17) as pancurses::chtype + 10));
                    window.mvprintw((coords.1 + 25) as i32, (coords.0 + 25) as i32, "█");
                    window.attroff(pancurses::COLOR_PAIR((p2_dist / 17) as pancurses::chtype + 10));
                }
            }
            if *tile == 2 {
                window.mvprintw((coords.1 + 25) as i32, (coords.0 + 25) as i32, "▒");
            }
        }

        window.mvprintw((y + 25) as i32, (x + 25) as i32, ["^", ">", "v", "<"][facing]);

        window.refresh();
    }
}

fn main() -> () {
    let program: Vec<i64> = INPUT_FILE.split(',').map(|register| register.parse::<i64>().expect("Parse fail")).collect();

    part_1(program.clone()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_runs_part1_example_programs() {
        let mut state = ProgramState{ mem: vec![1,9,10,3,2,3,11,0,99,30,40,50], ..Default::default() };
        run_program(&mut state);
        assert_eq!(state.mem, vec![3500,9,10,70,2,3,11,0,99,30,40,50]);

        let mut state = ProgramState{ mem: vec![1,0,0,0,99], ..Default::default() };
        run_program(&mut state);
        assert_eq!(state.mem, vec![2,0,0,0,99]);
        
        let mut state = ProgramState{ mem: vec![2,3,0,3,99], ..Default::default() };
        run_program(&mut state);
        assert_eq!(state.mem, vec![2,3,0,6,99]);
        
        let mut state = ProgramState{ mem: vec![2,4,4,5,99,0], ..Default::default() };
        run_program(&mut state);
        assert_eq!(state.mem, vec![2,4,4,5,99,9801]);
        
        let mut state = ProgramState{ mem: vec![1,1,1,4,99,5,6,0,99], ..Default::default() };
        run_program(&mut state);
        assert_eq!(state.mem, vec![30,1,1,4,2,5,6,0,99]);
    }

    #[test]
    fn it_solves_input_output_example() {
        let mut state = ProgramState{ mem: vec![3,0,4,0,99], input_buffer: VecDeque::from(vec![123]), ..Default::default() };
        run_program(&mut state);
        assert!(state.halt);
        assert_eq!(state.mem, vec![123,0,4,0,99]);
        assert_eq!(state.output_buffer, VecDeque::from(vec![123]));
    }

    #[test]
    fn it_solves_parameter_mode_examples() {
        let mut state = ProgramState{ mem: vec![1002,4,3,4,33], ..Default::default() };
        run_program(&mut state);
        assert!(state.halt);
        assert_eq!(state.mem, vec![1002,4,3,4,99]);
    }

    #[test]
    fn it_solves_negative_examples() {
        let mut state = ProgramState{ mem: vec![1101,100,-1,4,0], ..Default::default() };
        run_program(&mut state);
        assert!(state.halt);
        assert_eq!(state.mem, vec![1101,100,-1,4,99]);
    }

    #[test]
    fn it_solves_day5_part2_compare_example_1() {
        // Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        let program = vec![3,9,8,9,10,9,4,9,99,-1,8];

        let mut state = ProgramState{ mem: program.clone(), input_buffer: VecDeque::from(vec![8]), ..Default::default() };
        run_program(&mut state);
        assert!(state.halt);
        assert!(state.input_buffer.is_empty());
        assert_eq!(state.output_buffer, VecDeque::from(vec![1]));

        let mut state = ProgramState{ mem: program.clone(), input_buffer: VecDeque::from(vec![9]), ..Default::default() };
        run_program(&mut state);
        assert!(state.halt);
        assert!(state.input_buffer.is_empty());
        assert_eq!(state.output_buffer, VecDeque::from(vec![0]));
    }

    #[test]
    fn it_solves_day5_part2_compare_example_2() {
        // Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        let program = vec![3,9,7,9,10,9,4,9,99,-1,8];

        let mut state = ProgramState{ mem: program.clone(), input_buffer: VecDeque::from(vec![7]), ..Default::default() };
        run_program(&mut state);
        assert!(state.halt);
        assert!(state.input_buffer.is_empty());
        assert_eq!(state.output_buffer, VecDeque::from(vec![1]));

        let mut state = ProgramState{ mem: program.clone(), input_buffer: VecDeque::from(vec![9]), ..Default::default() };
        run_program(&mut state);
        assert!(state.halt);
        assert!(state.input_buffer.is_empty());
        assert_eq!(state.output_buffer, VecDeque::from(vec![0]));
    }

    #[test]
    fn it_solves_day5_part2_compare_example_3() {
        // Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        let program = vec![3,3,1108,-1,8,3,4,3,99];

        let mut state = ProgramState{ mem: program.clone(), input_buffer: VecDeque::from(vec![8]), ..Default::default() };
        run_program(&mut state);
        assert!(state.halt);
        assert!(state.input_buffer.is_empty());
        assert_eq!(state.output_buffer, VecDeque::from(vec![1]));

        let mut state = ProgramState{ mem: program.clone(), input_buffer: VecDeque::from(vec![9]), ..Default::default() };
        run_program(&mut state);
        assert!(state.halt);
        assert!(state.input_buffer.is_empty());
        assert_eq!(state.output_buffer, VecDeque::from(vec![0]));
    }

    #[test]
    fn it_solves_day5_part2_compare_example_4() {
        // Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not)
        let program = vec![3,3,1107,-1,8,3,4,3,99];

        let mut state = ProgramState{ mem: program.clone(), input_buffer: VecDeque::from(vec![7]), ..Default::default() };
        run_program(&mut state);
        assert!(state.halt);
        assert!(state.input_buffer.is_empty());
        assert_eq!(state.output_buffer, VecDeque::from(vec![1]));

        let mut state = ProgramState{ mem: program.clone(), input_buffer: VecDeque::from(vec![9]), ..Default::default() };
        run_program(&mut state);
        assert!(state.halt);
        assert!(state.input_buffer.is_empty());
        assert_eq!(state.output_buffer, VecDeque::from(vec![0]));
    }

    #[test]
    fn it_solves_day5_part2_jump_tests() {
        // Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:

        // using position mode
        let program = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        let mut state = ProgramState{ mem: program.clone(), input_buffer: VecDeque::from(vec![0]), ..Default::default() };
        run_program(&mut state);
        assert!(state.halt);
        assert!(state.input_buffer.is_empty());
        assert_eq!(state.output_buffer, VecDeque::from(vec![0]));

        let mut state = ProgramState{ mem: program.clone(), input_buffer: VecDeque::from(vec![1]), ..Default::default() };
        run_program(&mut state);
        assert!(state.halt);
        assert!(state.input_buffer.is_empty());
        assert_eq!(state.output_buffer, VecDeque::from(vec![1]));

        // // using immediate mode
        let program = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        let mut state = ProgramState{ mem: program.clone(), input_buffer: VecDeque::from(vec![0]), ..Default::default() };
        run_program(&mut state);
        assert!(state.halt);
        assert!(state.input_buffer.is_empty());
        assert_eq!(state.output_buffer, VecDeque::from(vec![0]));

        let mut state = ProgramState{ mem: program.clone(), input_buffer: VecDeque::from(vec![123]), ..Default::default() };
        run_program(&mut state);
        assert!(state.halt);
        assert!(state.input_buffer.is_empty());
        assert_eq!(state.output_buffer, VecDeque::from(vec![1]));
    }

    #[test]
    fn it_solves_day5_part2_large_example() {
        // The mem uses an input instruction to ask for a single number.
        // The mem will then output 999 if the input value is below 8
        let program = vec![
            3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];

        let mut state = ProgramState{ mem: program.clone(), input_buffer: VecDeque::from(vec![7]), ..Default::default() };
        run_program(&mut state);
        assert!(state.halt);
        assert!(state.input_buffer.is_empty());
        assert_eq!(state.output_buffer, VecDeque::from(vec![999]));

        // output 1000 if the input value is equal to 8
        let mut state = ProgramState{ mem: program.clone(), input_buffer: VecDeque::from(vec![8]), ..Default::default() };
        run_program(&mut state);
        assert!(state.halt);
        assert!(state.input_buffer.is_empty());
        assert_eq!(state.output_buffer, VecDeque::from(vec![1000]));

        // or output 1001 if the input value is greater than 8.
        let mut state = ProgramState{ mem: program.clone(), input_buffer: VecDeque::from(vec![9]), ..Default::default() };
        run_program(&mut state);
        assert!(state.halt);
        assert!(state.input_buffer.is_empty());
        assert_eq!(state.output_buffer, VecDeque::from(vec![1001]));
    }

    #[test]
    fn it_handles_input_buffers() {
        let mut state = ProgramState{ mem: vec![3,0,4,0,99], input_buffer: VecDeque::from(vec![123, 124, 125]), ..Default::default() };
        run_program(&mut state);
        assert!(state.halt);
        assert_eq!(state.mem, vec![123,0,4,0,99]);
        assert_eq!(state.output_buffer, VecDeque::from(vec![123]));
        assert!(state.input_buffer.len() == 2);
    }

    #[test]
    fn it_suspends_if_input_required() {
        let mut state = ProgramState{ mem: vec![3,0,4,0,99], input_buffer: VecDeque::from(vec![]), ..Default::default() };
        run_program(&mut state);
        assert!(!state.halt);
        assert_eq!(state.mem, vec![3,0,4,0,99]);
        assert_eq!(state.program_counter, 0);
    }

    #[test]
    fn it_keeps_track_of_pc_correctly() {
        let mut state = ProgramState{ mem: vec![3,0,4,0,99], input_buffer: VecDeque::from(vec![123]), ..Default::default() };
        run_program(&mut state);
        assert_eq!(state.program_counter, 4);
        assert!(state.halt);
    }

    #[test]
    fn it_can_start_from_pc_offset() {
        let mut state = ProgramState{ mem: vec![99,99,99,3,0,4,0,99], input_buffer: VecDeque::from(vec![123]), program_counter: 3, ..Default::default() };
        run_program(&mut state);
        assert_eq!(state.program_counter, 7);
        assert!(state.halt);
    }

    #[test]
    fn it_runs_day09_part1_example_program_1() {
        let mut program = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        program.resize(128, 0);
        let mut state = ProgramState{ mem: program, ..Default::default() };

        run_program(&mut state);
        assert_eq!(state.output_buffer, VecDeque::from(vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]));
    }

    #[test]
    fn it_runs_day09_part1_example_program_2() {
        let mut program = vec![1102,34915192,34915192,7,4,7,99,0];
        program.resize(128, 0);
        let mut state = ProgramState{ mem: program, ..Default::default() };

        run_program(&mut state);
        assert_eq!(state.output_buffer, VecDeque::from(vec![1219070632396864]));
    }

    #[test]
    fn it_runs_day09_part1_example_program_3() {
        let mut program = vec![104,1125899906842624,99];
        program.resize(128, 0);
        let mut state = ProgramState{ mem: program, ..Default::default() };

        run_program(&mut state);
        assert_eq!(state.output_buffer, VecDeque::from(vec![1125899906842624]));
    }
}