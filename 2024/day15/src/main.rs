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
use fxhash::FxHashSet;
use itertools::Itertools;
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

#[derive(Debug, Clone, Copy, PartialEq)]
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
            '[' => Element::BegBox,
            ']' => Element::EndBox,
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

#[derive(Debug, Clone, PartialEq)]
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

    fn print_with_pos(&self, positions: &FxHashSet<usize>) {
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
        self.print_with_pos(&FxHashSet::default());
    }

    fn col(&self, index: usize) -> usize {
        index % self.cols
    }

    fn row(&self, index: usize) -> usize {
        index / self.cols
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
            .filter(|(_, v)| matches!(v, Element::Box | Element::BegBox))
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

// Shift by one all elements indicated by the positions into the direction.
// This function assumes that the position(s) after the block is free (meaning it's overwritten).
#[allow(clippy::cast_possible_wrap)]
fn shift_block(map: &mut Grid, positions: &FxHashSet<usize>, direction: Direction) {
    positions
        .iter()
        .sorted_unstable_by_key(|p| match direction {
            Up => map.row(**p) as isize,
            Down => -(map.row(**p) as isize),
            Left => map.col(**p) as isize,
            Right => -(map.col(**p) as isize),
        })
        .for_each(|p| {
            let to = map.next_pos(*p, direction);
            let from = *p;
            map.values.swap(to, from);
        });
}

fn find_bloc_of_boxes(
    map: &Grid,
    dir: Direction,
    pos: usize,
    block_to_move: &mut FxHashSet<usize>,
) -> bool {
    match map.values[pos] {
        Element::Wall => {
            // Wall, robot can't move.
            false
        }
        Element::Empty => {
            // Empty space, let's move.
            true
        }
        Element::BegBox if matches!(dir, Up | Down) => {
            block_to_move.insert(pos);
            let right = map.next_pos(pos, Right);
            block_to_move.insert(right);

            find_bloc_of_boxes(map, dir, map.next_pos(pos, dir), block_to_move)
                & find_bloc_of_boxes(map, dir, map.next_pos(right, dir), block_to_move)
        }
        Element::EndBox if matches!(dir, Up | Down) => {
            block_to_move.insert(pos);
            let left = map.next_pos(pos, Left);
            block_to_move.insert(left);

            find_bloc_of_boxes(map, dir, map.next_pos(pos, dir), block_to_move)
                & find_bloc_of_boxes(map, dir, map.next_pos(left, dir), block_to_move)
        }
        Element::Box | Element::BegBox | Element::EndBox => {
            // If it's a box, keep exploring.
            block_to_move.insert(pos);
            find_bloc_of_boxes(map, dir, map.next_pos(pos, dir), block_to_move)
        }
        Element::Robot => panic!("Can't have two robots"),
    }
}

fn move_robot(map: &mut Grid, robot_pos: &mut usize, instruction: Direction) {
    // The maps have borders, so we can't fall out.
    let next_pos = map.next_pos(*robot_pos, instruction);

    match map.values[next_pos] {
        Element::Wall => {
            // Robot is next to wall, doesn't move.
        }
        Element::Box | Element::BegBox | Element::EndBox => {
            // Robot tries to push boxes.
            let mut block_to_move = FxHashSet::default();
            block_to_move.insert(*robot_pos);
            if find_bloc_of_boxes(map, instruction, next_pos, &mut block_to_move) {
                shift_block(map, &block_to_move, instruction);
                *robot_pos = map.find_robot();
            }
        }
        Element::Empty => {
            // Robot is next to an empty space, moves to it.
            map.values.swap(next_pos, *robot_pos);
            *robot_pos = next_pos;
        }
        Element::Robot => panic!("Can't have two robots"),
    }
}

fn apply_instructions(map: &Grid, instructions: &[Direction]) -> Grid {
    let mut map = map.clone();
    let mut robot_pos = map.find_robot();

    // println!("Initial state:");
    // map.print();

    for ins in instructions {
        // println!("Move {ins}:");
        move_robot(&mut map, &mut robot_pos, *ins);
        // map.print();
    }
    map
}

fn gps_coords_sum(map: &Grid, instructions: &[Direction]) -> usize {
    let map = apply_instructions(map, instructions);
    map.boxes_gps_coordinates()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (map, instructions) = build(&input);

    println!("Part 1: {}", gps_coords_sum(&map, &instructions));

    let large_map = map.enlarge();
    // large_map.print();

    println!("Part 2: {}", gps_coords_sum(&large_map, &instructions));
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
    fn test_shift_block() {
        let original_map = Grid::build(
            r"##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##.....@....##
##..........##
##############",
        );

        let mut map = original_map.clone();
        let mut positions: FxHashSet<usize> = [47, 48, 49, 50, 62, 63, 77].into_iter().collect();
        map.print_with_pos(&positions);

        shift_block(&mut map, &positions, Direction::Up);
        positions = positions.iter().map(|p| p - map.cols).collect();
        map.print_with_pos(&positions);

        shift_block(&mut map, &positions, Direction::Left);
        positions = positions.iter().map(|p| p - 1).collect();
        map.print_with_pos(&positions);

        shift_block(&mut map, &positions, Direction::Down);
        positions = positions.iter().map(|p| p + map.cols).collect();
        map.print_with_pos(&positions);

        shift_block(&mut map, &positions, Direction::Right);
        positions = positions.iter().map(|p| p + 1).collect();
        map.print_with_pos(&positions);

        assert_eq!(map, original_map);
    }

    #[test]
    fn test_part2_1() {
        let (map, instructions) = build(INPUT_TEST_3);
        let large_map = map.enlarge();

        let modified_map = apply_instructions(&large_map, &instructions);
        assert_eq!(
            modified_map,
            Grid::build(
                r"##############
##...[].##..##
##...@.[]...##
##....[]....##
##..........##
##..........##
##############"
            )
        );
    }

    #[test]
    fn test_part2_2() {
        let (map, instructions) = build(INPUT_TEST_2);
        let large_map = map.enlarge();
        assert_eq!(gps_coords_sum(&large_map, &instructions), 9021);
    }
}
