use std::io::{self, Read};

use itertools::Itertools;

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

fn get_price(secret_number: u64) -> u64 {
    secret_number % 10
}

fn gen_many_numbers(initial_secret_number: u64, count: usize) -> u64 {
    // Better than my initial:
    // (0..count).fold(initial_secret_number, |acc, _| gen_next_secret_number(acc))
    std::iter::successors(Some(initial_secret_number), |&n| {
        Some(gen_next_secret_number(n))
    })
    .nth(count)
    .unwrap()
}

const SECRET_NB_COUNT: usize = 2000;

fn sum_2000th_nb(initial_secret_numbers: &[u64]) -> u64 {
    initial_secret_numbers
        .iter()
        .map(|&secret_number| gen_many_numbers(secret_number, SECRET_NB_COUNT))
        .sum()
}

fn generate_secret_numbers_list(initial_secret_number: u64) -> Vec<u64> {
    std::iter::successors(Some(initial_secret_number), |&n| {
        Some(gen_next_secret_number(n))
    })
    .take(SECRET_NB_COUNT)
    .collect()
}

fn generate_price_differences_list(secret_numbers: &[u64]) -> Vec<i8> {
    secret_numbers
        .windows(2)
        .map(|w| i8::try_from(get_price(w[1])).unwrap() - i8::try_from(get_price(w[0])).unwrap())
        .collect()
}

fn get_price_after_change(
    secret_numbers: &[u64],
    price_differences: &[i8],
    change_sequence: &[i8],
) -> Option<u64> {
    price_differences
        .windows(change_sequence.len())
        .position(|window| window == change_sequence)
        .map(|pos| get_price(secret_numbers[pos + 4]))
}

// Brute-forced.
fn max_bananas(initial_secret_numbers: &[u64]) -> u64 {
    let all_secret_numbers = initial_secret_numbers
        .iter()
        .map(|&initial| generate_secret_numbers_list(initial))
        .collect_vec();
    let all_price_differences = all_secret_numbers
        .iter()
        .map(|secret_numbers| generate_price_differences_list(secret_numbers))
        .collect_vec();

    // Gives us all change combinations. We find then the one that is the biggest.
    itertools::repeat_n(-9_i8..9, 4)
        .multi_cartesian_product()
        .map(|change_sequence| {
            all_secret_numbers
                .iter()
                .zip(all_price_differences.iter())
                .map(|(secret_numbers, price_differences)| {
                    get_price_after_change(secret_numbers, price_differences, &change_sequence)
                        .unwrap_or_default()
                })
                .sum()
        })
        .max()
        .unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input_parsed = build(&input);

    println!("Part 1: {}", sum_2000th_nb(&input_parsed));
    println!("Part 2: {}", max_bananas(&input_parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

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
    fn test_get_price() {
        assert_eq!(get_price(123), 3);
        assert_eq!(get_price(15887950), 0);
    }

    #[test]
    fn test_get_price_after_change() {
        let change_sequence = [-2, 1, -1, 3];
        let test = |initial_secret_number| {
            let secret_numbers = generate_secret_numbers_list(initial_secret_number);
            let price_differences = generate_price_differences_list(&secret_numbers);
            get_price_after_change(&secret_numbers, &price_differences, &change_sequence)
        };
        assert_eq!(test(1), Some(7));
        assert_eq!(test(2), Some(7));
        assert_eq!(test(3), None);
        assert_eq!(test(2024), Some(9));
    }

    #[test]
    fn test_part1() {
        assert_eq!(sum_2000th_nb(&build(INPUT_TEST_1)), 37327623);
    }

    #[test]
    fn test_part2() {
        assert_eq!(max_bananas(&build(INPUT_TEST_2)), 23);
    }
}
