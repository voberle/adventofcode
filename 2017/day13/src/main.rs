use std::{
    io::{self, Read},
    vec,
};

fn build(input: &str) -> Vec<usize> {
    // The max depth isn't very big, so we can have a vector with entries for layers with range 0
    let mut layers: Vec<usize> = Vec::new();
    for line in input.lines() {
        let p: Vec<usize> = line.split(": ").map(|v| v.parse().unwrap()).collect();
        while layers.len() < p[0] {
            layers.push(0);
        }
        layers.push(p[1]);
    }
    layers
}

// The vector produced by this function allows us to calculate the scanner position with the simple
// base[i % base.len()];
fn create_base(range: usize) -> Vec<usize> {
    let mut base = Vec::with_capacity(range * 2);
    for u in 0..range {
        base.push(u);
    }
    for d in 1..range - 1 {
        base.push(range - 1 - d);
    }
    base
}

fn init_bases(layers: &[usize]) -> Vec<Vec<usize>> {
    let max_range = *layers.iter().max().unwrap();
    let mut bases: Vec<Vec<usize>> = vec![vec![]; max_range + 1];
    for r in layers {
        if *r > 0 {
            bases[*r] = create_base(*r);
        }
    }
    bases
}

fn scanner_position(bases: &[Vec<usize>], range: usize, time: usize) -> usize {
    let base = &bases[range];
    base[time % base.len()]
}

fn trip_severity(layers: &[usize]) -> usize {
    let bases = init_bases(layers);

    layers
        .iter()
        .enumerate()
        .map(|(i, r)| {
            if *r > 0 {
                let scanner_pos = scanner_position(&bases, *r, i);
                if scanner_pos == 0 {
                    // depth * range
                    i * r
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum()
}

fn part2(layers: &[usize]) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let layers = build(&input);

    println!("Part 1: {}", trip_severity(&layers));
    println!("Part 2: {}", part2(&layers));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_scanner_position() {
        let bases = [vec![], create_base(1), create_base(2), create_base(3)];
        assert_eq!(scanner_position(&bases, 3, 0), 0);
        assert_eq!(scanner_position(&bases, 3, 1), 1);
        assert_eq!(scanner_position(&bases, 3, 2), 2);
        assert_eq!(scanner_position(&bases, 3, 3), 1);
        assert_eq!(scanner_position(&bases, 3, 4), 0);
        assert_eq!(scanner_position(&bases, 3, 5), 1);

        assert_eq!(scanner_position(&bases, 2, 0), 0);
        assert_eq!(scanner_position(&bases, 2, 1), 1);
        assert_eq!(scanner_position(&bases, 2, 2), 0);
        assert_eq!(scanner_position(&bases, 2, 3), 1);
    }

    #[test]
    fn test_part1() {
        assert_eq!(trip_severity(&build(INPUT_TEST)), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
