// https://adventofcode.com/2023/day/8
// Part 1 test 1: 2
// Part 1 test 2: 6

use regex::Regex;
use std::{collections::HashMap, io};

fn main() {
    let stdin = io::stdin();

    let mut n = String::new();
    stdin.read_line(&mut n).unwrap();
    let instructions = n.trim();
    //println!("Instructions: {instructions}");

    let node_re = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
    let mut network: HashMap<String, (String, String)> = HashMap::new();
    for line in stdin.lines() {
        let s = line.unwrap();
        if s.is_empty() {
            continue;
        } // skip empty line
        let captures = node_re.captures(&s).unwrap();
        network.insert(
            captures[1].to_owned(),
            (captures[2].to_owned(), captures[3].to_owned()),
        );
    }

    // Part 1
    {
        let mut total_steps = 0;
        let mut key = "AAA";
        for i in instructions.chars().cycle() {
            //println!("'{i}' {key}");
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
        println!("Part 1: {}", total_steps);
    }

    // Part 2
    {
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
                    //println!("'{i}' {key}");
                    let current_node = network.get(key).unwrap();
                    steps_count += 1;

                    key = if i == 'L' {
                        &current_node.0
                    } else {
                        &current_node.1
                    };
                    if key.ends_with("Z") {
                        break;
                    }
                }
                steps_count
            })
            .collect();

        // I don't know why it's so..
        let total_steps: u64 = steps.iter().fold(1, |n, i| num_integer::lcm(n, *i));
        println!("Part 2: {}", total_steps);
    }
}
