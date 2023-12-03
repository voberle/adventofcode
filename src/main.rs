// https://adventofcode.com/2023/day/3
// Part 1: Test 4361
// Part 1: 529618

use std::{io, usize};

fn set_hotpoints(hotpoints: &mut Vec<Vec<bool>>, i: usize, j: usize) {
    let last_idx = hotpoints[i].len() - 1;
    hotpoints[i][j] = true;
    hotpoints[i][usize::max(0, j - 1)] = true;
    hotpoints[i][usize::min(j + 1, last_idx)] = true;
}

fn print_hotpoints(hotpoints: &Vec<Vec<bool>>) {
    hotpoints.iter().for_each(|line| {
        println!(
            "{}",
            line.iter()
                .map(|i| if *i { "*" } else { "." })
                .collect::<Vec<&str>>()
                .join("")
        );
    });
}

fn main() {
    let stdin = io::stdin();
    let mut schematic = Vec::new();
    for line in stdin.lines().map(|l| l.unwrap()) {
        schematic.push(line);
    }

    // A "hot point" is a point around a symbol.
    // All numbers that overlap such a point must be counted.
    let mut hotpoints: Vec<Vec<bool>> =
        vec![vec![false; schematic.first().unwrap().len()]; schematic.len()];
    schematic.iter().enumerate().for_each(|(i, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, d)| !d.is_digit(10) && !d.eq(&'.'))
            .for_each(|(j, _)| {
                set_hotpoints(&mut hotpoints, i, j);
                set_hotpoints(&mut hotpoints, usize::max(0, i - 1), j);
                set_hotpoints(&mut hotpoints, usize::min(i + 1, schematic.len() - 1), j);
            });
    });
    //print_hotpoints(&hotpoints);

    let mut total = 0;
    schematic.iter().enumerate().for_each(|(i, line)| {
        let mut n = 0;
        let mut include = false;
        line.chars().enumerate().for_each(|(j, c)| {
            if let Some(d) = c.to_digit(10) {
                n = n * 10 + d;
                //println!("{i}:{j}  {d} => {n}");
                if hotpoints[i][j] {
                    include = true;
                }
            } else {
                if include {
                    total += n;
                    // println!("{i}:{j}  Include {n}: Total={total}");
                } else if n > 0 {
                    // println!("{i}:{j}  Don't include {n}");
                }
                n = 0;
                include = false;
            }
        });
        // This is to handle the case of a number being last on the line
        if include {
            total += n;
            // println!("{i}  Include {n}: Total={total}");
        } else if n > 0 {
            // println!("{i}  Don't include {n}");
        }
    });

    println!("{}", total);
}
