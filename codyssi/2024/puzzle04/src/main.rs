use std::{
    collections::HashSet,
    io::{self, Read},
};

fn build(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|line| line.split(" <-> ").map(ToString::to_string).collect())
        .collect()
}

fn unique_locations_count(paths: &[Vec<String>]) -> usize {
    paths.iter().flatten().collect::<HashSet<&String>>().len()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let paths = build(&input);

    println!("Part 1: {}", unique_locations_count(&paths));
    // println!("Part 2: {}", part2(&paths));
    // println!("Part 3: {}", part3(&paths));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(unique_locations_count(&build(INPUT_TEST)), 7);
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
