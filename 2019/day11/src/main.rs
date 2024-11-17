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
            _ => panic!("Invalid color int {c}"),
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
            _ => panic!("Invalid direction int {d}"),
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
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    fn move_into(&mut self, dir: Direction) {
        match dir {
            Direction::North => self.y -= 1,
            Direction::South => self.y += 1,
            Direction::West => self.x -= 1,
            Direction::East => self.x += 1,
        }
    }
}

fn run(computer: &mut IntcodeComputer, input: i64) -> Option<(Color, Turn)> {
    computer.io.add_input(input);

    computer.exec();

    if computer.is_halted() {
        None
    } else {
        let color = Color::new(computer.io.get_output().unwrap());
        let direction = Turn::new(computer.io.get_output().unwrap());
        Some((color, direction))
    }
}

fn paint(computer: &IntcodeComputer, panels: &mut FxHashMap<Position, Color>) {
    let mut computer = computer.clone();

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
}

fn panels_painted_count(computer: &IntcodeComputer) -> usize {
    let mut panels: FxHashMap<Position, Color> = FxHashMap::default();
    paint(computer, &mut panels);
    // print_panels(&panels);
    panels.len()
}

fn borders(panels: &FxHashMap<Position, Color>) -> (Position, Position) {
    // Not using iterator min / max to keep only one loop.
    let mut min_pos = Position::new(i32::MAX, i32::MAX);
    let mut max_pos = Position::new(i32::MIN, i32::MIN);
    for pos in panels.keys() {
        min_pos.x = min_pos.x.min(pos.x);
        max_pos.x = max_pos.x.max(pos.x);
        min_pos.y = min_pos.y.min(pos.y);
        max_pos.y = max_pos.y.max(pos.y);
    }
    (min_pos, max_pos)
}

fn print_panels(panels: &FxHashMap<Position, Color>) {
    let (min_pos, max_pos) = borders(panels);
    for y in min_pos.y..=max_pos.y {
        for x in min_pos.x..=max_pos.x {
            if let Some(color) = panels.get(&Position::new(x, y)) {
                if *color == Color::White {
                    print!("\u{2B1B}");
                    continue;
                }
            }
            print!("\u{2B1C}");
        }
        println!();
    }
}

fn get_registration_identifier(computer: &IntcodeComputer) -> FxHashMap<Position, Color> {
    let mut panels: FxHashMap<Position, Color> = FxHashMap::default();
    panels.insert(Position::zero(), Color::White);

    paint(computer, &mut panels);
    panels
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let computer = IntcodeComputer::build(&input);

    println!("Part 1: {}", panels_painted_count(&computer));
    let panels = get_registration_identifier(&computer);
    println!("Part 2:");
    print_panels(&panels);
}
