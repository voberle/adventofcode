use std::io::{self, Read};

use itertools::Itertools;

fn build(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn sum_2020_product_2_entries(expense_report: &[u32]) -> u32 {
    for a in expense_report {
        for b in expense_report {
            let s = a + b;
            if s == 2020 {
                return a * b;
            }
            if s > 2020 {
                break;
            }
        }
    }
    panic!("No answer");
}

fn sum_2020_product_3_entries(expense_report: &[u32]) -> u32 {
    for a in expense_report {
        for b in expense_report {
            for c in expense_report {
                let s = a + b + c;
                if s == 2020 {
                    return a * b * c;
                }
                if s > 2020 {
                    break;
                }
            }
        }
    }
    panic!("No answer");
}

#[allow(dead_code)]
fn sums(expense_report: &[u32], combi_len: usize) -> u32 {
    expense_report
        .iter()
        .copied()
        .combinations(combi_len)
        .find(|c| c.iter().sum::<u32>() == 2020)
        .map(|r| r.iter().product())
        .unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut expense_report = build(&input);

    // Using a sorted array allows to interupt the loop and speed things up.
    expense_report.sort_unstable();

    println!("Part 1: {}", sum_2020_product_2_entries(&expense_report));
    println!("Part 2: {}", sum_2020_product_3_entries(&expense_report));

    // Itertools with combinations version. Much slower than naive version.
    // println!("Part 1: {}", sums(&expense_report, 2));
    // println!("Part 2: {}", sums(&expense_report, 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(sums(&build(INPUT_TEST), 2), 514579);
    }

    #[test]
    fn test_part2() {
        assert_eq!(sums(&build(INPUT_TEST), 3), 241861950);
    }
}
