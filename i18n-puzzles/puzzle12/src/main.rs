use std::{
    cmp::Ordering,
    io::{self, Read},
};

use deunicode::deunicode;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Entry {
    last_name: String,
    first_name: String,
    phone: u64,
}

fn build(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|line| {
            let (name, phone) = line.split(": ").collect_tuple().unwrap();
            let (last_name, first_name) = name
                .split(", ")
                .map(ToString::to_string)
                .collect_tuple()
                .unwrap();
            Entry {
                last_name,
                first_name,
                phone: phone.parse().unwrap(),
            }
        })
        .collect()
}

type SortFn = fn(&Entry, &Entry) -> Ordering;

// Makes all characters lowercase, removes all accents, drops non-letters.
fn normalize_for_english(s: &str) -> String {
    let lowercase = s.to_lowercase();
    // From deunicode crate: Replaces all Unicode chars with ASCII equivalents, which has the effect or removing the accents.
    let ascii = deunicode(&lowercase);
    ascii.chars().filter(|c| c.is_alphabetic()).collect()
}

fn english_cmp(a: &Entry, b: &Entry) -> Ordering {
    let last_name_a = normalize_for_english(&a.last_name);
    let last_name_b = normalize_for_english(&b.last_name);
    let res = last_name_a.cmp(&last_name_b);
    if res == Ordering::Equal {
        let first_name_a = normalize_for_english(&a.first_name);
        let first_name_b = normalize_for_english(&b.first_name);
        first_name_a.cmp(&first_name_b)
    } else {
        res
    }
}

fn swedish_cmp(a: &Entry, b: &Entry) -> Ordering {
    Ordering::Equal
}

fn dutch_cmp(a: &Entry, b: &Entry) -> Ordering {
    Ordering::Equal
}

fn sort_by(phonebook: &[Entry], cmp_fn: SortFn) -> Vec<Entry> {
    let mut sorted = phonebook.to_vec();
    sorted.sort_by(cmp_fn);
    sorted
}

fn middle_number_product(phonebook: &[Entry]) -> u64 {
    let sorted_english = sort_by(phonebook, english_cmp);
    let sorted_swedish = sort_by(phonebook, swedish_cmp);
    let sorted_dutch = sort_by(phonebook, dutch_cmp);

    let i = phonebook.len() / 2;
    sorted_english[i].phone * sorted_swedish[i].phone * sorted_dutch[i].phone
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let phonebook = build(&input);

    println!("Answer: {}", middle_number_product(&phonebook));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_sort_english() {
        const SORTED_ENGLISH: &str = include_str!("../resources/sorted_english");
        let phonebook = build(INPUT_TEST);
        let sorted = build(SORTED_ENGLISH);
        assert_eq!(sort_by(&phonebook, english_cmp), sorted);
    }

    #[test]
    fn test_sort_swedish() {
        const SORTED_SWEDISH: &str = include_str!("../resources/sorted_swedish");
        let phonebook = build(INPUT_TEST);
        let sorted = build(SORTED_SWEDISH);
        assert_eq!(sort_by(&phonebook, swedish_cmp), sorted);
    }

    #[test]
    fn test_sort_dutch() {
        const SORTED_DUTCH: &str = include_str!("../resources/sorted_dutch");
        let phonebook = build(INPUT_TEST);
        let sorted = build(SORTED_DUTCH);
        assert_eq!(sort_by(&phonebook, english_cmp), sorted);
    }

    #[test]
    fn test_answer() {
        let phonebook = build(INPUT_TEST);
        assert_eq!(middle_number_product(&phonebook), 1885816494308838);
    }
}
