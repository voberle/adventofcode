use super::Direction::{Down, Left, Right, Up};
use super::Grid;

#[allow(dead_code)]
pub fn simple(map: &Grid, visited: &[bool]) {
    for row in 0..map.rows {
        for (p, visit) in visited
            .iter()
            .enumerate()
            .take((row + 1) * map.cols)
            .skip(row * map.cols)
        {
            match map.values.get(p) {
                Some('#') => print!("#"),
                Some('^') => print!("X"), // initial position is always visited.
                Some('.') => {
                    if *visit {
                        print!("X");
                    } else {
                        print!(".");
                    }
                }
                _ => panic!("Invalid map element"),
            }
        }
        println!();
    }
}

#[allow(dead_code)]
pub fn print(
    map: &Grid,
    extra_obstacle_pos: usize,
    visited: &[[bool; 4]],
    positions: &[usize],
    pretty: bool,
) {
    const RED: &str = "\x1b[31m";
    const RESET: &str = "\x1b[0m";
    for row in 0..map.rows {
        for (p, visit) in visited
            .iter()
            .enumerate()
            .take((row + 1) * map.cols)
            .skip(row * map.cols)
        {
            if p == extra_obstacle_pos {
                print!("O");
                continue;
            }
            match map.values.get(p) {
                Some('#') => print!("#"),
                Some('^') => print!("^"),
                Some('.') => {
                    if visit.iter().any(|v| *v) {
                        if pretty {
                            // Pretty-printing like in the description.
                            if !visit[usize::from(Up)] && !visit[usize::from(Down)] {
                                print!("-");
                            } else if !visit[usize::from(Left)] && !visit[usize::from(Right)] {
                                print!("|");
                            } else {
                                print!("+");
                            }
                        } else {
                            // Prints the path with hex to see which directions are taken.
                            let code = visit
                                .iter()
                                .enumerate()
                                .map(|(i, v)| if *v { 1 << i } else { 0 })
                                .sum::<usize>();
                            if positions.contains(&p) {
                                print!("{RED}{code:X}{RESET}");
                            } else {
                                print!("{code:X}");
                            }
                        }
                    } else {
                        print!(".");
                    }
                }
                _ => panic!("Invalid map element"),
            }
        }
        println!();
    }
}
