use std::io::{self, Read};

use regex::Regex;

#[derive(Debug)]
struct Rule {
    name: String,
    range1: (u32, u32),
    range2: (u32, u32),
}

impl Rule {
    fn in_range(&self, val: u32) -> bool {
        (self.range1.0..=self.range1.1).contains(&val)
            || (self.range2.0..=self.range2.1).contains(&val)
    }
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

impl Puzzle {
    fn val_is_invalid(&self, val: u32) -> bool {
        self.rules.iter().all(|rule| !rule.in_range(val))
    }
}

fn scanning_error_rate(puzzle: &Puzzle) -> u32 {
    puzzle
        .nearby_tickets
        .iter()
        .flatten()
        .filter(|&&val| puzzle.val_is_invalid(val))
        .sum()
}

fn find_correct_order(puzzle: &Puzzle) -> Vec<usize> {
    // Discard tickets with invalid values.
    let valid_tickets: Vec<&Vec<u32>> = puzzle
        .nearby_tickets
        .iter()
        .filter(|ticket| !ticket.iter().any(|&val| puzzle.val_is_invalid(val)))
        .collect();

    // In this vector, index is the position if the field.
    // Items are the indexes of the words (aka rule) that may work.
    let mut possible_words: Vec<Vec<usize>> = vec![Vec::new(); puzzle.rules.len()];

    // Find which word work on each position.
    for pos in 0..puzzle.rules.len() {
        for (rule_idx, rule) in puzzle.rules.iter().enumerate() {
            if valid_tickets
                .iter()
                .all(|ticket| rule.in_range(ticket[pos]))
            {
                possible_words[pos].push(rule_idx);
            }
        }
    }

    // Now each list of possible words as different number of options.
    // Prune them until all have one.
    while possible_words.iter().any(|options| options.len() > 1) {
        // Get all words we know.
        let words_we_know_idx: Vec<_> = possible_words
            .iter()
            .filter_map(|options| {
                if options.len() == 1 {
                    Some(options[0])
                } else {
                    None
                }
            })
            .collect();

        // Remove the known words from the lists that are bigger than 1.
        possible_words
            .iter_mut()
            .filter(|options| options.len() > 1)
            .for_each(|options| options.retain(|o| !words_we_know_idx.contains(o)));
    }

    // Convert the vector of vectors of 1 into a vector of integer.
    possible_words
        .iter()
        .map(|options| *options.iter().next().unwrap())
        .collect()
}

fn departure_values(puzzle: &Puzzle) -> u64 {
    let word_positions = find_correct_order(puzzle);
    puzzle
        .rules
        .iter()
        .enumerate()
        .filter(|(_, rule)| rule.name.starts_with("departure"))
        .map(|(rule_idx, _)| {
            let pos = word_positions.iter().position(|&i| i == rule_idx).unwrap();
            u64::from(puzzle.your_ticket[pos])
        })
        .product()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let puzzle = Puzzle::build(&input);

    println!("Part 1: {}", scanning_error_rate(&puzzle));
    println!("Part 2: {}", departure_values(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part1() {
        assert_eq!(scanning_error_rate(&Puzzle::build(INPUT_TEST_1)), 71);
    }

    #[test]
    fn test_part2() {
        assert_eq!(find_correct_order(&Puzzle::build(INPUT_TEST_2)), [1, 0, 2]);
    }
}
