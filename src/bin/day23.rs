use std::collections::HashMap;
use std::collections::VecDeque;

use aoc19::{IntcodeComputer};

const INPUT_FILE: &str = include_str!("../../inputs/day23.txt");
const MEMORY_SIZE: usize = 4096;

fn part_1(program: Vec<i64>) -> i64 {
    let mut memory = program.clone();
    memory.resize(MEMORY_SIZE, 0);

    let mut network: HashMap<i64, IntcodeComputer> = HashMap::new();

    for address in 0..50 {
        let computer = IntcodeComputer{
            mem: memory.clone(),
            input_buffer: VecDeque::from(vec![address]),
            ..Default::default()
        };

        network.insert(address, computer);
    }

    loop {
        let mut pending_inputs: Vec<(i64, i64, i64)> = Vec::new();

        for (addr, computer) in &mut network {
            computer.run_program();

            if let Some(address) = computer.output_buffer.pop_front() {
                println!("address: {}", address);

                if address == 255 {
                    let _x = computer.output_buffer.pop_front().unwrap();
                    let y = computer.output_buffer.pop_front().unwrap();
                    return y;
                }

                let x = computer.output_buffer.pop_front().unwrap();
                let y = computer.output_buffer.pop_front().unwrap();

                println!("packet: {} - {} {}", address, x, y);

                pending_inputs.push((address, x, y));
            }
        }

        for (address, x, y) in pending_inputs {
            network.get_mut(&address).unwrap().input_buffer.push_back(x);
            network.get_mut(&address).unwrap().input_buffer.push_back(y);
        }

        for (_addr, computer) in &mut network {
            // Lazy, too many -1 but what ever
            computer.input_buffer.push_back(-1);
        }
    }
}

fn main() -> () {
    let program: Vec<i64> = INPUT_FILE.split(',').map(|register| register.parse::<i64>().expect("Parse fail")).collect();

    let damage1 = part_1(program.clone());

    println!("[PART1]: {}", damage1);
}
