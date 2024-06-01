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
}

fn visible_trees_count(map: &Grid) -> usize {
    let mut inside_cnt = 0;
    for r in 1..map.rows - 1 {
        for c in 1..map.cols - 1 {
            let tree_height = map.values[map.pos(r, c)];

            if (0..c).all(|cl| map.values[map.pos(r, cl)] < tree_height)
                || (c + 1..map.cols).all(|cr| map.values[map.pos(r, cr)] < tree_height)
                || (0..r).all(|ru| map.values[map.pos(ru, c)] < tree_height)
                || (r + 1..map.rows).all(|rd| map.values[map.pos(rd, c)] < tree_height)
            {
                inside_cnt += 1;
            }
        }
    }
    let edge_cnt = map.rows * 2 + map.cols * 2 - 4;

    edge_cnt + inside_cnt
}

fn highest_scenic_score(map: &Grid) -> usize {
    let mut best_scenic_score = 0;
    // No need to consider edge trees, as one viewing distance is 0, so score is 0.
    for r in 1..map.rows - 1 {
        for c in 1..map.cols - 1 {
            let tree_height = map.values[map.pos(r, c)];

            let mut left_cnt = 0;
            for cl in (0..c).rev() {
                let th = map.values[map.pos(r, cl)];
                left_cnt += 1;
                if th >= tree_height {
                    break;
                }
            }

            let mut right_cnt = 0;
            for cr in c + 1..map.cols {
                let th = map.values[map.pos(r, cr)];
                right_cnt += 1;
                if th >= tree_height {
                    break;
                }
            }

            let mut up_cnt = 0;
            for ru in (0..r).rev() {
                let th = map.values[map.pos(ru, c)];
                up_cnt += 1;
                if th >= tree_height {
                    break;
                }
            }

            let mut down_cnt = 0;
            for rd in r + 1..map.rows {
                let th = map.values[map.pos(rd, c)];
                down_cnt += 1;
                if th >= tree_height {
                    break;
                }
            }

            let score = left_cnt * right_cnt * up_cnt * down_cnt;
            best_scenic_score = best_scenic_score.max(score);
        }
    }
    best_scenic_score
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
