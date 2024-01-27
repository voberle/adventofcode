mod day2015_23;
mod day2016_12;
mod day2016_23;
mod day2016_25;
mod day2017_18;
mod day2017_23;

use std::fs::{self, File};
use std::io::{BufRead, BufReader};

use fxhash::FxHashMap;

use virtual_cpu::c_code::exec::exec_c_code;
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

fn parse_puzzle_name(s: &str) -> (u32, u32) {
    let p: Vec<_> = s.strip_prefix("day").unwrap().split('_').collect();
    (p[0].parse().unwrap(), p[1].parse().unwrap())
}

type PuzzleFn = fn(&str) -> String;
type GetCCodeFn = fn(&str) -> String;

#[rustfmt::skip]
const PUZZLES: [(&str, u8, PuzzleFn, Option<GetCCodeFn>); 8] = [
    ("day2015_23", 1, day2015_23::part1, Some(day2015_23::part1_c_code)),
    ("day2015_23", 2, day2015_23::part2, Some(day2015_23::part2_c_code)),
    ("day2016_12", 1, day2016_12::part1, None),
    ("day2016_12", 2, day2016_12::part2, None),
    ("day2016_23", 1, day2016_23::part1, None),
    ("day2016_25", 1, day2016_25::part1, None),
    ("day2017_18", 1, day2017_18::part1, None),
    ("day2017_23", 1, day2017_23::part1, None),
];

fn main() {
    const USE_C_CONVERSION: bool = false;

    let answers = load_answer_list();

    for (puzzle_name, part_nb, puzzle_fn, c_code_fn_option) in PUZZLES {
        let expected_result = answers
            .get(&format!("{}_{}", puzzle_name, part_nb))
            .unwrap_or_else(|| panic!("Missing answer for {} part {}", puzzle_name, part_nb));

        let input_file = test_utils::get_input_file(puzzle_name);
        let input = fs::read_to_string(input_file).expect("Unable to read input file");

        let res = if USE_C_CONVERSION {
            if let Some(c_code_fn) = c_code_fn_option {
                let code = c_code_fn(&input);
                // println!("{}", code);
                exec_c_code(&code)
            } else {
                continue;
            }
        } else {
            puzzle_fn(&input)
        };

        // println!("{}: {}", puzzle_name, res);

        let (year, day) = parse_puzzle_name(puzzle_name);
        println!(
            "{} {} day {}, part {}",
            if res == *expected_result {
                "✅"
            } else {
                "❌"
            },
            year,
            day,
            part_nb
        );
        if res != *expected_result {
            println!("Incorrect result is {}, expected {}", res, expected_result);
        }
    }
}
