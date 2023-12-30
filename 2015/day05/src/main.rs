use std::io::{self, Read};

fn is_nice(s: &str) -> bool {
    let vowels_count = s.chars().filter(|c| "aeiou".contains(*c)).count();
    // Converting to array of bytes to be able to use windows()
    let has_double_letter = s.as_bytes().windows(2).any(|p| p[0] == p[1]);
    let has_bad_string = ["ab", "cd", "pq", "xy"].iter().any(|i| s.contains(i));
    vowels_count >= 3 && has_double_letter && !has_bad_string
}

fn nice_strings_count(input: &str) -> usize {
    input.lines().filter(|line| is_nice(line)).count()
}

fn part2(input: &str) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", nice_strings_count(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(nice_strings_count(INPUT_TEST), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT_TEST), 0);
    }
}
