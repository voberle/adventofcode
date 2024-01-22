use std::io::{self, Read};

fn build(input: &str) -> (u64, u64) {
    let lines: Vec<&str> = input.lines().collect();
    let extract = |line: &str, p| line.strip_prefix(p).unwrap().parse::<u64>().unwrap();
    (
        extract(lines[0], "Generator A starts with "),
        extract(lines[1], "Generator B starts with "),
    )
}

type NextFn = fn(u64) -> u64;

fn final_count<const COUNT: usize>(
    a_start: u64,
    b_start: u64,
    a_next_fn: NextFn,
    b_next_fn: NextFn,
) -> usize {
    const MASK: u64 = 0x0FFFF;

    let mut matching = 0;
    let mut a_val = a_start;
    let mut b_val = b_start;
    for _ in 0..COUNT {
        a_val = a_next_fn(a_val);
        b_val = b_next_fn(b_val);

        if a_val & MASK == b_val & MASK {
            matching += 1;
        }
    }
    matching
}

fn next_val<const FACTOR: u64>(prev_val: u64) -> u64 {
    (prev_val * FACTOR) % 2_147_483_647
}

const A_FACTOR: u64 = 16807;
const B_FACTOR: u64 = 48271;

fn judge_final_count<const COUNT: usize>(a_start: u64, b_start: u64) -> usize {
    final_count::<COUNT>(a_start, b_start, next_val::<A_FACTOR>, next_val::<B_FACTOR>)
}

fn next_val_picky<const FACTOR: u64, const MULTIPLE_COND: u64>(prev_val: u64) -> u64 {
    let mut n = next_val::<FACTOR>(prev_val);
    while n % MULTIPLE_COND != 0 {
        n = next_val::<FACTOR>(n);
    }
    n
}

const A_MULT_COND: u64 = 4;
const B_MULT_COND: u64 = 8;

fn judge_final_count_picky<const COUNT: usize>(a_start: u64, b_start: u64) -> usize {
    final_count::<COUNT>(
        a_start,
        b_start,
        next_val_picky::<A_FACTOR, A_MULT_COND>,
        next_val_picky::<B_FACTOR, B_MULT_COND>,
    )
}

const TOTAL_ITERATIONS_PART1: usize = 40_000_000;
const TOTAL_ITERATIONS_PART2: usize = 5_000_000;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (a_start, b_start) = build(&input);

    println!(
        "Part 1: {}",
        judge_final_count::<TOTAL_ITERATIONS_PART1>(a_start, b_start)
    );
    println!(
        "Part 2: {}",
        judge_final_count_picky::<TOTAL_ITERATIONS_PART2>(a_start, b_start)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(judge_final_count::<5>(65, 8921), 1);
        assert_eq!(judge_final_count::<TOTAL_ITERATIONS_PART1>(65, 8921), 588);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            judge_final_count_picky::<TOTAL_ITERATIONS_PART2>(65, 8921),
            309
        );
    }
}
