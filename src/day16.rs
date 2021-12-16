
use crate::input_const;

pub fn solution1() -> u64 {
    calculate(input_const!("16").trim())
}

pub fn solution2() -> u64 {
    let input = input_const!("16").trim();
    parse_hex_packet(input).eval()
}
fn calculate(s: &str) -> u64 {
    let packet = parse_hex_packet(s);
    dbg!(packet).version_sum()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Packet {
    version: u8,
    type_id: u8,
    kind: PacketKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketKind {
    Literal(String),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Min(Vec<Packet>),
    Max(Vec<Packet>),
    Lt(Vec<Packet>),
    Gt(Vec<Packet>),
    Eq(Vec<Packet>),
}

impl Packet {
    fn version_sum(&self) -> u64 {
        let mut result = self.version as u64;
        for packet in self.sub_packets() {
            result += packet.version_sum();
        }
        result
    }

    fn sub_packets(&self) -> Vec<Packet> {
        use PacketKind::*;
        match &self.kind {
            Literal(_) => vec![],
            Sum(packets) => packets.clone(),
            Product(packets) => packets.clone(),
            Min(packets) => packets.clone(),
            Max(packets) => packets.clone(),
            Lt(packets) => packets.clone(),
            Gt(packets) => packets.clone(),
            Eq(packets) => packets.clone(),
        }
    }

    fn eval(&self) -> u64 {
        use PacketKind::*;
        match &self.kind {
            Literal(s) => u64::from_str_radix(s, 2).unwrap(),
            Sum(packets) => packets.iter().map(Self::eval).sum(),
            Product(packets) => packets.iter().map(Self::eval).product(),
            Min(packets) => packets.iter().map(Self::eval).min().unwrap(),
            Max(packets) => packets.iter().map(Self::eval).max().unwrap(),
            Lt(packets) => if packets[0].eval() < packets[1].eval() { 1 } else { 0 },
            Gt(packets) => if packets[0].eval() > packets[1].eval() { 1 } else { 0 },
            Eq(packets) => if packets[0].eval() == packets[1].eval() { 1 } else { 0 },
        }
    }
}

fn parse_hex_packet(s: &str) -> Packet {
    parse_packet(&hex_to_binary(s)).0
}

fn hex_to_binary(s: &str) -> String {
    let single = |c| match c {
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
        _ => unreachable!(),
    };

    s.chars().map(single).collect()
}

fn parse_packet(s: &str) -> (Packet, &str) {
    let (s, version) = parse_version(s);
    let (s, type_id) = parse_type_id(s);

    let (kind, s) = if type_id == 4 {
        parse_literal(s)
    } else {
        parse_operator(type_id, s)
    };
    (
        Packet {
            version,
            type_id,
            kind,
        },
        s,
    )
}

fn parse_version(s: &str) -> (&str, u8) {
    let (version, rest) = dbg!(s).split_at(3);
    (rest, u8::from_str_radix(version, 2).unwrap())
}

fn parse_type_id(s: &str) -> (&str, u8) {
    let (type_id, rest) = s.split_at(3);
    (rest, u8::from_str_radix(type_id, 2).unwrap())
}

fn parse_literal(mut s: &str) -> (PacketKind, &str) {
    let mut result = String::new();
    loop {
        let (rest, chunk, more_chunks) = parse_literal_chunk(s);
        result.push_str(chunk);
        s = rest;

        if !more_chunks {
            return (PacketKind::Literal(result), rest);
        }
    }
}

fn parse_literal_chunk(s: &str) -> (&str, &str, bool) {
    let (chunk, rest) = s.split_at(5);
    (rest, &chunk[1..], chunk.starts_with("1"))
}

fn parse_operator(type_id: u8, s: &str) -> (PacketKind, &str) {
    let (length_type_id, s) = s.split_at(1);
    let length_type_id = length_type_id == "1";
    let mut packets = vec![];
    let remaining = if length_type_id {
        let (num_packets, mut s) = s.split_at(11);
        let num_packets = usize::from_str_radix(num_packets, 2).unwrap();
        for _ in 0..num_packets {
            let (packet, remaining) = parse_packet(s);
            packets.push(packet);
            s = remaining;
        }

        s
    } else {
        let (num_bits, s) = s.split_at(15);
        let (mut packet_string, s) = s.split_at(usize::from_str_radix(num_bits, 2).unwrap());
        dbg!((num_bits, packet_string));
        loop {
            let (packet, remaining) = parse_packet(packet_string);
            packets.push(packet);
            packet_string = remaining;
            if packet_string.is_empty() {
                break s;
            }
        }
    };

    let kind = match type_id {
        0 => PacketKind::Sum(packets),
        1 => PacketKind::Product(packets),
        2 => PacketKind::Min(packets),
        3 => PacketKind::Max(packets),
        5 => PacketKind::Gt(packets),
        6 => PacketKind::Lt(packets),
        7 => PacketKind::Eq(packets),
        _ => unreachable!(),
    };

    (kind, remaining)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_hex_to_bin() {
        assert_eq!(hex_to_binary("D2FE28"), "110100101111111000101000");
    }

    #[test]
    fn full_example() {
        let s = "110100101111111000101000";
        let (s, version) = parse_version(s);
        assert_eq!((s, version), ("100101111111000101000", 6));
        let (s, type_id) = parse_type_id(s);
        assert_eq!((s, type_id), ("101111111000101000", 4));
        let (kind, _) = parse_literal(s);
        assert_eq!(kind, PacketKind::Literal("011111100101".to_string()));
    }

    #[test]
    fn given_examples() {
        assert_eq!(
            parse_hex_packet("D2FE28"),
            Packet {
                version: 6,
                type_id: 4,
                kind: PacketKind::Literal("011111100101".into()),
            }
        );

        let packet = parse_hex_packet("38006F45291200");
        assert_eq!(packet.version, 1);
        assert_eq!(packet.type_id, 6);

        assert_eq!(calculate("8A004A801A8002F478"), 16);
        assert_eq!(calculate("A0016C880162017C3686B18A3D4780"), 31);

        assert_eq!(parse_hex_packet("9C0141080250320F1802104A08").eval(), 1);
    }
}
