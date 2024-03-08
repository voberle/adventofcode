use std::io::{self, Read};

use fxhash::FxHashMap;
use intcode::IntcodeComputer;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    Black,
    White,
}

impl Color {
    fn new(c: i64) -> Self {
        match c {
            0 => Self::Black,
            1 => Self::White,
            _ => panic!("Invalid color int {}", c),
        }
    }

    fn get_program_input(self) -> i64 {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Turn {
    Left,
    Right,
}

impl Turn {
    fn new(d: i64) -> Self {
        match d {
            0 => Self::Left,
            1 => Self::Right,
            _ => panic!("Invalid direction int {}", d),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(self, turn: Turn) -> Self {
        use Direction::{East, North, South, West};
        use Turn::Left;
        match self {
            North => {
                if turn == Left {
                    West
                } else {
                    East
                }
            }
            South => {
                if turn == Left {
                    East
                } else {
                    West
                }
            }
            East => {
                if turn == Left {
                    North
                } else {
                    South
                }
            }
            West => {
                if turn == Left {
                    South
                } else {
                    North
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    fn move_into(&mut self, dir: Direction) {
        match dir {
            Direction::North => self.x -= 1,
            Direction::South => self.x += 1,
            Direction::West => self.y -= 1,
            Direction::East => self.y += 1,
        }
    }
}

fn run(computer: &mut IntcodeComputer, input: i64) -> Option<(Color, Turn)> {
    computer.input.push_back(input);

    computer.exec();

    if computer.halted {
        None
    } else {
        assert_eq!(computer.output.len(), 2);
        let direction = Turn::new(computer.output.pop().unwrap());
        let color = Color::new(computer.output.pop().unwrap());
        Some((color, direction))
    }
}

fn panels_painted_count(computer: &IntcodeComputer) -> usize {
    let mut computer = computer.clone();

    let mut panels: FxHashMap<Position, Color> = FxHashMap::default();
    let mut pos = Position::zero();
    let mut dir = Direction::North;

    loop {
        let color = panels.entry(pos).or_insert(Color::Black);
        if let Some((color_to_paint, turn_to_take)) = run(&mut computer, color.get_program_input())
        {
            panels.insert(pos, color_to_paint);
            dir = dir.turn(turn_to_take);
            pos.move_into(dir);
        } else {
            break;
        }
    }
    panels.len()
}

fn part2(computer: &IntcodeComputer) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let computer = IntcodeComputer::build(&input);

    println!("Part 1: {}", panels_painted_count(&computer));
    println!("Part 2: {}", part2(&computer));
}
