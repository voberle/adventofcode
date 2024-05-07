use std::io::{self, Read};

fn hex_to_bits(c: char) -> [u8; 4] {
    match c {
        '0' => [0, 0, 0, 0],
        '1' => [0, 0, 0, 1],
        '2' => [0, 0, 1, 0],
        '3' => [0, 0, 1, 1],
        '4' => [0, 1, 0, 0],
        '5' => [0, 1, 0, 1],
        '6' => [0, 1, 1, 0],
        '7' => [0, 1, 1, 1],
        '8' => [1, 0, 0, 0],
        '9' => [1, 0, 0, 1],
        'A' => [1, 0, 1, 0],
        'B' => [1, 0, 1, 1],
        'C' => [1, 1, 0, 0],
        'D' => [1, 1, 0, 1],
        'E' => [1, 1, 1, 0],
        'F' => [1, 1, 1, 1],
        _ => panic!("Invalid char"),
    }
}

fn build(input: &str) -> Vec<u8> {
    input.chars().flat_map(hex_to_bits).collect()
}

fn get_number(bits: &[u8]) -> u64 {
    bits.iter().fold(0, |acc, v| (acc << 1) + u64::from(*v))
}

// Result of the parsing of a packet.
struct Result {
    i: usize,
    version_sum: u64,
    value: u64,
}

macro_rules! next_bits {
    ($bits:expr, $i:expr, $len:literal) => {{
        let slice = &$bits[$i..$i + $len];
        $i += $len;
        slice
    }};
}

#[allow(clippy::cast_possible_truncation)]
fn parse_packet(bits: &[u8]) -> Result {
    let mut r = Result {
        i: 0,
        version_sum: 0,
        value: 0,
    };

    let version = get_number(next_bits!(bits, r.i, 3));
    r.version_sum = version;

    let type_id = get_number(next_bits!(bits, r.i, 3));

    if type_id == 4 {
        // Literal value
        let mut val_bits: Vec<u8> = Vec::new();
        loop {
            let five_bits = next_bits!(bits, r.i, 5);

            val_bits.extend(&five_bits[1..]);
            if five_bits[0] == 0 {
                break;
            }
        }
        r.value = get_number(&val_bits);
    } else {
        // Operator
        let length_type_id = next_bits!(bits, r.i, 1)[0];

        let mut sub_packet_values: Vec<u64> = Vec::new();

        if length_type_id == 0 {
            let total_length_in_bits = get_number(next_bits!(bits, r.i, 15)) as usize;

            let mut si = 0;
            loop {
                let sub_result = parse_packet(&bits[r.i + si..]);

                si += sub_result.i;
                r.version_sum += sub_result.version_sum;
                sub_packet_values.push(sub_result.value);

                if si == total_length_in_bits {
                    break;
                }
            }
            r.i += si;
        } else {
            assert_eq!(length_type_id, 1);
            let number_sub_packets = get_number(next_bits!(bits, r.i, 11));

            for _ in 0..number_sub_packets {
                let sub_result = parse_packet(&bits[r.i..]);

                r.i += sub_result.i;
                r.version_sum += sub_result.version_sum;
                sub_packet_values.push(sub_result.value);
            }
        }

        r.value = match type_id {
            0 => sub_packet_values.iter().sum(),
            1 => sub_packet_values.iter().product(),
            2 => *sub_packet_values.iter().min().unwrap(),
            3 => *sub_packet_values.iter().max().unwrap(),
            5 => bool::into(sub_packet_values[0] > sub_packet_values[1]),
            6 => bool::into(sub_packet_values[0] < sub_packet_values[1]),
            7 => bool::into(sub_packet_values[0] == sub_packet_values[1]),
            _ => panic!("Unknown type ID"),
        };
    }
    r
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bits = build(input.trim());

    // The outermost layer contains a single packet.
    let res = parse_packet(&bits);

    println!("Part 1: {}", res.version_sum);
    println!("Part 2: {}", res.value);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn version_sum(bits: &[u8]) -> u64 {
        parse_packet(bits).version_sum
    }

    fn eval(bits: &[u8]) -> u64 {
        parse_packet(bits).value
    }

    #[test]
    fn test_part1() {
        assert_eq!(version_sum(&build("8A004A801A8002F478")), 16);
        assert_eq!(version_sum(&build("620080001611562C8802118E34")), 12);
        assert_eq!(version_sum(&build("C0015000016115A2E0802F182340")), 23);
        assert_eq!(version_sum(&build("A0016C880162017C3686B18A3D4780")), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(eval(&build("C200B40A82")), 3);
        assert_eq!(eval(&build("04005AC33890")), 54);
        assert_eq!(eval(&build("880086C3E88112")), 7);
        assert_eq!(eval(&build("CE00C43D881120")), 9);
        assert_eq!(eval(&build("D8005AC2A8F0")), 1);
        assert_eq!(eval(&build("F600BC2D8F")), 0);
        assert_eq!(eval(&build("9C005AC2F8F0")), 0);
        assert_eq!(eval(&build("9C0141080250320F1802104A08")), 1);
    }
}
