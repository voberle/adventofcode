use std::{
    io::{self, Read},
    usize,
};

use fxhash::FxHashSet;
use itertools::Itertools;

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
use Direction::{Down, Left, Right, Up};

impl From<&str> for Direction {
    fn from(d: &str) -> Self {
        match d {
            "L" => Left,
            "R" => Right,
            "U" => Up,
            "D" => Down,
            _ => panic!("Unknown direction"),
        }
    }
}

struct Motion {
    dir: Direction,
    count: i32,
}

impl From<&str> for Motion {
    fn from(value: &str) -> Self {
        let (dir, count) = value.split_whitespace().collect_tuple().unwrap();
        Self {
            dir: dir.into(),
            count: count.parse().unwrap(),
        }
    }
}

fn build(input: &str) -> Vec<Motion> {
    input.lines().map(Into::into).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    fn is_zero(self) -> bool {
        self.x == 0 && self.y == 0
    }

    fn move_into(&mut self, dir: Direction) {
        match dir {
            Left => self.x -= 1,
            Right => self.x += 1,
            Up => self.y -= 1,
            Down => self.y += 1,
        }
    }

    fn move_left(&mut self) {
        self.x -= 1;
    }

    fn move_right(&mut self) {
        self.x += 1;
    }

    fn move_up(&mut self) {
        self.y -= 1;
    }

    fn move_down(&mut self) {
        self.y += 1;
    }
}

fn borders(map: &FxHashSet<Pos>, head: Pos, tail: Pos) -> (Pos, Pos) {
    let mut min_pos = Pos::new(i32::MAX, i32::MAX);
    let mut max_pos = Pos::new(i32::MIN, i32::MIN);
    for pos in map {
        min_pos.x = min_pos.x.min(pos.x);
        max_pos.x = max_pos.x.max(pos.x);
        min_pos.y = min_pos.y.min(pos.y);
        max_pos.y = max_pos.y.max(pos.y);
    }
    for pos in [head, tail] {
        min_pos.x = min_pos.x.min(pos.x);
        max_pos.x = max_pos.x.max(pos.x);
        min_pos.y = min_pos.y.min(pos.y);
        max_pos.y = max_pos.y.max(pos.y);
    }
    (min_pos, max_pos)
}

#[allow(dead_code)]
fn print(map: &FxHashSet<Pos>, head: Pos, tail: Pos) {
    let (min_pos, max_pos) = borders(map, head, tail);

    for y in min_pos.y..=max_pos.y {
        for x in min_pos.x..=max_pos.x {
            let pos = Pos::new(x, y);
            if pos == head {
                print!("H");
            } else if pos == tail {
                print!("T");
            } else if pos.is_zero() {
                print!("s");
            } else if map.contains(&pos) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[allow(clippy::comparison_chain)]
fn follow(head: Pos, tail: &mut Pos) {
    if head.x == tail.x {
        // On same column.
        if head.y > tail.y + 1 {
            tail.move_down();
        } else if head.y < tail.y - 1 {
            tail.move_up();
        }
    } else if head.y == tail.y {
        // On same row.
        if head.x > tail.x + 1 {
            tail.move_right();
        } else if head.x < tail.x - 1 {
            tail.move_left();
        }
    } else if head.y > tail.y + 1
        || head.y < tail.y - 1
        || head.x > tail.x + 1
        || head.x < tail.x - 1
    {
        // Diagonal.
        if head.y > tail.y {
            tail.move_down();
        } else if head.y < tail.y {
            tail.move_up();
        }
        if head.x > tail.x {
            tail.move_right();
        } else if head.x < tail.x {
            tail.move_left();
        }
    }
}

fn positions_visited<const NODES_COUNT: usize>(motions: &[Motion]) -> usize {
    let mut last_tail_positions: FxHashSet<Pos> = FxHashSet::default();

    // 0 is head
    let mut tails = [Pos::zero(); NODES_COUNT];
    last_tail_positions.insert(*tails.last().unwrap());

    for motion in motions {
        for _ in 0..motion.count {
            tails[0].move_into(motion.dir);

            for i in 1..tails.len() {
                // print(&last_tail_positions, tails[i - 1], tails[i]);

                if tails[i - 1] == tails[i] {
                    // Both nodes cover themselves, follower doesn't move.
                    continue;
                }
                follow(tails[i - 1], &mut tails[i]);
            }

            last_tail_positions.insert(*tails.last().unwrap());
        }
    }
    last_tail_positions.len()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let motions = build(&input);

    println!("Part 1: {}", positions_visited::<2>(&motions));
    println!("Part 2: {}", positions_visited::<10>(&motions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part1() {
        assert_eq!(positions_visited::<2>(&build(INPUT_TEST_1)), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(positions_visited::<10>(&build(INPUT_TEST_1)), 1);
        assert_eq!(positions_visited::<10>(&build(INPUT_TEST_2)), 36);
    }
}
