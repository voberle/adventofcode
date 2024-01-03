use std::{
    collections::HashMap,
    io::{self, Read},
};

fn build(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

// Change-making problem.
// https://en.wikipedia.org/wiki/Change-making_problem
//
// Recursive function.
// In params we take the list of numbers still to look at, the sum so far
// and the count of numbers included in the sum.
// We pass also the number of matching combinations found so far for each parts count.
fn subset_sum<const TARGET: u32>(
    numbers: &[u32],
    sum: u32,
    parts_cnt: u32,
    combinations: &mut HashMap<u32, u64>,
) {
    if sum == TARGET {
        // found one
        combinations
            .entry(parts_cnt)
            .and_modify(|e| *e += 1)
            .or_insert(1);
        return;
    }
    if sum > TARGET {
        // no point continuing
        return;
    }

    for i in 0..numbers.len() {
        let n = numbers[i];
        let remaining = &numbers[i + 1..];
        subset_sum::<TARGET>(remaining, sum + n, parts_cnt + 1, combinations);
    }
}

fn combination_count<const TARGET: u32>(containers: &[u32]) -> u64 {
    let mut combinations: HashMap<u32, u64> = HashMap::default();
    subset_sum::<TARGET>(containers, 0, 0, &mut combinations);
    combinations.values().sum()
}

// How many combinations are there of the minimum number of containers needed
fn combination_count_min_nb<const TARGET: u32>(containers: &[u32]) -> u64 {
    let mut combinations: HashMap<u32, u64> = HashMap::default();
    subset_sum::<TARGET>(containers, 0, 0, &mut combinations);
    *combinations.iter().min_by_key(|(k, _)| *k).unwrap().1
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let containers = build(&input);

    println!("Part 1: {}", combination_count::<150>(&containers));
    println!("Part 2: {}", combination_count_min_nb::<150>(&containers));
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
        assert_eq!(combination_count_min_nb::<25>(&build(INPUT_TEST)), 3);
    }
}
