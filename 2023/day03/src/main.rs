// https://adventofcode.com/2023/day/3
// Part 1: Test 4361
// Part 2: Test 467835

use std::{
    collections::{HashMap, HashSet},
    io, usize,
};

trait Is {
    fn is(c: char) -> bool;
}

struct IsSymbol;

impl Is for IsSymbol {
    fn is(c: char) -> bool {
        !c.is_digit(10) && c != '.'
    }
}

struct IsGear;

impl Is for IsGear {
    fn is(c: char) -> bool {
        c == '*'
    }
}

fn is_adj_line<I: Is>(s: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    I::is(s[i][j])
        || I::is(s[i][j.saturating_sub(1)])
        || I::is(s[i][usize::min(j + 1, s[i].len() - 1)])
}

fn is_adjacent<I: Is>(s: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    is_adj_line::<I>(s, i, j)
        || is_adj_line::<I>(s, i.saturating_sub(1), j)
        || is_adj_line::<I>(s, usize::min(i + 1, s.len() - 1), j)
}

fn add_if_gear(s: &Vec<Vec<char>>, i: usize, j: usize, adj_gears: &mut Vec<(usize, usize)>) {
    if s[i][j] == '*' {
        adj_gears.push((i, j));
    }
}

// Finds all the '*' adjacent to this position
fn find_adjacent_gears(s: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut adj_gears = Vec::new();
    add_if_gear(s, i, j.saturating_sub(1), &mut adj_gears);
    add_if_gear(s, i, usize::min(j + 1, s[i].len() - 1), &mut adj_gears);
    for k in [i.saturating_sub(1), usize::min(i + 1, s.len() - 1)] {
        add_if_gear(s, k, j.saturating_sub(1), &mut adj_gears);
        add_if_gear(s, k, j, &mut adj_gears);
        add_if_gear(s, k, usize::min(j + 1, s[i].len() - 1), &mut adj_gears);
    }
    adj_gears
}

fn main() {
    let stdin = io::stdin();
    let mut schematic: Vec<Vec<char>> = Vec::new();
    for line in stdin.lines().map(|l| l.unwrap()) {
        schematic.push(line.chars().collect());
    }

    let mut total = 0;
    // Maps the position of the '*' to the numbers around it.
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    schematic.iter().enumerate().for_each(|(i, line)| {
        let mut n = 0;
        let mut include = false;
        let mut adj_gears: HashSet<(usize, usize)> = HashSet::new();
        line.iter().enumerate().for_each(|(j, c)| {
            if let Some(d) = c.to_digit(10) {
                n = n * 10 + d;
                if is_adjacent::<IsSymbol>(&schematic, i, j) {
                    include = true;
                }
                adj_gears.extend(find_adjacent_gears(&schematic, i, j));
            } else {
                if include {
                    total += n;
                    for k in &adj_gears {
                        let list = gears.entry((k.0, k.1)).or_insert_with(Vec::new);
                        list.push(n);
                        // println!("{} added for gear {}:{}", n, k.0, k.1);
                    }
                }
                n = 0;
                include = false;
                adj_gears.clear();
            }
        });
        // This is to handle the case of a number being last on the line
        if include {
            total += n;
            for k in &adj_gears {
                let list = gears.entry((k.0, k.1)).or_insert_with(Vec::new);
                list.push(n);
                // println!("{} added for gear {}:{}", n, k.0, k.1);
            }
        }
    });

    let gear_ratio_sum: u32 = gears
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum();

    println!("Part 1: {}", total);
    println!("Part 2: {}", gear_ratio_sum);
}
