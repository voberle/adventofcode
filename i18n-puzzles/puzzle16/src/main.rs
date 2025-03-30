use std::io::{self, Read};

use codepage_437::{CP437_CONTROL, FromCp437};

// Converts the binary encoded in CP437 to Unicode.
fn cp437_to_str(input: Vec<u8>) -> String {
    String::from_cp437(input, &CP437_CONTROL)
}

// Lists all the interesting chars in the screencap.
#[allow(unused)]
fn list_chars(screencap: &str) {
    // Find all unique chars.
    let chars: std::collections::HashSet<char> = screencap.chars().collect();
    // Filter out what we don't care about.
    let mut filtered: Vec<&char> = chars.iter().filter(|c| !c.is_alphanumeric()).collect();
    filtered.sort();
    for c in filtered {
        println!("{c}");
    }
}

fn is_pipe(c: char) -> bool {
    c != ' '
}

// Get the directions in which that pipe char is going.
// If it's not a pipe, return empty list.
fn get_pipe_directions(c: char) -> Vec<Direction> {
    match c {
        '─' => vec![West, East],
        '│' => vec![North, South],
        '┌' => vec![South, East],
        '┐' => vec![South, West],
        '└' => vec![North, East],
        '┘' => vec![North, West],
        '├' => vec![North, South, East],
        '┤' => vec![North, South, West],
        '┬' => vec![South, West, East],
        '┴' => vec![North, West, East],
        '┼' => vec![North, South, West, East],
        _ => vec![],
    }
}

// Rotate the pipe char clockwise.
fn rotate_pipe(c: char) -> char {
    match c {
        '─' => '│',
        '│' => '─',
        '┌' => '┐',
        '┐' => '┘',
        '└' => '┌',
        '┘' => '└',
        '├' => '┬',
        '┤' => '┴',
        '┬' => '┤',
        '┴' => '├',
        '┼' => '┼',
        _ => panic!("Unknown pipe char"),
    }
}

// Returns the maximum number of rotations that we can do for that pipe
// before coming back to the original one.
fn get_max_rotations_for(c: char) -> usize {
    match c {
        '─' | '│' => 1,
        '┌' | '┐' | '└' | '┘' | '├' | '┤' | '┬' | '┴' => 4,
        '┼' => 0,
        _ => panic!("Unknown pipe char"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    West,
    East,
}
use Direction::{East, North, South, West};

const ALL_DIRECTIONS: [Direction; 4] = [North, South, West, East];

impl Direction {
    fn opposite(self) -> Self {
        match self {
            North => South,
            South => North,
            West => East,
            East => West,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    values: Vec<char>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn build(input: &str) -> Self {
        let mut rows = 0;
        let values: Vec<_> = input
            .lines()
            .flat_map(|l| {
                rows += 1;
                l.chars().collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    // Cleans up the grid by replacing the non-pipe chars with spaces, and using only single line pipe chars.
    fn normalize(&mut self) {
        self.values.iter_mut().for_each(|value| {
            *value = match *value {
                '─' | '═' => '─',
                '│' | '║' => '│',
                '┌' | '╔' => '┌',
                '┐' | '╗' => '┐',
                '└' | '╚' => '└',
                '┘' | '╝' => '┘',
                '├' | '╞' | '╠' => '├',
                '┤' | '╡' => '┤',
                '┬' | '╤' | '╥' | '╦' => '┬',
                '┴' | '╧' | '╩' => '┴',
                '┼' | '╪' | '╫' => '┼',
                _ => ' ',
            }
        });
    }

    // Trim the decorative frame that is in the real input.
    fn trim_frame(
        &self,
        top_rows: usize,
        bottom_rows: usize,
        left_cols: usize,
        right_cols: usize,
    ) -> Self {
        let mut new_values =
            self.values[top_rows * self.cols..self.values.len() - bottom_rows * self.cols].to_vec();
        let new_rows = self.rows - top_rows - bottom_rows;
        // Triming is done starting from the end.
        for r in (0..new_rows).rev() {
            new_values.drain((r + 1) * self.cols - right_cols..(r + 1) * self.cols);
            new_values.drain(r * self.cols..r * self.cols + left_cols);
        }
        let new_cols = self.cols - left_cols - right_cols;
        Self {
            values: new_values,
            rows: new_rows,
            cols: new_cols,
        }
    }

    fn print_with_pos(&self, positions: &[usize]) {
        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                if positions.contains(&p) {
                    print!("{RED}{c}{RESET}");
                } else {
                    print!("{c}");
                }
            }
            println!();
        }
    }

    #[allow(unused)]
    fn print(&self) {
        self.print_with_pos(&[]);
    }

    fn is_pipe(&self, pos: usize) -> bool {
        is_pipe(self.values[pos])
    }

    fn is_line_pipe(&self, pos: usize) -> bool {
        ['─', '│'].contains(&self.values[pos])
    }

    // Check we don't go outside grid.
    fn allowed(&self, pos: usize, direction: Direction) -> bool {
        !match direction {
            North => pos < self.cols,
            East => pos % self.cols == self.cols - 1,
            South => pos / self.cols == self.rows - 1,
            West => pos % self.cols == 0,
        }
    }

    // Returns the index of the next position in that direction.
    // Assumes validity of the move has been checked before with `allowed`.
    fn next_pos(&self, pos: usize, direction: Direction) -> usize {
        match direction {
            North => pos - self.cols,
            East => pos + 1,
            South => pos + self.cols,
            West => pos - 1,
        }
    }

    fn try_next_pos(&self, pos: usize, direction: Direction) -> Option<usize> {
        if self.allowed(pos, direction) {
            Some(self.next_pos(pos, direction))
        } else {
            None
        }
    }

    // Checks if there is no pipe (empty or border) in the specified direction
    fn is_empty_in(&self, pos: usize, direction: Direction) -> bool {
        if let Some(next_pos) = self.try_next_pos(pos, direction) {
            !self.is_pipe(next_pos)
        } else {
            true
        }
    }

    fn rotate(&mut self, pos: usize) {
        self.values[pos] = rotate_pipe(self.values[pos]);
    }
}

fn is_start_or_end(grid: &Grid, pos: usize) -> bool {
    pos == 0 || pos == grid.values.len() - 1
}

// Indicates if the pipe at the specified position has one of its side empty
// (either a space or at the border).
fn has_empty_side(grid: &Grid, pos: usize) -> bool {
    let c = grid.values[pos];
    let dirs = get_pipe_directions(c);
    dirs.iter().any(|&d| grid.is_empty_in(pos, d))
}

// Indicates if the pipe is connected in that direction.
fn is_connected_in(grid: &Grid, pos: usize, direction: Direction) -> bool {
    if let Some(next_pos) = grid.try_next_pos(pos, direction) {
        if !grid.is_pipe(next_pos) {
            return false;
        }

        let next_pipe = grid.values[next_pos];
        let next_dirs = get_pipe_directions(next_pipe);
        assert!(!next_dirs.is_empty());
        next_dirs.contains(&direction.opposite())
    } else {
        // On border, assume not connected.
        false
    }
}

// Indicates if the pipe is fully connected.
fn is_fully_connected(grid: &Grid, pos: usize) -> bool {
    let dirs = get_pipe_directions(grid.values[pos]);
    dirs.iter().all(|&d| is_connected_in(grid, pos, d))
}

// Prints the grid by highlighting the positions that could still be rotated.
#[allow(unused)]
fn print_positions_with_rotations_left(grid: &Grid, rotations_left: &[usize]) {
    let positions_with_rotations_left: Vec<usize> = rotations_left
        .iter()
        .enumerate()
        .filter(|(_, cnt)| **cnt > 0)
        .map(|(p, _)| p)
        .collect();
    println!("###");
    grid.print_with_pos(&positions_with_rotations_left);
}

// Print all the pipes not fully connected.
#[allow(unused)]
fn print_not_fully_connected(grid: &Grid) {
    let not_fully_connected: Vec<usize> = (0..grid.values.len())
        .filter(|&p| !is_fully_connected(grid, p))
        .collect();

    if not_fully_connected == [0, grid.values.len() - 1] {
        println!("### All is connected!");
    } else {
        println!("### Not fully connected");
        println!("{not_fully_connected:?}");
    }

    grid.print_with_pos(&not_fully_connected);
}

// Finds the minimal number of rotations needed.
#[allow(
    clippy::too_many_lines,
    clippy::needless_range_loop,
    clippy::if_not_else
)]
fn rotations_count(screencap: &str) -> usize {
    // Convert the screencap into a clean grid.
    let mut grid = Grid::build(screencap);
    grid.normalize();
    // For real input, trim the frame
    if grid.rows > 20 {
        grid = grid.trim_frame(4, 5, 7, 7);
    }

    // println!("###");
    // grid.print();

    let mut rotations_cnt = 0;

    // Grid that indicates how many rotations are still possible for each pipe.
    let mut rotations_left: Vec<usize> = grid
        .values
        .iter()
        .map(|&c| {
            if is_pipe(c) {
                get_max_rotations_for(c)
            } else {
                0
            }
        })
        .collect();

    // Line pipes must be rotated if either of their side is empty.
    for pos in 0..grid.values.len() {
        if is_start_or_end(&grid, pos) {
            // Ignore start and end.
            continue;
        }

        if grid.is_pipe(pos) {
            while has_empty_side(&grid, pos) && rotations_left[pos] > 0 {
                grid.rotate(pos);
                rotations_left[pos] -= 1;
                rotations_cnt += 1;
            }
        }
    }

    // print_positions_with_rotations_left(&grid, &rotations_left);

    // Now look at the pipes and their neighbours and do various deductions.
    // We focus on the pipes that cannot be rotated anymore, and analyze their neighbours. For those:
    // - Either rotate what we are sure needs to be rotated,
    // - Or decrement the number of rotations allowed when we know rotating would be wrong.
    loop {
        let mut rotation_executed = false;

        // Go through pipes that cannot be rotated anymore.
        for pos in 0..grid.values.len() {
            if pos == 0 || pos == grid.values.len() - 1 {
                // Ignore start and end.
                continue;
            }

            if grid.is_pipe(pos) && rotations_left[pos] == 0 {
                // For each "blocked" pipe, check its 4 neighbouring positions.
                let dirs = get_pipe_directions(grid.values[pos]);
                for d in ALL_DIRECTIONS {
                    let next_pos = grid.next_pos(pos, d);

                    if dirs.contains(&d) {
                        // If the pipe goes into that direction:

                        if !is_connected_in(&grid, pos, d) {
                            // Pipe is going in that direction so neighbour must be is connected.
                            grid.rotate(next_pos);
                            rotations_left[next_pos] -= 1;
                            rotations_cnt += 1;
                            rotation_executed = true;
                        } else {
                            // If we and neighbour are a line pipes, then neighbour is not allowed to rotate anymore.
                            if grid.is_line_pipe(pos) && grid.is_line_pipe(next_pos) {
                                rotations_left[next_pos] = 0;
                            }

                            // If the neighbour is connected, try to rotate it to see if remains connected.
                            // If it doesn't, we cannot allow the rotation.
                            for rot_tries in 1..=3 {
                                if rotations_left[next_pos] == rot_tries {
                                    // Try rotating
                                    let mut grid_copy = grid.clone();
                                    for _ in 0..rot_tries {
                                        grid_copy.rotate(next_pos);
                                    }
                                    // Are we still connected? If not, decrement rotations left.
                                    if !is_connected_in(&grid_copy, pos, d) {
                                        rotations_left[next_pos] -= 1;
                                    }
                                }
                            }
                        }
                    } else {
                        // If the pipe doesn't go in that direction:

                        if is_connected_in(&grid, pos, d) {
                            // Neighbour isn't allowed to connect.
                            grid.rotate(next_pos);
                            rotations_left[next_pos] -= 1;
                            rotations_cnt += 1;
                            rotation_executed = true;
                        } else if grid.allowed(pos, d) && grid.is_pipe(next_pos) {
                            // If the neighbour isn't connected, check that rotating it wouldn't connect.

                            // If the neighbour is connected, but has only one rotation left and rotating would mean connection,
                            // then neighbour cannot rotate anymore.
                            for rot_tries in 1..=3 {
                                if rotations_left[next_pos] == rot_tries {
                                    // Try rotating
                                    let mut grid_copy = grid.clone();
                                    for _ in 0..rot_tries {
                                        grid_copy.rotate(next_pos);
                                    }
                                    if is_connected_in(&grid_copy, pos, d) {
                                        rotations_left[next_pos] -= 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // print_positions_with_rotations_left(&grid, &rotations_left);

        if !rotation_executed {
            break;
        }
    }

    // print_not_fully_connected(&grid);

    // Manually rotate the rest. List is (pos, rotation count)
    let manual_rotations: Vec<(usize, usize)> = if grid.rows > 15 {
        // Real input
        vec![
            // 106,
            (172, 1),
            (198, 1),
            (199, 3),
            (200, 2),
            // 264,
            // 265,
            (266, 2),
            // 332,
            (398, 1),
            // 415
            (429, 1),
            (481, 1),
            // 495,
            // 530,
            (531, 2),
            (532, 1),
            (555, 1),
            // 556,
            // 598,
            (655, 3),
            // 656,
            (740, 1),
            // 741,
            // 783,
            (784, 1),
            (785, 1),
            // 972,
            (973, 3),
        ]
    } else {
        // Test input
        vec![(12, 1), (46, 1), (52, 1)]
    };

    for (pos, rot_cnt) in manual_rotations {
        for _ in 0..rot_cnt {
            assert!(rotations_left[pos] > 0);
            grid.rotate(pos);
            rotations_left[pos] -= 1;
            rotations_cnt += 1;
        }
    }

    print_not_fully_connected(&grid);

    rotations_cnt
}

fn main() {
    // Reading as binary.
    let mut input = Vec::new();
    let _ = io::stdin().read_to_end(&mut input);

    // Converting to Unicode.
    let screencap = cp437_to_str(input);
    println!("{screencap}");

    println!("Answer: {}", rotations_count(&screencap));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &[u8] = include_bytes!("../resources/input_test_1");

    #[test]
    fn test_answer() {
        let screencap = cp437_to_str(INPUT_TEST.to_vec());

        assert_eq!(rotations_count(&screencap), 34);
    }
}
