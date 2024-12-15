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

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn build(c: char) -> Self {
        match c {
            '<' => Direction::West,
            '>' => Direction::East,
            '^' => Direction::North,
            'v' => Direction::South,
            _ => panic!("Invalid direction char"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Element {
    Robot,
    Wall,
    Box,
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
                Element::Empty => '.',
            }
        )
    }
}

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

    fn print(&self) {
        self.print_with_pos(&[]);
    }
}

fn build(input: &str) -> (Grid, Vec<Direction>) {
    let input_parts = split_on_empty_lines(input);
    let map = Grid::build(input_parts[0]);
    let instructions = input_parts[1].chars().map(Direction::build).collect();
    (map, instructions)
}

fn gps_coords_sum(map: &Grid, instructions: &[Direction]) -> i64 {
    0
}

fn part2(map: &Grid, instructions: &[Direction]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (map, instructions) = build(&input);
    map.print();
    println!();
    println!("{:?}", instructions);

    println!("Part 1: {}", gps_coords_sum(&map, &instructions));
    println!("Part 2: {}", part2(&map, &instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

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
        // assert_eq!(part2(&build(INPUT_TEST_1)), 0);
    }
}
