use std::io::{self, Read};

fn build(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

// Change-making problem.
// https://en.wikipedia.org/wiki/Change-making_problem
//
// Recursive function.
// In params we take the list of numbers still to look at and the sum so far.
// Returns the number of matching combinations found so far.
fn subset_sum<const TARGET: u32>(numbers: &[u32], sum: u32) -> u64 {
    if sum == TARGET {
        // found one
        return 1;
    }
    if sum > TARGET {
        // no point continuing
        return 0;
    }

    let mut comb = 0;
    for i in 0..numbers.len() {
        let n = numbers[i];
        let remaining = &numbers[i + 1..];
        comb += subset_sum::<TARGET>(remaining, sum + n);
    }
    comb
}

fn combination_count<const TARGET: u32>(containers: &[u32]) -> u64 {
    subset_sum::<TARGET>(containers, 0)
}

fn part2(containers: &[u32]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let containers = build(&input);

    println!("Part 1: {}", combination_count::<150>(&containers));
    println!("Part 2: {}", part2(&containers));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(combination_count::<25>(&build(INPUT_TEST)), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
