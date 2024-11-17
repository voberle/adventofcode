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
                            .unwrap_or_else(|| panic!("Didn't find {item} in map"))
                    })
                    .collect(),
            );
        } else {
            programs.above.push(Vec::new());
        }
    }
    programs
}

fn bottom_program(programs: &Programs) -> usize {
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

    programs_above
        .iter()
        .enumerate()
        .find(|(i, p)| i != *p)
        .expect("Didn't find bottom program")
        .0
}

// Recursive function.
// Returns the weight of bottom + everything on top of it.
fn calc_sub_towers_weight(
    programs: &Programs,
    bottom: usize,
    sub_towers_weight: &mut Vec<Vec<(usize, u32)>>,
) -> u32 {
    if programs.above[bottom].is_empty() {
        programs.weight[bottom]
    } else {
        for p in &programs.above[bottom] {
            let w = calc_sub_towers_weight(programs, *p, sub_towers_weight);
            sub_towers_weight[bottom].push((*p, w));
        }
        programs.weight[bottom]
            + sub_towers_weight[bottom]
                .iter()
                .map(|(_, w)| w)
                .sum::<u32>()
    }
}

fn all_equals(v: &[(usize, u32)]) -> bool {
    let first = v[0].1;
    v.iter().all(|(_, w)| *w == first)
}

fn index_of_non_equal(v: &[(usize, u32)]) -> usize {
    assert!(v.len() > 2);
    let eq_val = if v[0].1 == v[1].1 || v[0].1 == v[2].1 {
        v[0].1
    } else {
        assert_eq!(v[1].1, v[2].1);
        v[1].1
    };
    v.iter().find(|(_, w)| *w != eq_val).unwrap().0
}

// Correct weight of the one program that has it wrong.
fn correct_weight(programs: &Programs, bottom_idx: usize) -> u32 {
    // For each program, has the list of towers on top of it.
    // Each tower is identified by its index (first item of the pair) and its total weight (second item).
    let mut sub_towers_weight: Vec<Vec<(usize, u32)>> = vec![Vec::new(); programs.names.len()];

    calc_sub_towers_weight(programs, bottom_idx, &mut sub_towers_weight);

    // Go down the graph to find the deepest with a wrong weight.
    let mut ne_idx = bottom_idx;
    let mut ne_disc = &sub_towers_weight[ne_idx];
    loop {
        ne_idx = index_of_non_equal(&sub_towers_weight[ne_idx]);
        let weights = &sub_towers_weight[ne_idx];
        if weights.is_empty() || all_equals(weights) {
            break;
        }
        // Updating ne_disc after above check as we want it to contain the last non equal disc.
        ne_disc = &sub_towers_weight[ne_idx];
    }
    // Now ne_idx is the one that is wrong, ne_disc are the weights that the wrong one is part of.

    // Find by how much we need to correct the weight
    assert!(ne_disc.len() > 1);
    let right_w = ne_disc.iter().find(|(i, _)| *i != ne_idx).unwrap().1;
    let wrong_w = ne_disc.iter().find(|(i, _)| *i == ne_idx).unwrap().1;

    if wrong_w > right_w {
        programs.weight[ne_idx] - (wrong_w - right_w)
    } else {
        programs.weight[ne_idx] + (right_w - wrong_w)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let programs = build(&input);

    let bottom_idx = bottom_program(&programs);
    println!("Part 1: {}", programs.names[bottom_idx]);
    println!("Part 2: {}", correct_weight(&programs, bottom_idx));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let programs = build(INPUT_TEST);
        let bottom_idx = bottom_program(&programs);
        assert_eq!(programs.names[bottom_idx], "tknk");
    }

    #[test]
    fn test_part2() {
        let programs = build(INPUT_TEST);
        let bottom_idx = bottom_program(&programs);
        assert_eq!(correct_weight(&programs, bottom_idx), 60);
    }
}
