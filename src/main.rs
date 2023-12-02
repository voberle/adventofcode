// https://adventofcode.com/2023/day/2

use std::io;

fn main() {
    let stdin = io::stdin();
    let mut sum_ids = 0;
    for line in stdin.lines() {
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let s = line.unwrap();
        let split: Vec<&str> = s.split(":").collect();
        let game_id: u32 = split[0].strip_prefix("Game ").unwrap().parse().unwrap();
        let reveals: Vec<&str> = split[1].split(";").collect();
        let mut test = true;
        for r in reveals {
            let col: Vec<&str> = r.split(",").collect();
            for c in col {
                let x: Vec<&str> = c.trim().split(" ").collect();
                let cube_count: u32 = x[0].parse().unwrap();
                let cube_color = x[1];
                match cube_color {
                    "red" => if cube_count > 12 { test = false; },
                    "green" => if cube_count > 13 { test = false; },
                    "blue" => if cube_count > 14 { test = false; },
                    _ => {}
                }
            }
        }
        if test {
            sum_ids += game_id;
        }
    }
    println!("{}", sum_ids);
}
