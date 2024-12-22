use std::{
    io::{self, Read},
    iter::once,
};

use itertools::Itertools;
use model::{DirKey, NumKey};

mod model;

fn build(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

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

fn convert_num_paths_to_directions(path: &[NumKey]) -> Vec<DirKey> {
    path.windows(2)
        .map(|pair| {
            // if both elements are the same, we are at a key that needs to be pressed
            if pair[0] == pair[1] {
                DirKey::A
            } else {
                pair[0].dir(pair[1])
            }
        })
        .chain(once(DirKey::A)) // last push
        .collect()
}

fn complexities_sum(codes: &[Vec<char>]) -> i64 {
    let paths = find_code_paths(&['A', '0', '2', '9', 'A']);
    for p in paths {
        // println!("{:?}", p);
        let dirs = convert_num_paths_to_directions(&p);
        // println!("{:?}", dirs);
        println!("{}", dirs.iter().map(std::string::ToString::to_string).join(""));
    }
    0
}

fn part2(codes: &[Vec<char>]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let codes = build(&input);

    // model::print_numeric_keypad(&[]);

    // for p in NumKey::K1.find_all_paths_to(NumKey::K9) {
    //     println!();
    //     model::print_numeric_keypad(&p);
    // }

    println!("Part 1: {}", complexities_sum(&codes));
    println!("Part 2: {}", part2(&codes));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(complexities_sum(&build(INPUT_TEST)), 126384);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
