use std::io::{self, Read};

fn dragon_curve(a: &str) -> String {
    let b: String = a
        .as_bytes()
        .iter()
        .rev()
        .map(|&c| if c == b'0' { '1' } else { '0' })
        .collect();
    a.to_string() + "0" + &b
}

fn generate_enough_data(initial_state: &str, length: usize) -> String {
    let mut a = initial_state.to_string();
    while a.len() < length {
        a = dragon_curve(&a);
    }
    a.truncate(length);
    a
}

fn calc_checksum(s: &str) -> String {
    let mut check = s.to_string();
    // Do while checksum length is even
    loop {
        check = check
            .as_bytes()
            .chunks(2)
            .map(|v| if v[0] == v[1] { '1' } else { '0' })
            .collect();

        if check.len() % 2 == 1 {
            break;
        }
    }
    check
}

fn checksum(initial_state: &str, length: usize) -> String {
    calc_checksum(&generate_enough_data(initial_state, length))
}

fn main() {
    let mut initial_state = String::new();
    io::stdin().read_to_string(&mut initial_state).unwrap();

    println!("Part 1: {}", checksum(&initial_state, 272));
    println!("Part 2: {}", checksum(&initial_state, 35_651_584));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dragon_curve() {
        assert_eq!(dragon_curve("1"), "100");
        assert_eq!(dragon_curve("0"), "001");
        assert_eq!(dragon_curve("11111"), "11111000000");
        assert_eq!(dragon_curve("111100001010"), "1111000010100101011110000");
    }

    #[test]
    fn test_generate_enough_data() {
        assert_eq!(generate_enough_data("10000", 20), "10000011110010000111");
    }

    #[test]
    fn test_calc_checksum() {
        assert_eq!(calc_checksum("110010110100"), "100");
    }

    #[test]
    fn test_part1() {
        assert_eq!(checksum("10000", 20), "01100");
    }
}
