use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Rearrangement {
    count: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Rearrangement {
    fn from(value: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }
        let parts = RE.captures(value).unwrap();
        Self {
            count: parts[1].parse::<usize>().unwrap(),
            // Fix the input to start counting from 0.
            from: parts[2].parse::<usize>().unwrap() - 1,
            to: parts[3].parse::<usize>().unwrap() - 1,
        }
    }
}

impl Rearrangement {
    fn execute_model_9000(&self, crates: &mut [Vec<char>]) {
        for _ in 0..self.count {
            let elt = crates[self.from].pop().expect("Stack is empty");
            crates[self.to].push(elt);
        }
    }

    fn execute_model_9001(&self, crates: &mut [Vec<char>]) {
        // Any better way to do this?
        let mut elts = Vec::new();
        for _ in 0..self.count {
            elts.push(crates[self.from].pop().expect("Stack is empty"));
        }
        crates[self.to].extend(elts.iter().rev());
    }
}

fn build(input: &str) -> (Vec<Vec<char>>, Vec<Rearrangement>) {
    let mut it = input.lines();

    let mut crates: Vec<Vec<char>> = Vec::new();
    for line in it.by_ref() {
        if line.is_empty() {
            break;
        }
        // We convert the line into a vector of chars so we can use the chunk method.
        let v: Vec<char> = line.chars().collect();
        for (stack_index, elt) in v.chunks(4).enumerate() {
            if crates.len() <= stack_index {
                crates.push(Vec::new());
            }
            if elt[0] == '[' {
                let c = elt[1];
                crates[stack_index].push(c);
            }
        }
    }

    for stack in &mut crates {
        stack.reverse();
    }

    let mut rearrangements = Vec::new();
    for line in it {
        rearrangements.push(line.into());
    }

    (crates, rearrangements)
}

fn top_crates(crates: &[Vec<char>], rearrangements: &[Rearrangement], model: u32) -> String {
    let mut crates = crates.to_vec();
    match model {
        9000 => {
            for ra in rearrangements {
                ra.execute_model_9000(&mut crates);
            }
        }
        9001 => {
            for ra in rearrangements {
                ra.execute_model_9001(&mut crates);
            }
        }
        _ => panic!("Unknown CrateMover"),
    }
    // println!("{:?}", crates);

    crates.iter().map(|stack| stack.last().unwrap()).collect()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (crates, rearrangements) = build(&input);
    // println!("{:?}", crates);
    // println!("{:?}", rearrangements);

    println!("Part 1: {}", top_crates(&crates, &rearrangements, 9000));
    println!("Part 2: {}", top_crates(&crates, &rearrangements, 9001));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (crates, rearrangements) = build(&INPUT_TEST);
        assert_eq!(top_crates(&crates, &rearrangements, 9000), "CMZ");
    }

    #[test]
    fn test_part2() {
        let (crates, rearrangements) = build(&INPUT_TEST);
        assert_eq!(top_crates(&crates, &rearrangements, 9001), "MCD");
    }
}
