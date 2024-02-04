use std::io::{self, Read};

use fxhash::FxHashSet;

fn build(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn resulting_frequency(changes: &[i32]) -> i32 {
    changes.iter().sum()
}

fn first_frequency_twice(changes: &[i32]) -> i32 {
    let mut seen: FxHashSet<i32> = FxHashSet::default();
    let mut f = 0;
    let mut i: usize = 0;
    while !seen.contains(&f) {
        seen.insert(f);
        f += changes[i];
        i = (i + 1).rem_euclid(changes.len());
    }
    f
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let changes = build(&input);

    println!("Part 1: {}", resulting_frequency(&changes));
    println!("Part 2: {}", first_frequency_twice(&changes));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_test(input: &str) -> Vec<i32> {
        build(&input.replace(", ", "\n"))
    }

    #[test]
    fn test_part1() {
        assert_eq!(resulting_frequency(&build_test("+1, +1, +1")), 3);
        assert_eq!(resulting_frequency(&build_test("+1, +1, -2")), 0);
        assert_eq!(resulting_frequency(&build_test("-1, -2, -3")), -6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(first_frequency_twice(&build_test("+1, -1")), 0);
        assert_eq!(first_frequency_twice(&build_test("+3, +3, +4, -2, -4")), 10);
        assert_eq!(first_frequency_twice(&build_test("-6, +3, +8, +5, -6")), 5);
        assert_eq!(first_frequency_twice(&build_test("+7, +7, -2, -7, -4")), 14);
    }
}
