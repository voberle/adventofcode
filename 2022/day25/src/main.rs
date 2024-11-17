use std::io::{self, Read};

fn build(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn digit_snafu_to_decimal(c: char) -> i64 {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!("Unknown char"),
    }
}

fn digit_decimal_to_snafu(d: i64) -> char {
    match d {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        _ => panic!("Unknown digit"),
    }
}

#[allow(clippy::cast_sign_loss)]
fn snafu_to_decimal(snafu: &str) -> i64 {
    let decimal = snafu
        .chars()
        .rev()
        .map(digit_snafu_to_decimal)
        .enumerate()
        .fold(0_i64, |acc, (i, d)| {
            acc + 5_i64.pow(u32::try_from(i).unwrap()) * d
        });
    assert!(decimal >= 0);
    decimal
}

fn decimal_to_snafu(decimal: i64) -> String {
    let mut decimal = decimal;
    let mut snafu = Vec::new();
    while decimal != 0 {
        let d = (decimal + 2) % 5;
        snafu.push(digit_decimal_to_snafu(d - 2));
        decimal = (decimal + 2) / 5;
    }
    snafu.iter().rev().collect()
}

// Variant with iterator.
// Not that much easier to read, and still needs an intermediary vector.
fn _decimal_to_snafu(decimal: i64) -> String {
    let digits =
        std::iter::successors(
            Some(decimal),
            |&d| {
                if d == 0 {
                    None
                } else {
                    Some((d + 2) / 5)
                }
            },
        )
        .take_while(|&d| d != 0)
        .map(|d| digit_decimal_to_snafu((d + 2) % 5 - 2))
        .collect::<Vec<_>>();

    digits.into_iter().rev().collect()
}

fn snafu_sum(snafu_list: &[&str]) -> String {
    let sum_decimal = snafu_list.iter().map(|s| snafu_to_decimal(s)).sum();
    decimal_to_snafu(sum_decimal)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let snafu_list = build(&input);

    println!("Part 1: {}", snafu_sum(&snafu_list));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    const EXAMPLES: [(&str, i64); 28] = [
        ("1", 1),
        ("2", 2),
        ("1=", 3),
        ("1-", 4),
        ("10", 5),
        ("11", 6),
        ("12", 7),
        ("2=", 8),
        ("2-", 9),
        ("20", 10),
        ("1=0", 15),
        ("1-0", 20),
        ("1=11-2", 2022),
        ("1-0---0", 12345),
        ("1121-1110-1=0", 314159265),
        ("1=-0-2", 1747),
        ("12111", 906),
        ("2=0=", 198),
        ("21", 11),
        ("2=01", 201),
        ("111", 31),
        ("20012", 1257),
        ("112", 32),
        ("1=-1=", 353),
        ("1-12", 107),
        ("12", 7),
        ("1=", 3),
        ("122", 37),
    ];

    #[test]
    fn test_snafu_to_decimal() {
        println!("SNAFU\tDecimal");
        for (snafu, decimal) in EXAMPLES {
            println!("{snafu}\t{decimal}");
            assert_eq!(snafu_to_decimal(snafu), decimal);
        }
    }

    #[test]
    fn test_decimal_to_snafu() {
        println!("Decimal\tSNAFU");
        for (snafu, decimal) in EXAMPLES {
            println!("{snafu}\t{decimal}");
            assert_eq!(decimal_to_snafu(decimal), snafu);
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(snafu_sum(&build(INPUT_TEST)), "2=-1=0");
    }
}
