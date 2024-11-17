use std::io::{self, BufRead, Read};

use intcode::IntcodeComputer;
use itertools::Itertools;

// Read a line from the terminal.
fn read_line() -> String {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin
        .lock()
        .read_line(&mut line)
        .expect("Could not read line");
    line.trim().to_string()
}

// Write a string to the computer.
fn write_string(computer: &mut IntcodeComputer, s: &str) {
    const NEWLINE: i64 = 10;
    s.chars().map(|c| c as i64).for_each(|i| {
        computer.io.add_input(i);
    });
    computer.io.add_input(NEWLINE);
}

// Get the output from the computer.
fn get_output(computer: &mut IntcodeComputer) -> String {
    let mut output: String = String::new();
    while let Some(i) = computer.io.get_output() {
        output.push(char::from_u32(u32::try_from(i).unwrap()).unwrap());
    }
    output
}

// Parses the saved commands from the string.
// There is one command per line.
// Lines starting with # are ignored.
fn build_saved_commands(saved_cmds: &str) -> Vec<String> {
    let mut replay_cmds: Vec<String> = saved_cmds
        .lines()
        .filter_map(|s| {
            if s.starts_with('#') {
                None
            } else {
                Some(s.to_string())
            }
        })
        .collect();
    replay_cmds.reverse();
    replay_cmds
}

// Interactively play the game.
fn play(computer: &IntcodeComputer, saved_cmds: &str) {
    let mut computer = computer.clone();

    let mut replay_cmds = build_saved_commands(saved_cmds);

    loop {
        computer.exec();
        let s = get_output(&mut computer);
        println!("{s}");

        if computer.is_halted() {
            println!("Game over");
            break;
        }

        print!("> ");
        let input = if let Some(cmd) = replay_cmds.pop() {
            println!("{cmd}");
            cmd
        } else {
            read_line()
        };

        write_string(&mut computer, &input);
    }
}

fn generate_all_combinations() -> Vec<Vec<String>> {
    // All items we collected.
    let items = [
        "jam",
        "coin",
        "fuel cell",
        "planetoid",
        "sand",
        "spool of cat6",
        "dark matter",
        "wreath",
    ];
    let mut all_combinations: Vec<Vec<String>> = Vec::new();
    for n in 1..=items.len() {
        let n_combi = items
            .iter()
            .combinations(n)
            .map(|v| v.iter().map(ToString::to_string).collect::<Vec<String>>());
        all_combinations.extend(n_combi);
    }
    all_combinations
}

fn try_all_combinations(computer: &mut IntcodeComputer) -> String {
    let all_combinations = generate_all_combinations();

    let mut last_combi: Vec<String> = Vec::new();
    for next_combi in all_combinations {
        computer.exec();
        let s = get_output(computer);
        // println!("{}", s);

        if computer.is_halted() {
            // We won, return last output.
            return s;
        }

        // Drop all we carry.
        for i in last_combi {
            write_string(computer, &format!("drop {i}"));
            computer.exec();
            let _ = get_output(computer);
        }

        last_combi = next_combi;
        // Take all from next combination.
        for i in &last_combi {
            write_string(computer, &format!("take {i}"));
            computer.exec();
            let _ = get_output(computer);
        }

        // Try moving.
        write_string(computer, "south");
        // If we succeed, the game is over and the computer is in halted state.
    }
    panic!("No successful combination");
}

// Extract the password from the last output.
fn extract_password(s: &str) -> String {
    let line_with_code = s.lines().find(|line| line.contains("typing")).unwrap();
    line_with_code
        .trim_matches(|c: char| !c.is_ascii_digit())
        .to_string()
}

fn password_for_airlock(computer: &IntcodeComputer) -> String {
    let mut computer = computer.clone();

    let saved_cmds = std::fs::read_to_string("resources/commands").unwrap();
    let replay_cmds = build_saved_commands(&saved_cmds);

    // Collect all items and move to the security checkpoint.
    for input in replay_cmds.iter().rev() {
        computer.exec();
        let _ = get_output(&mut computer);
        write_string(&mut computer, input);
    }

    // Try all combinations.
    let last_output = try_all_combinations(&mut computer);
    extract_password(&last_output)
}

fn main() {
    let param = std::env::args().nth(1).unwrap_or_default();
    if !param.is_empty() {
        // Not reading from stdin in this case, as it messes up with reading commands.
        let input = std::fs::read_to_string("resources/input").expect("Unable to read input file");
        let computer = IntcodeComputer::build(&input);

        if let Ok(saved_cmds) = std::fs::read_to_string(format!("resources/{param}")) {
            play(&computer, &saved_cmds);
        } else {
            play(&computer, "");
        }

        return;
    }

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let computer = IntcodeComputer::build(&input);

    println!("Part 1: {}", password_for_airlock(&computer));
}
