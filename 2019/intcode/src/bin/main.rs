use std::io::{self, Read};

use intcode::IntcodeComputer;

// Executing the Intcode from the file passed as argument.
// Input to the program is read fron stdin as a set of integers separated by spaces.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = &args[1];

    let intcode = std::fs::read_to_string(program).expect("Unable to read program file");
    let mut computer = IntcodeComputer::build(&intcode);

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input_ints: Vec<i64> = input
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    // println!("{:?}", input_ints);

    computer.io.extend_input(&input_ints);
    computer.exec();

    println!("{}", computer.io.dump_output());
}
