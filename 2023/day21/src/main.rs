use std::io::{self, Read};

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
struct Grid {
    values: Vec<char>,
    rows: usize,
    cols: usize,
}

#[allow(dead_code)]
impl Grid {
    fn build(input: &str) -> Self {
        let mut rows = 0;
        let values: Vec<_> = input
            .lines()
            .flat_map(|l| {
                rows += 1;
                l.chars()
                    // .map(|c| c)
                    .collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    fn print(&self) {
        for row in 0..self.rows {
            println!(
                "{}",
                self.values[row * self.cols..(row + 1) * self.cols]
                    .iter()
                    .collect::<String>()
            );
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
    // Assumes validity of the move has been checked before with `can_go`.
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

fn walk_one_step(grid: &mut Grid) {
    grid.values
        .iter()
        .enumerate()
        .filter_map(|(i, v)| if *v == 'O' { Some(i) } else { None })
        .collect::<Vec<usize>>()
        .iter_mut()
        .for_each(|i| {
            for d in ALL_DIRECTIONS {
                if let Some(n) = grid.try_next_pos(*i, d) {
                    if grid.values[n] == '.' {
                        grid.values[n] = 'O';
                    }
                }
            }
            grid.values[*i] = '.';
        });
}

#[test]
fn test_walk_one_step() {
    let source = "...........
.....###.#.
.###.##..#.
..#.#O..#..
....#.#....
.##O.O####.
.##.O#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    let result = "...........
.....###.#.
.###.##..#.
..#.#.O.#..
...O#O#....
.##.O.####.
.##O.#...#.
....O..##..
.##.#.####.
.##..##.##.
...........";
    let mut grid = Grid::build(source);
    walk_one_step(&mut grid);
    assert_eq!(grid, Grid::build(result));
}

fn get_initial_pos(grid: &Grid) -> Option<usize> {
    grid.values.iter().position(|v| *v == 'S')
}

fn prep_grid(grid: &mut Grid, pos: usize) {
    grid.values[pos] = 'O';
}

fn plots_count(grid: &Grid) -> usize {
    grid.values.iter().filter(|v| **v == 'O').count()
}

fn find_filled_grid(grid: &mut Grid, target_step_count: u64) -> (u64, u64, u64) {
    let mut counts: Vec<usize> = Vec::new();

    let mut steps: u64 = 0;
    loop {
        if steps == target_step_count {
            break;
        }
        steps += 1;

        walk_one_step(grid);

        let plot_count = plots_count(grid);
        let maybe_second_last = counts.len().checked_sub(2).map(|i| counts[i]);
        counts.push(plot_count);

        if let Some(last) = maybe_second_last {
            // println!("------- Last {}, curr {}", last, plot_count);
            if plot_count == last {
                // println!("Found period after {} steps", steps + 1);
                break;
            }
        }
        // grid.print();
    }
    // println!("{:?}", saved_counts);
    let plot_count = *counts.last().unwrap();
    let other_count = counts.len().checked_sub(2).map(|i| counts[i]).unwrap();

    (steps, plot_count as u64, other_count as u64)
}

// Part 1
fn garden_plots_count_after(grid: &Grid, target_step_count: u64) -> u64 {
    let initial_pos = get_initial_pos(grid).unwrap();
    let mut initial = grid.clone();
    prep_grid(&mut initial, initial_pos);

    let (mut step_count, mut plot_count, mut other_count) =
        find_filled_grid(&mut initial, target_step_count);

    loop {
        if step_count == target_step_count {
            break;
        }
        step_count += 1;
        std::mem::swap(&mut plot_count, &mut other_count);
    }
    plot_count
}

// Part 2
// Works only for real input, taking into account its patterns.
#[allow(clippy::unreadable_literal)]
fn mega_garden_count() -> u64 {
    // Once the input is full, we have following number of plots in base square
    // depending on number of steps:
    let base_count_even_nb_steps: u64 = 7688;
    let base_count_odd_nb_steps: u64 = 7656;

    // Number of steps total: 26501365 = 65 + (202300 * 131)
    let n: u64 = 202300;
    // The total is odd, so the middle square will be base_count_odd_nb_steps

    // Depending on n, we will have following number of big squares that are full:
    // N=2: even: 1; odd: 4
    // N=3: even: 4; odd: 9
    // N=4: even: 9, odd: 16
    // Those are square numbers.
    // Or if we look at all big squares even if they are not full:
    // N=2: even: 4; odd: 9
    // N=3: even: 9, odd: 16
    // The later is easier to manipulate, so:
    let count_even_big_squares = n * n;
    let count_odd_big_squares = (n + 1) * (n + 1);

    // The corners now. There are corners to add and to remove.
    // N=1: corner_add: 4, corner_remove: 8
    // N=2: corner_add: 8, corner_remove: 12
    // N=3: corner_add: 12, corner_remove: 16
    let count_corners_add = n * 4;
    let count_corners_remove = (n + 1) * 4;

    // Also if n is even, corners to remove are from even squares, to add from odd squares.

    // The corners to remove represent the plots that are more than 65 steps away in a base square.
    let count_65_steps = 3877;
    let corner_count_to_remove_group4 = base_count_odd_nb_steps - count_65_steps;
    // and the ones to add is similar
    let count_64_steps = 3768;
    let corner_count_to_add_group4 = base_count_even_nb_steps - count_64_steps;

    // Full formula
    count_odd_big_squares * base_count_odd_nb_steps
        + count_even_big_squares * base_count_even_nb_steps
        - count_corners_remove / 4 * corner_count_to_remove_group4
        + count_corners_add / 4 * corner_count_to_add_group4
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let grid = Grid::build(&input);
    // grid.print();

    println!("Part 1: {}", garden_plots_count_after(&grid, 64));
    println!("Part 2: {}", mega_garden_count());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test");

    #[test]
    fn test_part1() {
        let grid = Grid::build(INPUT_TEST);

        assert_eq!(garden_plots_count_after(&grid, 6), 16);
        assert_eq!(garden_plots_count_after(&grid, 64), 42);
    }
}
