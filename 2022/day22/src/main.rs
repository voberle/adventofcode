use std::{
    fmt::Display,
    io::{self, Read},
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
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

    fn turn(&self, t: PathTurn) -> Self {
        match t {
            PathTurn::Left => match self {
                Facing::Right => Facing::Up,
                Facing::Up => Facing::Left,
                Facing::Left => Facing::Down,
                Facing::Down => Facing::Right,
            },
            PathTurn::Right => match self {
                Facing::Right => Facing::Down,
                Facing::Down => Facing::Left,
                Facing::Left => Facing::Up,
                Facing::Up => Facing::Right,
            },
        }
    }
}

impl Display for Facing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Facing::Right => '>',
                Facing::Down => 'v',
                Facing::Left => '<',
                Facing::Up => '^',
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, PartialEq)]
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

        // Extend all lines to be as long as the longest.
        let max_len = tiles.iter().map(Vec::len).max().unwrap();
        for tile in &mut tiles {
            tile.resize(max_len, Tile::Void);
        }

        Self { tiles }
    }
}

impl Map {
    #[allow(dead_code)]
    fn print_with_pos(&self, positions: &[Pos]) {
        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";
        for (y, line) in self.tiles.iter().enumerate() {
            for (x, t) in line.iter().enumerate() {
                if positions.contains(&Pos::new(x, y)) {
                    print!("{RED}{}{RESET}", t);
                } else {
                    print!("{}", t);
                }
            }
            println!();
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        self.print_with_pos(&[]);
    }

    fn start_position(&self) -> Pos {
        Pos::new(
            self.tiles[1].iter().position(|t| *t == Tile::Open).unwrap(),
            1,
        )
    }

    fn line_first_pos(&self, y: usize) -> usize {
        self.tiles[y].iter().position(|t| *t != Tile::Void).unwrap()
    }

    fn line_last_pos(&self, y: usize) -> usize {
        let mut x = self.line_first_pos(y);
        while self.tiles[y][x] != Tile::Void {
            x += 1;
        }
        x - 1
    }

    fn column_first_pos(&self, x: usize) -> usize {
        let mut y = 0;
        while self.tiles[y][x] == Tile::Void {
            y += 1;
        }
        y
    }

    fn column_last_pos(&self, x: usize) -> usize {
        let mut y = self.column_first_pos(x);
        while self.tiles[y][x] != Tile::Void {
            y += 1;
        }
        y - 1
    }

    #[allow(clippy::match_on_vec_items)]
    fn move_to(&self, pos: &Pos, facing: &Facing, steps: usize) -> Pos {
        let mut x = pos.x;
        let mut y = pos.y;
        match facing {
            Facing::Left => {
                for _ in 0..steps {
                    match self.tiles[y][x - 1] {
                        Tile::Open => x -= 1,
                        Tile::Wall => break, // stop moving, continue with next instruction
                        Tile::Void => {
                            let n_x = self.line_last_pos(y);
                            if self.tiles[y][n_x] == Tile::Wall {
                                break;
                            }
                            x = n_x;
                        }
                    }
                }
            }
            Facing::Right => {
                for _ in 0..steps {
                    match self.tiles[y][x + 1] {
                        Tile::Open => x += 1,
                        Tile::Wall => break,
                        Tile::Void => {
                            let n_x = self.line_first_pos(y);
                            if self.tiles[y][n_x] == Tile::Wall {
                                break;
                            }
                            x = n_x;
                        }
                    }
                }
            }
            Facing::Up => {
                for _ in 0..steps {
                    match self.tiles[y - 1][x] {
                        Tile::Open => y -= 1,
                        Tile::Wall => break,
                        Tile::Void => {
                            let n_y = self.column_last_pos(x);
                            if self.tiles[n_y][x] == Tile::Wall {
                                break;
                            }
                            y = n_y;
                        }
                    }
                }
            }
            Facing::Down => {
                for _ in 0..steps {
                    match self.tiles[y + 1][x] {
                        Tile::Open => y += 1,
                        Tile::Wall => break,
                        Tile::Void => {
                            let n_y = self.column_first_pos(x);
                            if self.tiles[n_y][x] == Tile::Wall {
                                break;
                            }
                            y = n_y;
                        }
                    }
                }
            }
        }
        Pos { x, y }
    }
}

#[derive(Debug)]
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

fn follow_path(map: &Map, path: &[PathItem]) -> (Pos, Facing) {
    let mut pos = map.start_position();
    let mut facing = Facing::Right;

    for p in path {
        match p {
            PathItem::Number(steps) => {
                pos = map.move_to(&pos, &facing, *steps);
            }
            PathItem::Turn(turn) => {
                facing = facing.turn(*turn);
            }
        }
    }

    (pos, facing)
}

fn final_password(map: &Map, path: &[PathItem]) -> usize {
    let (pos, facing) = follow_path(map, path);
    1000 * pos.y + 4 * pos.x + facing.get_value()
}

fn part2(map: &Map, path: &[PathItem]) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (map, path) = build(&input);

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
