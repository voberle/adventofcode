use std::io::{self, Read};

mod cave;

use cave::Cave;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    const ZERO: Pos = Pos { x: 0, y: 0 };

    fn new(x: usize, y: usize) -> Self {
        Pos { x, y }
    }
}

fn part2(cave: &Cave) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let cave = Cave::new(&input);

    println!("Part 1: {}", cave.risk_level());
    println!("Part 2: {}", part2(&cave));
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let cave = Cave::new(INPUT_TEST);
        assert_eq!(cave.risk_level(), 114);
    }

    #[test]
    fn test_part2() {
        let cave = Cave::new(INPUT_TEST);
        assert_eq!(part2(&cave), 0);
    }
}
