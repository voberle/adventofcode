use std::io::{self, Read};

use regex::Regex;

#[inline]
fn char(s: &str) -> char {
    s.chars().next().unwrap()
}

fn build(input: &str) -> Vec<(char, char)> {
    let re = Regex::new(r"Step (\w) must be finished before step (\w) can begin.").unwrap();
    input
        .lines()
        .map(|line| {
            let p = re.captures(line).unwrap();
            (char(&p[1]), char(&p[2]))
        })
        .collect()
}

#[inline]
fn char2idx(c: char) -> usize {
    (c as u8 - b'A') as usize
}

#[inline]
fn idx2char(idx: usize) -> char {
    char::from(b'A' + idx as u8)
}

fn steps_in_order<const STEP_COUNT: usize>(deps: &[(char, char)]) -> String {
    // Since we just deal with up to 26 steps (letters in alphabetical order),
    // tracking them in an array is convenient.
    //  0: Not ready to exec.
    //  1-26: Executed, in that order.
    //  READY: Not executed, but ready.
    const NOT_READY: usize = 0;
    const READY: usize = usize::MAX;
    let mut letters = [READY; STEP_COUNT];

    // Mark all that cannot be executed initially.
    for d in deps {
        letters[char2idx(d.1)] = 0;
    }

    let mut pos = 1;
    while pos <= STEP_COUNT {
        // Mark the steps ready to be executed.
        for idx in 0..STEP_COUNT {
            if letters[idx] == NOT_READY {
                // If it's on the right side of a dependency and not ready, we can only do it if we have done all the prerequisites.
                if deps
                    .iter()
                    .filter(|(_, r)| char2idx(*r) == idx)
                    .all(|(l, _)| (1..=STEP_COUNT).contains(&letters[char2idx(*l)]))
                {
                    letters[idx] = READY;
                }
            }
        }

        // Do first in alphabetical order.
        if let Some(to_exec_idx) = letters.iter().position(|v| *v == READY) {
            letters[to_exec_idx] = pos;
            pos += 1;
        }
    }

    (1..=STEP_COUNT)
        .map(|idx| {
            let c_as_int = letters.iter().position(|i| *i == idx).unwrap();
            idx2char(c_as_int)
        })
        .collect()
}

fn part2(deps: &[(char, char)]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let dependencies = build(input.trim());
    println!("{:?}", dependencies);

    println!("Part 1: {}", steps_in_order::<26>(&dependencies));
    println!("Part 2: {}", part2(&dependencies));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(steps_in_order::<6>(&build(INPUT_TEST)), "CABDFE");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
