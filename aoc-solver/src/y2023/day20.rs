use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::solution::{AocError, Solution};

pub struct Day20;

#[derive(PartialEq, Eq)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Neutral,
}

struct Module {
    kind: ModuleType,
    state: bool,
    memory: HashMap<String, bool>,
    inputs: Vec<String>,
    outputs: Vec<String>,
}

fn parse(input: &str) -> Result<HashMap<String, Module>, AocError> {
    let mut modules: HashMap<String, Module> = input
        .lines()
        .map(|line| {
            let (name, outputs) = line
                .split_once(" -> ")
                .ok_or(AocError::parse(line, "Missing ' -> ' separator"))?;

            let outputs = outputs
                .split(", ")
                .map(|output| output.to_owned())
                .collect();

            let (kind, name) = match name.chars().next() {
                Some('%') => (
                    ModuleType::FlipFlop,
                    name.strip_prefix('%').ok_or(AocError::parse(name, "%"))?,
                ),
                Some('&') => (
                    ModuleType::Conjunction,
                    name.strip_prefix('&').ok_or(AocError::parse(name, "%"))?,
                ),
                _ => (ModuleType::Neutral, name),
            };

            Ok((
                name.to_owned(),
                Module {
                    kind,
                    state: false,           // FlipFlops
                    memory: HashMap::new(), // Conjunctions
                    inputs: Vec::new(),
                    outputs,
                },
            ))
        })
        .try_collect()?;

    let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
    for (name, module) in modules.iter() {
        for output in &module.outputs {
            inputs.entry(output.clone()).or_default().push(name.clone());
        }
    }

    // Fill in the inputs and conjunction memories
    for (name, module) in modules.iter_mut() {
        if let Some(module_inputs) = inputs.get(name) {
            for input in module_inputs {
                module.inputs.push(input.clone());
                if module.kind == ModuleType::Conjunction {
                    module.memory.insert(input.clone(), false);
                }
            }
        }
    }

    Ok(modules)
}

fn press_button(
    button_press: u32,
    modules: &mut HashMap<String, Module>,
    modules_to_find: &mut HashMap<String, Option<u32>>,
) -> (u32, u32) {
    let mut output_buffer =
        VecDeque::from([(String::from("broadcaster"), false, String::from("button"))]);

    let mut lows = 1;
    let mut highs = 0;

    while let Some((current, input, source)) = output_buffer.pop_front() {
        if let Some(module) = modules.get_mut(&current) {
            let output = match module.kind {
                ModuleType::Neutral => Some(input),
                ModuleType::Conjunction => {
                    module.memory.insert(source, input);
                    let output = module.memory.values().any(|value| !value);
                    if output {
                        for (to_find, presses) in modules_to_find.iter_mut() {
                            if current == *to_find {
                                *presses = Some(button_press);
                            }
                        }
                    }
                    Some(output)
                }
                ModuleType::FlipFlop => {
                    if !input {
                        module.state = !module.state;
                        Some(module.state)
                    } else {
                        None
                    }
                }
            };

            if let Some(output) = output {
                match output {
                    true => highs += module.outputs.len() as u32,
                    false => lows += module.outputs.len() as u32,
                }

                for target in &module.outputs {
                    output_buffer.push_back((target.clone(), output, current.to_owned()));
                }
            }
        }
    }

    (lows, highs)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

impl Solution for Day20 {
    type A = u64;
    type B = u64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day20.txt")
    }

    fn part_1(&self, input: &str) -> Result<u64, AocError> {
        let mut modules = parse(input)?;

        let mut lows = 0;
        let mut highs = 0;

        for button_press in 0..1000 {
            let pulses = press_button(button_press, &mut modules, &mut HashMap::new());
            lows += pulses.0;
            highs += pulses.1;
        }

        Ok(lows as u64 * highs as u64)
    }

    fn part_2(&self, input: &str) -> Result<u64, AocError> {
        let mut modules = parse(input)?;

        let (rx, _) = modules
            .iter()
            .find(|(_, module)| module.outputs.contains(&String::from("rx")))
            .ok_or(AocError::logic("Missing rx module"))?;

        let mut current = rx.as_str();
        let mut modules_to_find: HashMap<String, Option<u32>> = HashMap::new();

        // This isn't very generic solution - assume that there's a chain of conjunction
        // modules that produces the value of 'rx' module. Traverse this chain until we
        // see conjunction a module with more than one input. These inputs will trigger
        // periodically, find the durations of these cycles and the answer by determining
        // the LCM of those periodic input cycle durations.
        loop {
            let target_module = modules
                .get(current)
                .ok_or(AocError::logic("Missing module"))?;

            if target_module.kind != ModuleType::Conjunction {
                return Err(AocError::logic("Chain of conjunction must lead to rx"));
            }

            // In this puzzle picking the first module with more than one input already
            // had a period of ~4000 button presses, but if these cycles were longer or
            // more expensive to calculate we could continue deeper and just do more LCMs
            if target_module.inputs.len() > 1 {
                for input in &target_module.inputs {
                    modules_to_find.insert(input.clone(), None);
                }
                break;
            }

            current = target_module.inputs[0].as_str();
        }

        let mut button_press = 1;

        loop {
            press_button(button_press, &mut modules, &mut modules_to_find);

            if modules_to_find.values().all(|value| value.is_some()) {
                let fewest_presses = modules_to_find
                    .values()
                    .map(|value| value.unwrap_or(0))
                    .fold(1, |acc, period| lcm(acc, std::cmp::max(period as u64, 1)));

                return Ok(fewest_presses);
            }

            button_press += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example_1() {
        assert_eq!(
            Day20.part_1(
                "broadcaster -> a, b, c\n\
                 %a -> b\n\
                 %b -> c\n\
                 %c -> inv\n\
                 &inv -> a\n"
            ),
            Ok(32000000)
        );
    }

    #[test]
    fn it_solves_part1_example_2() {
        assert_eq!(
            Day20.part_1(
                "broadcaster -> a\n\
                 %a -> inv, con\n\
                 &inv -> b\n\
                 %b -> con\n\
                 &con -> output\n"
            ),
            Ok(11687500)
        );
    }

    #[test]
    fn it_solves_part2_real() {
        assert_eq!(Day20.part_2(Day20.default_input()), Ok(238920142622879));
    }
}
