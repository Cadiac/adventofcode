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

        for (_addr, computer) in &mut network {
            computer.run_program();

            if let Some(address) = computer.output_buffer.pop_front() {
                if address == 255 {
                    let _x = computer.output_buffer.pop_front().unwrap();
                    let y = computer.output_buffer.pop_front().unwrap();
                    return y;
                }

                let x = computer.output_buffer.pop_front().unwrap();
                let y = computer.output_buffer.pop_front().unwrap();

                println!("[DEBUG]: [{:0>3}][{: >16} {: >16}]", address, x, y);

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

fn part_2(program: Vec<i64>) -> i64 {
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

    // Initialize with values that don't conflict
    let mut nat: (i64, i64) = (-1, -1);
    let mut last_nat_y: (i64) = 0;

    'main: loop {
        let mut incoming_packets: Vec<(i64, i64, i64)> = Vec::new();

        'run_network: for (_addr, computer) in &mut network {
            computer.run_program();

            if let Some(address) = computer.output_buffer.pop_front() {
                if address == 255 {
                    let x = computer.output_buffer.pop_front().unwrap();
                    let y = computer.output_buffer.pop_front().unwrap();

                    nat.0 = x;
                    nat.1 = y;
                    continue 'run_network;
                }

                let x = computer.output_buffer.pop_front().unwrap();
                let y = computer.output_buffer.pop_front().unwrap();

                println!("[DEBUG]: [{:0>3}][{: >16} {: >16}]", address, x, y);

                incoming_packets.push((address, x, y));
            }
        }

        for (address, x, y) in incoming_packets {
            network.get_mut(&address).unwrap().input_buffer.push_back(x);
            network.get_mut(&address).unwrap().input_buffer.push_back(y);
        }

        let mut all_idle = true;
        for (_addr, computer) in &mut network {
            if computer.input_buffer.is_empty() {
                computer.input_buffer.push_back(-1);
            } else {
                all_idle = false;
            }
        }

        if all_idle {
            if last_nat_y == nat.1 {
                return last_nat_y;
            }
            last_nat_y = nat.1;
            network.get_mut(&0).unwrap().input_buffer.push_back(nat.0);
            network.get_mut(&0).unwrap().input_buffer.push_back(nat.1);
        }
    }
}

fn main() -> () {
    let program: Vec<i64> = INPUT_FILE.split(',').map(|register| register.parse::<i64>().expect("Parse fail")).collect();

    let first_y_packet = part_1(program.clone());
    println!("[INFO]: PART1 RESULT {}", first_y_packet);

    let first_y_repeat_packet = part_2(program.clone());
    println!("[INFO]: PART2 RESULT {}", first_y_repeat_packet);
}
