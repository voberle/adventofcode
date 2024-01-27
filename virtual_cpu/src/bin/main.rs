mod day2015_23;
mod day2016_12;
mod day2016_23;
mod day2016_25;
mod day2017_18;
mod day2017_23;

use virtual_cpu::c_code::exec::exec_c_code;
use virtual_cpu::run_utils::*;

fn main() {
    #[rustfmt::skip]
    let puzzles = vec![
        Puzzle::both("day2015_23", 1, day2015_23::part1, day2015_23::part1_c_code),
        Puzzle::both("day2015_23", 2, day2015_23::part2, day2015_23::part2_c_code),
        Puzzle::both("day2016_12", 1, day2016_12::part1, day2016_12::part1_c_code),
        Puzzle::both("day2016_12", 2, day2016_12::part2, day2016_12::part2_c_code),
        Puzzle::base("day2016_23", 1, day2016_23::part1), // C not possible
        Puzzle::base("day2016_25", 1, day2016_25::part1),
        Puzzle::base("day2017_18", 1, day2017_18::part1),
        Puzzle::base("day2017_23", 1, day2017_23::part1),
        Puzzle::with_c("day2017_23", 2, day2017_23::part2_c_code), // Only C, other too slow.
    ];

    let answers = load_answer_list();

    for puzzle in puzzles {
        let expected_result = answers
            .get(&format!("{}_{}", puzzle.name, puzzle.part_nb))
            .unwrap_or_else(|| {
                panic!("Missing answer for {} part {}", puzzle.name, puzzle.part_nb)
            });

        let input = puzzle.get_input();

        if let Some(c_code_fn) = puzzle.c_code_fn {
            let code = c_code_fn(&input);
            // println!("{}", code);
            let res = exec_c_code(&code);
            print_result(&puzzle, &res, expected_result, true);
        }

        if let Some(puzzle_fn) = puzzle.puzzle_fn {
            let res = puzzle_fn(&input);
            print_result(&puzzle, &res, expected_result, false);
        };
    }
}
