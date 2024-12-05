use std::io::{self, Read};

use itertools::Itertools;

// First return value are the ordering rules: It's a vector of size 100,
// with a each index a list of all the pages that need to follow the pages corresponding to that page.
// Second return value is the list of pages.
fn build(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut rules: Vec<Vec<usize>> = vec![Vec::new(); 100];
    let mut page_lists: Vec<Vec<usize>> = Vec::new();

    let mut it = input.lines();
    for line in it.by_ref() {
        if line.is_empty() {
            break;
        }
        let (before, after) = line
            .split('|')
            .map(|v| v.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        rules[before].push(after);
    }
    for line in it {
        page_lists.push(
            line.split(',')
                .map(|v| v.parse::<usize>().unwrap())
                .collect(),
        );
    }
    (rules, page_lists)
}

fn is_page_list_in_order(rules: &[Vec<usize>], pages: &[usize]) -> bool {
    // We maintain a table with the numbers we have seen so far.
    // For each new number we check, we look if there are any rules for it, and if any is not respected.
    let mut seen = [false; 100];
    for page in pages {
        if rules[*page].iter().any(|after| seen[*after]) {
            return false;
        }
        seen[*page] = true;
    }
    true
}

fn middle_numbers_sum(rules: &[Vec<usize>], page_lists: &[Vec<usize>]) -> usize {
    page_lists
        .iter()
        .filter(|pages| is_page_list_in_order(rules, pages))
        .map(|pages| pages[pages.len() / 2])
        .sum()
}

fn part2(rules: &[Vec<usize>], page_lists: &[Vec<usize>]) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (rules, page_lists) = build(&input);
    // println!("{:?}", rules);
    // println!("{:?}", page_lists);

    println!("Part 1: {}", middle_numbers_sum(&rules, &page_lists));
    println!("Part 2: {}", part2(&rules, &page_lists));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (rules, page_lists) = build(INPUT_TEST);
        assert_eq!(middle_numbers_sum(&rules, &page_lists), 143);
    }

    #[test]
    fn test_part2() {
        let (rules, page_lists) = build(INPUT_TEST);
        assert_eq!(part2(&rules, &page_lists), 0);
    }
}
