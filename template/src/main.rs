use std::io::{self, Read};

fn build(input: &str) -> String {
    input.to_string()
}

fn part1(input: &String) -> i64 {
    0
}

fn part2(input: &String) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input_parsed = build(&input);

    println!("Part 1: {}", part1(&input_parsed));
    println!("Part 2: {}", part2(&input_parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(part1(&build(INPUT_TEST)), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
