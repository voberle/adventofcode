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
    index: usize,
    version_sum: u64,
    value: u64,
}

#[allow(clippy::cast_possible_truncation)]
fn parse_packet(bits: &[u8]) -> Result {
    // println!("{}", bits.iter().join(""));
    let mut i = 0;
    let version = get_number(&bits[i..i + 3]);
    // println!("{}", version);
    i += 3;
    let mut version_sum = version;

    let type_id = get_number(&bits[i..i + 3]);
    i += 3;
    // println!("version={}, type_id={}", version, type_id);
    if type_id == 4 {
        // Literal value
        let mut val_bits: Vec<u8> = Vec::new();
        loop {
            let five_bits = &bits[i..i + 5];
            i += 5;
            val_bits.extend(&five_bits[1..]);
            if five_bits[0] == 0 {
                break;
            }
        }
        let literal_value = get_number(&val_bits);
        // println!("Literal value={}", literal_value);
    } else {
        // Operator
        let length_type_id = bits[i];
        i += 1;
        if length_type_id == 0 {
            let total_length_in_bits = get_number(&bits[i..i + 15]) as usize;
            // println!("total_length_in_bits={}", total_length_in_bits);
            i += 15;
            let mut si = 0;
            loop {
                let r = parse_packet(&bits[i + si..i + total_length_in_bits]);
                si += r.index;
                version_sum += r.version_sum;

                if si == total_length_in_bits {
                    break;
                }
            }
            i += si;
        } else {
            assert_eq!(length_type_id, 1);
            let number_sub_packets = get_number(&bits[i..i + 11]);
            // println!("number_sub_packets={}", number_sub_packets);
            i += 11;
            for _ in 0..number_sub_packets {
                let r = parse_packet(&bits[i..]);
                i += r.index;
                version_sum += r.version_sum;
            }
        }
    }
    Result {
        index: i,
        version_sum,
        value: 0,
    }
}

fn version_sum(bits: &[u8]) -> u64 {
    // The outermost layer contains a single packet.
    parse_packet(bits).version_sum
}

fn part2(bits: &[u8]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bits = build(input.trim());

    println!("Part 1: {}", version_sum(&bits));
    println!("Part 2: {}", part2(&bits));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(version_sum(&build("8A004A801A8002F478")), 16);
        assert_eq!(version_sum(&build("620080001611562C8802118E34")), 12);
        assert_eq!(version_sum(&build("C0015000016115A2E0802F182340")), 23);
        assert_eq!(version_sum(&build("A0016C880162017C3686B18A3D4780")), 31);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
