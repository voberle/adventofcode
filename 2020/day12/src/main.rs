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
            _ => panic!("Invalid index {i}"),
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

trait Position {
    fn apply_north(&mut self, val: i32);
    fn apply_south(&mut self, val: i32);
    fn apply_west(&mut self, val: i32);
    fn apply_east(&mut self, val: i32);
    fn apply_left(&mut self, val: i32);
    fn apply_right(&mut self, val: i32);
    fn apply_forward(&mut self, val: i32);

    fn distance_from_zero(&self) -> i32;
}

struct PosWithDirection {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Position for PosWithDirection {
    fn apply_north(&mut self, val: i32) {
        // Action N means to move north by the given value.
        self.y -= val;
    }

    fn apply_south(&mut self, val: i32) {
        self.y += val;
    }

    fn apply_west(&mut self, val: i32) {
        self.x -= val;
    }

    fn apply_east(&mut self, val: i32) {
        self.x += val;
    }

    fn apply_left(&mut self, val: i32) {
        // Action L means to turn left the given number of degrees.
        self.dir = self.dir.turn_left(val);
    }

    fn apply_right(&mut self, val: i32) {
        self.dir = self.dir.turn_right(val);
    }

    fn apply_forward(&mut self, val: i32) {
        // Action F means to move forward by the given value in the direction the ship is currently facing.
        match self.dir {
            Direction::North => self.apply_north(val),
            Direction::East => self.apply_east(val),
            Direction::South => self.apply_south(val),
            Direction::West => self.apply_west(val),
        }
    }

    fn distance_from_zero(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

struct PosWithWaypoint {
    x: i32,
    y: i32,
    wp_x: i32,
    wp_y: i32,
}

impl Position for PosWithWaypoint {
    fn apply_north(&mut self, val: i32) {
        // Action N means to move the waypoint north by the given value.
        self.wp_y -= val;
    }

    fn apply_south(&mut self, val: i32) {
        self.wp_y += val;
    }

    fn apply_west(&mut self, val: i32) {
        self.wp_x -= val;
    }

    fn apply_east(&mut self, val: i32) {
        self.wp_x += val;
    }

    fn apply_left(&mut self, val: i32) {
        // Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
        let turn_count = (val / 90) % 4;
        for _ in 0..turn_count {
            let x = self.wp_y;
            let y = -self.wp_x;
            self.wp_x = x;
            self.wp_y = y;
        }
    }

    fn apply_right(&mut self, val: i32) {
        let turn_count = (val / 90) % 4;
        for _ in 0..turn_count {
            let x = -self.wp_y;
            let y = self.wp_x;
            self.wp_x = x;
            self.wp_y = y;
        }
    }

    fn apply_forward(&mut self, val: i32) {
        // Action F means to move forward to the waypoint a number of times equal to the given value.
        self.x += self.wp_x * val;
        self.y += self.wp_y * val;
        // The waypoint is relative to the ship; that is, if the ship moves, the waypoint moves with it.
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

    fn apply(&self, ship_pos: &mut dyn Position) {
        match self {
            Instruction::North(val) => {
                ship_pos.apply_north(*val);
            }
            Instruction::South(val) => {
                ship_pos.apply_south(*val);
            }
            Instruction::East(val) => {
                ship_pos.apply_east(*val);
            }
            Instruction::West(val) => {
                ship_pos.apply_west(*val);
            }
            Instruction::Left(val) => {
                ship_pos.apply_left(*val);
            }
            Instruction::Right(val) => {
                ship_pos.apply_right(*val);
            }
            Instruction::Forward(val) => {
                ship_pos.apply_forward(*val);
            }
        }
    }
}

fn build(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::new).collect()
}

fn distance_from_start(instructions: &[Instruction], ship_pos: &mut dyn Position) -> i32 {
    for ins in instructions {
        ins.apply(ship_pos);
    }
    ship_pos.distance_from_zero()
}

fn distance_with_dir(instructions: &[Instruction]) -> i32 {
    let mut ship_pos = PosWithDirection {
        x: 0,
        y: 0,
        dir: Direction::East,
    };
    distance_from_start(instructions, &mut ship_pos)
}

fn distance_with_waypoint(instructions: &[Instruction]) -> i32 {
    let mut ship_pos = PosWithWaypoint {
        x: 0,
        y: 0,
        wp_x: 10,
        wp_y: -1,
    };
    distance_from_start(instructions, &mut ship_pos)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", distance_with_dir(&instructions));
    println!("Part 2: {}", distance_with_waypoint(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(distance_with_dir(&build(INPUT_TEST)), 25);
    }

    #[test]
    fn test_part2() {
        assert_eq!(distance_with_waypoint(&build(INPUT_TEST)), 286);
    }
}
