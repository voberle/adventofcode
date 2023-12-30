use std::io::{self, Read};

fn part1(input: &str) -> i64 {
    0
}

fn part2(input: &str) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT_TEST), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT_TEST), 0);
    }
}
