use std::{
    collections::HashSet,
    fmt,
    io::{self, Read},
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
            _ => panic!("Invalid tile: '{c}'"),
        }
    }

    // Returns the moves that are valid for this tile, as a list of (y, x) offsets.
    fn directions(self) -> Vec<(i32, i32)> {
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
                Self::Vertical => '┃',
                Self::Horizontal => '━',
                Self::NorthEast => '┗',
                Self::NorthWest => '┛',
                Self::SouthWest => '┓',
                Self::SouthEast => '┏',
                Self::Ground => '.',
                Self::StartingPos => 'S',
            }
        )
        // "-|F7LJ" => "━┃┏┓┗┛"
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

    fn north(&self) -> Result<Self, &'static str> {
        if self.y > 0 {
            Ok(Position::new(self.y - 1, self.x))
        } else {
            Err("Position at max north")
        }
    }

    fn south(&self, line_len: usize) -> Result<Self, &'static str> {
        if self.y < line_len - 1 {
            Ok(Position::new(self.y + 1, self.x))
        } else {
            Err("Position at max south")
        }
    }

    fn west(&self) -> Result<Self, &'static str> {
        if self.x > 0 {
            Ok(Position::new(self.y, self.x - 1))
        } else {
            Err("Position at max east")
        }
    }

    fn east(&self, line_len: usize) -> Result<Self, &'static str> {
        if self.x < line_len - 1 {
            Ok(Position::new(self.y, self.x + 1))
        } else {
            Err("Position at max west")
        }
    }
}

// Finds the position of the S pipe
fn find_start(grid: &[Vec<Pipe>]) -> Option<Position> {
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
#[allow(clippy::cast_sign_loss)]
fn next_pipes(grid: &[Vec<Pipe>], current: &Position) -> Vec<Position> {
    let current_pipe = grid[current.y][current.x];
    let move_offsets = current_pipe.directions();
    assert_eq!(move_offsets.len(), 2);

    let mut result: Vec<Position> = Vec::new();
    for move_offset in move_offsets {
        let y = i32::try_from(current.y).unwrap() + move_offset.0;
        if y < 0 || y >= i32::try_from(grid.len()).unwrap() {
            continue;
        }
        let x = i32::try_from(current.x).unwrap() + move_offset.1;
        if x < 0 || x >= i32::try_from(grid[0].len()).unwrap() {
            continue;
        }
        result.push(Position::new(y as usize, x as usize));
    }
    result
}

fn intersec_with(set: &mut HashSet<Pipe>, with: [Pipe; 3]) -> HashSet<Pipe> {
    set.intersection(&with.into()).copied().collect()
}

// Find which pipe is on the start position, by looking at the pipes around it.
// S in main input
// LJL
// FSF
// |LJ
// We can find the type of S by looking at the tiles around.
// In example above: 7
// Pipes never cross, so it's easy to find main one.
fn guess_start(grid: &[Vec<Pipe>], pos: Position) -> Pipe {
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
fn find_and_update_start(grid: &mut [Vec<Pipe>]) -> Position {
    // Find position of starting pipe
    let start: Position = find_start(grid).unwrap();
    // print_grid(&grid, &vec![start]);

    // and replace that spot in the grid with the real pipe
    let guessed_start: Pipe = guess_start(grid, start);
    // println!(
    //     "Guessed start for ({},{}) is {}",
    //     start.y, start.x, guessed_start
    // );
    grid[start.y][start.x] = guessed_start;
    start
}

fn build_grid(input: &str) -> Vec<Vec<Pipe>> {
    input
        .lines()
        .map(|l| l.chars().map(Pipe::new).collect())
        .collect()
}

#[allow(dead_code)]
fn print_grid(grid: &[Vec<Pipe>], loop_pos: &[Position], area_pos: &[Position], start: &Position) {
    for (y, row) in grid.iter().enumerate() {
        for (x, el) in row.iter().enumerate() {
            let pos = Position::new(y, x);
            // Colors from https://stackoverflow.com/questions/287871/how-do-i-print-colored-text-to-the-terminal/287944#287944
            if *start == pos {
                print!("\x1b[91m{}\x1b[0m", *el);
            } else if loop_pos.iter().any(|p| *p == pos) {
                print!("\x1b[92m{}\x1b[0m", *el);
            } else if area_pos.iter().any(|p| *p == pos) {
                print!("\x1b[93m{}\x1b[0m", *el);
            } else {
                print!("{}", *el);
            }
        }
        println!("\t{y}");
    }
}

// Find the loop (part 1)
fn find_loop(grid: &[Vec<Pipe>], start: Position) -> Vec<Position> {
    // We could move in both direction to do only half the iterations,
    // but it adds in complexity for minimal gain.
    let mut prev: Position = start;
    let mut curr: Position = next_pipes(grid, &prev)[0];
    // Starting at 1, as curr is already set to next pipe
    // let mut count: usize = 1;
    let mut loop_pipe: Vec<Position> = Vec::new();
    loop_pipe.push(curr); // start will be put at the end

    while curr != start {
        let next_pipes1 = next_pipes(grid, &curr);
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
    loop_pipe.iter().any(|p| *p == pos)
}

fn count_enclosed_area(grid: &[Vec<Pipe>], loop_pipe: &[Position], start: &Position) -> usize {
    // We don't which way to take the loop, so try one way and if it fails, try the other way
    if let Ok(enclosed_area_total) = count_enclosed_area_one_way(grid, loop_pipe, start) {
        enclosed_area_total
    } else {
        let mut rev_loop_pipe: Vec<Position> = loop_pipe.into();
        rev_loop_pipe.reverse();
        if let Ok(enclosed_area_total) = count_enclosed_area_one_way(grid, &rev_loop_pipe, start) {
            enclosed_area_total
        } else {
            panic!("Neither direction worked");
        }
    }
}

fn count_enclosed_area_one_way(
    grid: &[Vec<Pipe>],
    loop_pipe: &[Position],
    _start: &Position,
) -> Result<usize, &'static str> {
    // Follow the line in one direction and save all the dots on one side of the line.

    // All the enclosed dots we have found so far
    let mut set: HashSet<Position> = HashSet::new();

    let mut prev: Position = *loop_pipe.last().unwrap();
    let mut next: Position;
    for p in loop_pipe {
        let pipe = grid[p.y][p.x];
        // If the pipe cannot go north, look for possible are north.
        // The second line is when we hit a turn and go opposite site of where we are counting.
        if ([Pipe::Horizontal, Pipe::SouthWest, Pipe::SouthEast].contains(&pipe) && prev.x < p.x)
            || ([Pipe::SouthEast].contains(&pipe) && prev.x == p.x)
        {
            // look north
            if let Ok(next_p) = p.north() {
                next = next_p;
                while !in_loop(loop_pipe, next) {
                    set.insert(next);
                    next = next.north()?; // if we reach the border, it means we are looping in wrong direction
                }
            }
        }
        if ([Pipe::Horizontal, Pipe::NorthEast, Pipe::NorthWest].contains(&pipe) && prev.x > p.x)
            || ([Pipe::NorthWest].contains(&pipe) && prev.x == p.x)
        {
            // look south
            if let Ok(next_p) = p.south(grid.len()) {
                next = next_p;
                while !in_loop(loop_pipe, next) {
                    set.insert(next);
                    next = next.south(grid.len())?;
                }
            }
        }
        if ([Pipe::Vertical, Pipe::NorthWest, Pipe::SouthWest].contains(&pipe) && prev.y < p.y)
            || ([Pipe::SouthWest].contains(&pipe) && prev.y == p.y)
        {
            // look east
            if let Ok(next_p) = p.east(grid[0].len()) {
                next = next_p;
                while !in_loop(loop_pipe, next) {
                    set.insert(next);
                    next = next.east(grid[0].len())?;
                }
            }
        }
        if ([Pipe::Vertical, Pipe::SouthEast, Pipe::NorthEast].contains(&pipe) && prev.y > p.y)
            || ([Pipe::NorthEast].contains(&pipe) && prev.y == p.y)
        {
            // look west
            if let Ok(next_p) = p.west() {
                next = next_p;
                while !in_loop(loop_pipe, next) {
                    set.insert(next);
                    next = next.west()?;
                }
            }
        }
        prev = *p;
    }
    let total = set.len();
    // print_grid(grid, &loop_pipe, &Vec::from_iter(set), _start);
    Ok(total)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut grid: Vec<Vec<Pipe>> = build_grid(&input);

    let start = find_and_update_start(&mut grid);

    let loop_pipe: Vec<Position> = find_loop(&grid, start);
    println!("Part 1: {}", loop_pipe.len() / 2);

    // print_grid(&grid, &loop_pipe, &[]);
    println!("Part 2: {}", count_enclosed_area(&grid, &loop_pipe, &start));
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test2");
    const INPUT_TEST_3: &str = include_str!("../resources/input_test3");
    const INPUT_TEST_4: &str = include_str!("../resources/input_test4");
    const INPUT_TEST_5: &str = include_str!("../resources/input_test5");
    const INPUT_TEST_6: &str = include_str!("../resources/input_test6");

    #[test]
    fn check_guess_start() {
        let g = "\
LJL
FSF
|LJ";
        let grid = build_grid(g);
        assert_eq!(guess_start(&grid, Position::new(1, 1)), Pipe::new('7'));
    }

    #[test]
    fn check_next_pipes() {
        let g = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        let grid = build_grid(g);

        assert_eq!(
            next_pipes(&grid, &Position::new(3, 1)),
            [Position::new(2, 1), Position::new(3, 2)]
        );
    }

    fn part1(input: &str) -> usize {
        let mut grid: Vec<Vec<Pipe>> = build_grid(input);
        let start = find_and_update_start(&mut grid);

        let loop_pipe: Vec<Position> = find_loop(&grid, start);
        loop_pipe.len() / 2
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT_TEST_1), 4);
        assert_eq!(part1(INPUT_TEST_2), 8);
    }

    fn part2(input: &str) -> usize {
        let mut grid: Vec<Vec<Pipe>> = build_grid(input);
        let start = find_and_update_start(&mut grid);

        let loop_pipe: Vec<Position> = find_loop(&grid, start);
        count_enclosed_area(&grid, &loop_pipe, &start)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT_TEST_1), 1);
        assert_eq!(part2(INPUT_TEST_2), 1);
        assert_eq!(part2(INPUT_TEST_3), 4);
        assert_eq!(part2(INPUT_TEST_4), 4);
        assert_eq!(part2(INPUT_TEST_5), 8);
        assert_eq!(part2(INPUT_TEST_6), 10);
    }
}
