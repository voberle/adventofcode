// https://adventofcode.com/2023/day/16

use std::{collections::HashSet, io};
use table::Table;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: i32,
    col: i32,
}

impl Position {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    fn from_usize(row: usize, col: usize) -> Self {
        Self { row: row as i32, col: col as i32}
    }

    // Allows to create a position just outside the table.
    fn negative(idx: usize, dir: DirectionCb, dims: &Position) -> Self {
        match dir {
            RIGHT => Self::new(idx as i32, -1),
            LEFT => Self::new(idx as i32, dims.col),
            DOWN => Self::new(-1, idx as i32),
            UP => Self::new(dims.row, idx as i32),
            _ => {
                panic!("Invalid direction {:?}", dir);
            }
        }
    }

    fn row(&self) -> usize {
        self.row as usize
    }

    fn col(&self) -> usize {
        self.col as usize
    }
}

type DirectionCb = fn(&Position, &Position) -> Option<Position>;

const LEFT: DirectionCb = |p: &Position, _: &Position| {
    if p.col == 0 {
        return None;
    }
    Some(Position::new(p.row, p.col - 1))
};

const RIGHT: DirectionCb = |p: &Position, dims: &Position| {
    if p.col >= dims.col - 1 {
        return None;
    }
    Some(Position::new(p.row, p.col + 1))
};

const UP: DirectionCb = |p: &Position, _: &Position| {
    if p.row == 0 {
        return None;
    }
    Some(Position::new(p.row - 1, p.col))
};

const DOWN: DirectionCb = |p: &Position, dims: &Position| {
    if p.row >= dims.row - 1 {
        return None;
    }
    Some(Position::new(p.row + 1, p.col))
};

#[test]
fn test_move_functions() {
    let dims: Position = Position::new(20, 10);
    let p = Position::new(2, 3);
    assert_eq!(LEFT(&p, &dims), Some(Position::new(2, 2)));
    assert_eq!(RIGHT(&p, &dims), Some(Position::new(2, 4)));
    assert_eq!(UP(&p, &dims), Some(Position::new(1, 3)));
    assert_eq!(DOWN(&p, &dims), Some(Position::new(3, 3)));
    let p1 = Position::new(0, 0);
    assert_eq!(LEFT(&p1, &dims), None);
    assert_eq!(UP(&p1, &dims), None);
    let p2 = Position::new(19, 9);
    assert_eq!(RIGHT(&p2, &dims), None);
    assert_eq!(DOWN(&p2, &dims), None);
}

// We find the next directions to go by knowing which element we are on and from which direction we come.
fn next_directions(next_elt: char, direction: DirectionCb) -> Vec<DirectionCb> {
    match next_elt {
        '.' => vec![direction],
        '/' => match direction {
            LEFT => vec![DOWN],
            RIGHT => vec![UP],
            UP => vec![RIGHT],
            DOWN => vec![LEFT],
            _ => {
                panic!("Invalid direction {:?}", direction);
            }
        },
        '\\' => match direction {
            LEFT => vec![UP],
            RIGHT => vec![DOWN],
            UP => vec![LEFT],
            DOWN => vec![RIGHT],
            _ => {
                panic!("Invalid direction {:?}", direction);
            }
        },
        '|' => match direction {
            LEFT => vec![UP, DOWN],
            RIGHT => vec![UP, DOWN],
            UP => vec![direction],
            DOWN => vec![direction],
            _ => {
                panic!("Invalid direction {:?}", direction);
            }
        },
        '-' => match direction {
            UP => vec![LEFT, RIGHT],
            DOWN => vec![LEFT, RIGHT],
            LEFT => vec![direction],
            RIGHT => vec![direction],
            _ => {
                panic!("Invalid direction {:?}", direction);
            }
        },
        _ => {
            panic!("Invalid cave element {}", next_elt);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct DirectedPos {
    position: Position,
    direction: DirectionCb,
}

impl DirectedPos {
    fn new(position: Position, direction: DirectionCb) -> Self {
        Self {
            position,
            direction,
        }
    }
}

fn move_beam(
    cave: &Table<char>,
    dir_pos: DirectedPos,
    energized_points: &mut HashSet<DirectedPos>,
) {
    let dims = Position::from_usize(cave.height, cave.width);

    let mut position = dir_pos.position;
    let mut directions: Vec<DirectionCb> = vec![dir_pos.direction];
    assert!(!directions.is_empty());

    while let Some(next_pos) = directions[0](&position, &dims) {
        position = next_pos;
        let next_elt = cave.elt(position.row(), position.col());
        directions = next_directions(*next_elt, directions[0]);
        assert!(!directions.is_empty());

        if directions.len() == 1 {
            // If we have only one direction, we add it and go next, no recursion
            let dp = DirectedPos::new(position, directions[0]);
            if energized_points.contains(&dp) {
                break;
            }
            energized_points.insert(dp);
        } else {
            for d in directions {
                let dp = DirectedPos::new(position, d);
                if !energized_points.contains(&dp) {
                    energized_points.insert(dp);
                    move_beam(cave, dp, energized_points);
                }
            }
            break;
        }
    }
}

fn energized_count(cave: &Table<char>) -> usize {
    let dims = Position::from_usize(cave.height, cave.width);
    let initial_dir_pos = DirectedPos::new(Position::negative(0, RIGHT, &dims), RIGHT);

    let mut energized_points: HashSet<DirectedPos> = HashSet::new();
    move_beam(cave, initial_dir_pos, &mut energized_points);
    // print_cave(&cave, &energized_points);
    // print_energized_cave(&cave, &energized_points);

    // we must count only the points, not the directions
    energized_points
        .iter()
        .map(|dp| dp.position)
        .collect::<HashSet<Position>>()
        .len()
}

fn print_cave(cave: &Table<char>, energized_points: &HashSet<DirectedPos>) {
    for row in 0..cave.height {
        for col in 0..cave.width {
            let el = cave.elt(row, col);
            if energized_points
                .iter()
                .find(|&dp| dp.position.row() == row && dp.position.col() == col)
                .is_some()
            {
                print!("\x1b[91m{}\x1b[0m", *el);
            } else {
                print!("{}", *el);
            }
        }
        println!("");
    }
}

fn print_energized_cave(cave: &Table<char>, energized_points: &HashSet<DirectedPos>) {
    for row in 0..cave.height {
        for col in 0..cave.width {
            if energized_points
                .iter()
                .find(|&dp| dp.position.row() == row && dp.position.col() == col)
                .is_some()
            {
                print!("\x1b[91m#\x1b[0m");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn main() {
    let stdin = io::stdin();
    let cave = Table::build(&mut stdin.lock());

    println!("Part 1: {}", energized_count(&cave));
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_part1() {
        let mut reader = BufReader::new(File::open("resources/input_test").unwrap());
        let cave = Table::build(&mut reader);
        println!("{}", cave);
        assert_eq!(energized_count(&cave), 46);
    }
}
