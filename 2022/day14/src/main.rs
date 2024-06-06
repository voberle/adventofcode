use std::io::{self, Read};

use itertools::Itertools;

// Helper method to parse the input, before building the grid.
fn parse_input(input: &str) -> Vec<Vec<(usize, usize)>> {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|c| {
                    c.split(',')
                        .map(|v| v.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect()
        })
        .collect()
}

fn borders(coords: &[Vec<(usize, usize)>]) -> (usize, usize, usize, usize) {
    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut min_y = usize::MAX;
    let mut max_y = 0;
    for &(x, y) in coords.iter().flatten() {
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }
    (min_x, max_x, min_y, max_y)
}

#[derive(Debug, Clone, Copy)]
enum Element {
    Empty,
    Rock,
    Sand,
}
use Element::{Empty, Rock, Sand};

#[derive(Clone)]
struct Cave {
    elements: Vec<Element>,
    rows: usize,
    cols: usize,
    min_x: usize,
}

impl Cave {
    fn build(coords: &[Vec<(usize, usize)>]) -> Self {
        let (min_x, max_x, _, max_y) = borders(coords);
        // Top starts at 0.
        let rows = max_y + 1;
        let cols = max_x - min_x + 1;
        let mut elements = vec![Empty; rows * cols];

        for line in coords {
            for start_end in line.windows(2) {
                if start_end[0].0 == start_end[1].0 {
                    // x is same, so vertical line.
                    let x = start_end[0].0 - min_x;
                    let (y1, y2) = if start_end[0].1 < start_end[1].1 {
                        (start_end[0].1, start_end[1].1)
                    } else {
                        (start_end[1].1, start_end[0].1)
                    };
                    for y in y1..=y2 {
                        elements[y * cols + x] = Rock;
                    }
                } else if start_end[0].1 == start_end[1].1 {
                    // y is same, so horizontal line.
                    let y = start_end[0].1;
                    let (x1, x2) = if start_end[0].0 < start_end[1].0 {
                        (start_end[0].0, start_end[1].0)
                    } else {
                        (start_end[1].0, start_end[0].0)
                    };
                    for x in x1..=x2 {
                        let x = x - min_x;
                        elements[y * cols + x] = Rock;
                    }
                } else {
                    panic!("Diagonal lines not supported")
                }
            }
        }
        Self {
            elements,
            rows,
            cols,
            min_x,
        }
    }

    fn build_with_floor(coords: &[Vec<(usize, usize)>]) -> Self {
        let mut ext_coords = coords.to_vec();

        let y = coords.iter().flatten().map(|c| c.1).max().unwrap() + 2;
        // Since entry is at x=500, the base of the sand pyramid can only go as far as
        // pyramid height on each side.
        let min_x = 500 - y;
        let max_x = 500 + y;
        // println!("y={} min_x={} max_x={}", y, min_x, max_x);
        ext_coords.push(vec![(min_x, y), (max_x, y)]);

        Cave::build(&ext_coords)
    }

    fn get_entry(&self) -> usize {
        // 500,0
        500 - self.min_x
    }

    #[allow(dead_code, clippy::match_on_vec_items)]
    fn print(&self) {
        const RED: &str = "\x1b[31m";
        const BLUE: &str = "\x1b[94m";
        const RESET: &str = "\x1b[0m";
        let entry = self.get_entry();
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                if p == entry {
                    print!("{BLUE}+{RESET}");
                } else {
                    match self.elements[p] {
                        Empty => print!("."),
                        Rock => print!("#"),
                        Sand => print!("{RED}o{RESET}"),
                    }
                }
            }
            println!();
        }
    }

    fn down(&self, pos: usize) -> Option<usize> {
        if pos / self.cols == self.rows - 1 {
            None
        } else {
            Some(pos + self.cols)
        }
    }

    fn down_left(&self, pos: usize) -> Option<usize> {
        if pos / self.cols == self.rows - 1 || pos % self.cols == 0 {
            None
        } else {
            Some(pos + self.cols - 1)
        }
    }

    fn down_right(&self, pos: usize) -> Option<usize> {
        if pos / self.cols == self.rows - 1 || pos % self.cols == self.cols - 1 {
            None
        } else {
            Some(pos + self.cols + 1)
        }
    }

    // Try to drop a unit of sand.
    // Returns Some(previous_pos) if sand came to rest, with previous_pos being the position
    // just before the sand rested. We can use this as start position on next iteration.
    // Returns None when:
    // - with HAS_FLOOR == false, the sand fell into the abyss.
    // - with HAS_FLOOR == true, the sand reached the entry.
    fn drop_sand<const HAS_FLOOR: bool>(&mut self, mut pos: usize) -> Option<usize> {
        let mut previous_pos = self.get_entry();
        loop {
            if let Some(down) = self.down(pos) {
                if matches!(self.elements[down], Empty) {
                    previous_pos = pos;
                    pos = down;
                    continue;
                }
            } else if !HAS_FLOOR {
                return None;
            }
            if let Some(down_left) = self.down_left(pos) {
                if matches!(self.elements[down_left], Empty) {
                    previous_pos = pos;
                    pos = down_left;
                    continue;
                }
            } else if !HAS_FLOOR {
                return None;
            }
            if let Some(down_right) = self.down_right(pos) {
                if matches!(self.elements[down_right], Empty) {
                    previous_pos = pos;
                    pos = down_right;
                    continue;
                }
            } else if !HAS_FLOOR {
                return None;
            }
            // Sand came to rest, stopping the loop.
            break;
        }

        if HAS_FLOOR && pos == self.get_entry() {
            return None;
        }

        self.elements[pos] = Sand;
        Some(previous_pos)
    }

    // Fill the cave with sand and return the number of units of sand.
    // With HAS_FLOOR == false, fill until it starts falling into the abyss.
    // With HAS_FLOOR == true, fill until sand reaches the entry.
    fn fill_sand<const HAS_FLOOR: bool>(&mut self) -> usize {
        let mut count = 0;

        let mut pos_to_start_from = self.get_entry();
        while let Some(previous_pos) = self.drop_sand::<HAS_FLOOR>(pos_to_start_from) {
            pos_to_start_from = previous_pos;
            count += 1;
        }
        // self.print();

        if HAS_FLOOR {
            // Add the sand on the entry spot.
            count += 1;
        }
        count
    }
}

fn sand_count_before_abyss(coords: &[Vec<(usize, usize)>]) -> usize {
    let mut cave = Cave::build(coords);
    cave.fill_sand::<false>()
}

fn sand_count_with_floor(coords: &[Vec<(usize, usize)>]) -> usize {
    let mut cave = Cave::build_with_floor(coords);
    cave.fill_sand::<true>()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let coords = parse_input(&input);

    println!("Part 1: {}", sand_count_before_abyss(&coords));
    println!("Part 2: {}", sand_count_with_floor(&coords));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(sand_count_before_abyss(&parse_input(INPUT_TEST)), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(sand_count_with_floor(&parse_input(INPUT_TEST)), 93);
    }
}
