use std::io::{self, Read};

fn build(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn containers_combination_count(containers: &Vec<u32>, total: u32) -> i64 {
    0
}

fn part2(containers: &Vec<u32>) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let containers = build(&input);

    println!("Part 1: {}", containers_combination_count(&containers, 150));
    println!("Part 2: {}", part2(&containers));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(containers_combination_count(&build(INPUT_TEST), 25), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
