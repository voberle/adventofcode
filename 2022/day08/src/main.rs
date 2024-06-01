use std::io::{self, Read};

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

    fn pos(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn tree_height(&self, row: usize, col: usize) -> u32 {
        self.values[self.pos(row, col)]
    }

    fn is_inside_tree_visible(&self, r: usize, c: usize) -> bool {
        let tree_height = self.tree_height(r, c);

        (0..c).all(|cl| self.tree_height(r, cl) < tree_height)
            || (c + 1..self.cols).all(|cr| self.tree_height(r, cr) < tree_height)
            || (0..r).all(|ru| self.tree_height(ru, c) < tree_height)
            || (r + 1..self.rows).all(|rd| self.tree_height(rd, c) < tree_height)
    }

    fn scenic_score(&self, r: usize, c: usize) -> u32 {
        let tree_height = self.tree_height(r, c);

        let mut left_cnt = 0;
        for cl in (0..c).rev() {
            left_cnt += 1;
            if self.tree_height(r, cl) >= tree_height {
                break;
            }
        }

        let mut right_cnt = 0;
        for cr in c + 1..self.cols {
            right_cnt += 1;
            if self.tree_height(r, cr) >= tree_height {
                break;
            }
        }

        let mut up_cnt = 0;
        for ru in (0..r).rev() {
            up_cnt += 1;
            if self.tree_height(ru, c) >= tree_height {
                break;
            }
        }

        let mut down_cnt = 0;
        for rd in r + 1..self.rows {
            down_cnt += 1;
            if self.tree_height(rd, c) >= tree_height {
                break;
            }
        }

        left_cnt * right_cnt * up_cnt * down_cnt
    }
}

fn visible_trees_count(map: &Grid) -> usize {
    let inside_cnt: usize = (1..map.rows - 1)
        .map(|r| {
            (1..map.cols - 1)
                .filter(|&c| map.is_inside_tree_visible(r, c))
                .count()
        })
        .sum();

    let edge_cnt = map.rows * 2 + map.cols * 2 - 4;

    edge_cnt + inside_cnt
}

fn highest_scenic_score(map: &Grid) -> u32 {
    // No need to consider edge trees, as one viewing distance is 0, so score is 0.
    (1..map.rows - 1)
        .map(|r| {
            (1..map.cols - 1)
                .map(|c| map.scenic_score(r, c))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = Grid::build(&input);

    println!("Part 1: {}", visible_trees_count(&map));
    println!("Part 2: {}", highest_scenic_score(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(visible_trees_count(&Grid::build(INPUT_TEST)), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(highest_scenic_score(&Grid::build(INPUT_TEST)), 8);
    }
}
