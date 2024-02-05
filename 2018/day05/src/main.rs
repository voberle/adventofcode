use std::io::{self, Read};

fn build(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn is_opposite_polarity(s: &[char]) -> bool {
    const DIFF: u8 = b'a' - b'A';
    (s[0] as u8).abs_diff(s[1] as u8) == DIFF
}

fn scan(polymer: &[char]) -> Vec<char> {
    let mut polymer = polymer.to_vec();
    // chunks doesn't work as we don't get the proper index, which we need to remove the chars
    while let Some(i) = polymer
        .iter()
        .enumerate()
        .position(|(i, _)| i < polymer.len() - 1 && is_opposite_polarity(&polymer[i..=i + 1]))
    {
        polymer.drain(i..=i + 1);
        // println!("{}: {}", i, polymer.iter().map(char::to_string).collect::<Vec<_>>().join(""));
    }
    polymer
}

fn remaining_units_count(polymer: &[char]) -> usize {
    scan(polymer).len()
}

// unit is the lowercase char
fn remove_unit(polymer: &[char], unit: char) -> Vec<char> {
    let upper_unit = unit.to_ascii_uppercase();
    let mut polymer = polymer.to_vec();
    polymer.retain(|&v| v != unit && v != upper_unit);
    polymer
}

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn shortest_polymer(polymer: &[char]) -> usize {
    ASCII_LOWER
        .iter()
        .map(|unit| {
            let polymer = remove_unit(polymer, *unit);
            remaining_units_count(&polymer)
        })
        .min()
        .unwrap()
}

// Much faster/smarter solution using a stack. Doesn't do any copy.
// From https://www.reddit.com/r/adventofcode/comments/a3912m/comment/eb4fkwu/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
fn react<'a>(input: impl Iterator<Item = &'a u8>) -> usize {
    let mut v = Vec::new();
    for &c in input {
        match v.last() {
            None => v.push(c),
            Some(&d) => {
                if d.to_ascii_lowercase() == c.to_ascii_lowercase() && d != c {
                    v.pop();
                } else {
                    v.push(c);
                }
            }
        }
    }
    v.len()
}

#[allow(dead_code)]
fn fast_version(input: &str) {
    let input: Vec<u8> = input.chars().map(|c| c as u8).collect();
    println!("Part 1: {}", react(input.iter()));
    let mut min = std::usize::MAX;
    for i in 0u8..=26 {
        let v = input.iter().filter(|&&c| c != b'a' + i && c != b'A' + i);
        min = usize::min(react(v), min);
    }
    println!("Part 2: {}", min);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    // fast_version(&input);
    let polymer = build(input.trim());

    println!("Part 1: {}", remaining_units_count(&polymer));
    println!("Part 2: {}", shortest_polymer(&polymer));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_opposite_polarity() {
        assert!(is_opposite_polarity(&['b', 'B']));
        assert!(is_opposite_polarity(&['C', 'c']));
        assert!(!is_opposite_polarity(&['b', 'A']));
        assert!(!is_opposite_polarity(&['C', 'C']));
        assert!(!is_opposite_polarity(&['E', 'C']));
        assert!(!is_opposite_polarity(&['r', 'r']));
    }

    #[test]
    fn test_remaining_units() {
        assert_eq!(scan(&build("aA")), vec![]);
        assert_eq!(scan(&build("abBA")), vec![]);
        assert_eq!(scan(&build("abAB")), vec!['a', 'b', 'A', 'B']);
        assert_eq!(scan(&build("aabAAB")), vec!['a', 'a', 'b', 'A', 'A', 'B']);
    }

    #[test]
    fn test_part1() {
        assert_eq!(remaining_units_count(&build("dabAcCaCBAcCcaDA")), 10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(shortest_polymer(&build("dabAcCaCBAcCcaDA")), 4);
    }
}
