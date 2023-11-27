use nom::{
    bits::complete::{tag, take},
    branch::alt,
    multi::length_count,
    sequence::{terminated, tuple},
    IResult,
};

use std::num::ParseIntError;

use crate::solution::{AocError, Solution};

pub struct Day16;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    LiteraValue {
        packet_version: u8,
        packet_type: u8,
        value: u64,
    },
    Operator {
        packet_version: u8,
        packet_type: u8,
        len_type: u8,
        sub_packets: Vec<Packet>,
    },
}

type BitStream<'a> = (&'a [u8], usize);

pub fn decode_hex_input(input: &str) -> Result<Vec<u8>, ParseIntError> {
    let line = input.lines().next().unwrap();

    (0..line.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&input[i..i + 2], 16))
        .collect()
}

fn parse_packet(input: BitStream) -> IResult<BitStream, Packet> {
    alt((
        parse_literal_value,
        parse_operator_11_bits,
        parse_operator_15_bits,
    ))(input)
}

fn parse_literal_value(input: BitStream) -> IResult<BitStream, Packet> {
    let (mut unhandled, version) = terminated(take(3usize), tag(0b100, 3usize))(input)?;

    let mut value = 0u64;
    loop {
        let (remaining, prefix): (BitStream, u8) = take(1usize)(unhandled)?;
        let (remaining, bits): (BitStream, u8) = take(4usize)(remaining)?;

        unhandled = remaining;

        value <<= 4;
        value |= bits as u64;

        if prefix == 0 {
            break;
        }
    }

    let packet = Packet::LiteraValue {
        packet_version: version,
        packet_type: 0b100u8,
        value,
    };

    Ok((unhandled, packet))
}

fn parse_operator_15_bits(input: BitStream) -> IResult<BitStream, Packet> {
    let (mut unhandled, parsed): (BitStream, (u8, u8, u8, u32)) =
        tuple((take(3usize), take(3usize), tag(0, 1usize), take(15usize)))(input)?;

    // This would've been nice to do with nom::multi::length_value,
    // but I couldn't get it working with bits.
    let mut consumed_bits = 0;
    let mut sub_packets = vec![];

    while consumed_bits < parsed.3 as usize {
        let previous_bits = unhandled.0.len() * 8 - unhandled.1;
        let (remaining, sub_packet) = parse_packet(unhandled)?;
        let remaining_bits = remaining.0.len() * 8 - remaining.1;

        unhandled = remaining;
        sub_packets.push(sub_packet);

        consumed_bits += previous_bits - remaining_bits;
    }

    Ok((
        unhandled,
        Packet::Operator {
            packet_version: parsed.0,
            packet_type: parsed.1,
            len_type: parsed.2,
            sub_packets,
        },
    ))
}

fn parse_operator_11_bits(input: BitStream) -> IResult<BitStream, Packet> {
    let (unhandled, parsed): (BitStream, (u8, u8, u8, Vec<Packet>)) = tuple((
        take(3usize),
        take(3usize),
        tag(1, 1usize),
        length_count(take::<_, u32, _, _>(11usize), parse_packet),
    ))(input)?;

    Ok((
        unhandled,
        Packet::Operator {
            packet_version: parsed.0,
            packet_type: parsed.1,
            len_type: parsed.2,
            sub_packets: parsed.3,
        },
    ))
}

fn version_sum(packet: &Packet) -> u64 {
    let mut sum: u64 = 0;

    match packet {
        Packet::LiteraValue { packet_version, .. } => {
            sum += *packet_version as u64;
        }
        Packet::Operator {
            packet_version,
            sub_packets,
            ..
        } => {
            sum += *packet_version as u64;
            sum += sub_packets.iter().map(version_sum).sum::<u64>();
        }
    }

    sum
}

fn packet_value(packet: &Packet) -> u64 {
    match packet {
        Packet::LiteraValue { value, .. } => *value,
        Packet::Operator {
            packet_type,
            sub_packets,
            ..
        } => {
            let mut values = sub_packets.iter().map(packet_value);

            match packet_type {
                0 => {
                    // Packets with type ID 0 are sum packets
                    values.sum()
                }
                1 => {
                    // Packets with type ID 1 are product packets
                    values.product()
                }
                2 => {
                    // Packets with type ID 2 are minimum packets
                    values.min().unwrap()
                }
                3 => {
                    // Packets with type ID 3 are maximum packets
                    values.max().unwrap()
                }
                5 => {
                    // Packets with type ID 5 are greater than packets
                    if values.next().unwrap() > values.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    // Packets with type ID 6 are less than packets
                    if values.next().unwrap() < values.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    // Packets with type ID 7 are equal to packets
                    if values.next().unwrap() == values.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

impl Solution for Day16 {
    type F = u64;
    type S = u64;

    fn meta(&self) -> (u32, u32) {
        (16, 2021)
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2021/day16.txt")
    }

    fn part_1(&self, input: &str) -> Result<Self::F, AocError> {
        let bit_stream = decode_hex_input(input).unwrap();
        let (_remaining, packet) = parse_packet((&bit_stream, 0)).unwrap();

        Ok(version_sum(&packet))
    }

    fn part_2(&self, input: &str) -> Result<Self::S, AocError> {
        let bit_stream = decode_hex_input(input).unwrap();
        let (_remaining, packet) = parse_packet((&bit_stream, 0)).unwrap();

        Ok(packet_value(&packet))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_literal_value() {
        let bit_stream = &decode_hex_input("D2FE28").unwrap();
        let (_, parsed) = parse_literal_value((&bit_stream, 0)).unwrap();
        assert_eq!(
            parsed,
            Packet::LiteraValue {
                packet_version: 6,
                packet_type: 4,
                value: 2021
            }
        );
    }

    #[test]
    fn it_parses_operator_15_bits() {
        let bit_stream = &decode_hex_input("38006F45291200").unwrap();
        let (_, parsed) = parse_operator_15_bits((&bit_stream, 0)).unwrap();
        assert_eq!(
            parsed,
            Packet::Operator {
                packet_version: 1,
                packet_type: 6,
                len_type: 0,
                sub_packets: vec![
                    Packet::LiteraValue {
                        packet_version: 6,
                        packet_type: 4,
                        value: 10
                    },
                    Packet::LiteraValue {
                        packet_version: 2,
                        packet_type: 4,
                        value: 20
                    }
                ],
            }
        )
    }

    #[test]
    fn it_parses_operator_11_bits() {
        let bit_stream = &decode_hex_input("EE00D40C823060").unwrap();
        let (_, parsed) = parse_operator_11_bits((&bit_stream, 0)).unwrap();
        assert_eq!(
            parsed,
            Packet::Operator {
                packet_version: 7,
                packet_type: 3,
                len_type: 1,
                sub_packets: vec![
                    Packet::LiteraValue {
                        packet_version: 2,
                        packet_type: 4,
                        value: 1
                    },
                    Packet::LiteraValue {
                        packet_version: 4,
                        packet_type: 4,
                        value: 2
                    },
                    Packet::LiteraValue {
                        packet_version: 1,
                        packet_type: 4,
                        value: 3
                    }
                ],
            }
        );
    }

    #[test]
    fn it_solves_part1_examples() {
        assert_eq!(Day16.part_1("8A004A801A8002F478"), Ok(16));
        assert_eq!(Day16.part_1("620080001611562C8802118E34"), Ok(12));
        assert_eq!(Day16.part_1("C0015000016115A2E0802F182340"), Ok(23));
        assert_eq!(Day16.part_1("A0016C880162017C3686B18A3D4780"), Ok(31));
    }

    #[test]
    fn it_solves_part2_examples() {
        assert_eq!(Day16.part_2("C200B40A82"), Ok(3));
        assert_eq!(Day16.part_2("04005AC33890"), Ok(54));
        assert_eq!(Day16.part_2("880086C3E88112"), Ok(7));
        assert_eq!(Day16.part_2("CE00C43D881120"), Ok(9));
        assert_eq!(Day16.part_2("D8005AC2A8F0"), Ok(1));
        assert_eq!(Day16.part_2("F600BC2D8F"), Ok(0));
        assert_eq!(Day16.part_2("9C005AC2F8F0"), Ok(0));
        assert_eq!(Day16.part_2("9C0141080250320F1802104A08"), Ok(1));
    }
}
