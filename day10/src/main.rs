// https://adventofcode.com/2023/day/10
// Part 1 test 1: 4
// Part 1 test 2: 8
// Part 1: 6754
// Part 2 test 3: 4
// Part 2 test 4: 4
// Part 2 test 5: 8
// Part 2 test 6: 10

// S in main input
// LJL
// FSF
// |LJ
// We can find the type of S by looking at the tiles around.
// In example above: 7
// Pipes never cross, so it's easy to find main one.

use std::{
    collections::HashSet,
    fmt,
    io::{self, BufRead},
};

const TO_NORTH: (i32, i32) = (-1, 0);
const TO_SOUTH: (i32, i32) = (1, 0);
const TO_WEST: (i32, i32) = (0, -1);
const TO_EAST: (i32, i32) = (0, 1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    StartingPos,
}

impl Pipe {
    fn new(c: char) -> Self {
        match c {
            '|' => Self::Vertical,    // a vertical pipe connecting north and south.
            '-' => Self::Horizontal,  // a horizontal pipe connecting east and west.
            'L' => Self::NorthEast,   // a 90-degree bend connecting north and east.
            'J' => Self::NorthWest,   // a 90-degree bend connecting north and west.
            '7' => Self::SouthWest,   // a 90-degree bend connecting south and west.
            'F' => Self::SouthEast,   // a 90-degree bend connecting south and east.
            '.' => Self::Ground,      // ground; there is no pipe in this tile.
            'S' => Self::StartingPos, // the starting position of the animal;
            // there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
            _ => panic!("Invalid tile: '{}'", c),
        }
    }

    // Returns the moves that are valid for this tile, as a list of (y, x) offsets.
    fn directions(&self) -> Vec<(i32, i32)> {
        match self {
            Self::Vertical => vec![TO_NORTH, TO_SOUTH],
            Self::Horizontal => vec![TO_WEST, TO_EAST],
            Self::NorthWest => vec![TO_NORTH, TO_WEST],
            Self::NorthEast => vec![TO_NORTH, TO_EAST],
            Self::SouthWest => vec![TO_SOUTH, TO_WEST],
            Self::SouthEast => vec![TO_SOUTH, TO_EAST],
            Self::Ground => vec![],
            Self::StartingPos => panic!("valid_moves() cannot be called for StartingPos"),
        }
    }
}

impl fmt::Display for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Vertical => '|',
                Self::Horizontal => '-',
                Self::NorthEast => 'L',
                Self::NorthWest => 'J',
                Self::SouthWest => '7',
                Self::SouthEast => 'F',
                Self::Ground => '.',
                Self::StartingPos => 'S',
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    y: usize,
    x: usize,
}

impl Position {
    fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }
}

// Finds the position of the S pipe
fn find_start(grid: &Vec<Vec<Pipe>>) -> Option<Position> {
    for (y, row) in grid.iter().enumerate() {
        for (x, el) in row.iter().enumerate() {
            if *el == Pipe::StartingPos {
                return Some(Position::new(y, x));
            }
        }
    }
    None
}

// Pipes to which we can go from this one
fn next_pipes(grid: &Vec<Vec<Pipe>>, current: &Position) -> Vec<Position> {
    let current_pipe = grid[current.y][current.x];
    let move_offsets = current_pipe.directions();
    assert_eq!(move_offsets.len(), 2);

    let mut result: Vec<Position> = Vec::new();
    for move_offset in move_offsets {
        let y = current.y as i32 + move_offset.0;
        if y < 0 || y >= grid.len() as i32 {
            continue;
        }
        let x = current.x as i32 + move_offset.1;
        if x < 0 || x >= grid[0].len() as i32 {
            continue;
        }
        result.push(Position::new(y as usize, x as usize));
    }
    result
}

fn intersec_with(set: &mut HashSet<Pipe>, with: [Pipe; 3]) -> HashSet<Pipe> {
    set.intersection(&with.into()).cloned().collect()
}

// Find which pipe is on the start position, by looking at the pipes around it
fn guess_start(grid: &Vec<Vec<Pipe>>, pos: Position) -> Pipe {
    let mut set: HashSet<Pipe> = [
        Pipe::Vertical,
        Pipe::Horizontal,
        Pipe::NorthEast,
        Pipe::NorthWest,
        Pipe::SouthWest,
        Pipe::SouthEast,
    ]
    .into();
    if pos.y > 0 && dbg!(grid[pos.y - 1][pos.x].directions().contains(&TO_SOUTH)) {
        // If above has an element that goes south, we should consider all guesses that go north
        set = intersec_with(&mut set, [Pipe::Vertical, Pipe::NorthEast, Pipe::NorthWest]);
    };
    if pos.y < grid.len() - 1 && dbg!(grid[pos.y + 1][pos.x].directions().contains(&TO_NORTH)) {
        set = intersec_with(&mut set, [Pipe::Vertical, Pipe::SouthEast, Pipe::SouthWest]);
    };
    if pos.x > 0 && dbg!(grid[pos.y][pos.x - 1].directions().contains(&TO_EAST)) {
        set = intersec_with(
            &mut set,
            [Pipe::Horizontal, Pipe::NorthWest, Pipe::SouthWest],
        );
    };
    if pos.x < grid[0].len() - 1 && dbg!(grid[pos.y][pos.x + 1].directions().contains(&TO_WEST)) {
        set = intersec_with(
            &mut set,
            [Pipe::Horizontal, Pipe::NorthEast, Pipe::SouthEast],
        );
    };
    assert_eq!(set.len(), 1);
    *set.iter().next().unwrap()
}

fn build_grid<R>(reader: &mut R) -> Vec<Vec<Pipe>>
where
    R: BufRead,
{
    reader
        .lines()
        .map(|l| l.unwrap().chars().map(|c| Pipe::new(c)).collect())
        .collect()
}

fn print_grid(grid: &Vec<Vec<Pipe>>, highlight_pos: &Vec<Position>) {
    for (y, row) in grid.iter().enumerate() {
        for (x, el) in row.iter().enumerate() {
            if highlight_pos.iter().find(|p| **p == Position::new(y, x)).is_some() {
                print!("\x1b[92m{}\x1b[0m", *el);
            } else {
                print!("{}", *el);
            }
        }
        println!("");
    }
}

fn main() {
    let stdin = io::stdin();
    let mut grid: Vec<Vec<Pipe>> = build_grid(&mut stdin.lock());

    // Find position of starting pipe
    let start: Position = find_start(&grid).unwrap();
    print_grid(&grid, &vec![start]);

    // and replace that spot in the grid with the real pipe
    let guessed_start: Pipe = guess_start(&grid, start);
    println!(
        "Guessed start for ({},{}) is {}",
        start.y, start.x, guessed_start
    );
    grid[start.y][start.x] = guessed_start;
    // print_grid(&grid, start);

    // We could move in both direction to do only half the iterations,
    // but it adds in complexity for minimal gain.
    let mut prev: Position = start;
    let mut curr: Position = next_pipes(&grid, &prev)[0];
    // Starting at 1, as curr is already set to next pipe
    let mut count: usize = 1;
    let mut loop_pipe: Vec<Position> = Vec::new();
    loop_pipe.push(curr); // start will be put at the end

    while curr != start {
        let next_pipes1 = next_pipes(&grid, &curr);
        // println!("------");
        // print_grid(&grid, curr);

        for n in next_pipes1 {
            if n != prev {
                prev = curr;
                curr = n;
                break;
            }
        }
        loop_pipe.push(curr);
        count += 1;
    }

    println!("Part 1: {}", count / 2);

    print_grid(&grid, &loop_pipe);

}

#[test]
fn check_guess_start() {
    let mut g = b"\
LJL
FSF
|LJ" as &[u8];
    let grid = build_grid(&mut g);
    assert_eq!(guess_start(&grid, Position::new(1, 1)), Pipe::new('7'));
}

#[test]
fn check_next_pipes() {
    let mut g = b"\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF" as &[u8];
    let grid = build_grid(&mut g);

    assert_eq!(
        next_pipes(&grid, &Position::new(3, 1)),
        [Position::new(2, 1), Position::new(3, 2)]
    );
}
