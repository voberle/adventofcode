use std::io::{self, Read};

fn build(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|i| i.parse().unwrap()).collect())
        .collect()
}

fn part1_2(report: Vec<Vec<i32>>) -> (i32, i32) {
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
    (sum_extrapolated_values_end, sum_extrapolated_values_begin)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let report = build(&input);

    let (sum_extrapolated_values_end, sum_extrapolated_values_begin) = part1_2(report);

    println!("Part 1: {}", sum_extrapolated_values_end);
    println!("Part 2: {}", sum_extrapolated_values_begin);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test");

    #[test]
    fn test_part1_2() {
        let report = build(INPUT_TEST);
        let (sum_extrapolated_values_end, sum_extrapolated_values_begin) = part1_2(report);

        assert_eq!(sum_extrapolated_values_end, 114);
        assert_eq!(sum_extrapolated_values_begin, 2);
    }
}
