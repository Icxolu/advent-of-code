use super::*;
use bitvec::prelude::*;

#[derive(Debug, PartialEq, Eq)]
enum TypeId {
    Literal(u64),
    Operator {
        kind: Operation,
        content: Vec<Packet>,
    },
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl TryFrom<u8> for Operation {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Sum),
            1 => Ok(Self::Product),
            2 => Ok(Self::Minimum),
            3 => Ok(Self::Maximum),
            5 => Ok(Self::GreaterThan),
            6 => Ok(Self::LessThan),
            7 => Ok(Self::EqualTo),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Packet {
    version: u8,
    type_id: TypeId,
}

impl Packet {
    fn sum_versions(&self) -> u32 {
        self.version as u32
            + match self.type_id {
                TypeId::Literal(_) => 0,
                TypeId::Operator { ref content, .. } => content
                    .iter()
                    .fold(0, |sum, packet| sum + packet.sum_versions()),
            }
    }

    fn get_value(&self) -> u64 {
        match self.type_id {
            TypeId::Literal(v) => v,
            TypeId::Operator {
                ref kind,
                ref content,
            } => match kind {
                Operation::Sum => content.iter().map(|p| p.get_value()).sum(),
                Operation::Product => content.iter().map(|p| p.get_value()).product(),
                Operation::Minimum => content.iter().map(|p| p.get_value()).min().unwrap(),
                Operation::Maximum => content.iter().map(|p| p.get_value()).max().unwrap(),
                Operation::GreaterThan => {
                    if content[0].get_value() > content[1].get_value() {
                        1
                    } else {
                        0
                    }
                }
                Operation::LessThan => {
                    if content[0].get_value() < content[1].get_value() {
                        1
                    } else {
                        0
                    }
                }
                Operation::EqualTo => {
                    if content[0].get_value() == content[1].get_value() {
                        1
                    } else {
                        0
                    }
                }
            },
        }
    }
}

fn get_bytes(input: &str) -> Result<Vec<u8>, nom::Err<()>> {
    use nom::{bytes::complete::take, multi::many1, Parser};

    let mut parser = many1(take(2usize).map(|v| u8::from_str_radix(v, 16).unwrap()));
    let (_, bytes) = parser.parse(input)?;

    Ok(bytes)
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Result<Packet, nom::Err<()>> {
    let bytes = get_bytes(input)?;
    let bits = bytes.view_bits::<Msb0>();
    let (packet, _) = parse_packet(bits);
    Ok(packet)
}

fn parse_header(input: &BitSlice<Msb0, u8>) -> (u8, u8, &BitSlice<Msb0, u8>) {
    let (version, input) = input.split_at(3);
    let (id, input) = input.split_at(3);

    (version.load_be(), id.load_be(), input)
}

fn parse_literal(input: &BitSlice<Msb0, u8>) -> (u64, &BitSlice<Msb0, u8>) {
    let n = input
        .iter()
        .by_ref()
        .step_by(5)
        .enumerate()
        .skip_while(|(_, i)| **i)
        .map(|(i, _)| i + 1)
        .next()
        .unwrap();
    let (contents, input) = input.split_at(5 * n);
    let literal = BitVec::<Msb0, u8>::from_iter(
        contents
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 5 != 0)
            .map(|(_, b)| b),
    );

    (literal.load_be(), input)
}

fn parse_operator(input: &BitSlice<Msb0, u8>) -> (Vec<Packet>, &BitSlice<Msb0, u8>) {
    let (len_id, input) = input.split_first().unwrap();

    if *len_id {
        let (num_packets, input) = input.split_at(11);
        let num_packets = num_packets.load_be::<u16>();
        let (packets, input) = (0..num_packets).fold((vec![], input), |(mut out, input), _| {
            let (packet, input) = parse_packet(input);
            out.push(packet);
            (out, input)
        });
        (packets, input)
    } else {
        let (num_bits, input) = input.split_at(15);
        let num_bits = num_bits.load_be();
        let (mut bits, input) = input.split_at(num_bits);

        let mut packets = vec![];
        let packets = loop {
            let (packet, b) = parse_packet(bits);
            bits = b;
            packets.push(packet);
            if bits.is_empty() {
                break packets;
            }
        };

        (packets, input)
    }
}

fn parse_packet(input: &BitSlice<Msb0, u8>) -> (Packet, &BitSlice<Msb0, u8>) {
    let (version, id, input) = parse_header(input);

    match id {
        4 => {
            let (literal, input) = parse_literal(input);
            let packet = Packet {
                version,
                type_id: TypeId::Literal(literal),
            };
            (packet, input)
        }
        id => {
            let (packets, input) = parse_operator(input);
            let packet = Packet {
                version,
                type_id: TypeId::Operator {
                    kind: id.try_into().unwrap(),
                    content: packets,
                },
            };
            (packet, input)
        }
    }
}

#[aoc(day16, part1)]
pub fn solve_part1(packet: &Packet) -> u32 {
    packet.sum_versions()
}

#[aoc(day16, part2)]
pub fn solve_part2(packet: &Packet) -> u64 {
    packet.get_value()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_literal_packet() {
        let bytes = get_bytes("D2FE28").unwrap();
        let (packet, _) = parse_packet(bytes.view_bits());
        assert_eq!(
            packet,
            Packet {
                version: 6,
                type_id: TypeId::Literal(2021)
            }
        );
    }

    #[test]
    fn test_parse_operator_packet_type0() {
        let bytes = get_bytes("38006F45291200").unwrap();
        let (packet, _) = parse_packet(bytes.view_bits());
        assert_eq!(
            packet,
            Packet {
                version: 1,
                type_id: TypeId::Operator {
                    kind: Operation::LessThan,
                    content: vec![
                        Packet {
                            version: 6,
                            type_id: TypeId::Literal(10)
                        },
                        Packet {
                            version: 2,
                            type_id: TypeId::Literal(20)
                        }
                    ]
                }
            }
        );
    }

    #[test]
    fn test_parse_operator_packet_type1() {
        let bytes = get_bytes("EE00D40C823060").unwrap();
        let (packet, _) = parse_packet(bytes.view_bits());
        assert_eq!(
            packet,
            Packet {
                version: 7,
                type_id: TypeId::Operator {
                    kind: Operation::Maximum,
                    content: vec![
                        Packet {
                            version: 2,
                            type_id: TypeId::Literal(1)
                        },
                        Packet {
                            version: 4,
                            type_id: TypeId::Literal(2)
                        },
                        Packet {
                            version: 1,
                            type_id: TypeId::Literal(3)
                        }
                    ]
                }
            }
        );
    }

    #[test]
    fn test_parse_packet() {
        let bytes = get_bytes("8A004A801A8002F478").unwrap();
        let (packet, _) = parse_packet(bytes.view_bits());
        assert_eq!(packet.sum_versions(), 16);

        let bytes = get_bytes("620080001611562C8802118E34").unwrap();
        let (packet, _) = parse_packet(bytes.view_bits());
        assert_eq!(packet.sum_versions(), 12);

        let bytes = get_bytes("C0015000016115A2E0802F182340").unwrap();
        let (packet, _) = parse_packet(bytes.view_bits());
        assert_eq!(packet.sum_versions(), 23);

        let bytes = get_bytes("A0016C880162017C3686B18A3D4780").unwrap();
        let (packet, _) = parse_packet(bytes.view_bits());
        assert_eq!(packet.sum_versions(), 31)
    }

    #[test]
    fn test_sum_packet() {
        let bytes = get_bytes("C200B40A82").unwrap();
        let (packet, _) = parse_packet(bytes.view_bits());

        assert_eq!(packet.get_value(), 3)
    }

    #[test]
    fn test_product_packet() {
        let bytes = get_bytes("04005AC33890").unwrap();
        let (packet, _) = parse_packet(bytes.view_bits());

        assert_eq!(packet.get_value(), 54)
    }

    #[test]
    fn test_minimum_packet() {
        let bytes = get_bytes("880086C3E88112").unwrap();
        let (packet, _) = parse_packet(bytes.view_bits());

        assert_eq!(packet.get_value(), 7)
    }

    #[test]
    fn test_maximum_packet() {
        let bytes = get_bytes("CE00C43D881120").unwrap();
        let (packet, _) = parse_packet(bytes.view_bits());

        assert_eq!(packet.get_value(), 9)
    }

    #[test]
    fn test_less_than_packet() {
        let bytes = get_bytes("D8005AC2A8F0").unwrap();
        let (packet, _) = parse_packet(bytes.view_bits());

        assert_eq!(packet.get_value(), 1)
    }

    #[test]
    fn test_greater_than_packet() {
        let bytes = get_bytes("F600BC2D8F").unwrap();
        let (packet, _) = parse_packet(bytes.view_bits());

        assert_eq!(packet.get_value(), 0)
    }

    #[test]
    fn test_equals_to_packet() {
        let bytes = get_bytes("9C005AC2F8F0").unwrap();
        let (packet, _) = parse_packet(bytes.view_bits());

        assert_eq!(packet.get_value(), 0)
    }

    #[test]
    fn test_complex_packet() {
        let bytes = get_bytes("9C0141080250320F1802104A08").unwrap();
        let (packet, _) = parse_packet(bytes.view_bits());

        assert_eq!(packet.get_value(), 1)
    }
}
