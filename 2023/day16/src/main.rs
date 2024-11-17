use std::{collections::HashSet, io};
use std::{fmt, io::Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Table<T>
where
    T: Clone,
    T: From<char>,
{
    arr: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Table<T>
where
    T: Clone,
    T: From<char>,
{
    fn new(arr: Vec<T>, width: usize, height: usize) -> Self {
        assert_eq!(arr.len(), width * height);
        Self { arr, width, height }
    }

    fn empty() -> Self {
        Self::new(Vec::new(), 0, 0)
    }

    fn elt(&self, row: usize, col: usize) -> &T {
        &self.arr[row * self.width + col]
    }

    fn row(&self, row: usize) -> &[T] {
        let idx = row * self.width;
        &self.arr[idx..idx + self.width]
    }

    fn build(input: &str) -> Table<T> {
        let mut p = Table::empty();
        for line in input.lines() {
            p.arr.extend(line.chars().map(std::convert::Into::into));
            p.width = line.len();
            p.height += 1;
        }
        p
    }
}

impl<T> fmt::Display for Table<T>
where
    T: Clone,
    T: From<char>,
    String: for<'a> FromIterator<&'a T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cols={}; Rows={}", self.height, self.width)?;
        for row in 0..self.height {
            writeln!(f, "{}", self.row(row).iter().collect::<String>())?;
        }
        Ok(())
    }
}

#[allow(dead_code)]
fn build_tables<T>(input: &str) -> Vec<Table<T>>
where
    T: Clone,
    T: From<char>,
{
    let mut patterns: Vec<Table<T>> = Vec::new();
    let mut p = Table::empty();
    for line in input.lines() {
        if line.is_empty() {
            patterns.push(p);
            p = Table::empty();
        } else {
            p.arr.extend(line.chars().map(std::convert::Into::into));
            p.width = line.len();
            p.height += 1;
        }
    }
    patterns.push(p); // not forgetting last one
    patterns
}

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
        Self {
            row: i32::try_from(row).unwrap(),
            col: i32::try_from(col).unwrap(),
        }
    }

    // Allows to create a position just outside the table.
    fn negative(idx: usize, dir: Direction, dims: Position) -> Self {
        let i = i32::try_from(idx).unwrap();
        match dir {
            Right => Self::new(i, -1),
            Left => Self::new(i, dims.col),
            Down => Self::new(-1, i),
            Up => Self::new(dims.row, i),
        }
    }

    #[allow(clippy::cast_sign_loss)] // that's not nice, or buggy?
    fn row(self) -> usize {
        self.row as usize
    }

    #[allow(clippy::cast_sign_loss)]
    fn col(self) -> usize {
        self.col as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

use Direction::{Down, Left, Right, Up};

impl Direction {
    fn exec_move(self, p: Position, dims: Position) -> Option<Position> {
        match self {
            Left => {
                if p.col == 0 {
                    return None;
                }
                Some(Position::new(p.row, p.col - 1))
            }
            Right => {
                if p.col >= dims.col - 1 {
                    return None;
                }
                Some(Position::new(p.row, p.col + 1))
            }
            Up => {
                if p.row == 0 {
                    return None;
                }
                Some(Position::new(p.row - 1, p.col))
            }
            Down => {
                if p.row >= dims.row - 1 {
                    return None;
                }
                Some(Position::new(p.row + 1, p.col))
            }
        }
    }
}

#[test]
fn test_move_functions() {
    let dims: Position = Position::new(20, 10);
    let p = Position::new(2, 3);
    assert_eq!(Left.exec_move(p, dims), Some(Position::new(2, 2)));
    assert_eq!(Right.exec_move(p, dims), Some(Position::new(2, 4)));
    assert_eq!(Up.exec_move(p, dims), Some(Position::new(1, 3)));
    assert_eq!(Down.exec_move(p, dims), Some(Position::new(3, 3)));
    let p1 = Position::new(0, 0);
    assert_eq!(Left.exec_move(p1, dims), None);
    assert_eq!(Up.exec_move(p1, dims), None);
    let p2 = Position::new(19, 9);
    assert_eq!(Right.exec_move(p2, dims), None);
    assert_eq!(Down.exec_move(p2, dims), None);
}

// We find the next directions to go by knowing which element we are on and from which direction we come.
fn next_directions(next_elt: char, direction: Direction) -> Vec<Direction> {
    match next_elt {
        '.' => vec![direction],
        '/' => match direction {
            Left => vec![Down],
            Right => vec![Up],
            Up => vec![Right],
            Down => vec![Left],
        },
        '\\' => match direction {
            Left => vec![Up],
            Right => vec![Down],
            Up => vec![Left],
            Down => vec![Right],
        },
        '|' => match direction {
            Left | Right => vec![Up, Down],
            Up | Down => vec![direction],
        },
        '-' => match direction {
            Up | Down => vec![Left, Right],
            Left | Right => vec![direction],
        },
        _ => {
            panic!("Invalid cave element {next_elt}");
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct DirectedPos {
    position: Position,
    direction: Direction,
}

impl DirectedPos {
    fn new(position: Position, direction: Direction) -> Self {
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
    let mut directions: Vec<Direction> = vec![dir_pos.direction];
    assert!(!directions.is_empty());

    while let Some(next_pos) = directions[0].exec_move(position, dims) {
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

fn energized_count_from(cave: &Table<char>, initial_dir_pos: DirectedPos) -> usize {
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

fn energized_count(cave: &Table<char>) -> usize {
    let dims = Position::from_usize(cave.height, cave.width);
    let initial_dir_pos = DirectedPos::new(Position::negative(0, Right, dims), Right);
    energized_count_from(cave, initial_dir_pos)
}

fn highest_energized_count(cave: &Table<char>) -> usize {
    let dims = Position::from_usize(cave.height, cave.width);

    let mut initial_dp: Vec<DirectedPos> = Vec::new();
    for row in 0..cave.height {
        initial_dp.push(DirectedPos::new(
            Position::negative(row, Right, dims),
            Right,
        ));
        initial_dp.push(DirectedPos::new(Position::negative(row, Left, dims), Left));
    }
    for col in 0..cave.width {
        initial_dp.push(DirectedPos::new(Position::negative(col, Down, dims), Down));
        initial_dp.push(DirectedPos::new(Position::negative(col, Up, dims), Up));
    }

    initial_dp
        .iter()
        .map(|dp| energized_count_from(cave, *dp))
        .max()
        .unwrap()
}

#[allow(dead_code)]
fn print_cave(cave: &Table<char>, energized_points: &HashSet<DirectedPos>) {
    for row in 0..cave.height {
        for col in 0..cave.width {
            let el = cave.elt(row, col);
            if energized_points
                .iter()
                .any(|&dp| dp.position.row() == row && dp.position.col() == col)
            {
                print!("\x1b[91m{}\x1b[0m", *el);
            } else {
                print!("{}", *el);
            }
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_energized_cave(cave: &Table<char>, energized_points: &HashSet<DirectedPos>) {
    for row in 0..cave.height {
        for col in 0..cave.width {
            if energized_points
                .iter()
                .any(|&dp| dp.position.row() == row && dp.position.col() == col)
            {
                print!("\x1b[91m#\x1b[0m");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let cave = Table::build(&input);

    println!("Part 1: {}", energized_count(&cave));
    println!("Part 2: {}", highest_energized_count(&cave));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test");

    #[test]
    fn test_part1() {
        let cave = Table::build(INPUT_TEST);
        println!("{cave}");
        assert_eq!(energized_count(&cave), 46);
    }
}
