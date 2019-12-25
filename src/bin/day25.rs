use std::collections::VecDeque;
use std::io;

use aoc19::{IntcodeComputer, Flag};

const INPUT_FILE: &str = include_str!("../../inputs/day25.txt");
const MEMORY_SIZE: usize = 65536;

// I just played the game. Correct combination of items was 
// - fuel cell
// - astrolabe
// - ornament
// - hologram
fn part_1(program: Vec<i64>) {
    let mut memory = program.clone();
    memory.resize(MEMORY_SIZE, 0);

    let mut computer = IntcodeComputer{
        mem: memory.clone(),
        input_buffer: VecDeque::new(),
        ..Default::default()
    };

    while let Flag::InputRequired = &computer.run_program() {
        while let Some(output) = computer.output_buffer.pop_front() {
            print!("{}", (output as u8) as char);
        }

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let ascii_input: Vec<i64> = input
                    .as_bytes()
                    .iter()
                    .map(|character| *character as i64)
                    .collect();
    
                computer.input_buffer = VecDeque::from(ascii_input);
            }
            Err(error) => println!("error: {}", error),
        }
    }

    while let Some(output) = computer.output_buffer.pop_front() {
        print!("{}", (output as u8) as char);
    }
    println!("Game over.");
}

fn main() -> () {
    let program: Vec<i64> = INPUT_FILE.split(',').map(|register| register.parse::<i64>().expect("Parse fail")).collect();

    part_1(program.clone());
}
