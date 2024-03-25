use itertools::Itertools;
use std::io::{self, Read};

use intcode::IntcodeComputer;

    fn get_scaffolds_view(computer: &mut IntcodeComputer) -> Vec<char> {
        computer.exec();

        let mut scaffolds: Vec<char> = Vec::new();
        while let Some(i) = computer.io.get_output() {
            scaffolds.push(char::from_u32(u32::try_from(i).unwrap()).unwrap());
        }

        scaffolds
    }

    #[allow(dead_code)]
    fn print_scaffolds_view(scaffolds: &[char]) {
        for c in scaffolds {
            print!("{}", c);
        }
    }

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::{East, North, South, West};

struct Grid {
    values: Vec<char>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn convert(scaffolds: &[char]) -> Self {
        let cols = scaffolds.iter().position(|c| *c == '\n').unwrap();
        let mut values = scaffolds.to_vec();
        values.retain(|&c| c != '\n');
        let rows = values.len() / cols;
        Self { values, rows, cols }
    }

    fn col(&self, index: usize) -> usize {
        index % self.cols
    }

    fn row(&self, index: usize) -> usize {
        index / self.cols
    }

    fn allowed(&self, pos: usize, direction: Direction) -> bool {
        !match direction {
            North => pos < self.cols,
            East => pos % self.cols == self.cols - 1,
            South => pos / self.cols == self.rows - 1,
            West => pos % self.cols == 0,
        }
    }

    fn next_pos(&self, pos: usize, direction: Direction) -> usize {
        match direction {
            North => pos - self.cols,
            East => pos + 1,
            South => pos + self.cols,
            West => pos - 1,
        }
    }

    fn is_scaffold_in(&self, pos: usize, direction: Direction) -> bool {
        if self.allowed(pos, direction) {
            let np = self.next_pos(pos, direction);
            self.values[np] == '#'
        } else {
            false
        }
    }

    // An intersection is a point with scaffolds in all 4 directions.
    fn is_intersection(&self, pos: usize) -> bool {
        self.values[pos] == '#'
            && self.is_scaffold_in(pos, North)
            && self.is_scaffold_in(pos, South)
            && self.is_scaffold_in(pos, West)
            && self.is_scaffold_in(pos, East)
    }

    fn get_intersections(&self) -> Vec<usize> {
        self.values
            .iter()
            .enumerate()
            .filter_map(|(pos, _)| {
                if self.is_intersection(pos) {
                    Some(pos)
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_alignment_parameter(&self, pos: usize) -> usize {
        self.col(pos) * self.row(pos)
    }
}

fn alignment_params_sum(computer: &IntcodeComputer) -> usize {
    let mut computer = computer.clone();

    let scaffolds_view = get_scaffolds_view(&mut computer);
    // print_scaffolds_view(&scaffolds_view);

    let scaffolds = Grid::convert(&scaffolds_view);

    let intersections = scaffolds.get_intersections();
    intersections
        .iter()
        .map(|p| scaffolds.get_alignment_parameter(*p))
        .sum()
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum Instruction {
    A,
    B,
    C,
    Left(usize),
    Right(usize),
    VideoFeedOn,
    VideoFeedOff,
}
use Instruction::{Left, Right, VideoFeedOff, VideoFeedOn, A, B, C};

impl Instruction {
    fn get_string(self) -> String {
        match self {
            A => "A".to_string(),
            B => "B".to_string(),
            C => "C".to_string(),
            Left(n) => format!("L,{}", n),
            Right(n) => format!("R,{}", n),
            VideoFeedOn => "y".to_string(),
            VideoFeedOff => "n".to_string(),
        }
    }
}

fn build_computer_string(input: &[Instruction]) -> String {
    input.iter().map(|i| i.get_string()).join(",")
}

fn computer_write_line(computer: &mut IntcodeComputer, input: &[Instruction]) {
    const NEWLINE: i64 = 10;

    let s = build_computer_string(input);
    assert!(s.len() <= 20, "Input string too big: {}", s.len());

    s.chars().map(|c| c as i64).for_each(|i| {
        computer.io.add_input(i);
    });
    computer.io.add_input(NEWLINE);
}

fn collected_dust_amount(computer: &IntcodeComputer) -> i64 {
    let mut computer = computer.clone();
    // Wake the robot up.
    computer.write_mem(0, 2);

    // The path was computed by following the scaffolds ignoring the intersections,
    // and then finding the common parts in it by hand.
    // A: R10,L12,R6
    // B: R6,R10,R12,R6
    // C: R10,L12,L12
    // A,A,B,C,B,C,B,C,B,A
    let movement_fcts = vec![A, A, B, C, B, C, B, C, B, A];
    let a_fct = vec![Right(10), Left(12), Right(6)];
    let b_fct = vec![Right(6), Right(10), Right(12), Right(6)];
    let c_fct = vec![Right(10), Left(12), Left(12)];
    let video_feed = vec![VideoFeedOff];

    computer_write_line(&mut computer, &movement_fcts);
    computer_write_line(&mut computer, &a_fct);
    computer_write_line(&mut computer, &b_fct);
    computer_write_line(&mut computer, &c_fct);
    computer_write_line(&mut computer, &video_feed);

    computer.exec();

    // The robot prints all its map even if the feed is off, so skipping it
    while let Some(i) = computer.io.get_output() {
        if i > 255 {
            // Not an ASCII code
            return i;
        }
    }
    panic!("No dust amount found");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let computer = IntcodeComputer::build(&input);

    println!("Part 1: {}", alignment_params_sum(&computer));
    println!("Part 2: {}", collected_dust_amount(&computer));
}
