use std::io::{self, Read};

fn build(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn captcha(seq_digits: &[u32]) -> u32 {
    assert!(seq_digits.len() > 1);
    seq_digits
        .windows(2)
        .filter_map(|v| if v[0] == v[1] { Some(v[0]) } else { None })
        .sum::<u32>()
        // handling last digit case that way, to avoid doing a copy of the slice
        + if seq_digits[0] == *seq_digits.last().unwrap() {
            seq_digits[0]
        } else {
            0
        }
}

const fn wrapping_index(i: usize, len: usize) -> usize {
    (i % len + len) % len
}

fn captcha2(seq_digits: &[u32]) -> u32 {
    assert_eq!(seq_digits.len() % 2, 0);
    let mid = seq_digits.len() / 2;
    seq_digits
        .iter()
        .enumerate()
        .filter_map(|(i, d)| {
            if *d == seq_digits[wrapping_index(i + mid, seq_digits.len())] {
                Some(d)
            } else {
                None
            }
        })
        .sum::<u32>()
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
