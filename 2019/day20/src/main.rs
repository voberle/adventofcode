use fxhash::FxHashMap;
use std::io::{self, Read};

mod portals;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::{East, North, South, West};

const ALL_DIRECTIONS: [Direction; 4] = [North, East, South, West];

#[derive(Debug, Clone, PartialEq)]
enum Element {
    Wall,
    OpenPassage,
    Portal(String),
    Space,
}
use Element::{OpenPassage, Portal, Space, Wall};

impl Element {}

#[derive(Debug, Clone, PartialEq)]
struct Maze {
    values: Vec<Element>,
    rows: usize,
    cols: usize,
}

impl Maze {
    fn build(input: &str) -> Self {
        // First we get all the portal names and positions.
        let portals = portals::get_portals_from_input(input);
        // println!("{:#?}", portals);

        // Then create the map.
        let mut rows = 0;
        let mut values: Vec<_> = input
            .lines()
            .flat_map(|l| {
                rows += 1;
                l.chars()
                    .map(|c| match c {
                        '.' => OpenPassage,
                        '#' => Wall,
                        _ => Space, // Labels or empty space are treated like walls
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;

        // Finally inject the portals
        values.iter_mut().enumerate().for_each(|(p, e)| {
            if let Some(portal) = portals.get(&p) {
                assert_eq!(*e, OpenPassage);
                *e = Portal(portal.clone());
            }
        });

        Self { values, rows, cols }
    }

    fn print(&self) {
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = match self.values.get(p) {
                    Some(Wall) => '#',
                    Some(OpenPassage) => '.',
                    Some(Portal(_)) => 'O',
                    Some(Space) => ' ',
                    None => panic!("Bug in print()")
                };
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

    fn pos_as_str(&self, index: usize) -> String {
        format!("({},{})", self.row(index), self.col(index))
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
}

fn aa_to_zz_path_length(maze: &Maze) -> i64 {
    0
}

fn part2(maze: &Maze) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let maze = Maze::build(&input);
    maze.print();

    println!("Part 1: {}", aa_to_zz_path_length(&maze));
    println!("Part 2: {}", part2(&maze));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part1() {
        assert_eq!(aa_to_zz_path_length(&Maze::build(INPUT_TEST_1)), 23);
        assert_eq!(aa_to_zz_path_length(&Maze::build(INPUT_TEST_2)), 58);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
