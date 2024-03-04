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

fn part2(orbits_map: &FxHashMap<String, String>) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let orbits_map = build(&input);

    println!("Part 1: {}", orbits_count(&orbits_map));
    println!("Part 2: {}", part2(&orbits_map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(orbits_count(&build(INPUT_TEST)), 42);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
