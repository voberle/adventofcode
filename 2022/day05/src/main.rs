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

fn execute(crates: &mut [Vec<char>], rearrangements: &[Rearrangement]) {
    for ra in rearrangements {
        for _ in 0..ra.count {
            let elt = crates[ra.from].pop().expect("Stack is empty");
            crates[ra.to].push(elt);
        }
    }
}

fn top_crates_at_end(crates: &[Vec<char>], rearrangements: &[Rearrangement]) -> String {
    let mut crates = crates.to_vec();
    execute(&mut crates, rearrangements);
    // println!("{:?}", crates);

    crates.iter().map(|stack| stack.last().unwrap()).collect()
}

fn part2(crates: &[Vec<char>], rearrangements: &[Rearrangement]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (crates, rearrangements) = build(&input);
    // println!("{:?}", crates);
    // println!("{:?}", rearrangements);

    println!("Part 1: {}", top_crates_at_end(&crates, &rearrangements));
    println!("Part 2: {}", part2(&crates, &rearrangements));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (crates, rearrangements) = build(&INPUT_TEST);
        assert_eq!(top_crates_at_end(&crates, &rearrangements), "CMZ");
    }

    #[test]
    fn test_part2() {
        let (crates, rearrangements) = build(&INPUT_TEST);
        assert_eq!(part2(&crates, &rearrangements), 0);
    }
}
