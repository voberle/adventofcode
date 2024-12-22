use std::io::{self, Read};

fn build(input: &str) -> Vec<u64> {
    input.lines().map(|n| n.parse().unwrap()).collect()
}

fn mix(secret_number: u64, value: u64) -> u64 {
    secret_number ^ value
}

#[allow(clippy::unreadable_literal)]
fn prune(secret_number: u64) -> u64 {
    secret_number % 16777216
}

fn gen_next_secret_number(mut secret_number: u64) -> u64 {
    let val = secret_number * 64;
    secret_number = mix(secret_number, val);
    secret_number = prune(secret_number);

    let val = secret_number / 32;
    secret_number = mix(secret_number, val);
    secret_number = prune(secret_number);

    let val = secret_number * 2048;
    secret_number = mix(secret_number, val);
    secret_number = prune(secret_number);

    secret_number
}

fn gen_many_numbers(initial_secret_number: u64, count: usize) -> u64 {
    (0..count).fold(initial_secret_number, |acc, _| gen_next_secret_number(acc))
}

fn sum_2000th_nb(initial_secret_numbers: &[u64]) -> u64 {
    initial_secret_numbers
        .iter()
        .map(|&secret_number| gen_many_numbers(secret_number, 2000))
        .sum()
}

fn part2(initial_secret_numbers: &[u64]) -> u64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input_parsed = build(&input);

    println!("Part 1: {}", sum_2000th_nb(&input_parsed));
    println!("Part 2: {}", part2(&input_parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_mix() {
        assert_eq!(mix(42, 15), 37);
    }

    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920);
    }

    #[test]
    fn test_gen_next_secret_number() {
        assert_eq!(gen_next_secret_number(123), 15887950);
        assert_eq!(gen_next_secret_number(15887950), 16495136);
        assert_eq!(gen_next_secret_number(16495136), 527345);
    }

    #[test]
    fn test_gen_many_numbers() {
        assert_eq!(gen_many_numbers(1, 2000), 8685429);
        assert_eq!(gen_many_numbers(2024, 2000), 8667524);
    }

    #[test]
    fn test_part1() {
        assert_eq!(sum_2000th_nb(&build(INPUT_TEST)), 37327623);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
