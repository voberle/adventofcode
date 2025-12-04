use std::io::{self, Read};

fn build(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
        .collect()
}

// Finds the maximum value in the slice, and returns position of the first occurence and the value.
fn find_max(bank: &[u8]) -> (usize, u8) {
    let mut highest_pos = 0;
    let mut highest_val = 0;
    for (p, v) in bank.iter().enumerate() {
        if *v > highest_val {
            highest_val = *v;
            highest_pos = p;
        }
        if highest_val == 9 {
            // We can't find higher than 9.
            break;
        }
    }
    (highest_pos, highest_val)
}

fn max_joltage<const BATTERIES: usize>(bank: &[u8]) -> u64 {
    let mut max: u64 = 0;

    // Position where we should start searching from.
    // For first digit, it's 0, then for the next ones it's just after the previous biggest digit.
    let mut start = 0;

    for digit in 0..BATTERIES {
        // The current digit cannot be found too close to the end, as we need to leave enough
        // space to find the remaining ones.
        let end = bank.len() - (BATTERIES - digit);

        let (d_relative_pos, highest_digit) = find_max(&bank[start..=end]);

        // Next position to start is just after current max.
        start = start + d_relative_pos + 1;

        max *= 10;
        max += u64::from(highest_digit);
    }

    max
}

fn total_joltage<const BATTERIES: usize>(banks: &[Vec<u8>]) -> u64 {
    banks
        .iter()
        .map(|bank| max_joltage::<BATTERIES>(bank))
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let banks = build(&input);

    println!("Part 1: {}", total_joltage::<2>(&banks));
    println!("Part 2: {}", total_joltage::<12>(&banks));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    fn max_joltage_2_batteries(bank: &[u8]) -> u64 {
        let d1 = *bank[..bank.len() - 1].iter().max().unwrap();
        let d1_pos = bank.iter().position(|v| *v == d1).unwrap();
        let d2 = *bank[d1_pos + 1..].iter().max().unwrap();
        u64::from(d1) * 10 + u64::from(d2)
    }

    fn total_2_batteries(banks: &[Vec<u8>]) -> u64 {
        banks.iter().map(|bank| max_joltage_2_batteries(bank)).sum()
    }

    #[test]
    fn test_part1_old() {
        assert_eq!(total_2_batteries(&build(INPUT_TEST)), 357);
    }

    #[test]
    fn test_part1() {
        assert_eq!(total_joltage::<2>(&build(INPUT_TEST)), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(total_joltage::<12>(&build(INPUT_TEST)), 3121910778619);
    }
}
