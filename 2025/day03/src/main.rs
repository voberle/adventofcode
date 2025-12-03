use std::io::{self, Read};

fn build(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
        .collect()
}

fn max_joltage_2_batteries(bank: &[u8]) -> u64 {
    // How to do this with only one iterator?
    let d1 = *bank[..bank.len() - 1].iter().max().unwrap();
    let d1_pos = bank.iter().position(|v| *v == d1).unwrap();

    let d2 = *bank[d1_pos + 1..].iter().max().unwrap();

    u64::from(d1) * 10 + u64::from(d2)
}

fn total_2_batteries(banks: &[Vec<u8>]) -> u64 {
    banks.iter().map(|bank| max_joltage_2_batteries(bank)).sum()
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

        let highest_digit = *bank[start..=end].iter().max().unwrap();

        // Update next position to start: Just after current max.
        let d_relative_pos = bank[start..=end]
            .iter()
            .position(|v| *v == highest_digit)
            .unwrap();
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

    println!("Part 1: {}", total_2_batteries(&banks));
    println!("Part 2: {}", total_joltage::<12>(&banks));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(total_2_batteries(&build(INPUT_TEST)), 357);
    }

    #[test]
    fn test_part1_part2code() {
        assert_eq!(total_joltage::<2>(&build(INPUT_TEST)), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(total_joltage::<12>(&build(INPUT_TEST)), 3121910778619);
    }
}
