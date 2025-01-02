use std::io::{self, Read};

const TRACK: char = '.';
// const WALL: char = '#';
const START: char = 'S';
const END: char = 'E';

#[derive(Debug, Clone, PartialEq)]
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

    #[allow(dead_code)]
    fn print_with_pos(&self, positions: &[usize]) {
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

    fn pos(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn row(&self, index: usize) -> usize {
        index / self.cols
    }

    fn col(&self, index: usize) -> usize {
        index % self.cols
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn next_positions_iter(&self, pos: usize) -> impl Iterator<Item = usize> + '_ {
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

    fn find_position_of(&self, element: char) -> usize {
        self.values.iter().position(|c| *c == element).unwrap()
    }
}

// Finds the path from start to end.
// Returned path doesn't include the end position (but includes the start).
fn get_path(map: &Grid) -> Vec<usize> {
    fn is_not_in_path(path: &[usize], pos: usize) -> bool {
        // Maybe over-optimizing here..
        path.len() < 2 || !path[path.len() - 2..].contains(&pos)
    }

    let end = map.find_position_of(END);

    let mut path = Vec::new();

    let mut pos = map.find_position_of(START);
    while pos != end {
        path.push(pos);
        pos = map
            .next_positions_iter(pos)
            .find(|next_pos| {
                is_not_in_path(&path, *next_pos) && [TRACK, END].contains(&map.values[*next_pos])
            })
            .unwrap();
    }
    path
}

// Finds all the possible positions we can get to if we cheat from this position during that maximum duration.
fn get_cheating_destinations(map: &Grid, pos: usize, cheat_max_duration: usize) -> Vec<usize> {
    let row = map.row(pos);
    let col = map.col(pos);

    // We get a square around the position and filter all the tracks that are the right distance.
    // A bit wasteful approach, but simple.
    let min_row = row.saturating_sub(cheat_max_duration + 1);
    let max_row = map.rows.min(row + cheat_max_duration + 2);
    let min_col = col.saturating_sub(cheat_max_duration + 1);
    let max_col = map.cols.min(col + cheat_max_duration + 2);
    (min_row..max_row)
        .flat_map(|r| {
            (min_col..max_col).filter_map(move |c| {
                let p = map.pos(r, c);
                if p == pos || ![TRACK, END].contains(&map.values[p]) {
                    return None;
                }
                if row.abs_diff(r) + col.abs_diff(c) <= cheat_max_duration {
                    Some(map.pos(r, c))
                } else {
                    None
                }
            })
        })
        .collect()
}

// The cost of the cheat is the length of the cheat.
fn cheat_cost(map: &Grid, pos: usize, cheat_pos: usize) -> usize {
    let row = map.row(pos);
    let col = map.col(pos);
    let cheat_row = map.row(cheat_pos);
    let cheat_col = map.col(cheat_pos);
    row.abs_diff(cheat_row) + col.abs_diff(cheat_col)
}

fn cheats_count<const CHEAT_DURATION: usize>(map: &Grid, saving_at_least: usize) -> usize {
    // Important observations:
    // - There is only one path in the maze from start to end.
    // - When cheating, it means we can jump to any track space around within a circle of X picoseconds.
    // - We can cheat a maximum of one time.

    let mut path = get_path(map);

    let base_time = path.len();

    // Adding the end to the path, so that cheats that go straight to end are included.
    path.push(map.find_position_of(END));

    path.iter()
        .enumerate()
        .map(|(time_so_far, pos)| {
            get_cheating_destinations(map, *pos, CHEAT_DURATION)
                .iter()
                .filter(|cheat_pos| {
                    if let Some(pos_to_end) = path.iter().position(|p| p == *cheat_pos) {
                        let time_from_cheat_to_end = base_time - pos_to_end;
                        let cheat_cost = cheat_cost(map, *pos, **cheat_pos);

                        let time = time_so_far + cheat_cost + time_from_cheat_to_end;

                        time < base_time && base_time - time >= saving_at_least
                    } else {
                        false
                    }
                })
                .count()
        })
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = Grid::build(&input);

    println!("Part 1: {}", cheats_count::<2>(&map, 100));
    println!("Part 2: {}", cheats_count::<20>(&map, 100));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(cheats_count::<2>(&Grid::build(INPUT_TEST), 20), 5);
    }

    #[test]
    fn test_part2() {
        const RESULT: usize = 32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3;
        assert_eq!(cheats_count::<20>(&Grid::build(INPUT_TEST), 50), RESULT);
    }
}
