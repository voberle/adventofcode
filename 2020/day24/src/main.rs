use std::io::{self, Read};

// Cube coordinates
// See https://www.redblobgames.com/grids/hexagons/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CubeCoords {
    q: i32,
    r: i32,
    s: i32,
}

impl CubeCoords {
    fn new(q: i32, r: i32, s: i32) -> Self {
        CubeCoords { q, r, s }
    }
}

#[derive(Debug)]
enum Dir {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}
use fxhash::FxHashSet;
use Dir::{East, NorthEast, NorthWest, SouthEast, SouthWest, West};

impl Dir {
    fn next_pos(&self, pos: &CubeCoords) -> CubeCoords {
        match self {
            East => CubeCoords::new(pos.q + 1, pos.r, pos.s - 1),
            West => CubeCoords::new(pos.q - 1, pos.r, pos.s + 1),
            NorthWest => CubeCoords::new(pos.q, pos.r - 1, pos.s + 1),
            NorthEast => CubeCoords::new(pos.q + 1, pos.r - 1, pos.s),
            SouthEast => CubeCoords::new(pos.q, pos.r + 1, pos.s - 1),
            SouthWest => CubeCoords::new(pos.q - 1, pos.r + 1, pos.s),
        }
    }
}

fn build(input: &str) -> Vec<Vec<Dir>> {
    input
        .lines()
        .map(|line| {
            let mut directions = Vec::new();
            let mut it = line.chars();
            while let Some(c1) = it.next() {
                let dir = if c1 == 'e' {
                    East
                } else if c1 == 'w' {
                    West
                } else {
                    let c2 = it.next().unwrap();
                    if c1 == 's' && c2 == 'e' {
                        SouthEast
                    } else if c1 == 's' && c2 == 'w' {
                        SouthWest
                    } else if c1 == 'n' && c2 == 'e' {
                        NorthEast
                    } else if c1 == 'n' && c2 == 'w' {
                        NorthWest
                    } else {
                        panic!("Failed to parse directions")
                    }
                };
                directions.push(dir);
            }
            directions
        })
        .collect()
}

fn black_tiles_count(tiles_list: &[Vec<Dir>]) -> usize {
    let mut floor: FxHashSet<CubeCoords> = FxHashSet::default();
    for tiles in tiles_list {
        let mut pos = CubeCoords::new(0, 0, 0);
        for t in tiles {
            pos = t.next_pos(&pos);
        }

        if floor.contains(&pos) {
            floor.remove(&pos);
        } else {
            floor.insert(pos);
        }
    }
    floor.len()
}

fn part2(tiles_list: &[Vec<Dir>]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tiles_list = build(&input);

    println!("Part 1: {}", black_tiles_count(&tiles_list));
    println!("Part 2: {}", part2(&tiles_list));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(black_tiles_count(&build(INPUT_TEST)), 10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
