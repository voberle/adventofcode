use std::io::{self, Read};

fn pattern_to_index(s: &str) -> usize {
    s.chars().rev().enumerate().fold(0, |acc, (i, b)| {
        if b == '#' {
            acc + 2_usize.pow(i as u32)
        } else {
            acc
        }
    })
}

fn slice_to_index(s: &[bool]) -> usize {
    s.iter().rev().enumerate().fold(
        0,
        |acc, (i, b)| if *b { acc + 2_usize.pow(i as u32) } else { acc },
    )
}

fn build(input: &str) -> (Vec<bool>, Vec<bool>) {
    let initial_state: Vec<bool> = input
        .lines()
        .next()
        .unwrap()
        .strip_prefix("initial state: ")
        .unwrap()
        .chars()
        .map(|c| c == '#')
        .collect();

    // The instructions pattern can be represented as a bit map,
    // meaning it's 5 bits numbers, aka 32 mappings.
    let mut instructions = vec![false; 32];
    for line in input.lines().skip(2) {
        let p: Vec<_> = line.split(" => ").collect();
        let idx = pattern_to_index(p[0]);
        if p[1] == "#" {
            instructions[idx] = true;
        }
    }
    (initial_state, instructions)
}

fn state_to_string(state: &[bool]) -> String {
    state.iter().map(|v| if *v { '#' } else { '.' }).collect()
}

fn part1(initial_state: &[bool], instructions: &[bool]) -> i32 {
    const NB_OF_GENERATIONS: usize = 20;

    const TEN_FALSE: [bool; 10] = [false; 10];
    const INDEX_OF_ZERO: i32 = 10;

    let mut state = Vec::new();
    // Hard-coded padding for part 1, ugly but does the job
    state.extend(TEN_FALSE);
    state.extend(initial_state);
    state.extend(TEN_FALSE);

    println!("0: {}", state_to_string(&state));
    for g in 1..=NB_OF_GENERATIONS {
        let mut next_state: Vec<bool> = Vec::new();
        next_state.push(false);
        next_state.push(false);

        for i in 2..state.len() - 2 {
            let idx = slice_to_index(&state[i - 2..=i + 2]);
            next_state.push(instructions[idx]);
        }

        next_state.push(false);
        next_state.push(false);
        next_state.push(false);
        next_state.push(false);

        std::mem::swap(&mut state, &mut next_state);
        println!("{}: {}", g, state_to_string(&state));
    }

    state
        .iter()
        .enumerate()
        .map(|(i, v)| if *v { i as i32 - INDEX_OF_ZERO } else { 0 })
        .sum()
}

fn part2(initial_state: &[bool], instructions: &[bool]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (initial_state, instructions) = build(input.trim());

    println!("Part 1: {}", part1(&initial_state, &instructions));
    println!("Part 2: {}", part2(&initial_state, &instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_pattern2index() {
        assert_eq!(pattern_to_index("....."), 0);
        assert_eq!(pattern_to_index("....#"), 1);
        assert_eq!(pattern_to_index("...##"), 3);
        assert_eq!(pattern_to_index("####."), 30);
    }

    #[test]
    fn test_part1() {
        let (initial_state, instructions) = build(INPUT_TEST);
        assert_eq!(part1(&initial_state, &instructions), 325);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
