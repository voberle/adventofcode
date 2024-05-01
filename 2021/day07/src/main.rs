use std::io::{self, Read};

fn build(input: &str) -> Vec<u32> {
    input.split(',').map(|v| v.parse().unwrap()).collect()
}

fn fuel_used(horizontal_positions: &[u32]) -> u32 {
    let min = *horizontal_positions.iter().min().unwrap();
    let max = *horizontal_positions.iter().max().unwrap();
    (min..max)
        .map(|trying| {
            horizontal_positions
                .iter()
                .map(|p| trying.abs_diff(*p))
                .sum()
        })
        .min()
        .unwrap()
}

fn part2(horizontal_positions: &[u32]) -> u32 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let horizontal_positions = build(&input);

    println!("Part 1: {}", fuel_used(&horizontal_positions));
    println!("Part 2: {}", part2(&horizontal_positions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part1() {
        assert_eq!(fuel_used(&build(INPUT_TEST)), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
