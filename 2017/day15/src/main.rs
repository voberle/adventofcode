use std::io::{self, Read};

fn build(input: &str) -> (u64, u64) {
    let lines: Vec<&str> = input.lines().collect();
    let extract = |line: &str, p| line.strip_prefix(p).unwrap().parse::<u64>().unwrap();
    (
        extract(lines[0], "Generator A starts with "),
        extract(lines[1], "Generator B starts with "),
    )
}

fn next_val(prev_val: u64, factor: u64) -> u64 {
    (prev_val * factor) % 2_147_483_647
}

fn judge_final_count<const COUNT: usize>(a_start: u64, b_start: u64) -> usize {
    const A_FACTOR: u64 = 16807;
    const B_FACTOR: u64 = 48271;
    const MASK: u64 = 0x0FFFF;

    let mut matching = 0;
    let mut a_val = a_start;
    let mut b_val = b_start;
    for _ in 0..COUNT {
        a_val = next_val(a_val, A_FACTOR);
        b_val = next_val(b_val, B_FACTOR);

        if a_val & MASK == b_val & MASK {
            matching += 1;
        }
    }
    matching
}

fn part2(a_start: u64, b_start: u64) -> i64 {
    0
}

const TOTAL_ITERATIONS: usize = 40_000_000;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (a_start, b_start) = build(&input);

    println!(
        "Part 1: {}",
        judge_final_count::<TOTAL_ITERATIONS>(a_start, b_start)
    );
    println!("Part 2: {}", part2(a_start, b_start));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(judge_final_count::<5>(65, 8921), 1);
        assert_eq!(judge_final_count::<TOTAL_ITERATIONS>(65, 8921), 588);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(65, 8921), 0);
    }
}
