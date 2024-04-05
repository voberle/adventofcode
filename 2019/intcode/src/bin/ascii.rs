use std::io::{self, IsTerminal, Read};

use intcode::ASCIIIntcodeComputer;

// Executable supporting the Aft Scaffolding Control and Information Interface (ASCII) from Day 17.
// Everything output by Intcode should be displayed as ASCII characters, with character 10 meaning 'new line'.
//
// The program to execute is passed as argument.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = &args[1];

    let mut input = String::new();
    // Read stdin if there is any.
    if !io::stdin().is_terminal() {
        io::stdin().read_to_string(&mut input).unwrap();
    }

    let intcode = std::fs::read_to_string(program).expect("Unable to read program file");

    // Trim to avoid trouble with extra new line at the end.
    let mut computer = ASCIIIntcodeComputer::build(intcode.trim(), &input);

    computer.exec();
}
