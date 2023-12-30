use std::io::{self, Read};

fn build_dimensions(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| line.split('x').map(|c| c.parse().unwrap()).collect())
        .collect()
}

fn wrapping_paper_total(input: &str) -> u64 {
    let dims = build_dimensions(input);
    dims.iter()
        .map(|b| -> u64 {
            // Is there a more elegant way to do this one?
            let sides = [b[0] * b[1], b[1] * b[2], b[0] * b[2]];
            let min = sides.iter().min().unwrap();
            sides.iter().sum::<u64>() * 2 + min
        })
        .sum::<u64>()
}

fn ribbon_total(input: &str) -> u64 {
    let dims = build_dimensions(input);
    dims.iter()
        .map(|b| -> u64 {
            let perimeters = [(b[0] + b[1]) * 2, (b[1] + b[2]) * 2, (b[0] + b[2]) * 2];
            let around_length = perimeters.iter().min().unwrap();
            let ribbon = b.iter().product::<u64>();
            around_length + ribbon
        })
        .sum::<u64>()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", wrapping_paper_total(&input));
    println!("Part 2: {}", ribbon_total(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(wrapping_paper_total(INPUT_TEST), 58 + 43);
    }

    #[test]
    fn test_part2() {
        assert_eq!(ribbon_total(INPUT_TEST), 34 + 14);
    }
}
