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
                l.chars()
                    // .map(|c| c)
                    .collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    fn print_with_pos(&self, positions: &[usize]) {
        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                if positions.contains(&p) {
                    print!("{RED}{}{RESET}", c);
                } else {
                    print!("{}", c);
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

fn find_carts(map: &Grid) -> Vec<(usize, Direction)> {
    map.values
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if ['<', '^', '>', 'v'].contains(v) {
                Some((i, Direction::new(*v)))
            } else {
                None
            }
        })
        .collect()
}

// last_intersection can be None aka straight, Some(Left), Some(Right)
fn next_cart_position(
    map: &Grid,
    cart: usize,
    direction: Direction,
    last_intersection: &mut Option<Direction>,
) -> (usize, Direction) {

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
                *last_intersection = None;
                direction
            }
            None => {
                // last time was straight, now turn right
                *last_intersection = Some(Right);
                direction.turn_right()
            }
            Some(Right) => {
                // last time was straight, now turn right
                *last_intersection = Some(Left);
                direction.turn_left()
            }
            _ => panic!("Invalid last intersection!"),
        },
        _ => panic!("Cart left the track!"),
    };
    (next_pos, next_direction)
}

fn first_crash(map: &Grid) -> (usize, usize) {
    let mut carts = find_carts(map);
    // Which turn each cart took at last intersection.
    let mut last_intersections = vec![Some(Right); carts.len()];

    let carts_len = carts.len();
    loop {
        // map.print_with_pos(&carts.iter().map(|c| c.0).collect::<Vec<usize>>());

        // Top rows moves first, left to right. That fits well with our grid.
        for i in 0..carts_len {
            let next = next_cart_position(map, carts[i].0, carts[i].1, &mut last_intersections[i]);

            // Multiple carts can collide at the same time, so catching them here, not after the loop
            if carts.iter().any(|c| c.0 == next.0) {
                return (map.col(next.0), map.row(next.0));
            }
            carts[i] = next;
        }
    }
    panic!("No collisions found")
}

fn part2(map: &Grid) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = Grid::build(&input);

    let (x, y) = first_crash(&map);
    println!("Part 1: {},{}", x, y);
    println!("Part 2: {}", part2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(first_crash(&Grid::build(INPUT_TEST)), (7, 3));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&Grid::build(INPUT_TEST)), 0);
    }
}
