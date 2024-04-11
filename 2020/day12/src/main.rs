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
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
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

    fn apply(&self, ship_pos: &mut Pos, dir: &mut Direction) {
        match self {
            Instruction::North(val) => {
                // Action N means to move north by the given value.
                *ship_pos = ship_pos.move_to(Direction::North, *val);
            }
            Instruction::South(val) => {
                *ship_pos = ship_pos.move_to(Direction::South, *val);
            }
            Instruction::East(val) => {
                *ship_pos = ship_pos.move_to(Direction::East, *val);
            }
            Instruction::West(val) => {
                *ship_pos = ship_pos.move_to(Direction::West, *val);
            }
            Instruction::Left(val) => {
                // Action L means to turn left the given number of degrees.
                *dir = dir.turn_left(*val);
            }
            Instruction::Right(val) => {
                *dir = dir.turn_right(*val);
            }
            Instruction::Forward(val) => {
                // Action F means to move forward by the given value in the direction the ship is currently facing.
                *ship_pos = ship_pos.move_to(*dir, *val);
            }
        }
    }

    fn apply_with_waypoint(&self, ship_pos: &mut Pos, waypoint_relative_pos: &mut Pos) {
        match self {
            Instruction::North(val) => {
                // Action N means to move the waypoint north by the given value.
                *waypoint_relative_pos = waypoint_relative_pos.move_to(Direction::North, *val);
            }
            Instruction::South(val) => {
                *waypoint_relative_pos = waypoint_relative_pos.move_to(Direction::South, *val);
            }
            Instruction::East(val) => {
                *waypoint_relative_pos = waypoint_relative_pos.move_to(Direction::East, *val);
            }
            Instruction::West(val) => {
                *waypoint_relative_pos = waypoint_relative_pos.move_to(Direction::West, *val);
            }
            Instruction::Left(val) => {
                // Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
                let turn_count = (val / 90) % 4;
                for _ in 0..turn_count {
                    *waypoint_relative_pos = Pos {
                        x: waypoint_relative_pos.y,
                        y: -waypoint_relative_pos.x,
                    };
                }
            }
            Instruction::Right(val) => {
                let turn_count = (val / 90) % 4;
                for _ in 0..turn_count {
                    *waypoint_relative_pos = Pos {
                        x: -waypoint_relative_pos.y,
                        y: waypoint_relative_pos.x,
                    };
                }
            }
            Instruction::Forward(val) => {
                // Action F means to move forward to the waypoint a number of times equal to the given value.
                ship_pos.x += waypoint_relative_pos.x * val;
                ship_pos.y += waypoint_relative_pos.y * val;
                // The waypoint is relative to the ship; that is, if the ship moves, the waypoint moves with it.
            }
        }
    }
}

fn build(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::new).collect()
}

fn distance_from_start(instructions: &[Instruction]) -> i32 {
    let mut dir = Direction::East;
    let mut ship_pos = Pos { x: 0, y: 0 };
    for ins in instructions {
        ins.apply(&mut ship_pos, &mut dir);
    }
    ship_pos.distance_from_zero()
}

fn distance_with_waypoint(instructions: &[Instruction]) -> i32 {
    let mut ship_pos = Pos { x: 0, y: 0 };
    // 10 units east and 1 unit north of the ship.
    let mut waypoint = Pos { x: 10, y: -1 };
    for ins in instructions {
        ins.apply_with_waypoint(&mut ship_pos, &mut waypoint);
    }
    ship_pos.distance_from_zero()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", distance_from_start(&instructions));
    println!("Part 2: {}", distance_with_waypoint(&instructions));
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
        assert_eq!(distance_with_waypoint(&build(INPUT_TEST)), 286);
    }
}
