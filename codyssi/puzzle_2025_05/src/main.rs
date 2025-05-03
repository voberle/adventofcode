use std::io::{self, Read};

struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn build(line: &str) -> Self {
        let parts: Vec<i32> = line[1..line.len() - 1]
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();
        Self {
            x: parts[0],
            y: parts[1],
        }
    }

    fn dist_from_ship(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn dist_from(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

fn build(input: &str) -> Vec<Coord> {
    input.lines().map(Coord::build).collect()
}

fn diff_closest_furthest(islands: &[Coord]) -> i32 {
    let closest = islands.iter().min_by_key(|c| c.dist_from_ship()).unwrap();
    let furthest = islands.iter().max_by_key(|c| c.dist_from_ship()).unwrap();
    furthest.dist_from_ship() - closest.dist_from_ship()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let islands = build(&input);

    println!("Part 1: {}", diff_closest_furthest(&islands));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let islands = build(&INPUT_TEST);
        assert_eq!(diff_closest_furthest(&islands), 226);
    }
}
