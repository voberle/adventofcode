use std::io::{self, Read};

// Cube coordinates
// See https://www.redblobgames.com/grids/hexagons/
#[derive(Debug)]
struct CubeCoords {
    q: i32,
    r: i32,
    s: i32,
}

impl CubeCoords {
    fn new(q: i32, r: i32, s: i32) -> Self {
        CubeCoords { q, r, s }
    }

    fn distance_from_zero(&self) -> i32 {
        (self.q.abs() + self.r.abs() + self.s.abs()) / 2
    }
}

#[derive(Debug)]
enum Dir {
    North,
    NorthEast,
    NorthWest,
    South,
    SouthWest,
    SouthEast,
}
use Dir::{North, NorthEast, NorthWest, South, SouthEast, SouthWest};

impl Dir {
    fn new(s: &str) -> Self {
        match s {
            "n" => Self::North,
            "ne" => Self::NorthEast,
            "nw" => Self::NorthWest,
            "s" => Self::South,
            "se" => Self::SouthEast,
            "sw" => Self::SouthWest,
            _ => panic!("Invalid direction"),
        }
    }

    fn build(input: &str) -> Vec<Dir> {
        input.split(',').map(Dir::new).collect()
    }

    fn next_pos(&self, pos: &CubeCoords) -> CubeCoords {
        match self {
            North => CubeCoords::new(pos.q, pos.r - 1, pos.s + 1),
            NorthEast => CubeCoords::new(pos.q + 1, pos.r - 1, pos.s),
            NorthWest => CubeCoords::new(pos.q - 1, pos.r, pos.s + 1),
            South => CubeCoords::new(pos.q, pos.r + 1, pos.s - 1),
            SouthEast => CubeCoords::new(pos.q + 1, pos.r, pos.s - 1),
            SouthWest => CubeCoords::new(pos.q - 1, pos.r + 1, pos.s),
        }
    }
}

fn find_child_position(directions: &[Dir]) -> CubeCoords {
    let mut pos = CubeCoords::new(0, 0, 0);
    for d in directions {
        pos = d.next_pos(&pos);
    }
    pos
}

fn min_steps_to_child(directions: &[Dir]) -> i32 {
    let child_pos = find_child_position(directions);
    println!("Child pos {:?}", child_pos);

    child_pos.distance_from_zero()
}

fn part2(directions: &[Dir]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let directions = Dir::build(&input);

    println!("Part 1: {}", min_steps_to_child(&directions));
    println!("Part 2: {}", part2(&directions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(min_steps_to_child(&Dir::build("ne,ne,ne")), 3);
        assert_eq!(min_steps_to_child(&Dir::build("ne,ne,sw,sw")), 0);
        assert_eq!(min_steps_to_child(&Dir::build("ne,ne,s,s")), 2);
        assert_eq!(min_steps_to_child(&Dir::build("se,sw,se,sw,sw")), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&Dir::build("")), 0);
    }
}
