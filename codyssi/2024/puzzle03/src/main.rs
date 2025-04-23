use std::io::{self, Read};

#[derive(Clone)]
struct Reading {
    reading: String,
    base: u32,
}

impl Reading {
    fn build(line: &str) -> Self {
        let parts: Vec<_> = line.split_whitespace().collect();
        Self {
            reading: parts[0].to_string(),
            base: parts[1].parse().unwrap(),
        }
    }

    fn value(&self) -> u64 {
        u64::from_str_radix(&self.reading, self.base).unwrap()
    }
}

fn build(input: &str) -> Vec<Reading> {
    input.lines().map(Reading::build).collect()
}

fn composition_sum(readings: &[Reading]) -> u32 {
    readings.iter().map(|r| r.base).sum()
}

fn readings_base10_sum(readings: &[Reading]) -> u64 {
    readings.iter().map(Reading::value).sum()
}

fn to_base65(n: u64) -> String {
    #[rustfmt::skip]
    const CHARS: [char; 65] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        '!', '@', '#', 
    ];

    // We use a Vec instead of a String, as we need to reverse it at the end.
    let mut result = Vec::new();

    let mut n = n;
    while n != 0 {
        let rem = n % 65;
        result.push(CHARS[rem as usize]);
        n /= 65;
    }
    result.reverse();

    result.iter().collect()
}

fn readings_base65_sum(readings: &[Reading]) -> String {
    let sum = readings_base10_sum(readings);
    to_base65(sum)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let readings = build(&input);

    println!("Part 1: {}", composition_sum(&readings));
    println!("Part 2: {}", readings_base10_sum(&readings));
    println!("Part 3: {}", readings_base65_sum(&readings));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(composition_sum(&build(INPUT_TEST)), 78);
    }

    #[test]
    fn test_part2() {
        assert_eq!(readings_base10_sum(&build(INPUT_TEST)), 3487996082);
    }

    #[test]
    fn test_part3() {
        assert_eq!(readings_base65_sum(&build(INPUT_TEST)), "30PzDC");
    }
}
