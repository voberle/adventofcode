use std::{
    collections::HashMap,
    io::{self, Read},
};

use regex::Regex;

// Data structure representing our programs.
// Idea is that for the algos, we only have to manipulate vectors and indexes,
// no strings or hash maps.
struct Programs {
    // In the other data structures, we use indexes, not program names.
    // This structure allows us to find the program name for each index.
    names: Vec<String>,
    // Here we have the weight of each program.
    weight: Vec<u32>,
    // The programs immediately above.
    above: Vec<Vec<usize>>,
}

fn build(input: &str) -> Programs {
    let re = Regex::new(r"(\w+) \((\d+)\)( -> )?(.*)").unwrap();
    let mut programs = Programs {
        names: Vec::new(),
        weight: Vec::new(),
        above: Vec::new(),
    };
    // Temporary map used to build the vectors.
    let mut name2id: HashMap<String, usize> = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let parts = re.captures(line).unwrap();
        programs.names.push(parts[1].to_string());
        name2id.insert(parts[1].to_string(), i);
        programs.weight.push(parts[2].parse().unwrap());
    }
    for line in input.lines() {
        let parts = re.captures(line).unwrap();
        if parts.len() > 3 && !parts[4].is_empty() {
            programs.above.push(
                parts[4]
                    .split(", ")
                    .map(|item| {
                        *name2id
                            .get(item)
                            .unwrap_or_else(|| panic!("Didn't find {} in map", item))
                    })
                    .collect(),
            );
        } else {
            programs.above.push(Vec::new());
        }
    }
    programs
}

fn bottom_program(programs: &Programs) -> String {
    // Idea is that the bottom program isn't on a platform (not in programs.above)
    // and also isn't any of the programs that doesn't have anything on top
    // (not any of those that have programs.above empty).
    let mut programs_above: Vec<usize> = programs.above.iter().flatten().copied().collect();
    programs_above.extend(
        programs
            .above
            .iter()
            .enumerate()
            .filter(|(_, p)| p.is_empty())
            .map(|(i, _)| i),
    );
    // Sorts the slice, but might not preserve the order of equal elements.
    programs_above.sort_unstable();
    programs_above.dedup();
    let bottom_idx = programs_above
        .iter()
        .enumerate()
        .find(|(i, p)| i != *p)
        .expect("Didn't find bottom program")
        .0;
    programs.names[bottom_idx].clone()
}

fn part2(programs: &Programs) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let programs = build(&input);

    println!("Part 1: {}", bottom_program(&programs));
    println!("Part 2: {}", part2(&programs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(bottom_program(&build(INPUT_TEST)), "tknk");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
