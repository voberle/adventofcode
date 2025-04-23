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

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let readings = build(&input);

    println!("Part 1: {}", composition_sum(&readings));
    println!("Part 2: {}", readings_base10_sum(&readings));
    // println!("Part 3: {}", part3(&values));
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
    fn test_part3() {}
}
