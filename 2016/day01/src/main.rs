use std::io::{self, Read};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Turn {
    Left,
    Right,
}

fn build(input: &str) -> Vec<(Turn, i32)> {
    input
        .split(", ")
        .map(|i| {
            let p = i.split_at(1);
            (
                if p.0 == "L" { Turn::Left } else { Turn::Right },
                p.1.parse().unwrap(),
            )
        })
        .collect()
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;

impl Direction {
    fn turn(&self, t: Turn) -> Self {
        match self {
            North => {
                if t == Turn::Left {
                    West
                } else {
                    East
                }
            }
            East => {
                if t == Turn::Left {
                    North
                } else {
                    South
                }
            }
            South => {
                if t == Turn::Left {
                    East
                } else {
                    West
                }
            }
            West => {
                if t == Turn::Left {
                    South
                } else {
                    North
                }
            }
        }
    }
}

fn blocks_away_count(instructions: &[(Turn, i32)]) -> i32 {
    let mut hor_idx: i32 = 0;
    let mut ver_idx: i32 = 0;
    let mut dir = North;
    for i in instructions {
        dir = dir.turn(i.0);
        match dir {
            North => ver_idx -= i.1,
            South => ver_idx += i.1,
            West => hor_idx -= i.1,
            East => hor_idx += i.1,
        }
    }
    hor_idx.abs() + ver_idx.abs()
}

fn part2(instructions: &[(Turn, i32)]) -> i32 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", blocks_away_count(&instructions));
    println!("Part 2: {}", part2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(blocks_away_count(&build("R2, L3")), 5);
        assert_eq!(blocks_away_count(&build("R2, R2, R2")), 2);
        assert_eq!(blocks_away_count(&build("R5, L5, R5, R3")), 12);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build("")), 0);
    }
}
