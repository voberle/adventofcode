use fxhash::FxHashSet;
use std::io::{self, Read};

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::{East, North, South, West};

const ALL_DIRECTIONS: [Direction; 4] = [North, East, South, West];

struct Grid {
    values: Vec<u32>,
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
                l.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    #[allow(dead_code)]
    fn print(&self, positions: &[usize]) {
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

    fn allowed(&self, pos: usize, direction: Direction) -> bool {
        !match direction {
            North => pos < self.cols,
            East => pos % self.cols == self.cols - 1,
            South => pos / self.cols == self.rows - 1,
            West => pos % self.cols == 0,
        }
    }

    fn next_pos(&self, pos: usize, direction: Direction) -> usize {
        match direction {
            North => pos - self.cols,
            East => pos + 1,
            South => pos + self.cols,
            West => pos - 1,
        }
    }

    fn low_points_iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.values
            .iter()
            .enumerate()
            .filter(move |(pos, value)| {
                ALL_DIRECTIONS
                    .iter()
                    .filter(|d| self.allowed(*pos, **d))
                    .all(|d| {
                        let p = self.next_pos(*pos, *d);
                        **value < self.values[p]
                    })
            })
            .map(|(pos, _)| pos)
    }
}

fn sum_risk_levels(heightmap: &Grid) -> u32 {
    heightmap
        .low_points_iter()
        .map(|pos| 1 + heightmap.values[pos])
        .sum()
}

fn three_largest_basins_product(heightmap: &Grid) -> u64 {
    // Each low point is in one basin.
    // Basins are area with only 9 or border around.
    // Being part of a basins means your adjacents are either parts of a basin too or 9s.

    // Go through each low point. Find their basin.
    let mut basins_sizes = Vec::new();
    for low_point in heightmap.low_points_iter() {
        // Inspired from Dijkstra shortest path algorithm.
        let mut visited: FxHashSet<usize> = FxHashSet::default();

        let mut queue: Vec<usize> = vec![low_point];
        while let Some(pos) = queue.pop() {
            visited.insert(pos);

            queue.extend(ALL_DIRECTIONS.iter().filter_map(|d| {
                if !heightmap.allowed(pos, *d) {
                    return None;
                }
                let next_pos = heightmap.next_pos(pos, *d);

                if heightmap.values[next_pos] == 9 {
                    return None;
                }

                if visited.contains(&next_pos) {
                    return None;
                }
                Some(next_pos)
            }));
        }
        basins_sizes.push(visited.len());

        // println!("----------");
        // heightmap.print(&visited.iter().cloned().collect::<Vec<_>>());
    }

    basins_sizes.sort_unstable();
    basins_sizes.iter().rev().take(3).product::<usize>() as u64
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let heightmap = Grid::build(&input);

    println!("Part 1: {}", sum_risk_levels(&heightmap));
    println!("Part 2: {}", three_largest_basins_product(&heightmap));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(sum_risk_levels(&Grid::build(INPUT_TEST)), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(three_largest_basins_product(&Grid::build(INPUT_TEST)), 1134);
    }
}
