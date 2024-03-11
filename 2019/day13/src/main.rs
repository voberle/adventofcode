use std::io::{self, Read};

use intcode::IntcodeComputer;

#[derive(Debug, Clone, Copy)]
enum TileType {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl TileType {
    fn new(v: i64) -> Self {
        match v {
            0 => TileType::Empty,
            1 => TileType::Wall,
            2 => TileType::Block,
            3 => TileType::Paddle,
            4 => TileType::Ball,
            _ => panic!("Invalid tile type {}", v),
        }
    }
}

struct Tile {
    x: usize,
    y: usize,
    r#type: TileType,
}

impl Tile {
    fn new(x: i64, y: i64, t: i64) -> Self {
        Self {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
            r#type: TileType::new(t),
        }
    }

    fn get_type(&self) -> TileType {
        self.r#type
    }
}

fn last_tile(computer: &mut IntcodeComputer) -> Option<Tile> {
    if let Some(tile_type) = computer.io.get_output() {
        let y = computer.io.get_output().unwrap();
        let x = computer.io.get_output().unwrap();
        Some(Tile::new(x, y, tile_type))
    } else {
        None
    }
}

fn draw(computer: &mut IntcodeComputer) -> Vec<Tile> {
    computer.exec();

    let mut tiles = Vec::new();
    while let Some(tile) = last_tile(computer) {
        tiles.push(tile);
    }
    tiles
}

fn block_tiles_count(computer: &IntcodeComputer) -> usize {
    let mut computer = computer.clone();
    let tiles = draw(&mut computer);
    tiles
        .iter()
        .filter(|t| matches!(t.get_type(), TileType::Block))
        .count()
}

fn part2(computer: &IntcodeComputer) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let computer = IntcodeComputer::build(&input);

    println!("Part 1: {}", block_tiles_count(&computer));
    println!("Part 2: {}", part2(&computer));
}
