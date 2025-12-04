use std::io::{self, Read};

struct Grid {
    values: Vec<bool>,
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
                l.chars().map(|c| c == '@').collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    fn print_with_pos(&self, positions: &[usize]) {
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                if positions.contains(&p) {
                    print!("x");
                } else if c {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        self.print_with_pos(&[]);
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn next_positions_iter(&self, pos: usize) -> impl Iterator<Item = usize> + '_ {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .into_iter()
        .map(move |(d_row, d_col)| {
            (
                ((pos / self.cols) as isize + d_row) as usize,
                ((pos % self.cols) as isize + d_col) as usize,
            )
        })
        .filter(|&(row, col)| row < self.rows && col < self.cols)
        .map(|(row, col)| row * self.cols + col)
    }
}

fn accessible_rolls(map: &Grid) -> usize {
    let forklifts: Vec<_> = (0..map.values.len())
        .filter(|pos| {
            // Forklifts that have less than 4 forklifts around.
            map.values[*pos]
                && map
                    .next_positions_iter(*pos)
                    .filter(|p| map.values[*p])
                    .count()
                    < 4
        })
        .collect();

    map.print_with_pos(&forklifts);

    forklifts.len()
}

fn part2(map: &Grid) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = Grid::build(&input);

    println!("Part 1: {}", accessible_rolls(&map));
    println!("Part 2: {}", part2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(accessible_rolls(&Grid::build(INPUT_TEST)), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&Grid::build(INPUT_TEST)), 0);
    }
}
