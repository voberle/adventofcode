use std::io::{self, Read};

fn build(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn fuel_req(mass: u32) -> u32 {
    mass / 3 - 2
}

fn fuel_req_sum(masses: &[u32]) -> u32 {
    masses.iter().map(|m| fuel_req(*m)).sum()
}

fn part2(masses: &[u32]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let masses = build(&input);

    println!("Part 1: {}", fuel_req_sum(&masses));
    println!("Part 2: {}", part2(&masses));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_req() {
        assert_eq!(fuel_req(12), 2);
        assert_eq!(fuel_req(14), 2);
        assert_eq!(fuel_req(1969), 654);
        assert_eq!(fuel_req(100756), 33583);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
