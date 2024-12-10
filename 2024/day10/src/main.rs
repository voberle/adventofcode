use std::io::{self, Read};

use fxhash::FxHashSet;

struct Grid {
    values: Vec<u8>,
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
                l.chars().map(|c| c as u8 - b'0').collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn neighbors_iter(&self, pos: usize) -> impl Iterator<Item = usize> + '_ {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(move |(d_row, d_col)| {
                (
                    ((pos / self.cols) as isize + d_row) as usize,
                    ((pos % self.cols) as isize + d_col) as usize,
                )
            })
            .filter(|&(row, col)| (row < self.rows && col < self.cols))
            .map(|(row, col)| row * self.cols + col)
    }
}

// Adds all the different trail tails (9s) that can be reached from this position.
// Recursive function.
fn walk_and_count(map: &Grid, pos: usize, height: u8, tails: &mut FxHashSet<usize>) {
    for neighbor_pos in map.neighbors_iter(pos) {
        let neighbor_height = map.values[neighbor_pos];
        if neighbor_height == height + 1 {
            if neighbor_height == 9 {
                tails.insert(neighbor_pos);
            } else {
                walk_and_count(map, neighbor_pos, neighbor_height, tails);
            }
        }
    }
}

fn scores_sum(map: &Grid) -> usize {
    map.values
        .iter()
        .enumerate()
        .filter(|(_, &height)| height == 0)
        .map(|(trailhead_pos, trailhead_height)| {
            let mut tails: FxHashSet<usize> = FxHashSet::default();
            walk_and_count(map, trailhead_pos, *trailhead_height, &mut tails);
            tails.len()
        })
        .sum()
}

fn part2(map: &Grid) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = Grid::build(&input);

    println!("Part 1: {}", scores_sum(&map));
    println!("Part 2: {}", part2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part1() {
        assert_eq!(scores_sum(&Grid::build(INPUT_TEST_1)), 1);
        assert_eq!(scores_sum(&Grid::build(INPUT_TEST_2)), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&Grid::build(INPUT_TEST_1)), 0);
    }
}
