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
    // Position of the robot, to avoid having to search for it each time.
    robot_pos: usize,
}

impl Grid {
    fn find_robot(values: &[Element]) -> usize {
        values
            .iter()
            .position(|v| matches!(v, Element::Robot))
            .unwrap()
    }

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
        let robot_pos = Self::find_robot(&values);
        Self {
            values,
            rows,
            cols,
            robot_pos,
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

    fn boxes_gps_coordinates(&self) -> usize {
        self.values
            .iter()
            .enumerate()
            .filter(|(_, v)| matches!(v, Element::Box))
            .map(|(p, _)| 100 * self.row(p) + self.col(p))
            .sum()
    }

    // Helper function for the move_robot() function, to explore a line where boxes might be pushed.
    // Returns true if done with exploring the line.
    fn explore_line(&mut self, next_pos: usize, r: usize, c: usize) -> bool {
        let p = self.pos(r, c);
        match self.values[p] {
            Element::Wall => true, // wall, can't move
            Element::Empty => {
                // Found an empty space, adjust the robot and boxes.
                self.values[next_pos] = Element::Robot;
                self.values[self.robot_pos] = Element::Empty;
                self.values[p] = Element::Box;
                self.robot_pos = next_pos;
                true
            }
            Element::Box => false, // continue
            Element::Robot => panic!("Can't have two robots"),
            Element::BegBox | Element::EndBox => todo!(),
        }
    }

    fn move_robot(&mut self, instruction: Direction) {
        if !self.allowed(self.robot_pos, instruction) {
            return;
        }
        let next_pos = self.next_pos(self.robot_pos, instruction);

        match self.values[next_pos] {
            Element::Wall => {}
            Element::Box => {
                // Try to move boxes in that direction.
                // Search for next empty space (before a wall). If there isn't any, can't move.
                // If there is, move all boxes one step in that direction.
                match instruction {
                    Left => {
                        for c in (0..self.col(next_pos)).rev() {
                            if self.explore_line(next_pos, self.row(next_pos), c) {
                                break;
                            }
                        }
                    }
                    Right => {
                        for c in self.col(next_pos) + 1..self.cols {
                            if self.explore_line(next_pos, self.row(next_pos), c) {
                                break;
                            }
                        }
                    }
                    Up => {
                        for r in (0..self.row(next_pos)).rev() {
                            if self.explore_line(next_pos, r, self.col(next_pos)) {
                                break;
                            }
                        }
                    }
                    Down => {
                        for r in self.row(next_pos) + 1..self.rows {
                            if self.explore_line(next_pos, r, self.col(next_pos)) {
                                break;
                            }
                        }
                    }
                }
            }
            Element::Empty => {
                self.values[next_pos] = Element::Robot;
                self.values[self.robot_pos] = Element::Empty;
                self.robot_pos = next_pos;
            }
            Element::Robot => panic!("Can't have two robots"),
            Element::BegBox | Element::EndBox => todo!(),
        }
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
        let robot_pos = Self::find_robot(&values);
        Self {
            values,
            rows: self.rows,
            cols: self.cols * 2,
            robot_pos,
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

fn gps_coords_sum(map: &Grid, instructions: &[Direction]) -> usize {
    let mut map = map.clone();

    // println!("Initial state:");
    // map.print();

    for ins in instructions {
        map.move_robot(*ins);

        // println!("Move {ins}:");
        // map.print();
    }
    map.boxes_gps_coordinates()
}

fn part2(map: &Grid, instructions: &[Direction]) -> i64 {
    0
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
        assert_eq!(gps_coords_sum(&map, &instructions), 9021);
    }
}
