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

#[allow(dead_code)]
fn state_to_string_trimed(state: &[bool]) -> String {
    state_to_string(state)
        .trim_start_matches(|c| c == '.')
        .trim_end_matches(|c| c == '.')
        .to_string()
}

fn get_sum_of_plant_pots(state: &[bool], index_of_zero: i32) -> i32 {
    state
        .iter()
        .enumerate()
        .map(|(i, v)| if *v { i as i32 - index_of_zero } else { 0 })
        .sum()
}

fn plant_pots_sum(initial_state: &[bool], instructions: &[bool], nb_of_generations: usize) -> i32 {
    const TEN_FALSE: [bool; 10] = [false; 10];
    const INDEX_OF_ZERO: i32 = 10;

    let mut state = Vec::new();
    // Hard-coded padding, ugly but does the job.
    state.extend(TEN_FALSE);
    state.extend(initial_state);
    state.extend(TEN_FALSE);

    // println!("0: {}", state_to_string(&state));
    for _g in 1..=nb_of_generations {
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
        // println!("{}: {}", _g, state_to_string_trimed(&state));
        // println!("{}: {}", _g, get_sum_of_plant_pots(&state, INDEX_OF_ZERO));
    }

    get_sum_of_plant_pots(&state, INDEX_OF_ZERO)
}

fn plant_pots_sum_small(initial_state: &[bool], instructions: &[bool]) -> i32 {
    plant_pots_sum(initial_state, instructions, 20)
}

fn plant_pots_sum_huge(initial_state: &[bool], instructions: &[bool]) -> i64 {
    const NB_OF_GENERATIONS: usize = 50_000_000_000;

    // By printing the state trimmed, we see it stabilizes from generation 124,
    // meaning it's the same pattern that just shifts.
    const STABILIZATION_GENERATION: usize = 125;

    // So we find the starting value and the shift.
    let sum = plant_pots_sum(initial_state, instructions, STABILIZATION_GENERATION) as i64;
    let sum2 = plant_pots_sum(initial_state, instructions, STABILIZATION_GENERATION + 1) as i64;
    let diff = sum2 - sum;

    sum + diff * (NB_OF_GENERATIONS - STABILIZATION_GENERATION) as i64
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (initial_state, instructions) = build(input.trim());

    println!(
        "Part 1: {}",
        plant_pots_sum_small(&initial_state, &instructions)
    );
    println!(
        "Part 2: {}",
        plant_pots_sum_huge(&initial_state, &instructions)
    );
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
        assert_eq!(plant_pots_sum_small(&initial_state, &instructions), 325);
    }
}
