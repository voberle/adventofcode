use std::io::{self, Read};

fn build(input: &str) -> Vec<u32> {
    input.split(',').map(|v| v.parse().unwrap()).collect()
}

fn fuel_used(horizontal_positions: &[u32], fuel_consumption_fn: fn(u32, u32) -> u32) -> u32 {
    let min_pos = *horizontal_positions.iter().min().unwrap();
    let max_pos = *horizontal_positions.iter().max().unwrap();
    (min_pos..max_pos)
        .map(|to_pos| {
            horizontal_positions
                .iter()
                .map(|from_pos| fuel_consumption_fn(*from_pos, to_pos))
                .sum()
        })
        .min()
        .unwrap()
}

fn fuel_at_constant_rate(horizontal_positions: &[u32]) -> u32 {
    fuel_used(horizontal_positions, u32::abs_diff)
}

// The first few costs are following:
// 0  0
// 1  1
// 2  3
// 3  6
// 4  10
// 5  15
//
// OEIS tells us these are triangular numbers - https://oeis.org/A000217
// and a formula to find them is: n * (n + 1) / 2;
fn cost(from: u32, to: u32) -> u32 {
    let n = from.abs_diff(to);
    n * (n + 1) / 2
}

fn fuel_at_increasing_rate(horizontal_positions: &[u32]) -> u32 {
    fuel_used(horizontal_positions, cost)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let horizontal_positions = build(&input);

    println!("Part 1: {}", fuel_at_constant_rate(&horizontal_positions));
    println!("Part 2: {}", fuel_at_increasing_rate(&horizontal_positions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part1() {
        assert_eq!(fuel_at_constant_rate(&build(INPUT_TEST)), 37);
    }

    #[test]
    fn test_cost_fn() {
        assert_eq!(cost(16, 5), 66);
        assert_eq!(cost(1, 5), 10);
        assert_eq!(cost(2, 5), 6);
        assert_eq!(cost(0, 5), 15);
        assert_eq!(cost(4, 5), 1);
        assert_eq!(cost(2, 5), 6);
        assert_eq!(cost(7, 5), 3);
        assert_eq!(cost(1, 5), 10);
        assert_eq!(cost(2, 5), 6);
        assert_eq!(cost(14, 5), 45);
    }

    #[test]
    fn test_part2() {
        assert_eq!(fuel_at_increasing_rate(&build(INPUT_TEST)), 168);
    }
}
