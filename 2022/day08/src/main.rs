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
                // println!("{} {} is visible", map.pos_as_str(map.pos(r, c)), tree_height);
                inside_cnt += 1;
            }
        }
    }
    let edge_cnt = map.rows * 2 + map.cols * 2 - 4;
    // println!("Edge: {}; Inside: {}", edge_cnt, inside_cnt);

    edge_cnt + inside_cnt
}

fn part2(map: &Grid) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = Grid::build(&input);

    println!("Part 1: {}", visible_trees_count(&map));
    println!("Part 2: {}", part2(&map));
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
        assert_eq!(part2(&Grid::build(INPUT_TEST)), 0);
    }
}
