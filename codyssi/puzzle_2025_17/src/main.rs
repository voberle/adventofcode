use std::io::{self, Read};

use fxhash::FxHashMap;
use itertools::Itertools;

struct Stair {
    id: String,
    step_from: u32,
    step_to: u32,
    from: String,
    to: String,
}

impl Stair {
    fn build(line: &str) -> Self {
        let parts: Vec<_> = line.split(" : ").collect();
        let (step_from, step_to) = parts[1]
            .split(" -> ")
            .map(|v| v.parse().unwrap())
            .collect_tuple()
            .unwrap();
        let (from, to) = parts[2]
            .trim_start_matches("FROM ")
            .split(" TO ")
            .collect_tuple()
            .unwrap();
        Self {
            id: parts[0].to_string(),
            step_from,
            step_to,
            from: from.to_string(),
            to: to.to_string(),
        }
    }
}

fn build(input: &str) -> (Vec<Stair>, Vec<u32>) {
    let parts: Vec<_> = input.split("\n\n").collect();
    let stairs = parts[0].lines().map(Stair::build).collect();
    let moves = parts[1]
        .trim_start_matches("Possible Moves : ")
        .split(", ")
        .map(|m| m.parse().unwrap())
        .collect();
    (stairs, moves)
}

#[allow(clippy::comparison_chain)]
fn valid_paths(steps: u32, moves: &[u32], cache: &mut FxHashMap<u32, u128>) -> u128 {
    let mut cnt = 0;
    for &mv in moves {
        if mv < steps {
            let remaining_steps = steps - mv;
            if let Some(cached_result) = cache.get(&remaining_steps) {
                cnt += cached_result;
            } else {
                let res = valid_paths(remaining_steps, moves, cache);
                cache.insert(remaining_steps, res);
                cnt += res;
            }
        } else if mv == steps {
            cnt += 1;
        }
    }
    cnt
}

fn valid_paths_count(stairs: &[Stair], moves: &[u32]) -> u128 {
    let first_stair = stairs.first().unwrap();
    assert_eq!(first_stair.id, "S1");
    assert_eq!(first_stair.step_from, 0);
    assert_eq!(first_stair.from, "START");
    assert_eq!(first_stair.to, "END");
    // This is actually steps transtions.
    let steps = first_stair.step_to;

    let mut cache: FxHashMap<u32, u128> = FxHashMap::default();

    valid_paths(steps, moves, &mut cache)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (stairs, moves) = build(&input);

    println!("Part 1: {}", valid_paths_count(&stairs, &moves));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");
    const INPUT_TEST_3: &str = include_str!("../resources/input_test_3");

    #[test]
    fn test_part1_1() {
        let (stairs, moves) = build(&INPUT_TEST_1);
        assert_eq!(valid_paths_count(&stairs, &moves), 6);
    }

    #[test]
    fn test_part1_2() {
        let (stairs, moves) = build(&INPUT_TEST_2);
        assert_eq!(valid_paths_count(&stairs, &moves), 13);
    }

    #[test]
    fn test_part1_3() {
        let (stairs, moves) = build(&INPUT_TEST_3);
        assert_eq!(valid_paths_count(&stairs, &moves), 231843173048269749794);
    }
}
