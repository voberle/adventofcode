use std::io::{self, Read};

use itertools::Itertools;

const PAGES_COUNT: usize = 100;

// First return value are the ordering rules: It's a vector of size 100,
// with a each index a list of all the pages that need to follow the pages corresponding to that page.
// Second return value is the list of pages.
fn build(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut it = input.lines();

    let mut rules: Vec<Vec<usize>> = vec![Vec::new(); PAGES_COUNT];
    for line in it.by_ref().take_while(|line| !line.is_empty()) {
        let (before, after) = line
            .split('|')
            .map(|v| v.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        rules[before].push(after);
    }

    let page_lists = it
        .map(|line| {
            line.split(',')
                .map(|v| v.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    (rules, page_lists)
}

fn is_page_list_in_order(rules: &[Vec<usize>], pages: &[usize]) -> bool {
    // We maintain a table with the numbers we have seen so far.
    // For each new number we check, we look if there are any rules for it, and if any is not respected.
    let mut seen = [false; PAGES_COUNT];
    for page in pages {
        if rules[*page].iter().any(|after| seen[*after]) {
            return false;
        }
        seen[*page] = true;
    }
    true
}

// Split the list of pages into the ordered and the incorrectly ordered ones.
fn partition_pages<'a>(
    rules: &[Vec<usize>],
    page_lists: &'a [Vec<usize>],
) -> (Vec<&'a Vec<usize>>, Vec<&'a Vec<usize>>) {
    page_lists
        .iter()
        .partition(|pages| is_page_list_in_order(rules, pages))
}

fn get_middle_number(pages: &[usize]) -> usize {
    pages[pages.len() / 2]
}

fn middle_numbers_sum(_rules: &[Vec<usize>], ordered_pages_list: &[&Vec<usize>]) -> usize {
    ordered_pages_list
        .iter()
        .map(|pages| get_middle_number(pages))
        .sum()
}

fn reorder_pages(rules: &[Vec<usize>], pages: &[usize]) -> Vec<usize> {
    // We take each number and we place it in the list just before the first
    // number it needs to precede. If none, we place it at the end.
    let mut sorted_pages = Vec::new();
    for page in pages {
        let rule = &rules[*page];
        // Find first page in `sorted_pages` that is in the rules.
        if let Some(pos) = sorted_pages.iter().position(|c| rule.contains(c)) {
            sorted_pages.insert(pos, *page);
        } else {
            sorted_pages.push(*page);
        }
    }
    sorted_pages
}

fn middle_after_reordering_sum(
    rules: &[Vec<usize>],
    unordered_pages_list: &[&Vec<usize>],
) -> usize {
    unordered_pages_list
        .iter()
        .map(|pages| reorder_pages(rules, pages))
        .map(|pages| get_middle_number(&pages))
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (rules, page_lists) = build(&input);

    let (ordered_pages_list, unordered_pages_list) = partition_pages(&rules, &page_lists);

    println!(
        "Part 1: {}",
        middle_numbers_sum(&rules, &ordered_pages_list)
    );
    println!(
        "Part 2: {}",
        middle_after_reordering_sum(&rules, &unordered_pages_list)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (rules, page_lists) = build(INPUT_TEST);
        let (ordered_pages_list, _) = partition_pages(&rules, &page_lists);
        assert_eq!(middle_numbers_sum(&rules, &ordered_pages_list), 143);
    }

    #[test]
    fn test_part2() {
        let (rules, page_lists) = build(INPUT_TEST);
        let (_, unordered_pages_list) = partition_pages(&rules, &page_lists);
        assert_eq!(
            middle_after_reordering_sum(&rules, &unordered_pages_list),
            123
        );
    }
}
