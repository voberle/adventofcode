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
    fs::File,
    io::{self, BufRead, BufReader},
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    if pos.y > 0 && grid[pos.y - 1][pos.x].directions().contains(&TO_SOUTH) {
        // If above has an element that goes south, we should consider all guesses that go north
        set = intersec_with(&mut set, [Pipe::Vertical, Pipe::NorthEast, Pipe::NorthWest]);
    };
    if pos.y < grid.len() - 1 && grid[pos.y + 1][pos.x].directions().contains(&TO_NORTH) {
        set = intersec_with(&mut set, [Pipe::Vertical, Pipe::SouthEast, Pipe::SouthWest]);
    };
    if pos.x > 0 && grid[pos.y][pos.x - 1].directions().contains(&TO_EAST) {
        set = intersec_with(
            &mut set,
            [Pipe::Horizontal, Pipe::NorthWest, Pipe::SouthWest],
        );
    };
    if pos.x < grid[0].len() - 1 && grid[pos.y][pos.x + 1].directions().contains(&TO_WEST) {
        set = intersec_with(
            &mut set,
            [Pipe::Horizontal, Pipe::NorthEast, Pipe::SouthEast],
        );
    };
    assert_eq!(set.len(), 1);
    *set.iter().next().unwrap()
}

// Finds the location of the start position, figure out what pipe it is,
// and change it to the correct pipe in the grid.
fn find_and_update_start(grid: &mut Vec<Vec<Pipe>>) -> Position {
    // Find position of starting pipe
    let start: Position = find_start(&grid).unwrap();
    // print_grid(&grid, &vec![start]);

    // and replace that spot in the grid with the real pipe
    let guessed_start: Pipe = guess_start(&grid, start);
    println!(
        "Guessed start for ({},{}) is {}",
        start.y, start.x, guessed_start
    );
    grid[start.y][start.x] = guessed_start;
    start
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

// TODO uses slices
fn print_grid(grid: &Vec<Vec<Pipe>>, highlight_pos: &[Position]) {
    for (y, row) in grid.iter().enumerate() {
        for (x, el) in row.iter().enumerate() {
            if highlight_pos
                .iter()
                .find(|p| **p == Position::new(y, x))
                .is_some()
            {
                print!("\x1b[92m{}\x1b[0m", *el);
            } else {
                print!("{}", *el);
            }
        }
        println!("");
    }
}

// Find the loop (part 1)
fn find_loop(grid: &Vec<Vec<Pipe>>, start: Position) -> Vec<Position> {
    // We could move in both direction to do only half the iterations,
    // but it adds in complexity for minimal gain.
    let mut prev: Position = start;
    let mut curr: Position = next_pipes(&grid, &prev)[0];
    // Starting at 1, as curr is already set to next pipe
    // let mut count: usize = 1;
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
        // count += 1;
    }
    loop_pipe
}

// The animal can be anywhere that is not under our loop, not only under grounds,
// but also under pipes that are not part of the loop.
fn in_loop(loop_pipe: &[Position], pos: Position) -> bool {
    loop_pipe.iter().find(|p| **p == pos).is_some()
}

fn count_enclosed_area(grid: &Vec<Vec<Pipe>>, loop_pipe: &[Position]) -> usize {
    // We don't which way to take the loop, so try one way and if it fails, try the other way
    if let Ok(enclosed_area_total) = count_enclosed_area_one_way(grid, loop_pipe) {
        enclosed_area_total
    } else {
        let mut rev_loop_pipe: Vec<Position> = loop_pipe.into();
        rev_loop_pipe.reverse();
        if let Ok(enclosed_area_total) = count_enclosed_area_one_way(grid, &rev_loop_pipe) {
            enclosed_area_total
        } else {
            panic!("Neither direction worked");
        }
    }
}

fn count_enclosed_area_one_way(
    grid: &Vec<Vec<Pipe>>,
    loop_pipe: &[Position],
) -> Result<usize, &'static str> {
    // Follow the line one direction and save all the dots on the left of the line
    // from that direction's perspective

    // we start from top left corner, so we know the direction and side
    let min_y = loop_pipe.iter().map(|p| p.y).min().unwrap();
    let min_x = loop_pipe
        .iter()
        .filter(|p| p.y == min_y)
        .map(|p| p.x)
        .min()
        .unwrap();
    let start = Position::new(min_y, min_x);
    println!("Start {:?}", start);
    // normally start should always be a SouthEast one
    assert_eq!(grid[start.y][start.x], Pipe::SouthEast);

    let mut set: HashSet<Position> = HashSet::new();

    let mut prev: Position = start;
    let mut started = false;
    for p in loop_pipe.iter().cycle() {
        if !started {
            if *p != start {
                continue;
            } else {
                started = true;
            }
        } else if *p == start {
            break;
        }
        // println!("p {:?}", p);

        let pipe = grid[p.y][p.x];
        if (pipe == Pipe::Horizontal || pipe == Pipe::SouthWest || pipe == Pipe::SouthEast)
            && prev.x < p.x
            && p.y > 0
        {
            // look up
            let mut y = p.y - 1;
            while !in_loop(loop_pipe, Position::new(y, p.x)) {
                // println!("{}, {}: Up {},{}", p.y, p.x, y, p.x);
                set.insert(Position::new(y, p.x));
                if y == 0 {
                    return Err("Wrong dir");
                }
                y -= 1;
            }
        } else if (pipe == Pipe::Horizontal || pipe == Pipe::NorthEast || pipe == Pipe::NorthWest)
            && prev.x > p.x
            && p.y < grid.len() - 1
        {
            // look below
            let mut y = p.y + 1;
            while !in_loop(loop_pipe, Position::new(y, p.x)) {
                // println!("{}, {}: Below {},{}", p.y, p.x, y, p.x);
                set.insert(Position::new(y, p.x));
                if y == grid.len() - 1 {
                    return Err("Wrong dir");
                }
                y += 1;
            }
        }
        if (pipe == Pipe::Vertical || pipe == Pipe::NorthWest || pipe == Pipe::SouthWest)
            && prev.y < p.y
            && p.x < grid[0].len() - 1
        {
            // look right
            let mut x = p.x + 1;
            while !in_loop(loop_pipe, Position::new(p.y, x)) {
                // println!("{}, {}: Right {},{}", p.y, p.x, p.y, x);
                set.insert(Position::new(p.y, x));
                if x == grid[0].len() - 1 {
                    return Err("Wrong dir");
                }
                x += 1;
            }
        } else if (pipe == Pipe::Vertical || pipe == Pipe::SouthEast || pipe == Pipe::NorthEast)
            && prev.y > p.y
            && p.x > 0
        {
            // look left
            let mut x = p.x - 1;
            while !in_loop(loop_pipe, Position::new(p.y, x)) {
                // println!("{}, {}: Left {},{}", p.y, p.x, p.y, x);
                set.insert(Position::new(p.y, x));
                if x == 0 {
                    return Err("Wrong dir");
                }
                x -= 1;
            }
        }
        prev = *p;
    }

    // here we seem to have the areas, but incomplete (not sure why)
    // so let's add the missing pieces
    let v: Vec<_> = set.clone().into_iter().collect();
    for p in v {
        if p.x > 0 && !in_loop(loop_pipe, Position::new(p.y, p.x - 1)) {
            set.insert(Position::new(p.y, p.x - 1));
        }
        if p.x < grid[0].len() - 1 && !in_loop(loop_pipe, Position::new(p.y, p.x + 1)) {
            set.insert(Position::new(p.y, p.x + 1));
        }
        if p.y > 0 && !in_loop(loop_pipe, Position::new(p.y - 1, p.x)) {
            set.insert(Position::new(p.y - 1, p.x));
        }
        if p.y < grid.len() - 1 && !in_loop(loop_pipe, Position::new(p.y + 1, p.x)) {
            set.insert(Position::new(p.y + 1, p.x));
        }
    }

    let total = set.len();
    print_grid(grid, &Vec::from_iter(set));
    Ok(total)
}

fn main() {
    let stdin = io::stdin();
    let mut grid: Vec<Vec<Pipe>> = build_grid(&mut stdin.lock());

    let start = find_and_update_start(&mut grid);

    let loop_pipe: Vec<Position> = find_loop(&grid, start);
    println!("Part 1: {}", loop_pipe.len() / 2);

    print_grid(&grid, &loop_pipe);
    // loop_pipe.reverse();
    println!("Part 2: {}", count_enclosed_area(&grid, &loop_pipe));
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

fn part1(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut grid: Vec<Vec<Pipe>> = build_grid(&mut reader);
    let start = find_and_update_start(&mut grid);

    let loop_pipe: Vec<Position> = find_loop(&grid, start);
    loop_pipe.len() / 2
}

#[test]
fn test_part1() {
    assert_eq!(part1("resources/input_test1"), 4);
    assert_eq!(part1("resources/input_test2"), 8);
    assert_eq!(part1("resources/input_puzzle"), 6754);
}

fn part2(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut grid: Vec<Vec<Pipe>> = build_grid(&mut reader);
    let start = find_and_update_start(&mut grid);

    let loop_pipe: Vec<Position> = find_loop(&grid, start);
    count_enclosed_area(&grid, &loop_pipe)
}

#[test]
fn test_part2() {
    assert_eq!(part2("resources/input_test3"), 4);
    assert_eq!(part2("resources/input_test4"), 4);
    assert_eq!(part2("resources/input_test5"), 8);
    assert_eq!(part2("resources/input_test6"), 10);
}
