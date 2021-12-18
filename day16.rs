use crate::aoc;

pub fn run() {
    println!("AOC 2021 - Day 16");

    assert_eq!("110100101111111000101000", parse_transmission("D2FE28"));
    assert_eq!("00111000000000000110111101000101001010010001001000000000", parse_transmission("38006F45291200"));
    assert_eq!("11101110000000001101010000001100100000100011000001100000", parse_transmission("EE00D40C823060"));

    let sample_1 = get_outer_packet("D2FE28");
    assert_eq!(6, sample_1.version);
    assert_eq!(4, sample_1.type_id);
    assert_eq!(2021, sample_1.literal_value.unwrap());

    let sample_2 = get_outer_packet("38006F45291200");
    assert_eq!(1, sample_2.version);
    assert_eq!(6, sample_2.type_id);
    assert_eq!(2, sample_2.subpackets.len());

    let sample_3 = get_outer_packet("EE00D40C823060");
    assert_eq!(7, sample_3.version);
    assert_eq!(3, sample_3.type_id);
    assert_eq!(3, sample_3.subpackets.len());

    let real_input = aoc::read_input("input/day16.txt");
    decode_transmission(&real_input);
}

fn decode_transmission(input: &[String]) {
    let outer_packet = get_outer_packet(&input[0]);
    println!("part 1 = {}", outer_packet.calculate_version_sum());
    println!("part 2 = {}", outer_packet.calculate_value());
}

fn get_outer_packet(input: &str) -> Packet {
    let mut tx = parse_transmission(input);
    parse_packet(&mut tx)
}

fn parse_transmission(input: &str) -> String {
    input.chars().map(|c| hex_to_bin(c)).collect()
}

fn parse_packet(tx: &mut String) -> Packet {
    let version = parse_i8(chomp_at_the_bits(tx, 3));
    let type_id = parse_i8(chomp_at_the_bits(tx, 3));

    let mut packet = Packet::new(version, type_id);

    if type_id == 4 {
        // literal value packet
        let mut binary_value = String::new();
        loop {
            let group = chomp_at_the_bits(tx, 5);
            binary_value.push_str(&group[1..]);
            if group[0..1] == *"0" {
                break;
            }
        }
        packet.literal_value = Some(parse_i64(binary_value));
    } else {
        // operator packet
        let mut subpackets: Vec<Packet> = Vec::new();
        let mode = chomp_at_the_bits(tx, 1);
        if mode == "0" {
            let bit_len = parse_usize(chomp_at_the_bits(tx, 15));
            let mut subpacket_bits = chomp_at_the_bits(tx, bit_len);
            while subpacket_bits.len() > 0 {
                let subpacket = parse_packet(&mut subpacket_bits);
                subpackets.push(subpacket);
            }
        } else {
            let num_sub_packets = parse_i32(chomp_at_the_bits(tx, 11));
            for _ in 0..num_sub_packets {
                let subpacket = parse_packet(tx);
                subpackets.push(subpacket);
            }
        }
        packet.subpackets = subpackets;
    }

    packet
}

fn chomp_at_the_bits(tx: &mut String, len: usize) -> String {
    tx.drain(..len).collect()
}

fn parse_usize(binary: String) -> usize {
    usize::from_str_radix(&binary, 2).unwrap()
}

fn parse_i64(binary: String) -> i64 {
    i64::from_str_radix(&binary, 2).unwrap()
}

fn parse_i32(binary: String) -> i32 {
    i32::from_str_radix(&binary, 2).unwrap()
}

fn parse_i8(binary: String) -> i8 {
    i8::from_str_radix(&binary, 2).unwrap()
}

fn hex_to_bin(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

#[derive(Debug)]
struct Packet {
    version: i8,
    type_id: i8,
    literal_value: Option<i64>,
    subpackets: Vec<Packet>
}

impl Packet {
    fn new(version: i8, type_id: i8) -> Packet {
        Packet {
            version: version,
            type_id: type_id,
            literal_value: None,
            subpackets: Vec::new()
        }
    }

    fn calculate_version_sum(&self) -> i32 {
        self.version as i32 + self.subpackets.iter().fold(0, |sum, subpacket| sum + subpacket.calculate_version_sum())
    }

    fn calculate_value(&self) -> i64 {
        let subpacket_values = self.subpackets.iter().map(|p| p.calculate_value());
        match self.type_id {
            0 => subpacket_values.sum(),
            1 => subpacket_values.product(),
            2 => subpacket_values.min().unwrap(),
            3 => subpacket_values.max().unwrap(),
            4 => self.literal_value.unwrap(),
            5 => if self.subpackets[0].calculate_value() > self.subpackets[1].calculate_value() { 1 } else { 0 }
            6 => if self.subpackets[0].calculate_value() < self.subpackets[1].calculate_value() { 1 } else { 0 },
            7 => if self.subpackets[0].calculate_value() == self.subpackets[1].calculate_value() { 1 } else { 0 },
            _ => 0
        }
    }
}