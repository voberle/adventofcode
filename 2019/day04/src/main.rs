use std::io::{self, Read};

fn build(input: &str) -> (u32, u32) {
    let p: Vec<u32> = input.split('-').map(|v| v.parse().unwrap()).collect();
    (p[0], p[1])
}

// From 2018/day14
fn get_digits(n: u32) -> Vec<u32> {
    fn inner(n: u32, xs: &mut Vec<u32>) {
        if n >= 10 {
            inner(n / 10, xs);
        }
        xs.push(n % 10);
    }
    let mut xs = Vec::new();
    inner(n, &mut xs);
    xs
}

fn has_adjacent_digits(digits: &[u32]) -> bool {
    digits.windows(2).any(|w| w[0] == w[1])
}

fn digits_dont_decrease(digits: &[u32]) -> bool {
    digits.windows(2).all(|w| w[0] <= w[1])
}

// Returns an array with the number of consecutive digits. E.g. 123444 becomes 1,1,1,3.
fn repeat_digits(digits: &[u32]) -> Vec<usize> {
    let mut repeat_occs: Vec<usize> = vec![1];
    for i in 1..digits.len() {
        if digits[i - 1] == digits[i] {
            *repeat_occs.last_mut().unwrap() += 1;
        } else {
            repeat_occs.push(1);
        }
    }
    repeat_occs
}

fn extra_rule_on_adjacents(digits: &[u32]) -> bool {
    repeat_digits(digits).contains(&2)
}

fn password_counts(min: u32, max: u32) -> usize {
    (min..=max)
        .map(get_digits)
        .filter(|n| has_adjacent_digits(n) && digits_dont_decrease(n))
        .count()
}

fn password_counts_extra_rule(min: u32, max: u32) -> usize {
    (min..=max)
        .map(get_digits)
        .filter(|n| extra_rule_on_adjacents(n) && digits_dont_decrease(n))
        .count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (min, max) = build(&input);

    println!("Part 1: {}", password_counts(min, max));
    println!("Part 2: {}", password_counts_extra_rule(min, max));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_adjacent_digits() {
        assert!(has_adjacent_digits(&get_digits(111111)));
        assert!(has_adjacent_digits(&get_digits(223450)));
        assert!(!has_adjacent_digits(&get_digits(123789)));
    }

    #[test]
    fn test_digits_dont_decrease() {
        assert!(digits_dont_decrease(&get_digits(111111)));
        assert!(!digits_dont_decrease(&get_digits(223450)));
        assert!(digits_dont_decrease(&get_digits(123789)));
    }

    #[test]
    fn test_extra_rule_on_adjacent_digits() {
        assert!(extra_rule_on_adjacents(&get_digits(112233)));
        assert!(!extra_rule_on_adjacents(&get_digits(123444)));
        assert!(extra_rule_on_adjacents(&get_digits(111122)));
    }
}
