// https://adventofcode.com/2023/day/8
// Part 1 test 1: 2
// Part 1 test 2: 6
// Part 1: 12643
// Part 2 test 3: 6
// Part 2: 

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

    let mut total_steps = 0;
    let mut key = "AAA";
    'outer: loop {
        for i in instructions.chars() {
            //println!("'{i}' {key}");
            let current_node = network.get(key).unwrap();
            total_steps += 1;

            key = if i == 'L' {
                &current_node.0
            } else {
                &current_node.1
            };
            if key == "ZZZ" {
                break 'outer;
            }
        }
    }

    println!("Part 1: {}", total_steps);
}
