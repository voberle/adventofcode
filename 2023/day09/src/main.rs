use std::io::{self, Read};

fn build(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|i| i.parse().unwrap()).collect())
        .collect()
}

fn sum_extrapolated_values(report: Vec<Vec<i32>>) -> (i32, i32) {
    let mut sum_begin = 0;
    let mut sum_end = 0;

    // Going through each line in the report.
    for history in report {
        let mut first_item: Vec<i32> = Vec::new();
        let mut next_line: Vec<i32> = history.clone();

        // Generate new lines until it's full of zeroes.
        while next_line.iter().any(|i| *i != 0) {
            first_item.push(*next_line.first().unwrap());
            sum_end += next_line.last().unwrap();
            next_line = next_line.windows(2).map(|w| w[1] - w[0]).collect();
        }

        first_item.reverse();
        let first_val = first_item.iter().fold(0, |n, i| i - n);
        sum_begin += first_val;
    }
    (sum_end, sum_begin)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let report = build(&input);

    let (sum_end, sum_begin) = sum_extrapolated_values(report);

    println!("Part 1: {}", sum_end);
    println!("Part 2: {}", sum_begin);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test");

    #[test]
    fn test_sum_extrapolated_values() {
        let report = build(INPUT_TEST);
        let (sum_extrapolated_values_end, sum_extrapolated_values_begin) =
            sum_extrapolated_values(report);

        assert_eq!(sum_extrapolated_values_end, 114);
        assert_eq!(sum_extrapolated_values_begin, 2);
    }
}
