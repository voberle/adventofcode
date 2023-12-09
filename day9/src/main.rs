// https://adventofcode.com/2023/day/9
// Part 1 test: 114
// Part 1: 1974232246
// Part 2 test: 2
// Part 2: 928

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

    let mut sum_extrapolated_values_begin = 0;
    let mut sum_extrapolated_values_end = 0;
    for history in report {
        let mut first_item: Vec<i32> = Vec::new();
        let mut last_item: Vec<i32> = Vec::new();
        let mut next_line: Vec<i32> = history.clone();
        // println!("{:?}", next_line);
        while next_line.iter().any(|i| *i != 0) {
            first_item.push(*next_line.first().unwrap());
            last_item.push(*next_line.last().unwrap());
            next_line = next_line.windows(2).map(|w| w[1] - w[0]).collect();
            // println!("{:?}", next_line);
        }

        sum_extrapolated_values_end += last_item.iter().sum::<i32>();

        first_item.reverse();
        // println!("first_item {:?}", first_item);
        let first_val = first_item.iter().fold(0, |n, i| i - n);
        // println!("first_val {}", first_val);
        sum_extrapolated_values_begin += first_val;
    }
    println!("Part 1: {}", sum_extrapolated_values_end);
    println!("Part 2: {}", sum_extrapolated_values_begin);
}
