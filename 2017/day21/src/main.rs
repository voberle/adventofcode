use std::io::{self, Read};

use fxhash::FxHashMap;

#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
fn sqrt(v: usize) -> usize {
    (v as f64).sqrt() as usize
}

#[allow(dead_code)]
fn print_grid(grid: &[bool]) {
    let grid_size = sqrt(grid.len());
    for row in 0..grid_size {
        for v in grid
            .iter()
            .take((row + 1) * grid_size)
            .skip(row * grid_size)
        {
            print!("{}", if *v { '#' } else { '.' });
        }
        println!();
    }
}

struct EnhancementRules {
    rules2to3: FxHashMap<Vec<bool>, Vec<bool>>,
    rules3to4: FxHashMap<Vec<bool>, Vec<bool>>,
}

impl EnhancementRules {
    fn new() -> Self {
        Self {
            rules2to3: FxHashMap::default(),
            rules3to4: FxHashMap::default(),
        }
    }

    fn get_rule(&self, square: &[bool]) -> Vec<bool> {
        if square.len() == 2 * 2 {
            self.rules2to3
                .get(square)
                .expect("Didn't find matching rule")
                .clone()
        } else if square.len() == 3 * 3 {
            self.rules3to4
                .get(square)
                .expect("Didn't find matching rule")
                .clone()
        } else {
            panic!("Wrong small square len {}", square.len())
        }
    }

    fn build_pattern(s: &str) -> Vec<bool> {
        s.chars().filter(|c| *c != '/').map(|c| c == '#').collect()
    }

    fn build(input: &str) -> Self {
        let mut rules = Self::new();
        for line in input.lines() {
            let parts: Vec<&str> = line.split(" => ").collect();
            let k: Vec<bool> = Self::build_pattern(parts[0]);
            let v: Vec<bool> = Self::build_pattern(parts[1]);
            if k.len() == 2 * 2 && v.len() == 3 * 3 {
                rules.rules2to3.insert(k, v);
            } else if k.len() == 3 * 3 && v.len() == 4 * 4 {
                rules.rules3to4.insert(k, v);
            } else {
                panic!("Invalid input");
            }
        }

        // Extend the maps with all the rules we can get with flips and rotations.
        rules.extend_rules();

        rules
    }

    fn extend_rules_map(
        rules_map: &mut FxHashMap<Vec<bool>, Vec<bool>>,
        flip_vertically_fn: fn(&[bool]) -> Vec<bool>,
        flip_horizontally_fn: fn(&[bool]) -> Vec<bool>,
        rotate_once_fn: fn(&[bool]) -> Vec<bool>,
    ) {
        let rules: Vec<_> = rules_map
            .iter()
            .flat_map(|(k, v)| {
                let mut rules: Vec<(Vec<bool>, Vec<bool>)> = Vec::new();
                rules.push((flip_vertically_fn(k), v.clone()));
                rules.push((flip_horizontally_fn(k), v.clone()));
                let mut rk = k.clone();
                for _ in 0..3 {
                    rk = rotate_once_fn(&rk);
                    rules.push((rk.clone(), v.clone()));
                    // After each rotation we may also flip again
                    rules.push((flip_vertically_fn(&rk), v.clone()));
                    rules.push((flip_horizontally_fn(&rk), v.clone()));
                }
                rules
            })
            .collect();
        // Add only the ones that not yet in the map.
        for r in rules {
            rules_map.entry(r.0).or_insert(r.1);
        }
    }

    fn extend_rules(&mut self) {
        Self::extend_rules_map(
            &mut self.rules2to3,
            |p| vec![p[1], p[0], p[3], p[2]], // vertical flip
            |p| vec![p[2], p[3], p[0], p[1]], // horizontal flip
            |p| vec![p[2], p[0], p[3], p[1]], // one rotation
        );
        Self::extend_rules_map(
            &mut self.rules3to4,
            |p| vec![p[2], p[1], p[0], p[5], p[4], p[3], p[8], p[7], p[6]],
            |p| vec![p[6], p[7], p[8], p[3], p[4], p[5], p[0], p[1], p[2]],
            |p| vec![p[6], p[3], p[0], p[7], p[4], p[1], p[8], p[5], p[2]],
        );
        // After extensions we should have all permutations covered
        // assert_eq!(eh_rules.rules2to3.len(), usize::pow(2, 2 * 2));
        // assert_eq!(eh_rules.rules3to4.len(), usize::pow(2, 3 * 3));
    }
}

// Divide the grid into 2x2 or 3x3 squares, depending on the divisible rules.
fn divide_grid(grid: &[bool]) -> Vec<Vec<bool>> {
    let grid_size = sqrt(grid.len());
    let pos_fn = |row: usize, col: usize| row * grid_size + col;

    let small_square_size = if grid_size % 2 == 0 {
        2
    } else if grid_size % 3 == 0 {
        3
    } else {
        panic!("Something wrong with grid sizes {}", grid_size)
    };

    let mut small_squares: Vec<Vec<bool>> = Vec::new();
    for row in (0..grid_size).step_by(small_square_size) {
        for col in (0..grid_size).step_by(small_square_size) {
            small_squares.push(if small_square_size == 2 {
                // Extract 2x2 square
                vec![
                    grid[pos_fn(row, col)],
                    grid[pos_fn(row, col + 1)],
                    grid[pos_fn(row + 1, col)],
                    grid[pos_fn(row + 1, col + 1)],
                ]
            } else if small_square_size == 3 {
                // Extract 3x3 square
                vec![
                    grid[pos_fn(row, col)],
                    grid[pos_fn(row, col + 1)],
                    grid[pos_fn(row, col + 2)],
                    grid[pos_fn(row + 1, col)],
                    grid[pos_fn(row + 1, col + 1)],
                    grid[pos_fn(row + 1, col + 2)],
                    grid[pos_fn(row + 2, col)],
                    grid[pos_fn(row + 2, col + 1)],
                    grid[pos_fn(row + 2, col + 2)],
                ]
            } else {
                panic!("Something wrong with grid sizes {}", grid_size)
            });
        }
    }
    small_squares
}

// Merge a list of smaller 2x2 or 3x3 squares back into a bigger grid.
fn merge_grid(small_squares: &[Vec<bool>]) -> Vec<bool> {
    let small_square_size = sqrt(small_squares[0].len());
    let small_pos_fn = |row: usize, col: usize| row * small_square_size + col;

    let grid_square_cnt = sqrt(small_squares.len());
    let grid_size = small_square_size * grid_square_cnt;
    let big_pos_fn = |row: usize, col: usize| row * grid_size + col;

    let mut grid: Vec<bool> = vec![false; grid_size * grid_size];
    let mut row = 0;
    let mut col = 0;
    for small_sq in small_squares {
        // Copy small square into big grid.
        for r in 0..small_square_size {
            for c in 0..small_square_size {
                grid[big_pos_fn(row + r, col + c)] = small_sq[small_pos_fn(r, c)];
            }
        }
        col += small_square_size;
        if col >= grid_size {
            row += small_square_size;
            col = 0;
        }
    }
    grid
}

// Find the enhanced square for each of the small squares.
fn match_rules(rules: &EnhancementRules, small_squares: &[Vec<bool>]) -> Vec<Vec<bool>> {
    (0..small_squares.len())
        .map(|i| rules.get_rule(&small_squares[i]))
        .collect()
}

// .#.
// ..#
// ###
const INITIAL_GRID: [bool; 9] = [false, true, false, false, false, true, true, true, true];

fn pixel_on_count_after(rules: &EnhancementRules, iterations: usize) -> usize {
    let mut grid = INITIAL_GRID.to_vec();
    // print_grid(&grid);

    for _ in 0..iterations {
        let divided = divide_grid(&grid);

        let new_squares = match_rules(rules, &divided);

        grid = merge_grid(&new_squares);
        // print_grid(&grid);
    }
    grid.iter().filter(|v| **v).count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let rules = EnhancementRules::build(&input);

    println!("Part 1: {}", pixel_on_count_after(&rules, 5));
    println!("Part 2: {}", pixel_on_count_after(&rules, 18));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    fn build_square(s: &str) -> Vec<bool> {
        s.lines()
            .flat_map(|line| line.trim().chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect()
    }

    #[test]
    fn test_merge_grid3() {
        let squares = vec![
            build_square(
                r"#..
                  .#.
                  ..#",
            ),
            build_square(
                r"#..
                  ###
                  #..",
            ),
            build_square(
                r"#..
                  .#.
                  ###",
            ),
            build_square(
                r"#..
                  ###
                  #..",
            ),
        ];
        let merged = merge_grid(&squares);
        assert_eq!(
            merged,
            build_square(
                r"#..#..
                  .#.###
                  ..##..
                  #..#..
                  .#.###
                  ####.."
            )
        );
    }

    #[test]
    fn test_merge_and_divide_grid2() {
        let squares = vec![
            build_square(
                r"#.
                  ..",
            ),
            build_square(
                r"#.
                  #.",
            ),
            build_square(
                r"#.
                  ##",
            ),
            build_square(
                r"#.
                  #.",
            ),
        ];
        let merged = merge_grid(&squares);
        assert_eq!(
            merged,
            build_square(
                r"#.#.
                  ..#.
                  #.#.
                  ###."
            )
        );
        let divided = divide_grid(&merged);
        assert_eq!(divided, squares);
    }

    #[test]
    fn test_divide_grid3() {
        let grid = build_square(
            r"#.#..#.##
              ..###.#..
              .##...#.#
              #..#...#.
              #..#...##
              #.#.#.#..
              .#.##...#
              .#.##.#.#
              ###.###..",
        );
        let divided = divide_grid(&grid);
        let result = vec![
            build_square(
                r"#.#
                  ..#
                  .##",
            ),
            build_square(
                r"..#
                  ##.
                  ...",
            ),
            build_square(
                r".##
                  #..
                  #.#",
            ),
            build_square(
                r"#..
                  #..
                  #.#",
            ),
            build_square(
                r"#..
                  #..
                  .#.",
            ),
            build_square(
                r".#.
                  .##
                  #..",
            ),
            build_square(
                r".#.
                  .#.
                  ###",
            ),
            build_square(
                r"##.
                  ##.
                  .##",
            ),
            build_square(
                r"..#
                  #.#
                  #..",
            ),
        ];
        assert_eq!(divided, result);
    }

    #[test]
    fn test_part1() {
        let rules = EnhancementRules::build(INPUT_TEST);

        assert_eq!(pixel_on_count_after(&rules, 2), 12);
    }
}
