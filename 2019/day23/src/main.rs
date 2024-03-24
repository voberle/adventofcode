use std::io::{self, Read};

use intcode::IntcodeComputer;

fn build_network(computer: &IntcodeComputer) -> Vec<IntcodeComputer> {
    (0..50)
        .map(|a| {
            let mut computer = computer.clone();
            computer.io.add_input(a);
            computer
        })
        .collect()
}

fn exec(computer: &mut IntcodeComputer) -> Option<(usize, (i64, i64))> {
    computer.exec();
    if let Some(dest_addr) = computer.io.get_output() {
        let x = computer.io.get_output().unwrap();
        let y = computer.io.get_output().unwrap();
        Some((usize::try_from(dest_addr).unwrap(), (x, y)))
    } else {
        None
    }
}

fn first_packet_to_255_y_val(computer: &IntcodeComputer) -> i64 {
    let mut network = build_network(computer);

    for a in (0..50).cycle() {
        let c = &mut network[a];
        
        // This simulates the computer having received no packet.
        c.io.add_input(-1);

        if let Some((dest_addr, (x, y))) = exec(c) {
            // println!("{} => {}: {};{}", a, dest_addr, x, y);
            if dest_addr == 255 {
                return y;
            }

            let dest = &mut network[dest_addr];
            dest.io.add_input(x);
            dest.io.add_input(y);
        }
    }
    panic!("Impossible")
}

fn part2(computer: &IntcodeComputer) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let computer = IntcodeComputer::build(&input);

    println!("Part 1: {}", first_packet_to_255_y_val(&computer));
    println!("Part 2: {}", part2(&computer));
}
