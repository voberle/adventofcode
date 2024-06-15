use std::{
    fmt::Display,
    io::{self, Read},
};

use itertools::Itertools;

enum Facing {
    Right,
    Down,
    Left,
    Up,
}

impl Facing {
    fn get_value(&self) -> usize {
        match self {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Open,
    Wall,
    Void, // what's around the map.
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Open,
            '#' => Tile::Wall,
            ' ' => Tile::Void,
            _ => panic!("Invalid tile char '{}'", value),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Open => '.',
                Tile::Wall => '#',
                Tile::Void => ' ',
            }
        )
    }
}

// The map is not the usual rectangle. For simplicity,
// we don't use the usual single vector, but more simple vec of vec.
// We add a whitespace all around the map, to act as border.
// Note that all lines don't need to have the same length.
struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut tiles = Vec::new();
        tiles.push(vec![]);
        for line in value.lines() {
            let mut v = vec![Tile::Void];
            v.extend(line.chars().map(Into::<Tile>::into));
            v.push(Tile::Void);
            tiles.push(v);
        }
        tiles.push(vec![]);

        // Make sure first and last lines are long enough.
        let max_len = tiles.iter().map(Vec::len).max().unwrap();
        tiles.first_mut().unwrap().resize(max_len, Tile::Void);
        tiles.last_mut().unwrap().resize(max_len, Tile::Void);

        Self { tiles }
    }
}

impl Map {
    #[allow(dead_code)]
    fn print(&self) {
        for line in &self.tiles {
            println!("{}", line.iter().join(""));
        }
    }
}

enum PathTurn {
    Left,
    Right,
}

impl From<char> for PathTurn {
    fn from(value: char) -> Self {
        match value {
            'L' => PathTurn::Left,
            'R' => PathTurn::Right,
            _ => panic!("Invalid turn char"),
        }
    }
}

enum PathItem {
    Number(usize),
    Turn(PathTurn),
}

fn build_path(input: &str) -> Vec<PathItem> {
    let mut result = Vec::new();
    let mut current_number = String::new();

    for c in input.chars() {
        if c.is_ascii_digit() {
            current_number.push(c);
        } else {
            if !current_number.is_empty() {
                result.push(PathItem::Number(current_number.parse().unwrap()));
                current_number.clear();
            }

            result.push(PathItem::Turn(c.into()));
        }
    }

    if !current_number.is_empty() {
        result.push(PathItem::Number(current_number.parse().unwrap()));
    }

    result
}

fn build(input: &str) -> (Map, Vec<PathItem>) {
    let (m, p) = input.split("\n\n").collect_tuple().unwrap();
    (m.into(), build_path(p))
}

fn final_password(map: &Map, path: &[PathItem]) -> usize {
    0
}

fn part2(map: &Map, path: &[PathItem]) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (map, path) = build(&input);
    // map.print();

    println!("Part 1: {}", final_password(&map, &path));
    println!("Part 2: {}", part2(&map, &path));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (map, path) = build(INPUT_TEST);
        assert_eq!(final_password(&map, &path), 6032);
    }

    #[test]
    fn test_part2() {
        let (map, path) = build(INPUT_TEST);
        assert_eq!(part2(&map, &path), 0);
    }
}
