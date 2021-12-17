use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::str;
use std::vec::Vec;
use num_enum::TryFromPrimitive;

// TODO: really use Rust's enum correctly by adding the subpackets as values in the enum.
// This is not as easy as it appears because this enum is used as a field in the struct
// creating a type recursion. Adding the packets as references introduces lifetime
// constraints...
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum PacketType {
    Sum,
    Product,
    Min,
    Max,
    Literal,
    Gt,
    Lt,
    Eq,
    Invalid = 255,
}

#[derive(Debug)]
struct Packet {
    version: u8,
    packet_type: PacketType,
    number: u64,
    subpackets: Vec<Packet>,
    bit_length: u32,
}

#[derive(Debug)]
struct PacketDecoder {
    hex: Vec<u8>,
    cur_ofs: usize,
    bin_cache: Vec<u8>,
}

#[derive(Debug, Clone)]
struct PacketDecodeError;

impl fmt::Display for PacketDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid packet")
    }
}

impl PacketDecoder {
    fn new(packets: &str) -> PacketDecoder {
        let hex = packets
            .bytes()
            .collect::<Vec<u8>>()
            .chunks(2)
            .map(|u| u8::from_str_radix(str::from_utf8(u).unwrap(), 16).unwrap())
            // TODO: instead of converting the whole packet string at once we could store the iterator
            .collect();

        PacketDecoder {
            hex: hex,
            cur_ofs: 0,
            bin_cache: Vec::new(),
        }
    }

    fn next_bits(&mut self, n: usize) -> Result<String, PacketDecodeError> {
        while self.bin_cache.len() < n {
            if self.cur_ofs >= self.hex.len() {
                return Err(PacketDecodeError);
            }

            let mut bin = format!("{:08b}", self.hex[self.cur_ofs]).bytes().collect();
            self.cur_ofs += 1;
            self.bin_cache.append(&mut bin);
        }

        let result: String = String::from(str::from_utf8(&self.bin_cache[0..n]).unwrap());
        self.bin_cache.drain(0..n);
        Ok(result)
    }

    fn clear_cache(&mut self) {
        self.bin_cache.clear();
    }

    fn get_u8(&mut self, n: usize) -> u8 {
        // TODO: avoid the byte -> str -> byte conversion
        u8::from_str_radix(&self.next_bits(n).unwrap(), 2).unwrap()
    }

    fn get_u32(&mut self, n: usize) -> u32 {
        // TODO: avoid the byte -> str -> byte conversion
        u32::from_str_radix(&self.next_bits(n).unwrap(), 2).unwrap()
    }
}

fn parse_packet(decoder: &mut PacketDecoder) -> Packet {
    let mut result: Packet = Packet {
        version: 0,
        packet_type: PacketType::Invalid,
        number: 0,
        subpackets: Vec::new(),
        bit_length: 0,
    };

    result.version = decoder.get_u8(3);
    result.packet_type = PacketType::try_from(decoder.get_u8(3)).unwrap();
    result.bit_length += 6;

    match result.packet_type {
        PacketType::Literal => {
            let mut num: String = String::from("");
            while decoder.get_u8(1) == 1 {
                num.push_str(&decoder.next_bits(4).unwrap());
                result.bit_length += 5;
            }
            num.push_str(&decoder.next_bits(4).unwrap());
            result.bit_length += 5;
            result.number = u64::from_str_radix(&num, 2).unwrap();
        }
        _ => {
            let length_type_id = decoder.get_u8(1);
            result.bit_length += 1;
            match length_type_id {
                0 => {
                    let length_bits = decoder.get_u32(15);
                    result.bit_length += 15;
                    let mut bits_consumed: u32 = 0;
                    while bits_consumed < length_bits {
                        let packet = parse_packet(decoder);
                        bits_consumed += packet.bit_length;
                        result.bit_length += packet.bit_length;
                        result.subpackets.push(packet);
                    }
                },
                1 => {
                    let length_subpackets = decoder.get_u32(11);
                    result.bit_length += 11;
                    for _ in 0..length_subpackets {
                        let packet = parse_packet(decoder);
                        result.bit_length += packet.bit_length;
                        result.subpackets.push(packet);
                    }
                },
                _ => panic!("invalid length type id {}", length_type_id)
            }
        }
    }

    result
}

fn calc_version_sum(packet: &Packet) -> usize {
    packet.subpackets.iter().map(|s| calc_version_sum(s)).sum::<usize>() + packet.version as usize
}

fn calc_expression(packet: &Packet) -> u64 {
    match packet.packet_type {
        PacketType::Sum => packet.subpackets.iter().map(|sp| calc_expression(sp)).sum(),
        PacketType::Product => packet.subpackets.iter().map(|sp| calc_expression(sp)).fold(1, |acc, val| acc * val),
        PacketType::Min => packet.subpackets.iter().map(|sp| calc_expression(sp)).min().unwrap(),
        PacketType::Max => packet.subpackets.iter().map(|sp| calc_expression(sp)).max().unwrap(),
        PacketType::Literal => packet.number,
        PacketType::Gt => {
            if packet.subpackets.len() != 2 {
                panic!("gt packet not exactly two subpackets");
            }
            if calc_expression(&packet.subpackets[0]) > calc_expression(&packet.subpackets[1]) {
                1
            } else {
                0
            }
        },
        PacketType::Lt => {
            if packet.subpackets.len() != 2 {
                panic!("lt packet not exactly two subpackets");
            }
            if calc_expression(&packet.subpackets[0]) < calc_expression(&packet.subpackets[1]) {
                1
            } else {
                0
            }
        }
        PacketType::Eq => {
            if packet.subpackets.len() != 2 {
                panic!("lt packet not exactly two subpackets");
            }
            if calc_expression(&packet.subpackets[0]) == calc_expression(&packet.subpackets[1]) {
                1
            } else {
                0
            }
        },
        PacketType::Invalid => panic!("invalid packet type while evaluating expression")
    }
}

fn star_one(lines: &Vec<String>) -> usize {
    let mut decoder = PacketDecoder::new(&lines[0]);
    let packet = parse_packet(&mut decoder);
    decoder.clear_cache();
    calc_version_sum(&packet)
}

fn star_two(lines: &Vec<String>) -> u64 {
    let mut decoder = PacketDecoder::new(&lines[0]);
    let packet = parse_packet(&mut decoder);
    decoder.clear_cache();
    calc_expression(&packet)
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .collect();

    let ans = star_one(&lines);
    println!("Star one: {}", ans);

    let ans = star_two(&lines);
    println!("Star two: {}", ans);
}

#[cfg(test)]
mod tests {
    static NUMBER_TEST_PACKET: &str = "D2FE28";

    #[test]
    fn decode_number() {
        let mut decoder = super::PacketDecoder::new(&NUMBER_TEST_PACKET);
        assert_eq!(decoder.get_u8(3), 6);
        assert_eq!(decoder.get_u8(3), 4);
        assert_eq!(decoder.get_u8(5), 23);
        assert_eq!(decoder.get_u8(5), 30);
        assert_eq!(decoder.get_u8(5), 5);
    }

    #[test]
    fn parse_type_4() {
        let mut decoder = super::PacketDecoder::new(&NUMBER_TEST_PACKET);
        let packet = super::parse_packet(&mut decoder);
        assert_eq!(packet.version, 6);
        assert_eq!(packet.packet_type, super::PacketType::Literal);
        assert_eq!(packet.number, 2021);
    }

    static OPERATOR_TEST_PACKET: &str = "38006F45291200";

    #[test]
    fn parse_operator() {
        let mut decoder = super::PacketDecoder::new(&OPERATOR_TEST_PACKET);
        let packet = super::parse_packet(&mut decoder);
        assert_eq!(packet.version, 1);
        assert_eq!(packet.packet_type, super::PacketType::Lt);
        assert_eq!(packet.subpackets.len(), 2);
        assert_eq!(packet.subpackets[0].packet_type, super::PacketType::Literal);
        assert_eq!(packet.subpackets[0].number, 10);
        assert_eq!(packet.subpackets[1].packet_type, super::PacketType::Literal);
        assert_eq!(packet.subpackets[1].number, 20);
    }
    
    static OTHER_OPERATOR_TEST_PACKET: &str = "EE00D40C823060";

    #[test]
    fn parse_other_operator() {
        let mut decoder = super::PacketDecoder::new(&OTHER_OPERATOR_TEST_PACKET);
        let packet = super::parse_packet(&mut decoder);
        assert_eq!(packet.version, 7);
        assert_eq!(packet.packet_type, super::PacketType::Max);
        assert_eq!(packet.subpackets.len(), 3);
        assert_eq!(packet.subpackets[0].packet_type, super::PacketType::Literal);
        assert_eq!(packet.subpackets[0].number, 1);
        assert_eq!(packet.subpackets[1].packet_type, super::PacketType::Literal);
        assert_eq!(packet.subpackets[1].number, 2);
        assert_eq!(packet.subpackets[2].packet_type, super::PacketType::Literal);
        assert_eq!(packet.subpackets[2].number, 3);
    }

    static NESTED_OPERATOR_TEST_PACKET: &str = "8A004A801A8002F478";

    #[test]
    fn parse_nested_operator() {
        let mut decoder = super::PacketDecoder::new(&NESTED_OPERATOR_TEST_PACKET);
        let packet = super::parse_packet(&mut decoder);
        assert_eq!(packet.version, 4);
        assert_eq!(packet.subpackets.len(), 1);
        assert_eq!(packet.subpackets[0].version, 1);
        assert_eq!(packet.subpackets[0].subpackets.len(), 1);
        assert_eq!(packet.subpackets[0].subpackets[0].version, 5);
        assert_eq!(packet.subpackets[0].subpackets[0].subpackets.len(), 1);
        assert_eq!(packet.subpackets[0].subpackets[0].subpackets[0].version, 6);
        assert_eq!(super::calc_version_sum(&packet), 16);
    }
}
