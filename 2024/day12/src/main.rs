use std::io::{self, Read};

use fxhash::FxHashSet;

#[derive(Debug, Clone, PartialEq)]
struct Grid {
    values: Vec<char>,
    rows: usize,
    cols: usize,
}
use Direction::{Down, Left, Right, Up};
use itertools::Itertools;

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

    fn col(&self, index: usize) -> usize {
        index % self.cols
    }

    fn row(&self, index: usize) -> usize {
        index / self.cols
    }

    fn allowed(&self, pos: usize, direction: Direction) -> bool {
        !match direction {
            Up => pos < self.cols,
            Right => pos % self.cols == self.cols - 1,
            Down => pos / self.cols == self.rows - 1,
            Left => pos % self.cols == 0,
        }
    }

    fn next_pos(&self, pos: usize, direction: Direction) -> usize {
        match direction {
            Up => pos - self.cols,
            Right => pos + 1,
            Down => pos + self.cols,
            Left => pos - 1,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Recursive function.
fn walk(map: &Grid, pos: usize, region: &mut FxHashSet<usize>) {
    let value = map.values[pos];
    for dir in [Up, Down, Left, Right] {
        if let Some(next_pos) = map.try_next_pos(pos, dir) {
            if !region.contains(&next_pos) && map.values[next_pos] == value {
                region.insert(next_pos);
                walk(map, next_pos, region);
            }
        }
    }
}

fn find_region(map: &Grid, pos: usize) -> FxHashSet<usize> {
    let mut region = FxHashSet::default();
    region.insert(pos);
    walk(map, pos, &mut region);
    region
}

fn all_regions(map: &Grid) -> Vec<FxHashSet<usize>> {
    // We need to track all positions that were already put in a region, to avoid duplicating them.
    let mut visited: FxHashSet<usize> = FxHashSet::default();
    (0..map.values.len())
        .filter_map(|pos| {
            if visited.contains(&pos) {
                None
            } else {
                let r = find_region(map, pos);
                visited.extend(r.iter());
                Some(r)
            }
        })
        .collect()
}

fn area(_map: &Grid, region: &FxHashSet<usize>) -> usize {
    region.len()
}

// Given an ordered list of unique numbers, determines how many sequences of consecutive numbers it contains.
fn count_sequences(numbers: &[usize]) -> usize {
    numbers.windows(2).filter(|w| w[1] > w[0] + 1).count() + 1
}

fn number_of_sides(borders: &FxHashSet<(usize, usize, Direction)>) -> usize {
    // Go through the borders by col/directions and row/direction.
    // Order the positions and see how many groups there are.
    [Up, Down, Left, Right]
        .iter()
        .map(|direction| {
            borders
                .iter()
                .filter(|(_, _, dir)| dir == direction)
                .map(|&(row, col, _)| (row, col))
                .into_group_map_by(|(row, col)| match direction {
                    Up | Down => *row,
                    Left | Right => *col,
                })
                .into_values()
                .map(|v| {
                    let columns = v
                        .iter()
                        .map(|(row, col)| match direction {
                            Up | Down => *col,
                            Left | Right => *row,
                        })
                        .sorted_unstable()
                        .collect_vec();
                    count_sequences(&columns)
                })
                .sum::<usize>()
        })
        .sum()
}

// If DISCOUNT is false, returns the perimeter.
// If DISCOUNT is true, returns the number of sides.
fn perimeter_or_side_count<const DISCOUNT: bool>(map: &Grid, region: &FxHashSet<usize>) -> usize {
    // The values are separated into row / col.
    // This isn't needed to get the perimeter, but makes getting the side count easier (for part 2)
    let mut borders: FxHashSet<(usize, usize, Direction)> = FxHashSet::default();

    for &plot_pos in region {
        let row = map.row(plot_pos);
        let col = map.col(plot_pos);
        for dir in [Up, Down, Left, Right] {
            if let Some(up_pos) = map.try_next_pos(plot_pos, dir) {
                if !region.contains(&up_pos) {
                    borders.insert((row, col, dir));
                }
            } else {
                // We are at the border of the map.
                borders.insert((row, col, dir));
            }
        }
    }

    if DISCOUNT {
        number_of_sides(&borders)
    } else {
        borders.len()
    }
}

fn price<const DISCOUNT: bool>(map: &Grid) -> usize {
    let regions = all_regions(map);
    regions
        .iter()
        .map(|r| {
            let a = area(map, r);
            let p = perimeter_or_side_count::<DISCOUNT>(map, r);
            a * p
        })
        .sum()
}

fn total_price(map: &Grid) -> usize {
    price::<false>(map)
}

fn total_with_bulk_discount(map: &Grid) -> usize {
    price::<true>(map)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = Grid::build(&input);

    println!("Part 1: {}", total_price(&map));
    println!("Part 2: {}", total_with_bulk_discount(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");
    const INPUT_TEST_3: &str = include_str!("../resources/input_test_3");
    const INPUT_TEST_4: &str = include_str!("../resources/input_test_4");
    const INPUT_TEST_5: &str = include_str!("../resources/input_test_5");

    const TEST_1_A: [usize; 4] = [0, 1, 2, 3];
    const TEST_1_B: [usize; 4] = [4, 5, 8, 9];
    const TEST_1_C: [usize; 4] = [6, 10, 11, 15];
    const TEST_1_D: [usize; 1] = [7];
    const TEST_1_E: [usize; 3] = [12, 13, 14];

    fn set(data: &[usize]) -> FxHashSet<usize> {
        FxHashSet::from_iter(data.iter().cloned())
    }

    #[test]
    fn test_all_regions() {
        let map = Grid::build(INPUT_TEST_1);
        let regions = all_regions(&map);
        assert!(regions.contains(&set(&TEST_1_A)));
        assert!(regions.contains(&set(&TEST_1_B)));
        assert!(regions.contains(&set(&TEST_1_C)));
        assert!(regions.contains(&set(&TEST_1_D)));
        assert!(regions.contains(&set(&TEST_1_E)));
    }

    #[test]
    fn test_perimeter() {
        // AAAA
        // BBCD
        // BBCC
        // EEEC
        let map = Grid::build(INPUT_TEST_1);
        assert_eq!(perimeter_or_side_count::<false>(&map, &set(&TEST_1_A)), 10);
        assert_eq!(perimeter_or_side_count::<false>(&map, &set(&TEST_1_B)), 8);
        assert_eq!(perimeter_or_side_count::<false>(&map, &set(&TEST_1_C)), 10);
        assert_eq!(perimeter_or_side_count::<false>(&map, &set(&TEST_1_D)), 4);
        assert_eq!(perimeter_or_side_count::<false>(&map, &set(&TEST_1_E)), 8);
    }

    #[test]
    fn test_perimeter_discount() {
        let map = Grid::build(INPUT_TEST_1);
        assert_eq!(perimeter_or_side_count::<true>(&map, &set(&TEST_1_A)), 4);
        assert_eq!(perimeter_or_side_count::<true>(&map, &set(&TEST_1_B)), 4);
        assert_eq!(perimeter_or_side_count::<true>(&map, &set(&TEST_1_C)), 8);
        assert_eq!(perimeter_or_side_count::<true>(&map, &set(&TEST_1_D)), 4);
        assert_eq!(perimeter_or_side_count::<true>(&map, &set(&TEST_1_E)), 4);
    }

    #[test]
    fn test_count_sequences() {
        assert_eq!(count_sequences(&[0, 1, 2, 3]), 1);
        assert_eq!(count_sequences(&[3, 4]), 1);
        assert_eq!(count_sequences(&[3, 4, 6, 7, 8, 9]), 2);
        assert_eq!(count_sequences(&[3, 4, 6, 7, 9]), 3);
    }

    #[test]
    fn test_part1() {
        assert_eq!(total_price(&Grid::build(INPUT_TEST_1)), 140);
        assert_eq!(total_price(&Grid::build(INPUT_TEST_2)), 772);
        assert_eq!(total_price(&Grid::build(INPUT_TEST_3)), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(total_with_bulk_discount(&Grid::build(INPUT_TEST_1)), 80);
        assert_eq!(total_with_bulk_discount(&Grid::build(INPUT_TEST_2)), 436);
        assert_eq!(total_with_bulk_discount(&Grid::build(INPUT_TEST_4)), 236);
        assert_eq!(total_with_bulk_discount(&Grid::build(INPUT_TEST_5)), 368);
        assert_eq!(total_with_bulk_discount(&Grid::build(INPUT_TEST_3)), 1206);
    }
}
