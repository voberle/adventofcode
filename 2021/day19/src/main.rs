use std::io::{self, Read};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn build(s: &str) -> Self {
        let p: Vec<_> = s.split(',').map(|n| n.parse().unwrap()).collect();
        Self {
            x: p[0],
            y: p[1],
            z: p[2],
        }
    }
}

#[derive(Debug)]
struct Scanner(Vec<Pos>);

fn build(input: &str) -> Vec<Scanner> {
    input
        .split("\n\n")
        .map(|part| Scanner(part.lines().skip(1).map(Pos::build).collect()))
        .collect()
}

fn beacons_count(scanners: &[Scanner]) -> i64 {
    0
}

fn part2(scanners: &[Scanner]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let scanners = build(&input);
    println!("{:?}", scanners);

    println!("Part 1: {}", beacons_count(&scanners));
    println!("Part 2: {}", part2(&scanners));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(beacons_count(&build(INPUT_TEST)), 79);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
