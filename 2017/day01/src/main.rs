use std::io::{self, Read};

fn build(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn captcha(seq_digits: &[u32]) -> u32 {
    let mut extended = seq_digits.to_owned();
    extended.push(seq_digits[0]);
    extended
        .windows(2)
        .filter(|v| v[0] == v[1])
        .map(|v| v[0])
        .sum()
}

fn captcha2(seq_digits: &[u32]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let seq_digits = build(&input);

    println!("Part 1: {}", captcha(&seq_digits));
    println!("Part 2: {}", captcha2(&seq_digits));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_captcha() {
        assert_eq!(captcha(&build("1122")), 3);
        assert_eq!(captcha(&build("1111")), 4);
        assert_eq!(captcha(&build("1234")), 0);
        assert_eq!(captcha(&build("91212129")), 9);
    }

    #[test]
    fn test_captcha2() {
        assert_eq!(captcha2(&build("1212")), 6);
        assert_eq!(captcha2(&build("1221")), 0);
        assert_eq!(captcha2(&build("123425")), 4);
        assert_eq!(captcha2(&build("123123")), 12);
        assert_eq!(captcha2(&build("12131415")), 4);
    }
}
