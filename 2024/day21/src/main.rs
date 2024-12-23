use std::{
    io::{self, Read},
    iter::once,
};

mod model;

use itertools::Itertools;
use model::{DirKey, NumKey};

fn build(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

// Finds the short path(s) to reach all the numerical code values.
// Returns the path as numerical keys: It still needs to be convered to directions then.
//
// The code needs to include the starting position.
// Meaning if we want the path to enter 029A from A, the code must be A029A.
fn find_code_paths(code: &[char]) -> Vec<Vec<NumKey>> {
    let mut paths: Vec<Vec<NumKey>> = vec![vec![]];
    for pair in code.windows(2) {
        let paths_for_pair = NumKey::from(pair[0]).find_all_paths_to(NumKey::from(pair[1]));
        assert!(!paths_for_pair.is_empty());

        paths = paths_for_pair
            .iter()
            .flat_map(|path| {
                paths.iter().map(move |base_path| {
                    let mut p = base_path.clone();
                    p.extend(path);
                    p
                })
            })
            .collect();
    }
    paths
}

// Converts the path format generated by find_code_paths to a set of directions.
fn convert_num_paths_to_directions(path: &[NumKey]) -> Vec<DirKey> {
    path.windows(2)
        .map(|pair| {
            // If both elements are the same, we are at a key that needs to be pressed
            if pair[0] == pair[1] {
                DirKey::A
            } else {
                pair[0].dir(pair[1])
            }
        })
        .chain(once(DirKey::A)) // last push
        .collect()
}

// Finds the short path(s) to reach all the direction values.
// The code needs to include the starting position.
fn find_dir_paths(directions: &[DirKey]) -> Vec<Vec<DirKey>> {
    let mut paths: Vec<Vec<DirKey>> = vec![vec![]];
    for pair in directions.windows(2) {
        let paths_for_pair = pair[0].go_press(pair[1]);
        assert!(!paths_for_pair.is_empty());

        paths = paths_for_pair
            .iter()
            .flat_map(|path| {
                paths.iter().map(move |base_path| {
                    let mut p = base_path.clone();
                    p.extend(path);
                    p
                })
            })
            .collect();
    }
    paths
}

fn prepend<T: Clone>(input: &[T], elt: T) -> Vec<T> {
    let mut v = vec![elt];
    v.extend(input.iter().cloned());
    v
}

fn shortest_sequence_length(code: &[char], robots_count: usize) -> usize {
    let paths = find_code_paths(&prepend(code, 'A'));
    let mut next_paths = paths
        .iter()
        .map(|p| convert_num_paths_to_directions(p))
        .collect_vec();

    for _ in 0..robots_count {
        next_paths = next_sequence(&next_paths);
    }

    next_paths.iter().map(std::vec::Vec::len).min().unwrap()
}

fn next_sequence(paths_as_dirs: &[Vec<DirKey>]) -> Vec<Vec<DirKey>> {
    paths_as_dirs
        .iter()
        .flat_map(|dirs| find_dir_paths(&prepend(dirs, DirKey::A)))
        .collect()
}

fn code_numeric_part(code: &[char]) -> usize {
    (code[0] as usize - '0' as usize) * 100
        + (code[1] as usize - '0' as usize) * 10
        + (code[2] as usize - '0' as usize)
}

fn complexities_sum(codes: &[Vec<char>], robots_count: usize) -> usize {
    codes
        .iter()
        .map(|code| shortest_sequence_length(code, robots_count) * code_numeric_part(code))
        .sum()
}

fn complexities_sum_2_robots(codes: &[Vec<char>]) -> usize {
    complexities_sum(codes, 2)
}

fn complexities_sum_25_robots(codes: &[Vec<char>]) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let codes = build(&input);

    println!("Part 1: {}", complexities_sum_2_robots(&codes));
    println!("Part 2: {}", complexities_sum_25_robots(&codes));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_shortest_sequence_length() {
        assert_eq!(shortest_sequence_length(&['0', '2', '9', 'A'], 2), 68)
    }

    #[test]
    fn test_code_numeric_part() {
        assert_eq!(code_numeric_part(&['0', '2', '9', 'A']), 29)
    }

    #[test]
    fn test_part1() {
        assert_eq!(complexities_sum_2_robots(&build(INPUT_TEST)), 126384);
    }
}