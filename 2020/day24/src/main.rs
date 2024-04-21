use std::io::{self, Read};

use fxhash::FxHashSet;

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

    fn adjacents(&self) -> Vec<CubeCoords> {
        [East, NorthEast, NorthWest, SouthEast, SouthWest, West]
            .iter()
            .map(|dir| dir.next_pos(self))
            .collect()
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

fn get_floor(tiles_list: &[Vec<Dir>]) -> FxHashSet<CubeCoords> {
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
    floor
}

fn black_tiles_after_days(floor: &FxHashSet<CubeCoords>) -> usize {
    const DAYS_COUNT: usize = 100;

    let mut floor = floor.clone();
    for _ in 0..DAYS_COUNT {
        let mut new_floor = floor.clone();

        // We collect all tiles adjacent to black, as tiles that may be white and are worth checking.
        let mut all_adjacents: Vec<CubeCoords> = Vec::new();

        // Any black tile with zero or more than 2 black tiles immediately adjacent to it
        // is flipped to white.
        for t in &floor {
            let adjs = t.adjacents();
            let black_count = adjs.iter().filter(|c| floor.contains(c)).count();
            if black_count == 0 || black_count > 2 {
                new_floor.remove(t);
            }

            all_adjacents.extend(adjs);
        }

        // Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
        for t in all_adjacents {
            // We should only look at white tiles, so we skip black ones.
            if floor.contains(&t) {
                continue;
            }
            let adjs = t.adjacents();
            let black_count = adjs.iter().filter(|c| floor.contains(c)).count();
            if black_count == 2 {
                new_floor.insert(t);
            }
        }

        std::mem::swap(&mut floor, &mut new_floor);
    }

    floor.len()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tiles_list = build(&input);

    let floor = get_floor(&tiles_list);

    println!("Part 1: {}", floor.len());
    println!("Part 2: {}", black_tiles_after_days(&floor));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let floor = get_floor(&build(INPUT_TEST));
        assert_eq!(floor.len(), 10);
    }

    #[test]
    fn test_part2() {
        let floor = get_floor(&build(INPUT_TEST));
        assert_eq!(black_tiles_after_days(&floor), 2208);
    }
}
