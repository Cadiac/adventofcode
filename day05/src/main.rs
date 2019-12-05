const INPUT_FILE: &str = include_str!("../input.txt");

// Opcode 1 adds together numbers read from two positions and stores the result in a third position.
fn add(mut program: Vec<i32>, mut program_counter: usize) -> (Vec<i32>, usize) {
    let a_addr = program[program_counter + 1] as usize;
    let b_addr = program[program_counter + 2] as usize;
    let mem_addr = program[program_counter + 3] as usize;

    program[mem_addr] = program[a_addr] + program[b_addr];
    program_counter += 4;

    return (program, program_counter);
}

// Opcode 2 works exactly like opcode 1, except it multiplies the two inputs instead of adding them.
fn mul(mut program: Vec<i32>, mut program_counter: usize) -> (Vec<i32>, usize) {
    let a_addr = program[program_counter + 1] as usize;
    let b_addr = program[program_counter + 2] as usize;
    let mem_addr = program[program_counter + 3] as usize;

    program[mem_addr] = program[a_addr] * program[b_addr];
    program_counter += 4;

    return (program, program_counter);
}

fn run_program(mut program: Vec<i32>) -> Vec<i32> {
    let mut program_counter = 0;

    loop {
        let (new_program, new_program_counter) = match program[program_counter] {
            1 => add(program, program_counter),
            2 => mul(program, program_counter),
            99 => break,
            _ => {
                println!("pc: {}, opcode: {}, program: {:?}", program_counter, program[program_counter], program);
                panic!("unknown opcode");
            }
        };
        program = new_program;
        program_counter = new_program_counter;
    }

    return program;
}

fn part_1(main_program: &Vec<i32>) -> i32 {
    let mut program = main_program.clone();

    // To do this, before running the program, replace position 1 with the value 12 and replace position 2 with the value 2.
    program[1] = 12;
    program[2] = 2;

    let halt_state = run_program(program);

    // What value is left at position 0 after the program halts?
    return halt_state[0];
}

fn part_2(main_program: &Vec<i32>) -> i32 {
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
        assert_eq!(run_program(vec![1,9,10,3,2,3,11,0,99,30,40,50]), vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
        assert_eq!(run_program(vec![1,0,0,0,99]), vec![2,0,0,0,99]);
        assert_eq!(run_program(vec![2,3,0,3,99]), vec![2,3,0,6,99]);
        assert_eq!(run_program(vec![2,4,4,5,99,0]), vec![2,4,4,5,99,9801]);
        assert_eq!(run_program(vec![1,1,1,4,99,5,6,0,99]), vec![30,1,1,4,2,5,6,0,99]);
    }

    #[test]
    fn it_solves_part2() {
        let program: Vec<i32> = INPUT_FILE.split(',').map(|register| register.parse::<i32>().expect("Parse fail")).collect();
        assert_eq!(part_2(&program), 8444);
    }
}
