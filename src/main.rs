// https://adventofcode.com/2023/day/3
// Part 1: Test 4361
// Part 1: 529618

use std::{io, usize};

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

fn is_symbol_adj_line(s: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    is_symbol(s[i][j])
        || is_symbol(s[i][j.saturating_sub(1)])
        || is_symbol(s[i][usize::min(j + 1, s[i].len() - 1)])
}

fn is_symbol_adjacent(s: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    is_symbol_adj_line(s, i, j)
        || is_symbol_adj_line(s, i.saturating_sub(1), j)
        || is_symbol_adj_line(s, usize::min(i + 1, s.len() - 1), j)
}

fn main() {
    let stdin = io::stdin();
    let mut schematic: Vec<Vec<char>> = Vec::new();
    for line in stdin.lines().map(|l| l.unwrap()) {
        schematic.push(line.chars().collect());
    }

    let mut total = 0;
    schematic.iter().enumerate().for_each(|(i, line)| {
        let mut n = 0;
        let mut include = false;
        line.iter().enumerate().for_each(|(j, c)| {
            if let Some(d) = c.to_digit(10) {
                n = n * 10 + d;
                //println!("{i}:{j}  {d} => {n}");
                if is_symbol_adjacent(&schematic, i, j) {
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
