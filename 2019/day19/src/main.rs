use std::io::{self, Read};

use intcode::IntcodeComputer;

fn is_drone_pulled(computer: &IntcodeComputer, x: usize, y: usize) -> bool {
    // Computer has to be restarted for each attempt.
    let mut computer = computer.clone();

    computer.io.add_input(x.try_into().unwrap());
    computer.io.add_input(y.try_into().unwrap());

    computer.exec();

    computer.io.get_output().unwrap() == 1
}

fn beam_size(computer: &IntcodeComputer) -> usize {
    let mut count = 0;
    for y in 0..50 {
        for x in 0..50 {
            if is_drone_pulled(computer, x, y) {
                count += 1;
            }
        }
    }
    count
}

fn part2(computer: &IntcodeComputer) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let computer = IntcodeComputer::build(&input);

    println!("Part 1: {}", beam_size(&computer));
    println!("Part 2: {}", part2(&computer));
}
