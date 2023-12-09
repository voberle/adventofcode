// https://adventofcode.com/2023/day/9
// Part 1 test: 114
// Part 1: 1974232246

use std::io;

fn main() {
    let stdin = io::stdin();
    let report: Vec<Vec<i32>> = stdin
        .lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|i| i.parse().unwrap())
                .collect()
        })
        .collect();

    let mut sum_extrapolated_values = 0;
    for history in report {
        let mut last_item: Vec<i32> = Vec::new();
        let mut next_line: Vec<i32> = history.clone();
        println!("{:?}", next_line);
        while next_line.iter().any(|i| *i != 0) {
            last_item.push(*next_line.last().unwrap());
            next_line = next_line.windows(2).map(|w| w[1] - w[0]).collect();
            println!("{:?}", next_line);
        }
        println!("last_item {:?}", last_item);

        let next_val: i32 = last_item.iter().sum();
        println!("Next val {}", next_val);
        sum_extrapolated_values += next_val;
    }
    println!("Part 1: {}", sum_extrapolated_values);
}
