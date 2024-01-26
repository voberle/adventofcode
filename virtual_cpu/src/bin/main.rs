mod day2017_23;
mod day2017_18;

use std::fs::{self, File};
use std::io::{BufRead, BufReader};

use fxhash::FxHashMap;

use virtual_cpu::test_utils;

// Loads the list of puzzle answers, saving it into a map "puzzle name" => "answer".
fn load_answer_list() -> FxHashMap<String, String> {
    let reader =
        BufReader::new(File::open(format!("{}/answers", test_utils::RESOURCES_DIR)).unwrap());
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

const PUZZLES: [(&str, PuzzleFn); 2] = [
    ("day2017_23", day2017_23::part1),
    ("day2017_18", day2017_18::part1),
];

fn main() {
    let answers = load_answer_list();

    for (puzzle_name, puzzle_fn) in PUZZLES {
        let expected_result = answers.get(puzzle_name).expect(&format!("Missing answer for {}", puzzle_name));
        let input_file = test_utils::get_input_file(puzzle_name);
        let input = fs::read_to_string(input_file).expect("Unable to read input file");
        let res = puzzle_fn(&input);
        // println!("{}: {}", puzzle_name, res);

        if res == *expected_result {
            println!("✅ {}", puzzle_name);
        } else {
            println!("❌ {}", puzzle_name);
        }
    }
}
