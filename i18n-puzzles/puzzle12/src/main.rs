use std::{
    cmp::Ordering,
    fmt::Display,
    io::{self, Read},
};

use deunicode::{deunicode, deunicode_char};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Entry {
    last_name: String,
    first_name: String,
    phone: u64,
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.last_name, self.first_name)
    }
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

type NormalizeStrFn = fn(&str) -> String;
type CmpCharFn = fn(char, char) -> Ordering;
type CmpStrFn = fn(&str, &str) -> Ordering;
type CmpEntryFn = fn(&Entry, &Entry) -> Ordering;

/// Letter-by-letter filtering
fn letter_by_letter(s: &str) -> String {
    s.chars().filter(|c| c.is_alphabetic()).collect()
}

/// Compares two entries, last names first, first names if last names are equals.
/// All names are normalized before comparaison.
fn entry_cmp(
    a: &Entry,
    b: &Entry,
    normalize_last_name_fn: NormalizeStrFn,
    normalize_first_name_fn: NormalizeStrFn,
    cmp_fn: CmpStrFn,
) -> Ordering {
    let last_name_a = normalize_last_name_fn(&a.last_name);
    let last_name_b = normalize_last_name_fn(&b.last_name);
    let res = cmp_fn(&last_name_a, &last_name_b);
    if res == Ordering::Equal {
        let first_name_a = normalize_first_name_fn(&a.first_name);
        let first_name_b = normalize_first_name_fn(&b.first_name);
        cmp_fn(&first_name_a, &first_name_b)
    } else {
        res
    }
}

/// Compares two strings based on a custom char cmp.
fn custom_str_cmp(s1: &str, s2: &str, cmp_fn: CmpCharFn) -> Ordering {
    let mut s1_chars = s1.chars();
    let mut s2_chars = s2.chars();

    loop {
        match (s1_chars.next(), s2_chars.next()) {
            (Some(c1), Some(c2)) => {
                let char_cmp = cmp_fn(c1, c2);
                if char_cmp != std::cmp::Ordering::Equal {
                    return char_cmp;
                }
            }
            (Some(_), None) => return Ordering::Greater, // s1 is longer
            (None, Some(_)) => return Ordering::Less,    // s2 is longer
            (None, None) => return Ordering::Equal,      // Both strings are equal
        }
    }
}

// English
// -------

/// Makes all characters lowercase, removes all accents, drops non-letters.
fn normalize_for_english(s: &str) -> String {
    // Letter-by-letter
    let letters = letter_by_letter(s);

    let lowercase = letters.to_lowercase();

    // From deunicode crate: Replaces all Unicode chars with ASCII equivalents, which has the effect or removing the accents.
    deunicode(&lowercase)
}

fn english_cmp(a: &Entry, b: &Entry) -> Ordering {
    entry_cmp(a, b, normalize_for_english, normalize_for_english, str::cmp)
}

// Swedish
// -------

fn normalize_for_swedish(s: &str) -> String {
    // Letter-by-letter
    let letters = letter_by_letter(s);

    let norm = letters.to_lowercase();

    // The letter Æ is considered the degenerate Danish variant of Ä, so Är < Æs < Ät,
    // and Ø is an odd variant of Ö, so Ök < Øl < Öm
    let norm = norm.replace('æ', "ä");
    let norm = norm.replace('ø', "ö");

    // We can't use deunicode(), as some chars should be left untouched.
    norm.chars()
        .map(|c| {
            if ['å', 'ä', 'ö'].contains(&c) {
                c.to_string()
            } else {
                deunicode_char(c).unwrap().to_string()
            }
        })
        .collect()
}

fn swedish_char_cmp(a: char, b: char) -> Ordering {
    // 29 letters, and they are ordered A through X, then Y, Z, Å, Ä and Ö.
    // We convert the letters to an integer, so custom sorting can be done.
    let letter_val_fn = |c: char| {
        // Assuming normalized, so handling only lowercase.
        assert!(c.is_lowercase());
        match c {
            'å' => 27,
            'ä' => 28,
            'ö' => 29,
            _ => c as u8 - b'a',
        }
    };

    let a_val = letter_val_fn(a);
    let b_val = letter_val_fn(b);
    assert!((0..=29).contains(&a_val));
    assert!((0..=29).contains(&b_val));
    a_val.cmp(&b_val)
}

fn swedish_str_cmp(s1: &str, s2: &str) -> Ordering {
    custom_str_cmp(s1, s2, swedish_char_cmp)
}

fn swedish_cmp(a: &Entry, b: &Entry) -> Ordering {
    entry_cmp(
        a,
        b,
        normalize_for_swedish,
        normalize_for_swedish,
        swedish_str_cmp,
    )
}

// Dutch
// -----

fn normalize_last_name_for_dutch(s: &str) -> String {
    let mut norm = s.to_string();

    // Removes the infixes.
    // We assume that the part of the last name leading up to the first capital letter are infixes.
    let i = norm.find(|c: char| c.is_uppercase()).unwrap();
    if i > 0 {
        norm = norm[i..].to_string();
    }

    normalize_for_english(&norm)
}

fn dutch_cmp(a: &Entry, b: &Entry) -> Ordering {
    // English rules except for the last name infixes.
    entry_cmp(
        a,
        b,
        normalize_last_name_for_dutch,
        normalize_for_english,
        str::cmp,
    )
}

// -----

fn sort_by(phonebook: &[Entry], cmp_fn: CmpEntryFn) -> Vec<Entry> {
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

    fn print_side_by_side(book1: &[Entry], book2: &[Entry]) {
        println!("{:40} | {}", "Expected", "Actual");
        println!();
        for (e1, e2) in book1.iter().zip(book2.iter()) {
            println!("{:40} | {}", e1.to_string(), e2.to_string());
        }
    }

    #[test]
    fn test_sort_english() {
        const SORTED_ENGLISH: &str = include_str!("../resources/sorted_english");
        let phonebook = build(INPUT_TEST);
        let expected = build(SORTED_ENGLISH);
        let sorted = sort_by(&phonebook, english_cmp);
        print_side_by_side(&expected, &sorted);
        assert_eq!(sorted, expected);
    }

    #[test]
    fn test_sort_swedish() {
        const SORTED_SWEDISH: &str = include_str!("../resources/sorted_swedish");
        let phonebook = build(INPUT_TEST);
        let expected = build(SORTED_SWEDISH);
        let sorted = sort_by(&phonebook, swedish_cmp);
        print_side_by_side(&expected, &sorted);
        assert_eq!(sorted, expected);
    }

    #[test]
    fn test_sort_dutch() {
        // I modified the test input from the description to put the infixes at their original place with the last name.
        const SORTED_DUTCH: &str = include_str!("../resources/sorted_dutch");
        let phonebook = build(INPUT_TEST);
        let expected = build(SORTED_DUTCH);
        let sorted = sort_by(&phonebook, dutch_cmp);
        print_side_by_side(&expected, &sorted);
        assert_eq!(sorted, expected);
    }

    #[test]
    fn test_answer() {
        let phonebook = build(INPUT_TEST);
        assert_eq!(middle_number_product(&phonebook), 1885816494308838);
    }
}
