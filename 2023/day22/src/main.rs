// https://adventofcode.com/2023/day/22

use std::io::{stdin, BufRead};

struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

impl Coord {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}

struct Brick {
    pos: (Coord, Coord),
}

impl Brick {
    fn new(c1: Coord, c2: Coord) -> Self {
        Self { pos: (c1, c2) }
    }
}

fn safely_disintegrated_count(snapshot: &Vec<Brick>) -> u32 {
    0
}

fn build_snapshot<R>(reader: &mut R) -> Vec<Brick>
where
    R: BufRead,
{
    let mut snapshot: Vec<Brick> = Vec::new();
    for l in reader.lines() {
        let line = l.unwrap();
        let p: Vec<Vec<usize>> = line
            .split('~')
            .map(|c| c.split(',').map(|i| i.parse().unwrap()).collect())
            .collect();
        snapshot.push(Brick::new(
            Coord::new(p[0][0], p[0][1], p[0][2]),
            Coord::new(p[1][0], p[1][1], p[1][2]),
        ));
    }
    snapshot
}

fn main() {
    let stdin = stdin();

    println!("Hello, world!");
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_part1() {
        let mut reader = BufReader::new(File::open("resources/input_test").unwrap());
        let mut snapshot = build_snapshot(&mut reader);

        assert_eq!(safely_disintegrated_count(&snapshot), 5);
    }
}
