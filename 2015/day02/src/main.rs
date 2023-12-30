use std::io::{self, Read};

fn build_dimensions(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| line.split('x').map(|c| c.parse().unwrap()).collect())
        .collect()
}

fn part1(input: &str) -> u64 {
    let dims = build_dimensions(input);
    dims.iter()
        .map(|b| -> u64 {
            // Is there a more elegant way to do this one?
            let sides = [b[0] * b[1], b[1] * b[2], b[0] * b[2]];
            let min = sides.iter().min().unwrap();
            sides.iter().sum::<u64>() * 2 + min
        })
        .sum::<u64>()
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
        assert_eq!(part1(INPUT_TEST), 58 + 43);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT_TEST), 0);
    }
}
