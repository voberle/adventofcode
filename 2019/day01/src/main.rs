use std::io::{self, Read};

fn build(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn fuel_req(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

fn fuel_req_sum(masses: &[u32]) -> u32 {
    masses.iter().map(|m| fuel_req(*m)).sum()
}

fn fuel_for_fuel(mass: u32) -> u32 {
    let mut f = mass;
    let mut total = 0;
    while f != 0 {
        f = fuel_req(f);
        total += f;
    }
    total
}

fn fuel_for_fuel_sum(masses: &[u32]) -> u32 {
    masses.iter().map(|m| fuel_for_fuel(*m)).sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let masses = build(&input);

    println!("Part 1: {}", fuel_req_sum(&masses));
    println!("Part 2: {}", fuel_for_fuel_sum(&masses));
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
    fn test_fuel_for_fuel() {
        assert_eq!(fuel_for_fuel(14), 2);
        assert_eq!(fuel_for_fuel(1969), 966);
        assert_eq!(fuel_for_fuel(100756), 50346);
    }
}
