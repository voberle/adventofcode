// https://adventofcode.com/2023/day/2

use std::io;

fn main() {
    let stdin = io::stdin();
    let mut sum_ids = 0;
    let mut power_sum = 0;
    for line in stdin.lines() {
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let s = line.unwrap();
        let split: Vec<&str> = s.split(":").collect();
        let game_id: u32 = split[0].strip_prefix("Game ").unwrap().parse().unwrap();
        let reveals: Vec<&str> = split[1].split(";").collect();
        let mut test = true;

        let mut red_count = 0;
        let mut green_count = 0;
        let mut blue_count = 0;
        for r in reveals {
            let col: Vec<&str> = r.split(",").collect();
            for c in col {
                let x: Vec<&str> = c.trim().split(" ").collect();
                let cube_count: u32 = x[0].parse().unwrap();
                let cube_color = x[1];
                match cube_color {
                    "red" => {
                        if cube_count > 12 { test = false; }
                        red_count = u32::max(red_count, cube_count);
                    }
                    "green" => {
                        if cube_count > 13 { test = false; }
                        green_count = u32::max(green_count, cube_count);
                    }
                    "blue" => {
                        if cube_count > 14 { test = false; }
                        blue_count = u32::max(blue_count, cube_count);
                    }
                    _ => {}
                }
            }
        }
        if test {
            sum_ids += game_id;
        }
        power_sum += red_count * green_count * blue_count;
    }
    println!("{}", sum_ids);
    println!("{}", power_sum);
}
