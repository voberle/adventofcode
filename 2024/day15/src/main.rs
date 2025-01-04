#![allow(clippy::match_on_vec_items)]

use std::{
    fmt,
    io::{self, Read},
};

fn split_on_empty_lines(text: &str) -> Vec<&str> {
    text.split("\n\n")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect()
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::{Down, Left, Right, Up};

impl Direction {
    fn build(c: char) -> Self {
        match c {
            '^' => Up,
            'v' => Down,
            '<' => Left,
            '>' => Right,
            _ => panic!("Invalid direction char"),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Direction::Up => '^',
                Direction::Down => 'v',
                Direction::Left => '<',
                Direction::Right => '>',
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
enum Element {
    Robot,
    Wall,
    Box,
    BegBox, // part 2
    EndBox, // part 2
    Empty,
}

impl Element {
    fn build(c: char) -> Self {
        match c {
            '@' => Element::Robot,
            '#' => Element::Wall,
            'O' => Element::Box,
            '.' => Element::Empty,
            _ => panic!("Invalid element char"),
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Element::Robot => '@',
                Element::Wall => '#',
                Element::Box => 'O',
                Element::BegBox => '[',
                Element::EndBox => ']',
                Element::Empty => '.',
            }
        )
    }
}

#[derive(Clone)]
struct Grid {
    values: Vec<Element>,
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
                l.chars().map(Element::build).collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
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

    #[allow(dead_code)]
    fn print(&self) {
        self.print_with_pos(&[]);
    }

    fn col(&self, index: usize) -> usize {
        index % self.cols
    }

    fn row(&self, index: usize) -> usize {
        index / self.cols
    }

    fn pos(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn allowed(&self, pos: usize, direction: Direction) -> bool {
        !match direction {
            Up => pos < self.cols,
            Right => pos % self.cols == self.cols - 1,
            Down => pos / self.cols == self.rows - 1,
            Left => pos % self.cols == 0,
        }
    }

    fn next_pos(&self, pos: usize, direction: Direction) -> usize {
        match direction {
            Up => pos - self.cols,
            Right => pos + 1,
            Down => pos + self.cols,
            Left => pos - 1,
        }
    }

    fn find_robot(&self) -> usize {
        self.values
            .iter()
            .position(|v| matches!(v, Element::Robot))
            .unwrap()
    }

    fn boxes_gps_coordinates(&self) -> usize {
        self.values
            .iter()
            .enumerate()
            .filter(|(_, v)| matches!(v, Element::Box))
            .map(|(p, _)| 100 * self.row(p) + self.col(p))
            .sum()
    }

    fn enlarge(&self) -> Self {
        use Element::{BegBox, Box, Empty, EndBox, Robot, Wall};
        let values: Vec<Element> = (0..self.rows)
            .flat_map(|row| {
                (row * self.cols..(row + 1) * self.cols).flat_map(|p| match self.values[p] {
                    Wall => [Wall, Wall],
                    Box => [BegBox, EndBox],
                    Empty => [Empty, Empty],
                    Robot => [Robot, Empty],
                    BegBox | EndBox => panic!("Can't happen"),
                })
            })
            .collect();
        Self {
            values,
            rows: self.rows,
            cols: self.cols * 2,
        }
    }
}

fn build(input: &str) -> (Grid, Vec<Direction>) {
    let input_parts = split_on_empty_lines(input);
    let map = Grid::build(input_parts[0]);
    let instructions = input_parts[1]
        .replace('\n', "")
        .chars()
        .map(Direction::build)
        .collect();
    (map, instructions)
}

// Helper function for the move_robot() function, to explore a line where boxes might be pushed.
// Returns true if done with exploring the line.
fn explore_line(
    map: &mut Grid,
    robot_pos: &mut usize,
    next_pos: usize,
    r: usize,
    c: usize,
) -> bool {
    let p = map.pos(r, c);
    match map.values[p] {
        Element::Wall => true, // wall, can't move
        Element::Empty => {
            // Found an empty space, adjust the robot and boxes.
            map.values[next_pos] = Element::Robot;
            map.values[*robot_pos] = Element::Empty;
            map.values[p] = Element::Box;
            *robot_pos = next_pos;
            true
        }
        Element::Box => false, // continue
        Element::Robot => panic!("Can't have two robots"),
        Element::BegBox | Element::EndBox => todo!(),
    }
}

fn move_robot(map: &mut Grid, robot_pos: &mut usize, instruction: Direction) {
    if !map.allowed(*robot_pos, instruction) {
        return;
    }
    let next_pos = map.next_pos(*robot_pos, instruction);

    match map.values[next_pos] {
        Element::Wall => {}
        Element::Box => {
            // Try to move boxes in that direction.
            // Search for next empty space (before a wall). If there isn't any, can't move.
            // If there is, move all boxes one step in that direction.
            match instruction {
                Left => {
                    for c in (0..map.col(next_pos)).rev() {
                        if explore_line(map, robot_pos, next_pos, map.row(next_pos), c) {
                            break;
                        }
                    }
                }
                Right => {
                    for c in map.col(next_pos) + 1..map.cols {
                        if explore_line(map, robot_pos, next_pos, map.row(next_pos), c) {
                            break;
                        }
                    }
                }
                Up => {
                    for r in (0..map.row(next_pos)).rev() {
                        if explore_line(map, robot_pos, next_pos, r, map.col(next_pos)) {
                            break;
                        }
                    }
                }
                Down => {
                    for r in map.row(next_pos) + 1..map.rows {
                        if explore_line(map, robot_pos, next_pos, r, map.col(next_pos)) {
                            break;
                        }
                    }
                }
            }
        }
        Element::Empty => {
            map.values[next_pos] = Element::Robot;
            map.values[*robot_pos] = Element::Empty;
            *robot_pos = next_pos;
        }
        Element::Robot => panic!("Can't have two robots"),
        Element::BegBox | Element::EndBox => todo!(),
    }
}

fn gps_coords_sum(map: &Grid, instructions: &[Direction]) -> usize {
    let mut map = map.clone();
    let mut robot_pos = map.find_robot();

    // println!("Initial state:");
    // map.print();

    for ins in instructions {
        move_robot(&mut map, &mut robot_pos, *ins);

        // println!("Move {ins}:");
        // map.print();
    }
    map.boxes_gps_coordinates()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (map, instructions) = build(&input);

    println!("Part 1: {}", gps_coords_sum(&map, &instructions));
    let large_map = map.enlarge();
    large_map.print();
    // println!("Part 2: {}", gps_coords_sum(&large_map, &instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");
    const INPUT_TEST_3: &str = include_str!("../resources/input_test_3");

    #[test]
    fn test_part1_1() {
        let (map, instructions) = build(INPUT_TEST_1);
        assert_eq!(gps_coords_sum(&map, &instructions), 2028);
    }

    #[test]
    fn test_part1_2() {
        let (map, instructions) = build(INPUT_TEST_2);
        assert_eq!(gps_coords_sum(&map, &instructions), 10092);
    }

    #[test]
    fn test_part2() {
        let (map, instructions) = build(INPUT_TEST_3);
        let large_map = map.enlarge();
        // assert_eq!(gps_coords_sum(&map, &instructions), 9021);
    }
}
