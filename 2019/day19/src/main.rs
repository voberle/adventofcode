use std::{
    io::{self, Read},
    usize,
};

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

fn is_pos_in_square(
    x: usize,
    y: usize,
    ship_position: Option<(usize, usize)>,
    ship_size: usize,
) -> bool {
    if let Some((ship_x, ship_y)) = ship_position {
        x >= ship_x && x < ship_x + ship_size && y >= ship_y && y < ship_y + ship_size
    } else {
        false
    }
}

fn print_beam_with_ship(
    computer: &IntcodeComputer,
    square_size_to_print: usize,
    ship_position: Option<(usize, usize)>,
    ship_size: usize,
) {
    for y in 0..square_size_to_print {
        for x in 0..square_size_to_print {
            let c = if is_pos_in_square(x, y, ship_position, ship_size) {
                'O'
            } else if is_drone_pulled(computer, x, y) {
                '#'
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_beam(computer: &IntcodeComputer, square_size_to_print: usize) {
    print_beam_with_ship(computer, square_size_to_print, None, 0);
}

fn is_ship_in_beam(computer: &IntcodeComputer, x: usize, y: usize) -> bool {
    // Square is 100 wide, so most right/bottom corner is at index + 99.
    is_drone_pulled(computer, x, y + 99) && is_drone_pulled(computer, x + 99, y)
}

fn santa_ship_position(computer: &IntcodeComputer) -> usize {
    // We test if the 100x100 square is in the beam by checking if both the top-right and bottom-left corners are.
    // Find a spot where the 100x100 square is in,
    // then move the top-right up until the border, and the bottom-left left until the border.

    // Find a spot where it fits.
    let mut x = 5000;
    let mut y = 1000;
    // Note that this might not work well if the beam is very vertical.
    while !is_ship_in_beam(computer, x, y) {
        y += 50;
    }

    // Now move as close as possible to the zero.
    loop {
        let mut did_move = false;
        // Try moving up
        if is_ship_in_beam(computer, x, y - 1) {
            y -= 1;
            did_move = true;
        }
        // Moving left
        if is_ship_in_beam(computer, x - 1, y) {
            x -= 1;
            did_move = true;
        }
        // Also diagonally, which might work even if previous ones didn't.
        if is_ship_in_beam(computer, x - 1, y - 1) {
            x -= 1;
            y -= 1;
            did_move = true;
        }

        if !did_move {
            break;
        }
    }
    // println!("Result {};{}", x, y);

    x * 10000 + y
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let computer = IntcodeComputer::build(&input);

    // print_beam(&computer, 50);
    // print_beam_with_ship(&computer, 1100, Some((x, y)), 100);

    println!("Part 1: {}", beam_size(&computer));
    println!("Part 2: {}", santa_ship_position(&computer));
}
