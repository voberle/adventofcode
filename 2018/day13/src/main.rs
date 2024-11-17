use std::io::{self, Read};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::{Down, Left, Right, Up};

impl Direction {
    fn new(c: char) -> Self {
        match c {
            '<' => Left,
            '^' => Up,
            '>' => Right,
            'v' => Down,
            _ => panic!("Invalid direction"),
        }
    }

    fn opposite(self) -> Self {
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }

    fn turn_left(self) -> Self {
        self.turn_right().opposite()
    }
}

#[derive(Debug)]
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

    fn print_with_pos(&self, positions: &[usize]) {
        const BOLD: &str = "\x1b[1m";
        const GREEN: &str = "\x1b[32m";
        const RESET: &str = "\x1b[0m";
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                if positions.contains(&p) {
                    print!("{BOLD}{GREEN}{c}{RESET}");
                } else {
                    print!("{c}");
                }
            }
            println!();
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        self.print_with_pos(&[]);
    }

    fn col(&self, index: usize) -> usize {
        index % self.cols
    }

    fn row(&self, index: usize) -> usize {
        index / self.cols
    }

    fn next_pos(&self, pos: usize, direction: Direction) -> usize {
        match direction {
            Up => pos - self.cols,
            Right => pos + 1,
            Down => pos + self.cols,
            Left => pos - 1,
        }
    }
}

// Returns the next position, next direction and decision we took at the last intersection.
// last_intersection can be None (aka straight), Some(Left), Some(Right)
fn next_cart_position(
    map: &Grid,
    cart: usize,
    direction: Direction,
    mut last_intersection: Option<Direction>,
) -> (usize, Direction, Option<Direction>) {
    // Not checking if we can leave the grid, as it should not be possible.
    let next_pos = map.next_pos(cart, direction);
    let next_symbol = map.values[next_pos];

    let next_direction = match next_symbol {
        '-' | '|' | '<' | '^' | '>' | 'v' => direction,
        '/' => match direction {
            Up => Right,
            Down => Left,
            Left => Down,
            Right => Up,
        },
        '\\' => match direction {
            Up => Left,
            Down => Right,
            Left => Up,
            Right => Down,
        },
        '+' => match last_intersection {
            Some(Left) => {
                // last time we turned left, now go straight
                last_intersection = None;
                direction
            }
            None => {
                // last time was straight, now turn right
                last_intersection = Some(Right);
                direction.turn_right()
            }
            Some(Right) => {
                // last time was straight, now turn right
                last_intersection = Some(Left);
                direction.turn_left()
            }
            _ => panic!("Invalid last intersection!"),
        },
        _ => panic!("Cart left the track!"),
    };
    (next_pos, next_direction, last_intersection)
}

// Returns the initial position and direction of all carts.
// The last params is the decision made at previous intersection.
// We want to have first decision to be left, so setting it to right.
fn find_carts(map: &Grid) -> Vec<(usize, Direction, Option<Direction>)> {
    map.values
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if ['<', '^', '>', 'v'].contains(v) {
                Some((i, Direction::new(*v), Some(Right)))
            } else {
                None
            }
        })
        .collect()
}

fn move_carts<const REMOVE_ON_CRASH: bool>(
    map: &Grid,
    carts: &mut [(usize, Direction, Option<Direction>)],
) -> usize {
    // We use the magic value MAX for the position to indicate that a cart has been removed from the track.
    const REMOVED: usize = usize::MAX;

    let carts_len = carts.len();

    let mut number_of_carts_left = carts_len;
    while number_of_carts_left > 1 {
        // map.print_with_pos(&carts.iter().map(|c| c.0).collect::<Vec<usize>>());

        // Top rows moves first, left to right, so we need to have the carts sorted in that order before moving them.
        carts.sort_by_key(|c| c.0);

        for i in 0..carts_len {
            let pos = carts[i].0;
            if pos == REMOVED {
                continue;
            }

            let next = next_cart_position(map, pos, carts[i].1, carts[i].2);

            // Catching immediately any crash.
            if REMOVE_ON_CRASH {
                let crash_idxs: Vec<_> = carts
                    .iter()
                    .enumerate()
                    .filter_map(|(i, c)| if c.0 == next.0 { Some(i) } else { None })
                    .collect();
                if crash_idxs.is_empty() {
                    carts[i] = next;
                } else {
                    // Note that if we have one cart left, we let the tick finish to make sure it gets at its final location.
                    number_of_carts_left -= crash_idxs.len() + 1;
                    carts[i].0 = REMOVED;
                    for o in crash_idxs {
                        carts[o].0 = REMOVED;
                    }
                }
            } else {
                if carts.iter().any(|c| c.0 == next.0) {
                    return next.0;
                }
                carts[i] = next;
            }
        }
    }
    carts.iter().find(|c| c.0 != REMOVED).unwrap().0
}

fn first_crash(map: &Grid) -> (usize, usize) {
    let mut carts = find_carts(map);

    let crash_pos = move_carts::<false>(map, &mut carts);
    (map.col(crash_pos), map.row(crash_pos))
}

fn last_cart_location(map: &Grid) -> (usize, usize) {
    let mut carts = find_carts(map);

    let last_cart_pos = move_carts::<true>(map, &mut carts);
    (map.col(last_cart_pos), map.row(last_cart_pos))
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = Grid::build(&input);

    let (x, y) = first_crash(&map);
    println!("Part 1: {x},{y}");

    let (x, y) = last_cart_location(&map);
    println!("Part 2: {x},{y}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(first_crash(&Grid::build(INPUT_TEST_1)), (7, 3));
    }

    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part2() {
        assert_eq!(last_cart_location(&Grid::build(INPUT_TEST_2)), (6, 4));
    }
}
