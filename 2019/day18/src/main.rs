use std::{
    fmt,
    io::{self, Read},
};

enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::{East, North, South, West};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Element {
    Entrance,
    Open,
    Wall,
    Key(char),
    Door(char),
}
use Element::{Entrance, Open, Wall, Key, Door};

impl Element {
    fn new(c: char) -> Self {
        match c {
            '@' => Entrance,
            '.' => Open,
            '#' => Wall,
            'a'..='z' => Key(c),
            'A'..='Z' => Door(c.to_ascii_lowercase()),
            _ => panic!("Unrecognized char {}", c),
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Entrance => '@',
                Open => '.',
                Wall => '#',
                Key(c) => c,
                Door(c) => c.to_ascii_uppercase(),
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Map {
    values: Vec<Element>,
    rows: usize,
    cols: usize,
}

impl Map {
    fn build(input: &str) -> Self {
        let mut rows = 0;
        let values: Vec<_> = input
            .lines()
            .flat_map(|l| {
                rows += 1;
                l.chars().map(Element::new).collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    fn print(&self) {
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                print!("{}", c);
            }
            println!();
        }
    }

    fn pos(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn col(&self, index: usize) -> usize {
        index % self.cols
    }

    fn row(&self, index: usize) -> usize {
        index / self.cols
    }
}

fn shortest_path(map: &Map) -> usize {
    0
}

fn part2(map: &Map) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = Map::build(&input);
    map.print();

    println!("Part 1: {}", shortest_path(&map));
    println!("Part 2: {}", part2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");
    const INPUT_TEST_3: &str = include_str!("../resources/input_test_3");
    const INPUT_TEST_4: &str = include_str!("../resources/input_test_4");
    const INPUT_TEST_5: &str = include_str!("../resources/input_test_5");

    #[test]
    fn test_part1() {
        assert_eq!(shortest_path(&Map::build(INPUT_TEST_1)), 8);
        assert_eq!(shortest_path(&Map::build(INPUT_TEST_2)), 86);
        assert_eq!(shortest_path(&Map::build(INPUT_TEST_3)), 132);
        assert_eq!(shortest_path(&Map::build(INPUT_TEST_4)), 136);
        assert_eq!(shortest_path(&Map::build(INPUT_TEST_5)), 81);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
