use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
pub enum Flag {
    InputRequired,
    Halted,
    Running,
    Exception
}

#[derive(Debug, Default)]
pub struct IntcodeComputer {
    pub mem: Vec<i64>,
    pub program_counter: usize,
    pub input_buffer: VecDeque<i64>,
    pub output_buffer: VecDeque<i64>,
    pub relative_base: i64,
    pub halt: bool
}

impl IntcodeComputer {
    #[inline]
    fn read_param(&self, param: i64, mode: i64) -> i64 {
        match mode {
            0 => self.mem[param as usize],
            1 => param as i64,
            2 => self.mem[(self.relative_base + param) as usize],
            _ => panic!("unknown mode")
        }
    }
    
    #[inline]
    fn write_param(&self, param: i64, mode: i64) -> usize {
        match mode {
            0 => param as usize,
            1 => param as usize,
            2 => (self.relative_base + param) as usize,
            _ => panic!("unknown mode")
        }
    }
    
    // Opcode 1 adds together numbers read from two positions and stores the result in a third position.
    #[inline]
    fn add(&mut self, a_mode: i64, b_mode: i64, write_mode: i64) -> Flag {
        let param_0 = self.mem[self.program_counter + 1];
        let param_1 = self.mem[self.program_counter + 2];
        let param_2 = self.mem[self.program_counter + 3];

        let write_addr = self.write_param(param_2, write_mode);

        self.mem[write_addr] = self.read_param(param_0, a_mode) + self.read_param(param_1, b_mode);
        self.program_counter += 4;
    
        return Flag::Running;
    }

    
    // Opcode 2 works exactly like opcode 1, except it multiplies the two inputs instead of adding them.
    #[inline]
    fn mul(&mut self, a_mode: i64, b_mode: i64, write_mode: i64) -> Flag {
        let param_0 = self.mem[self.program_counter + 1];
        let param_1 = self.mem[self.program_counter + 2];
        let param_2 = self.mem[self.program_counter + 3];
    
        let write_addr = self.write_param(param_2, write_mode);
    
        self.mem[write_addr] = self.read_param(param_0, a_mode) * self.read_param(param_1, b_mode);
        self.program_counter += 4;
    
        return Flag::Running;
    }
    
    // Opcode 3 takes a single integer as input and saves it to the position given by its only parameter. 
    // For example, the instruction 3,50 would take an input value and store it at address 50.
    #[inline]
    fn load_in(&mut self, a_mode: i64) -> Flag {
        if self.input_buffer.len() == 0 {
            return Flag::InputRequired;
        }
    
        let param_0 = self.mem[self.program_counter + 1];
        let write_addr = self.write_param(param_0, a_mode);
    
        self.mem[write_addr] = self.input_buffer.pop_front().expect("not enough input values in buffer");
        self.program_counter += 2;
    
        return Flag::Running;
    }
    
    // Opcode 4 outputs the value of its only parameter.
    // For example, the instruction 4,50 would output the value at address 50.
    #[inline]
    fn load_out(&mut self, a_mode: i64) -> Flag {
        let param_0 = self.mem[self.program_counter + 1];
        self.output_buffer.push_back(self.read_param(param_0, a_mode));
        self.program_counter += 2;
    
        return Flag::Running;
    }
    
    // Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
    #[inline]
    fn jump_if_true(&mut self, a_mode: i64, b_mode: i64) -> Flag {
        let param_0 = self.mem[self.program_counter + 1];
        let param_1 = self.mem[self.program_counter + 2];
    
        if self.read_param(param_0, a_mode) != 0 {
            self.program_counter = self.read_param(param_1, b_mode) as usize;
        } else {
            self.program_counter += 3;
        }
    
        return Flag::Running;
    }
    
    // Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
    #[inline]
    fn jump_if_false(&mut self, a_mode: i64, b_mode: i64) -> Flag {
        let param_0 = self.mem[self.program_counter + 1];
        let param_1 = self.mem[self.program_counter + 2];
    
        if self.read_param(param_0, a_mode) == 0 {
            self.program_counter = self.read_param(param_1, b_mode) as usize;
        } else {
            self.program_counter += 3;
        }
    
        return Flag::Running;
    }
    
    // Opcode 7 is less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
    #[inline]
    fn cmp_less_than(&mut self, a_mode: i64, b_mode: i64, write_mode: i64) -> Flag {
        let param_0 = self.mem[self.program_counter + 1];
        let param_1 = self.mem[self.program_counter + 2];
        let param_2 = self.mem[self.program_counter + 3];
    
        let write_addr = self.write_param(param_2, write_mode);
    
        if self.read_param(param_0, a_mode) < self.read_param(param_1, b_mode) {
            self.mem[write_addr] = 1;
        } else {
            self.mem[write_addr] = 0;
        }
    
        self.program_counter += 4;
    
        return Flag::Running;
    }
    
    // Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
    #[inline]
    fn cmp_equals(&mut self, a_mode: i64, b_mode: i64, write_mode: i64) -> Flag {
        let param_0 = self.mem[self.program_counter + 1];
        let param_1 = self.mem[self.program_counter + 2];
        let param_2 = self.mem[self.program_counter + 3];
    
        let write_addr = self.write_param(param_2, write_mode);
    
        if self.read_param(param_0, a_mode) == self.read_param(param_1, b_mode) {
            self.mem[write_addr] = 1;
        } else {
            self.mem[write_addr] = 0;
        }
    
        self.program_counter += 4;
    
        return Flag::Running;
    }
    
    // Opcode 9 adjusts relative base
    #[inline]
    fn adj_rel_base(&mut self, a_mode: i64) -> Flag {
        let param_0 = self.mem[self.program_counter + 1];
    
        self.relative_base += self.read_param(param_0, a_mode);
        self.program_counter += 2;
    
        return Flag::Running;
    }
    
    
    pub fn run_program(&mut self) -> Flag {
        loop {
            let instruction = self.mem[self.program_counter];
            let opcode = instruction % 100;
            let a_mode = (instruction % 1000) / 100;
            let b_mode = (instruction % 10000) / 1000;
            let c_mode = (instruction % 100000) / 10000;
                
            let exec_flag = match opcode {
                1 => self.add(a_mode, b_mode, c_mode),
                2 => self.mul(a_mode, b_mode, c_mode),
                3 => self.load_in(a_mode),
                4 => self.load_out(a_mode),
                5 => self.jump_if_true(a_mode, b_mode),
                6 => self.jump_if_false(a_mode, b_mode),
                7 => self.cmp_less_than(a_mode, b_mode, c_mode),
                8 => self.cmp_equals(a_mode, b_mode, c_mode),
                9 => self.adj_rel_base(a_mode),
                99 => Flag::Halted,
                _ => Flag::Exception
            };
    
            match exec_flag {
                Flag::Running => continue,
                Flag::InputRequired => {
                    return Flag::InputRequired;
                },
                Flag::Halted => {
                    self.halt = true;
                    return Flag::Halted;
                },
                Flag::Exception => {
                    self.halt = true;
                    println!("[ERROR]: computer: {:?}", self);
                    return Flag::Exception;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_runs_part1_example_programs() {
        let mut computer = IntcodeComputer{ mem: vec![1,9,10,3,2,3,11,0,99,30,40,50], ..Default::default() };
        computer.run_program();
        assert_eq!(computer.mem, vec![3500,9,10,70,2,3,11,0,99,30,40,50]);

        let mut computer = IntcodeComputer{ mem: vec![1,0,0,0,99], ..Default::default() };
        computer.run_program();
        assert_eq!(computer.mem, vec![2,0,0,0,99]);
        
        let mut computer = IntcodeComputer{ mem: vec![2,3,0,3,99], ..Default::default() };
        computer.run_program();
        assert_eq!(computer.mem, vec![2,3,0,6,99]);
        
        let mut computer = IntcodeComputer{ mem: vec![2,4,4,5,99,0], ..Default::default() };
        computer.run_program();
        assert_eq!(computer.mem, vec![2,4,4,5,99,9801]);
        
        let mut computer = IntcodeComputer{ mem: vec![1,1,1,4,99,5,6,0,99], ..Default::default() };
        computer.run_program();
        assert_eq!(computer.mem, vec![30,1,1,4,2,5,6,0,99]);
    }

    #[test]
    fn it_solves_input_output_example() {
        let mut computer = IntcodeComputer{ mem: vec![3,0,4,0,99], input_buffer: VecDeque::from(vec![123]), ..Default::default() };
        computer.run_program();
        assert!(computer.halt);
        assert_eq!(computer.mem, vec![123,0,4,0,99]);
        assert_eq!(computer.output_buffer, VecDeque::from(vec![123]));
    }

    #[test]
    fn it_solves_parameter_mode_examples() {
        let mut computer = IntcodeComputer{ mem: vec![1002,4,3,4,33], ..Default::default() };
        computer.run_program();
        assert!(computer.halt);
        assert_eq!(computer.mem, vec![1002,4,3,4,99]);
    }

    #[test]
    fn it_solves_negative_examples() {
        let mut computer = IntcodeComputer{ mem: vec![1101,100,-1,4,0], ..Default::default() };
        computer.run_program();
        assert!(computer.halt);
        assert_eq!(computer.mem, vec![1101,100,-1,4,99]);
    }

    #[test]
    fn it_solves_day5_part2_compare_example_1() {
        // Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        let program = vec![3,9,8,9,10,9,4,9,99,-1,8];

        let mut computer = IntcodeComputer{ mem: program.clone(), input_buffer: VecDeque::from(vec![8]), ..Default::default() };
        computer.run_program();
        assert!(computer.halt);
        assert!(computer.input_buffer.is_empty());
        assert_eq!(computer.output_buffer, VecDeque::from(vec![1]));

        let mut computer = IntcodeComputer{ mem: program.clone(), input_buffer: VecDeque::from(vec![9]), ..Default::default() };
        computer.run_program();
        assert!(computer.halt);
        assert!(computer.input_buffer.is_empty());
        assert_eq!(computer.output_buffer, VecDeque::from(vec![0]));
    }

    #[test]
    fn it_solves_day5_part2_compare_example_2() {
        // Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        let program = vec![3,9,7,9,10,9,4,9,99,-1,8];

        let mut computer = IntcodeComputer{ mem: program.clone(), input_buffer: VecDeque::from(vec![7]), ..Default::default() };
        computer.run_program();
        assert!(computer.halt);
        assert!(computer.input_buffer.is_empty());
        assert_eq!(computer.output_buffer, VecDeque::from(vec![1]));

        let mut computer = IntcodeComputer{ mem: program.clone(), input_buffer: VecDeque::from(vec![9]), ..Default::default() };
        computer.run_program();
        assert!(computer.halt);
        assert!(computer.input_buffer.is_empty());
        assert_eq!(computer.output_buffer, VecDeque::from(vec![0]));
    }

    #[test]
    fn it_solves_day5_part2_compare_example_3() {
        // Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        let program = vec![3,3,1108,-1,8,3,4,3,99];

        let mut computer = IntcodeComputer{ mem: program.clone(), input_buffer: VecDeque::from(vec![8]), ..Default::default() };
        computer.run_program();
        assert!(computer.halt);
        assert!(computer.input_buffer.is_empty());
        assert_eq!(computer.output_buffer, VecDeque::from(vec![1]));

        let mut computer = IntcodeComputer{ mem: program.clone(), input_buffer: VecDeque::from(vec![9]), ..Default::default() };
        computer.run_program();
        assert!(computer.halt);
        assert!(computer.input_buffer.is_empty());
        assert_eq!(computer.output_buffer, VecDeque::from(vec![0]));
    }

    #[test]
    fn it_solves_day5_part2_compare_example_4() {
        // Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not)
        let program = vec![3,3,1107,-1,8,3,4,3,99];

        let mut computer = IntcodeComputer{ mem: program.clone(), input_buffer: VecDeque::from(vec![7]), ..Default::default() };
        computer.run_program();
        assert!(computer.halt);
        assert!(computer.input_buffer.is_empty());
        assert_eq!(computer.output_buffer, VecDeque::from(vec![1]));

        let mut computer = IntcodeComputer{ mem: program.clone(), input_buffer: VecDeque::from(vec![9]), ..Default::default() };
        computer.run_program();
        assert!(computer.halt);
        assert!(computer.input_buffer.is_empty());
        assert_eq!(computer.output_buffer, VecDeque::from(vec![0]));
    }

    #[test]
    fn it_solves_day5_part2_jump_tests() {
        // Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:

        // using position mode
        let program = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        let mut computer = IntcodeComputer{ mem: program.clone(), input_buffer: VecDeque::from(vec![0]), ..Default::default() };
        computer.run_program();
        assert!(computer.halt);
        assert!(computer.input_buffer.is_empty());
        assert_eq!(computer.output_buffer, VecDeque::from(vec![0]));

        let mut computer = IntcodeComputer{ mem: program.clone(), input_buffer: VecDeque::from(vec![1]), ..Default::default() };
        computer.run_program();
        assert!(computer.halt);
        assert!(computer.input_buffer.is_empty());
        assert_eq!(computer.output_buffer, VecDeque::from(vec![1]));

        // // using immediate mode
        let program = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        let mut computer = IntcodeComputer{ mem: program.clone(), input_buffer: VecDeque::from(vec![0]), ..Default::default() };
        computer.run_program();
        assert!(computer.halt);
        assert!(computer.input_buffer.is_empty());
        assert_eq!(computer.output_buffer, VecDeque::from(vec![0]));

        let mut computer = IntcodeComputer{ mem: program.clone(), input_buffer: VecDeque::from(vec![123]), ..Default::default() };
        computer.run_program();
        assert!(computer.halt);
        assert!(computer.input_buffer.is_empty());
        assert_eq!(computer.output_buffer, VecDeque::from(vec![1]));
    }

    #[test]
    fn it_solves_day5_part2_large_example() {
        // The mem uses an input instruction to ask for a single number.
        // The mem will then output 999 if the input value is below 8
        let program = vec![
            3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];

        let mut computer = IntcodeComputer{ mem: program.clone(), input_buffer: VecDeque::from(vec![7]), ..Default::default() };
        computer.run_program();
        assert!(computer.halt);
        assert!(computer.input_buffer.is_empty());
        assert_eq!(computer.output_buffer, VecDeque::from(vec![999]));

        // output 1000 if the input value is equal to 8
        let mut computer = IntcodeComputer{ mem: program.clone(), input_buffer: VecDeque::from(vec![8]), ..Default::default() };
        computer.run_program();
        assert!(computer.halt);
        assert!(computer.input_buffer.is_empty());
        assert_eq!(computer.output_buffer, VecDeque::from(vec![1000]));

        // or output 1001 if the input value is greater than 8.
        let mut computer = IntcodeComputer{ mem: program.clone(), input_buffer: VecDeque::from(vec![9]), ..Default::default() };
        computer.run_program();
        assert!(computer.halt);
        assert!(computer.input_buffer.is_empty());
        assert_eq!(computer.output_buffer, VecDeque::from(vec![1001]));
    }

    #[test]
    fn it_handles_input_buffers() {
        let mut computer = IntcodeComputer{ mem: vec![3,0,4,0,99], input_buffer: VecDeque::from(vec![123, 124, 125]), ..Default::default() };
        computer.run_program();
        assert!(computer.halt);
        assert_eq!(computer.mem, vec![123,0,4,0,99]);
        assert_eq!(computer.output_buffer, VecDeque::from(vec![123]));
        assert!(computer.input_buffer.len() == 2);
    }

    #[test]
    fn it_suspends_if_input_required() {
        let mut computer = IntcodeComputer{ mem: vec![3,0,4,0,99], input_buffer: VecDeque::from(vec![]), ..Default::default() };
        computer.run_program();
        assert!(!computer.halt);
        assert_eq!(computer.mem, vec![3,0,4,0,99]);
        assert_eq!(computer.program_counter, 0);
    }

    #[test]
    fn it_keeps_track_of_pc_correctly() {
        let mut computer = IntcodeComputer{ mem: vec![3,0,4,0,99], input_buffer: VecDeque::from(vec![123]), ..Default::default() };
        computer.run_program();
        assert_eq!(computer.program_counter, 4);
        assert!(computer.halt);
    }

    #[test]
    fn it_can_start_from_pc_offset() {
        let mut computer = IntcodeComputer{ mem: vec![99,99,99,3,0,4,0,99], input_buffer: VecDeque::from(vec![123]), program_counter: 3, ..Default::default() };
        computer.run_program();
        assert_eq!(computer.program_counter, 7);
        assert!(computer.halt);
    }

    #[test]
    fn it_runs_day09_part1_example_program_1() {
        let mut program = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        program.resize(128, 0);
        let mut computer = IntcodeComputer{ mem: program, ..Default::default() };

        computer.run_program();
        assert_eq!(computer.output_buffer, VecDeque::from(vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]));
    }

    #[test]
    fn it_runs_day09_part1_example_program_2() {
        let mut program = vec![1102,34915192,34915192,7,4,7,99,0];
        program.resize(128, 0);
        let mut computer = IntcodeComputer{ mem: program, ..Default::default() };

        computer.run_program();
        assert_eq!(computer.output_buffer, VecDeque::from(vec![1219070632396864]));
    }

    #[test]
    fn it_runs_day09_part1_example_program_3() {
        let mut program = vec![104,1125899906842624,99];
        program.resize(128, 0);
        let mut computer = IntcodeComputer{ mem: program, ..Default::default() };

        computer.run_program();
        assert_eq!(computer.output_buffer, VecDeque::from(vec![1125899906842624]));
    }
}
