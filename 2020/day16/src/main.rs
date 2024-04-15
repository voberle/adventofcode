use std::io::{self, Read};

use regex::Regex;

#[derive(Debug)]
struct Rule {
    name: String,
    range1: (u32, u32),
    range2: (u32, u32),
}

#[derive(Debug)]
struct Puzzle {
    rules: Vec<Rule>,
    your_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

impl Puzzle {
    fn build(input: &str) -> Self {
        let rule_re = Regex::new(r"(.+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        let mut it = input.lines();

        let mut rules: Vec<Rule> = Vec::new();
        for line in it.by_ref() {
            if line.is_empty() {
                break;
            }
            let parts = rule_re.captures(line).unwrap();
            rules.push(Rule {
                name: parts[1].to_string(),
                range1: (parts[2].parse().unwrap(), parts[3].parse().unwrap()),
                range2: (parts[4].parse().unwrap(), parts[5].parse().unwrap()),
            });
        }

        assert_eq!(it.next().unwrap(), "your ticket:");
        let your_ticket: Vec<u32> = it
            .next()
            .unwrap()
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();
        it.next();

        assert_eq!(it.next().unwrap(), "nearby tickets:");
        let mut nearby_tickets: Vec<Vec<u32>> = Vec::new();
        for line in it.by_ref() {
            if line.is_empty() {
                break;
            }
            nearby_tickets.push(line.split(',').map(|v| v.parse().unwrap()).collect());
        }

        Puzzle {
            rules,
            your_ticket,
            nearby_tickets,
        }
    }
}

fn scanning_error_rate(puzzle: &Puzzle) -> u32 {
    0
}

fn part2(puzzle: &Puzzle) -> u32 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let puzzle = Puzzle::build(&input);
    println!("{:?}", puzzle);

    println!("Part 1: {}", scanning_error_rate(&puzzle));
    println!("Part 2: {}", part2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(scanning_error_rate(&Puzzle::build(INPUT_TEST)), 71);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&Puzzle::build(INPUT_TEST)), 0);
    }
}
