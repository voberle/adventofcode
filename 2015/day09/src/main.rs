use std::{
    collections::HashMap,
    io::{self, Read},
};

use regex::Regex;

type Location = String;
type Graph = HashMap<Location, Vec<(Location, u32)>>;

fn build(input: &str) -> Graph {
    let mut graph = HashMap::new();
    let re = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    input.lines().for_each(|line| {
        let caps = re.captures(line).unwrap();
        graph
            .entry(caps[1].to_string())
            .or_insert_with(Vec::new)
            .push((caps[2].to_string(), caps[3].parse().unwrap()))
    });
    graph
}

fn part1(graph: &Graph) -> u32 {
    0
}

fn part2(graph: &Graph) -> u32 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let graph = build(&input);
    // println!("{:#?}", graph);
    println!("Part 1: {}", part1(&graph));
    println!("Part 2: {}", part2(&graph));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(part1(&build(INPUT_TEST)), 605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
