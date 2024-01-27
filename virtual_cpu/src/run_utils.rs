//! Support code for running all the puzzles.

use std::fs::{self, File};
use std::io::{BufRead, BufReader};

use fxhash::FxHashMap;

pub const RESOURCES_DIR: &str = "src/bin/input";

pub fn get_input_file(puzzle_name: &str) -> String {
    format!("{}/{}_input", RESOURCES_DIR, puzzle_name)
}

// Loads the list of puzzle answers, saving it into a map "puzzle name" => "answer".
pub fn load_answer_list() -> FxHashMap<String, String> {
    let reader = BufReader::new(File::open(format!("{}/answers", RESOURCES_DIR)).unwrap());
    reader
        .lines()
        .map(|line| {
            let l = line.unwrap();
            let p: Vec<_> = l.split(' ').collect();
            (p[0].to_string(), p[1].to_string())
        })
        .collect()
}

type PuzzleFn = fn(&str) -> String;
type GetCCodeFn = fn(&str) -> String;

pub struct Puzzle {
    pub name: String,
    pub year: u32,
    pub day: u32,
    pub part_nb: u8,
    pub puzzle_fn: Option<PuzzleFn>,
    pub c_code_fn: Option<GetCCodeFn>,
}

impl Puzzle {
    fn new(
        name: &str,
        part_nb: u8,
        puzzle_fn: Option<PuzzleFn>,
        c_code_fn: Option<GetCCodeFn>,
    ) -> Self {
        let (year, day) = Self::parse_puzzle_name(name);
        Self {
            name: name.to_string(),
            year,
            day,
            part_nb,
            puzzle_fn,
            c_code_fn,
        }
    }

    pub fn both(name: &str, part_nb: u8, puzzle_fn: PuzzleFn, c_code_fn: GetCCodeFn) -> Self {
        Self::new(name, part_nb, Some(puzzle_fn), Some(c_code_fn))
    }

    pub fn base(name: &str, part_nb: u8, puzzle_fn: PuzzleFn) -> Self {
        Self::new(name, part_nb, Some(puzzle_fn), None)
    }

    pub fn with_c(name: &str, part_nb: u8, c_code_fn: GetCCodeFn) -> Self {
        Self::new(name, part_nb, None, Some(c_code_fn))
    }

    pub fn parse_puzzle_name(s: &str) -> (u32, u32) {
        let p: Vec<_> = s.strip_prefix("day").unwrap().split('_').collect();
        (p[0].parse().unwrap(), p[1].parse().unwrap())
    }

    pub fn get_input(&self) -> String {
        let input_file = get_input_file(&self.name);
        fs::read_to_string(input_file).expect("Unable to read input file")
    }
}

pub fn print_result(puzzle: &Puzzle, res: &str, expected: &str, is_c: bool) {
    println!(
        "{} {} day {}, part {}{}",
        if res == expected { "✅" } else { "❌" },
        puzzle.year,
        puzzle.day,
        puzzle.part_nb,
        if is_c { " C version" } else { "" },
    );
    if res != expected {
        println!("Incorrect result is {}, expected {}", res, expected);
    }
}
