use std::collections::HashMap;

use itertools::Itertools;

use crate::solution::{AocError, Solution};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Gate {
    And,
    Or,
    Xor,
}

type Gates<'a> = Vec<(Gate, &'a str, &'a str, &'a str)>;
type Registers<'a> = HashMap<&'a str, Option<u8>>;

fn parse(input: &'_ str) -> Result<(Gates<'_>, Registers<'_>), AocError> {
    let (inputs_str, gates_str) = input
        .split_once("\n\n")
        .ok_or_else(|| AocError::parse(input, "Missing sections"))?;

    let mut registers: HashMap<&str, Option<u8>> = HashMap::new();

    for line in inputs_str.lines() {
        let (register, value_input) = line
            .split_once(": ")
            .ok_or_else(|| AocError::parse(line, "Invalid initial value"))?;
        let value = value_input
            .parse::<u8>()
            .map_err(|err| AocError::parse(value_input, err))?;

        registers.insert(register, Some(value));
    }

    let mut gates: Vec<(Gate, &str, &str, &str)> = Vec::new();

    for line in gates_str.lines() {
        let (inputs, output) = line
            .split_once(" -> ")
            .ok_or_else(|| AocError::parse(line, "Invalid gate"))?;

        let (a, gate, b) = inputs
            .split_ascii_whitespace()
            .collect_tuple()
            .ok_or_else(|| AocError::parse(inputs, "Invalid gate inputs"))?;

        let gate = match gate {
            "AND" => Gate::And,
            "OR" => Gate::Or,
            "XOR" => Gate::Xor,
            unknown => return Err(AocError::parse(unknown, "Unsupported gate")),
        };

        registers.entry(a).or_insert(None);
        registers.entry(b).or_insert(None);
        registers.entry(output).or_insert(None);

        gates.push((gate, a, b, output));
    }

    Ok((gates, registers))
}

fn read_output(registers: HashMap<&str, Option<u8>>) -> u64 {
    let mut bits = registers
        .into_iter()
        .filter(|(name, _value)| name.starts_with("z"))
        .collect::<Vec<_>>();

    bits.sort_by(|a, b| a.0.cmp(b.0));
    bits.iter().enumerate().fold(0u64, |acc, (i, &(_, bit))| {
        acc | ((bit.unwrap_or(0) as u64) << i)
    })
}

fn check_next(gates: &Gates, a: &&str, b: &&str, output: &&str, expected: &[Gate]) -> bool {
    let next_gates: Vec<_> = gates
        .iter()
        .filter(|(_, next_a, next_b, _)| {
            (next_a != a && next_b != b) && (output == next_a || output == next_b)
        })
        .collect();

    if next_gates.len() != expected.len() {
        return false;
    }

    expected.iter().all(|expected_gate| {
        next_gates
            .iter()
            .filter(|(gate, _, _, _)| gate == expected_gate)
            .count()
            == 1
    })
}

pub struct Day24;
impl Solution for Day24 {
    type Part1 = u64;
    type Part2 = String;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day24.txt")
    }

    fn part_1(&self, input: &str) -> Result<u64, AocError> {
        let (gates, mut registers) = parse(input)?;

        while registers
            .iter()
            .any(|(name, value)| name.starts_with('z') && value.is_none())
        {
            for (gate, a, b, output) in gates.iter() {
                if let (Some(a), Some(b), None) = (registers[a], registers[b], registers[output]) {
                    match gate {
                        Gate::And => *registers.entry(output).or_default() = Some(a & b),
                        Gate::Or => *registers.entry(output).or_default() = Some(a | b),
                        Gate::Xor => *registers.entry(output).or_default() = Some(a ^ b),
                    }
                }
            }
        }

        Ok(read_output(registers))
    }

    fn part_2(&self, input: &str) -> Result<String, AocError> {
        let (gates, _) = parse(input)?;

        // Lets assume this is a perfect Ripple-Carry full adder with no useless gates.
        // After drawing the circuit on paper and inspecting it the following logic emerged:

        let last_bit = gates
            .iter()
            .filter_map(|(_, _, _, output)| output.starts_with("z").then_some(output))
            .max()
            .cloned()
            .ok_or_else(|| AocError::logic("Missing last bit"))?;

        let mut incorrect: Vec<_> = gates
            .iter()
            .filter(|(gate, a, b, output)| {
                let is_first_bit = *a == "x00" && *b == "y00" || *a == "y00" && *b == "x00";
                let is_last_bit = *output == last_bit;
                let is_x_y = (a.starts_with("x") && b.starts_with("y"))
                    || (a.starts_with("y") && b.starts_with("x"));

                match gate {
                    // Z outputs can only be from XOR gates, except from OR gate at the last bit
                    _ if output.starts_with("z") && is_last_bit => *gate != Gate::Or,
                    _ if output.starts_with("z") => *gate != Gate::Xor,
                    // XOR connected to X and Y has to be connected to AND and XOR gates
                    Gate::Xor if is_x_y => {
                        !check_next(&gates, a, b, output, &[Gate::And, Gate::Xor])
                    }
                    // AND has to be connected to OR gate, except to AND and XOR at the first bit with no C_in
                    Gate::And if is_first_bit => {
                        !check_next(&gates, a, b, output, &[Gate::And, Gate::Xor])
                    }
                    Gate::And => !check_next(&gates, a, b, output, &[Gate::Or]),
                    // OR has to be connected to AND and XOR gates
                    Gate::Or => !check_next(&gates, a, b, output, &[Gate::And, Gate::Xor]),
                    // Any other gates are invalid
                    _ => true,
                }
            })
            .map(|(_, _, _, output)| *output)
            .collect();

        incorrect.sort();

        Ok(incorrect.join(","))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example_1() {
        assert_eq!(
            Day24.part_1(
                "x00: 1\n\
                 x01: 1\n\
                 x02: 1\n\
                 y00: 0\n\
                 y01: 1\n\
                 y02: 0\n\
                 \n\
                 x00 AND y00 -> z00\n\
                 x01 XOR y01 -> z01\n\
                 x02 OR y02 -> z02"
            ),
            Ok(4)
        );
    }

    #[test]
    fn it_solves_part1_example_2() {
        assert_eq!(
            Day24.part_1(
                "x00: 1\n\
                 x01: 0\n\
                 x02: 1\n\
                 x03: 1\n\
                 x04: 0\n\
                 y00: 1\n\
                 y01: 1\n\
                 y02: 1\n\
                 y03: 1\n\
                 y04: 1\n\
                 \n\
                 ntg XOR fgs -> mjb\n\
                 y02 OR x01 -> tnw\n\
                 kwq OR kpj -> z05\n\
                 x00 OR x03 -> fst\n\
                 tgd XOR rvg -> z01\n\
                 vdt OR tnw -> bfw\n\
                 bfw AND frj -> z10\n\
                 ffh OR nrd -> bqk\n\
                 y00 AND y03 -> djm\n\
                 y03 OR y00 -> psh\n\
                 bqk OR frj -> z08\n\
                 tnw OR fst -> frj\n\
                 gnj AND tgd -> z11\n\
                 bfw XOR mjb -> z00\n\
                 x03 OR x00 -> vdt\n\
                 gnj AND wpb -> z02\n\
                 x04 AND y00 -> kjc\n\
                 djm OR pbm -> qhw\n\
                 nrd AND vdt -> hwm\n\
                 kjc AND fst -> rvg\n\
                 y04 OR y02 -> fgs\n\
                 y01 AND x02 -> pbm\n\
                 ntg OR kjc -> kwq\n\
                 psh XOR fgs -> tgd\n\
                 qhw XOR tgd -> z09\n\
                 pbm OR djm -> kpj\n\
                 x03 XOR y03 -> ffh\n\
                 x00 XOR y04 -> ntg\n\
                 bfw OR bqk -> z06\n\
                 nrd XOR fgs -> wpb\n\
                 frj XOR qhw -> z04\n\
                 bqk OR frj -> z07\n\
                 y03 OR x01 -> nrd\n\
                 hwm AND bqk -> z03\n\
                 tgd XOR rvg -> z12\n\
                 tnw OR pbm -> gnj"
            ),
            Ok(2024)
        );
    }

    #[test]
    fn it_solves_part2_real() {
        assert_eq!(
            Day24.part_2(Day24.default_input()),
            Ok(String::from("drg,gvw,jbp,jgc,qjb,z15,z22,z35"))
        )
    }
}
