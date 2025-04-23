use std::io::{self, Read};

fn build(input: &str) -> Vec<bool> {
    input.lines().map(|line| line == "TRUE").collect()
}

fn part1(values: &[bool]) -> usize {
    values
        .iter()
        .enumerate()
        .filter_map(|(id, val)| if *val { Some(id + 1) } else { None })
        .sum()
}

fn part2(values: &[u64]) -> u64 {
    0
}

#[allow(clippy::cast_possible_wrap)]
fn part3(values: &[u64]) -> u64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let values = build(&input);

    println!("Part 1: {}", part1(&values));
    // println!("Part 2: {}", part2(&values));
    // println!("Part 3: {}", part3(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(part1(&build(INPUT_TEST)), 19);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(&build(INPUT_TEST)), );
    }

    #[test]
    fn test_part3() {
        // assert_eq!(part3(&build(INPUT_TEST)), );
    }
}
