use std::{
    fmt,
    io::{self, Read},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::{Down, Left, Right, Up};

impl Direction {
    fn index(self) -> usize {
        match self {
            Up => 0,
            Down => 1,
            Left => 2,
            Right => 3,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Up => 'U',
                Down => 'D',
                Left => 'L',
                Right => 'R',
            }
        )
    }
}

// Note that the order is relevant here, corresponds to the positions of the hash characters used
const ALL_DIRECTIONS: [Direction; 4] = [Up, Down, Left, Right];

fn hash(passcode: &str, path: &str) -> String {
    let digest = md5::compute(format!("{}{}", passcode, path).as_bytes());
    format!("{:x}", digest)
}

fn open_doors(passcode: &str, path: &str) -> Vec<bool> {
    const OPEN_CHARS: [char; 5] = ['b', 'c', 'd', 'e', 'f'];
    let h = hash(passcode, path);
    h.chars().take(4).map(|c| OPEN_CHARS.contains(&c)).collect()
}

// Coordinates for a 4x4 grid
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.row, self.col)
    }
}

impl Position {
    const ROWS: usize = 4;
    const COLS: usize = 4;

    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn allowed(&self, direction: Direction) -> bool {
        match direction {
            Up => self.row > 0,
            Down => self.row < Self::ROWS - 1,
            Left => self.col > 0,
            Right => self.col < Self::COLS - 1,
        }
    }

    // Assumes validity of the move has been checked before with `allowed`.
    fn next_pos(&self, direction: Direction) -> Self {
        match direction {
            Up => Self::new(self.row - 1, self.col),
            Down => Self::new(self.row + 1, self.col),
            Left => Self::new(self.row, self.col - 1),
            Right => Self::new(self.row, self.col + 1),
        }
    }
}

// Recursive function.
fn find_shortest_path(
    passcode: &str,
    goal: &Position,
    current_pos: &Position,
    current_path: &str,
    shortest_path_found: &mut String,
) {
    let door_states = open_doors(passcode, current_path);

    // println!("{} {}: {:?}", current_pos, current_path, door_states);
    for dir in ALL_DIRECTIONS {
        if !current_pos.allowed(dir) {
            // Not allowed: Going outside grid
            continue;
        }

        if !door_states[dir.index()] {
            // Door closed
            continue;
        }

        let next_pos = current_pos.next_pos(dir);
        let next_path = current_path.to_owned() + &dir.to_string();
        // println!("{} {}: {:?} - {}. Goal {}", current_pos, current_path, dir, next_pos, goal);

        if next_pos == *goal {
            // println!("Reached, new_path {}, vs {}", next_path, shortest_path_found);
            if shortest_path_found.is_empty() || next_path.len() < shortest_path_found.len() {
                *shortest_path_found = next_path.to_string();
            }
            continue;
        }

        find_shortest_path(passcode, goal, &next_pos, &next_path, shortest_path_found);
    }
}

fn shortest_path(passcode: &str) -> String {
    let mut shortest_path_found = String::new();
    find_shortest_path(
        passcode,
        &Position::new(3, 3),
        &Position::new(0, 0),
        "",
        &mut shortest_path_found,
    );

    shortest_path_found
}

fn part2(passcode: &str) -> i64 {
    0
}

fn main() {
    let mut passcode = String::new();
    io::stdin().read_to_string(&mut passcode).unwrap();

    println!("Part 1: {}", shortest_path(passcode.trim()));
    println!("Part 2: {}", part2(passcode.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_doors() {
        assert_eq!(open_doors("hijkl", ""), &[true, true, true, false]);
        assert_eq!(open_doors("hijkl", "D"), &[true, false, true, true]);
        assert_eq!(open_doors("hijkl", "DR"), &[false, false, false, false]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(shortest_path("ihgpwlah"), "DDRRRD");
        assert_eq!(shortest_path("kglvqrro"), "DDUDRLRRUDRD");
        assert_eq!(shortest_path("ulqzkmiv"), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(""), 0);
    }
}
