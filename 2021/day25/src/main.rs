use std::{
    fmt,
    io::{self, Read},
};

#[derive(Clone, PartialEq, Eq)]
struct Grid {
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

    fn pos(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn is_empty(&self, pos: usize) -> bool {
        self.values[pos] == '.'
    }

    fn move_east(&mut self, previous: &Grid) {
        for row in 0..self.rows {
            let mut col = 0;
            while col < self.cols {
                let p = self.pos(row, col);
                if previous.values[p] == '>' {
                    let east_pos = if col < self.cols - 1 {
                        self.pos(row, col + 1)
                    } else {
                        self.pos(row, 0)
                    };
                    if previous.is_empty(east_pos) {
                        self.values[p] = '.';
                        self.values[east_pos] = '>';
                        col += 1;
                    }
                }
                col += 1;
            }
        }
    }

    fn move_south(&mut self, previous: &Grid) {
        for col in 0..self.cols {
            let mut row = 0;
            while row < self.rows {
                let p = self.pos(row, col);
                if previous.values[p] == 'v' {
                    let south_pos = if row < self.rows - 1 {
                        self.pos(row + 1, col)
                    } else {
                        self.pos(0, col)
                    };
                    if previous.is_empty(south_pos) {
                        self.values[p] = '.';
                        self.values[south_pos] = 'v';
                        row += 1;
                    }
                }
                row += 1;
            }
        }
    }

    fn move_all(&mut self, previous: &Grid) {
        self.move_east(previous);
        let previous = self.clone();
        self.move_south(&previous);
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                write!(f, "{}", self.values[p])?;
            }
            if row < self.rows - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

fn step_no_moves(grid: &Grid) -> usize {
    let mut previous = grid.clone();
    let mut grid = grid.clone();
    // println!("{}", grid);

    for step in 1.. {
        grid.move_all(&previous);

        // println!("\nAfter {} steps:", step);
        // println!("{}", grid);

        if grid == previous {
            return step;
        }
        previous = grid.clone();
    }
    panic!("Can't happen");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let grid = Grid::build(&input);

    println!("Part 1: {}", step_no_moves(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_move_east() {
        let mut grid = Grid::build("...>>>>>...");
        let previous = grid.clone();
        grid.move_east(&previous);
        assert_eq!(grid.to_string(), "...>>>>.>..");
    }

    #[test]
    fn test_move_all_once() {
        let mut grid = Grid::build(
            r"..........
.>v....v..
.......>..
..........",
        );
        let previous = grid.clone();
        grid.move_all(&previous);
        assert_eq!(
            grid.to_string(),
            r"..........
.>........
..v....v>.
.........."
        );
    }

    #[test]
    fn test_move_all_4_steps() {
        let mut grid = Grid::build(
            r"...>...
.......
......>
v.....>
......>
.......
..vvv..",
        );
        let mut previous = grid.clone();
        for _ in 0..4 {
            grid.move_all(&previous);
            previous = grid.clone();
        }
        assert_eq!(
            grid.to_string(),
            r">......
..v....
..>.v..
.>.v...
...>...
.......
v......"
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(step_no_moves(&Grid::build(INPUT_TEST)), 58);
    }
}
