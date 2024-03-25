use std::io::{self, BufRead, Read};

use intcode::IntcodeComputer;

fn read_line() -> String {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin
        .lock()
        .read_line(&mut line)
        .expect("Could not read line");
    line.trim().to_string()
}

fn write_string(computer: &mut IntcodeComputer, s: &str) {
    const NEWLINE: i64 = 10;
    s.chars().map(|c| c as i64).for_each(|i| {
        computer.io.add_input(i);
    });
    computer.io.add_input(NEWLINE);
}

fn get_output(computer: &mut IntcodeComputer) -> String {
    let mut output: String = String::new();
    while let Some(i) = computer.io.get_output() {
        output.push(char::from_u32(u32::try_from(i).unwrap()).unwrap());
    }
    output
}

fn play(computer: &IntcodeComputer) {
    let mut computer = computer.clone();

    loop {
        computer.exec();
        let output = get_output(&mut computer);
        println!("{}", output);

        let input = read_line();
        write_string(&mut computer, &input);
    }
}

fn password_for_airlock(computer: &IntcodeComputer) -> i64 {
    0
}

fn main() {
    let param = std::env::args().nth(1).unwrap_or_default();
    if param == "interactive" {
        // Not reading from stdin in this case, as it messes up with
        let input = std::fs::read_to_string("resources/input").expect("Unable to read input file");
        let computer = IntcodeComputer::build(&input);

        play(&computer);

        return;
    }

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let computer = IntcodeComputer::build(&input);

    println!("Part 1: {}", password_for_airlock(&computer));
}
