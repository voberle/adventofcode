use std::{
    fmt::Display,
    io::{self, Read},
    usize,
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

impl Facing {
    fn get_value(self) -> usize {
        match self {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
    }

    fn turn(self, t: PathTurn) -> Self {
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
struct PosDir {
    x: usize,
    y: usize,
    facing: Facing,
}

impl PosDir {
    fn new(x: usize, y: usize, facing: Facing) -> Self {
        Self { x, y, facing }
    }

    fn calc_password(&self) -> usize {
        1000 * self.y + 4 * self.x + self.facing.get_value()
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
    fn print_with_pos(&self, pos_dir: Option<PosDir>) {
        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";
        for (y, line) in self.tiles.iter().enumerate() {
            for (x, t) in line.iter().enumerate() {
                if let Some(pos) = pos_dir {
                    if pos.x == x && pos.y == y {
                        print!("{RED}{}{RESET}", pos.facing);
                        continue;
                    }
                }
                print!("{}", t);
            }
            println!();
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        self.print_with_pos(None);
    }

    fn start_position(&self) -> PosDir {
        PosDir::new(
            self.tiles[1].iter().position(|t| *t == Tile::Open).unwrap(),
            1,
            Facing::Right,
        )
    }

    fn line_first_pos(&self, y: usize) -> usize {
        self.tiles[y].iter().position(|t| *t != Tile::Void).unwrap()
    }

    fn line_last_pos(&self, y: usize) -> usize {
        self.tiles[y]
            .iter()
            .enumerate()
            .skip(self.line_first_pos(y))
            .take_while(|&(_, tile)| tile != &Tile::Void)
            .map(|(x, _)| x)
            .last()
            .unwrap()
    }

    fn column_first_pos(&self, x: usize) -> usize {
        self.tiles
            .iter()
            .position(|row| row[x] != Tile::Void)
            .unwrap()
    }

    fn column_last_pos(&self, x: usize) -> usize {
        self.tiles
            .iter()
            .enumerate()
            .skip(self.column_first_pos(x))
            .take_while(|&(_, row)| row[x] != Tile::Void)
            .map(|(y, _)| y)
            .last()
            .unwrap()
    }

    #[allow(clippy::match_on_vec_items)]
    fn move_to(&self, pos_dir: &PosDir, steps: usize, wrapping: &impl Wrapping) -> PosDir {
        let mut p = *pos_dir;
        for _ in 0..steps {
            // println!("{:?}", p);
            // self.print_with_pos(Some(p));
            match p.facing {
                Facing::Left => match self.tiles[p.y][p.x - 1] {
                    Tile::Open => p.x -= 1,
                    Tile::Wall => break, // stop moving, continue with next instruction
                    Tile::Void => {
                        let new_pd = wrapping.left(self, &p);
                        if self.tiles[new_pd.y][new_pd.x] == Tile::Wall {
                            break;
                        }
                        p = new_pd;
                    }
                },
                Facing::Right => match self.tiles[p.y][p.x + 1] {
                    Tile::Open => p.x += 1,
                    Tile::Wall => break,
                    Tile::Void => {
                        let new_pd = wrapping.right(self, &p);
                        if self.tiles[new_pd.y][new_pd.x] == Tile::Wall {
                            break;
                        }
                        p = new_pd;
                    }
                },
                Facing::Up => match self.tiles[p.y - 1][p.x] {
                    Tile::Open => p.y -= 1,
                    Tile::Wall => break,
                    Tile::Void => {
                        let new_pd = wrapping.up(self, &p);
                        if self.tiles[new_pd.y][new_pd.x] == Tile::Wall {
                            break;
                        }
                        p = new_pd;
                    }
                },
                Facing::Down => match self.tiles[p.y + 1][p.x] {
                    Tile::Open => p.y += 1,
                    Tile::Wall => break,
                    Tile::Void => {
                        let new_pd = wrapping.down(self, &p);
                        if self.tiles[new_pd.y][new_pd.x] == Tile::Wall {
                            break;
                        }
                        p = new_pd;
                    }
                },
            }
        }
        p
    }

    fn follow_path(&self, path: &[PathItem], wrapping: &impl Wrapping) -> PosDir {
        let mut pos_dir = self.start_position();

        for p in path {
            match p {
                PathItem::Number(steps) => {
                    pos_dir = self.move_to(&pos_dir, *steps, wrapping);
                }
                PathItem::Turn(turn) => {
                    pos_dir.facing = pos_dir.facing.turn(*turn);
                }
            }
        }
        pos_dir
    }
}

trait Wrapping {
    fn left(&self, map: &Map, pos_dir: &PosDir) -> PosDir;
    fn right(&self, map: &Map, pos_dir: &PosDir) -> PosDir;
    fn up(&self, map: &Map, pos_dir: &PosDir) -> PosDir;
    fn down(&self, map: &Map, pos_dir: &PosDir) -> PosDir;
}

struct FlatModel;

impl Wrapping for FlatModel {
    fn left(&self, map: &Map, pos_dir: &PosDir) -> PosDir {
        let x = map.line_last_pos(pos_dir.y);
        PosDir::new(x, pos_dir.y, pos_dir.facing)
    }

    fn right(&self, map: &Map, pos_dir: &PosDir) -> PosDir {
        let x = map.line_first_pos(pos_dir.y);
        PosDir::new(x, pos_dir.y, pos_dir.facing)
    }

    fn up(&self, map: &Map, pos_dir: &PosDir) -> PosDir {
        let y = map.column_last_pos(pos_dir.x);
        PosDir::new(pos_dir.x, y, pos_dir.facing)
    }

    fn down(&self, map: &Map, pos_dir: &PosDir) -> PosDir {
        let y = map.column_first_pos(pos_dir.x);
        PosDir::new(pos_dir.x, y, pos_dir.facing)
    }
}

struct Cube {
    len: usize,
}

impl Cube {
    fn new(len: usize) -> Self {
        Self { len }
    }

    fn calc_offset(&self, v: usize) -> usize {
        (v - 1) % self.len
    }

    fn cube_number(&self, v: usize) -> usize {
        (v - 1) / self.len
    }

    fn horiz(&self, offset: usize, x: usize, y: usize, facing: Facing) -> PosDir {
        assert!([Facing::Up, Facing::Down].contains(&facing));
        let x = 1 + x * self.len + offset;
        let y = if facing == Facing::Up {
            y * self.len
        } else {
            1 + y * self.len
        };
        PosDir::new(x, y, facing)
    }

    fn horiz_rev(&self, offset: usize, x: usize, y: usize, facing: Facing) -> PosDir {
        assert!([Facing::Up, Facing::Down].contains(&facing));
        let x = 1 + (x + 1) * self.len - 1 - offset;
        let y = if facing == Facing::Up {
            y * self.len
        } else {
            1 + y * self.len
        };
        PosDir::new(x, y, facing)
    }

    fn vert(&self, offset: usize, x: usize, y: usize, facing: Facing) -> PosDir {
        assert!([Facing::Left, Facing::Right].contains(&facing));
        let x = if facing == Facing::Left {
            x * self.len
        } else {
            1 + x * self.len
        };
        let y = 1 + y * self.len + offset;
        PosDir::new(x, y, facing)
    }

    fn vert_rev(&self, offset: usize, x: usize, y: usize, facing: Facing) -> PosDir {
        assert!([Facing::Left, Facing::Right].contains(&facing));
        let x = if facing == Facing::Left {
            x * self.len
        } else {
            1 + x * self.len
        };
        let y = 1 + (y + 1) * self.len - 1 - offset;
        PosDir::new(x, y, facing)
    }
}

// Cube numbers:
//   1
// 234
//   56
struct TestCubeModel(Cube);

impl TestCubeModel {
    const LEN: usize = 4;

    fn new() -> Self {
        Self(Cube::new(Self::LEN))
    }
}

impl Wrapping for TestCubeModel {
    fn left(&self, _map: &Map, pos_dir: &PosDir) -> PosDir {
        assert_eq!(pos_dir.facing, Facing::Left);
        let offset = self.0.calc_offset(pos_dir.y);
        match self.0.cube_number(pos_dir.y) {
            0 => {
                // Cube 1 => Cube 3 down
                self.0.horiz(offset, 1, 1, Facing::Down)
            }
            1 => {
                // Cube 2 => Cube 6 up
                self.0.horiz_rev(offset, 3, 2, Facing::Up)
            }
            2 => {
                // Cube 5 => Cube 3 up
                self.0.horiz_rev(offset, 1, 2, Facing::Up)
            }
            _ => panic!("Wrapping error"),
        }
    }

    fn right(&self, _map: &Map, pos_dir: &PosDir) -> PosDir {
        assert_eq!(pos_dir.facing, Facing::Right);
        let offset = self.0.calc_offset(pos_dir.y);
        match self.0.cube_number(pos_dir.y) {
            0 => {
                // Cube 1 => Cube 6 left
                self.0.vert_rev(offset, 4, 2, Facing::Left)
            }
            1 => {
                // Cube 4 => Cube 6 down
                self.0.horiz_rev(offset, 3, 2, Facing::Down)
            }
            2 => {
                // Cube 6 => Cube 1 left
                self.0.vert_rev(offset, 3, 0, Facing::Left)
            }
            _ => panic!("Wrapping error"),
        }
    }

    fn up(&self, _map: &Map, pos_dir: &PosDir) -> PosDir {
        assert_eq!(pos_dir.facing, Facing::Up);
        let offset = self.0.calc_offset(pos_dir.x);
        match self.0.cube_number(pos_dir.x) {
            0 => {
                // Cube 2 => Cube 1 down
                self.0.horiz_rev(offset, 2, 0, Facing::Down)
            }
            1 => {
                // Cube 3 => Cube 1 right
                self.0.vert(offset, 2, 0, Facing::Right)
            }
            2 => {
                // Cube 1 => Cube 2 down
                self.0.horiz_rev(offset, 0, 1, Facing::Down)
            }
            3 => {
                // Cube 6 => Cube 4 left
                self.0.vert_rev(offset, 3, 1, Facing::Left)
            }
            _ => panic!("Wrapping error"),
        }
    }

    fn down(&self, _map: &Map, pos_dir: &PosDir) -> PosDir {
        assert_eq!(pos_dir.facing, Facing::Down);
        let offset = self.0.calc_offset(pos_dir.x);
        match self.0.cube_number(pos_dir.x) {
            0 => {
                // Cube 2 => Cube 5 up
                self.0.horiz_rev(offset, 2, 3, Facing::Up)
            }
            1 => {
                // Cube 3 => Cube 5 right
                self.0.vert_rev(offset, 2, 2, Facing::Right)
            }
            2 => {
                // Cube 5 => Cube 2 up
                self.0.horiz_rev(offset, 0, 2, Facing::Up)
            }
            3 => {
                // Cube 6 => Cube 2 right
                self.0.vert_rev(offset, 0, 1, Facing::Right)
            }
            _ => panic!("Wrapping error"),
        }
    }
}

fn build(input: &str) -> (Map, Vec<PathItem>) {
    let (m, p) = input.split("\n\n").collect_tuple().unwrap();
    (m.into(), build_path(p))
}

fn final_password(map: &Map, path: &[PathItem]) -> usize {
    let pos_dir = map.follow_path(path, &FlatModel);
    pos_dir.calc_password()
}

fn final_password_on_cube(map: &Map, path: &[PathItem]) -> usize {
    // let pos_dir = map.follow_path(path, &RealCubeModel);
    let pos_dir = map.follow_path(path, &TestCubeModel::new());
    pos_dir.calc_password()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (map, path) = build(&input);

    println!("Part 1: {}", final_password(&map, &path));
    println!("Part 2: {}", final_password_on_cube(&map, &path));
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

    fn final_password_on_cube_test(map: &Map, path: &[PathItem]) -> usize {
        let pos_dir = map.follow_path(path, &TestCubeModel::new());
        pos_dir.calc_password()
    }

    #[test]
    fn test_part2() {
        let (map, path) = build(INPUT_TEST);
        assert_eq!(final_password_on_cube_test(&map, &path), 5031);
    }
}
