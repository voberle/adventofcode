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

fn password_counts(min: u32, max: u32) -> usize {
    (min..=max)
        .map(get_digits)
        .filter(|n| has_adjacent_digits(n) && digits_dont_decrease(n))
        .count()
}

fn part2(min: u32, max: u32) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (min, max) = build(&input);

    println!("Part 1: {}", password_counts(min, max));
    println!("Part 2: {}", part2(min, max));
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
}
