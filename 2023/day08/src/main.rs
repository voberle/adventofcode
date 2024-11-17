use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};

mod brute_force;

fn build(input: &str) -> (String, HashMap<String, (String, String)>) {
    let (instructions, nodes) = input.split("\n\n").collect_tuple().unwrap();

    let node_re = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
    let network: HashMap<String, (String, String)> = nodes
        .lines()
        .map(|line| {
            let captures = node_re.captures(line).unwrap();
            (
                captures[1].to_owned(),
                (captures[2].to_owned(), captures[3].to_owned()),
            )
        })
        .collect();
    (instructions.to_string(), network)
}

fn part1(instructions: &str, network: &HashMap<String, (String, String)>) -> u64 {
    let mut total_steps = 0;
    let mut key = "AAA";
    for i in instructions.chars().cycle() {
        let current_node = network.get(key).unwrap();
        total_steps += 1;

        key = if i == 'L' {
            &current_node.0
        } else {
            &current_node.1
        };
        if key == "ZZZ" {
            break;
        }
    }
    total_steps
}

fn part2(instructions: &str, network: &HashMap<String, (String, String)>) -> u64 {
    let keys: Vec<String> = network
        .keys()
        .filter(|k| k.ends_with('A'))
        .cloned()
        .collect();
    let steps: Vec<u64> = keys
        .iter()
        .map(|k| {
            let mut steps_count = 0;
            let mut key = k;
            for i in instructions.chars().cycle() {
                let current_node = network.get(key).unwrap();
                steps_count += 1;

                key = if i == 'L' {
                    &current_node.0
                } else {
                    &current_node.1
                };
                if key.ends_with('Z') {
                    break;
                }
            }
            steps_count
        })
        .collect();

    // I don't know why it's so..
    steps.iter().fold(1, |n, i| num_integer::lcm(n, *i))
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (instructions, network) = build(&input);

    println!("Part 1: {}", part1(&instructions, &network));
    println!("Part 2: {}", part2(&instructions, &network));

    // println!("Part 2: {}", brute_force::part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");
    const INPUT_TEST_3: &str = include_str!("../resources/input_test_3");

    #[test]
    fn test_part1() {
        let (instructions, network) = build(INPUT_TEST_1);
        assert_eq!(part1(&instructions, &network), 2);

        let (instructions, network) = build(INPUT_TEST_2);
        assert_eq!(part1(&instructions, &network), 6);
    }

    #[test]
    fn test_part2() {
        let (instructions, network) = build(INPUT_TEST_3);
        assert_eq!(part2(&instructions, &network), 6);
    }
}
