use std::io::{self, Read};

fn build(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .flat_map(|part| {
                    part.split('-')
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect()
        })
        .collect()
}

fn boxes_count(inventory: &[Vec<u32>]) -> u32 {
    inventory
        .iter()
        .map(|range| (range[1] - range[0] + 1) + (range[3] - range[2] + 1))
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let inventory = build(&input);

    println!("Part 1: {}", boxes_count(&inventory));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let inventory = build(&INPUT_TEST);
        assert_eq!(boxes_count(&inventory), 43);
    }
}
