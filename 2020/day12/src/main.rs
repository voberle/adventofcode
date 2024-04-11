use std::io::{self, Read};

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    // Index clock-wise.
    fn index(self) -> i32 {
        match self {
            Self::North => 0,
            Self::East => 1,
            Self::South => 2,
            Self::West => 3,
        }
    }

    fn from_index(i: i32) -> Self {
        match i {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            _ => panic!("Invalid index {}", i),
        }
    }

    fn turn_left(self, val: i32) -> Direction {
        let turn_count = val / 90;
        Self::from_index((self.index() + 4 - turn_count) % 4)
    }

    fn turn_right(self, val: i32) -> Direction {
        let turn_count = val / 90;
        Self::from_index((self.index() + turn_count) % 4)
    }
}

struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn move_to(&self, dir: Direction, val: i32) -> Self {
        match dir {
            Direction::North => Self {
                x: self.x,
                y: self.y - val,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y + val,
            },
            Direction::West => Self {
                x: self.x - val,
                y: self.y,
            },
            Direction::East => Self {
                x: self.x + val,
                y: self.y,
            },
        }
    }

    fn distance_from_zero(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

enum Instruction {
    // Action N means to move north by the given value.
    North(i32),
    // Action S means to move south by the given value.
    South(i32),
    // Action E means to move east by the given value.
    East(i32),
    // Action W means to move west by the given value.
    West(i32),
    // Action L means to turn left the given number of degrees.
    Left(i32),
    // Action R means to turn right the given number of degrees.
    Right(i32),
    // Action F means to move forward by the given value in the direction the ship is currently facing.
    Forward(i32),
}

impl Instruction {
    fn new(s: &str) -> Self {
        let action = s.chars().next().unwrap();
        let val = s[1..].parse().unwrap();
        match action {
            'N' => Self::North(val),
            'S' => Self::South(val),
            'E' => Self::East(val),
            'W' => Self::West(val),
            'L' => Self::Left(val),
            'R' => Self::Right(val),
            'F' => Self::Forward(val),
            _ => panic!("Invalid action"),
        }
    }

    fn apply(&self, pos: &mut Pos, dir: &mut Direction) {
        match self {
            Instruction::North(val) => {
                *pos = pos.move_to(Direction::North, *val);
            }
            Instruction::South(val) => {
                *pos = pos.move_to(Direction::South, *val);
            }
            Instruction::East(val) => {
                *pos = pos.move_to(Direction::East, *val);
            }
            Instruction::West(val) => {
                *pos = pos.move_to(Direction::West, *val);
            }
            Instruction::Left(val) => {
                *dir = dir.turn_left(*val);
            }
            Instruction::Right(val) => {
                *dir = dir.turn_right(*val);
            }
            Instruction::Forward(val) => {
                *pos = pos.move_to(*dir, *val);
            }
        }
    }
}

fn build(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::new).collect()
}

fn distance_from_start(instructions: &[Instruction]) -> i32 {
    let mut dir = Direction::East;
    let mut pos = Pos { x: 0, y: 0 };
    for ins in instructions {
        ins.apply(&mut pos, &mut dir);
    }
    pos.distance_from_zero()
}

fn part2(instructions: &[Instruction]) -> i32 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", distance_from_start(&instructions));
    println!("Part 2: {}", part2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(distance_from_start(&build(INPUT_TEST)), 25);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
