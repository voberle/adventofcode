use std::io::{self, Read};

use fxhash::FxHashMap;

// In returned map, keys are objects being in orbit, values are "center" objects,
// meaning the inverse of the input.
fn build(input: &str) -> FxHashMap<String, String> {
    input
        .lines()
        .map(|line| {
            let p: Vec<_> = line.split(')').collect();
            (p[1].to_owned(), p[0].to_owned())
        })
        .collect()
}

fn orbits_count(orbits_map: &FxHashMap<String, String>) -> usize {
    let mut count = 0;
    // It would be nicer to reuse counts already done, but it's annoying to do with Rust, so naive implementation for now.
    for object in orbits_map.keys() {
        let mut o = object;
        while let Some(v) = orbits_map.get(o) {
            o = v;
            count += 1;
        }
    }
    count
}

fn build_path_to_center(from: &str, orbits_map: &FxHashMap<String, String>) -> Vec<String> {
    let mut path = Vec::new();
    let mut o = orbits_map.get(from).unwrap();
    path.push(o.clone());
    while let Some(v) = orbits_map.get(o) {
        path.push(v.clone());
        o = v;
    }
    path
}

fn you_to_san_move_counts(orbits_map: &FxHashMap<String, String>) -> usize {
    // We just need to move down from YOU and from SAN, counting how many steps there are until we meet.

    // First get the paths to the center.
    let you_path = build_path_to_center("YOU", orbits_map);
    let san_path = build_path_to_center("SAN", orbits_map);

    // Then find the intersection: We start at center and look when paths separate.
    let intersection = you_path
        .iter()
        .rev()
        .zip(san_path.iter().rev())
        .take_while(|(y, s)| y == s)
        .last()
        .unwrap()
        .0;

    // Count steps to intersection
    you_path.iter().position(|o| o == intersection).unwrap()
        + san_path.iter().position(|o| o == intersection).unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let orbits_map = build(&input);

    println!("Part 1: {}", orbits_count(&orbits_map));
    println!("Part 2: {}", you_to_san_move_counts(&orbits_map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part1() {
        assert_eq!(orbits_count(&build(INPUT_TEST_1)), 42);
    }

    #[test]
    fn test_part2() {
        assert_eq!(you_to_san_move_counts(&build(INPUT_TEST_2)), 4);
    }
}
