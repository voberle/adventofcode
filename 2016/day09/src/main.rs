use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

#[cfg(test)]
fn decompress(input: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\((\d+)x(\d+)\)").unwrap();
    }
    let mut result = String::new();
    let mut i = 0;
    while i < input.len() {
        if input[i..].starts_with('(') {
            let parts = RE.captures(&input[i..]).unwrap();
            let start = i + parts[0].len();
            let letter_count: usize = parts[1].parse().unwrap();
            let times: usize = parts[2].parse().unwrap();
            for _ in 0..times {
                result += &input[start..start + letter_count];
            }
            i = start + letter_count;
        } else {
            result += &input[i..=i];
            i += 1;
        }
    }
    result
}

// Version of decompress that only cares about the length
fn decompress_length(input: &str) -> usize {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\((\d+)x(\d+)\)").unwrap();
    }
    let mut result = 0;
    let mut i = 0;
    while i < input.len() {
        if input[i..].starts_with('(') {
            let parts = RE.captures(&input[i..]).unwrap();
            let start = i + parts[0].len();
            let letter_count: usize = parts[1].parse().unwrap();
            let times: usize = parts[2].parse().unwrap();
            result += letter_count * times;
            i = start + letter_count;
        } else {
            result += 1;
            i += 1;
        }
    }
    result
}

// Recursive
fn decompress_length_v2(input: &str) -> usize {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\((\d+)x(\d+)\)").unwrap();
    }
    let mut result = 0;
    let mut i = 0;
    while i < input.len() {
        if input[i..].starts_with('(') {
            let parts = RE.captures(&input[i..]).unwrap();
            let start = i + parts[0].len();
            let letter_count: usize = parts[1].parse().unwrap();
            let times: usize = parts[2].parse().unwrap();

            let sub_str = &input[start..start + letter_count];
            if sub_str.contains('(') {
                // or should we use the regex?
                result += times * decompress_length_v2(sub_str);
            } else {
                result += letter_count * times;
            }
            i = start + letter_count;
        } else {
            result += 1;
            i += 1;
        }
    }
    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", decompress_length(&input));
    println!("Part 2: {}", decompress_length_v2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompress() {
        assert_eq!(decompress("ADVENT"), "ADVENT");
        assert_eq!(decompress_length("ADVENT"), 6);
        assert_eq!(decompress("A(1x5)BC"), "ABBBBBC");
        assert_eq!(decompress_length("A(1x5)BC"), 7);
        assert_eq!(decompress("(3x3)XYZ"), "XYZXYZXYZ");
        assert_eq!(decompress_length("(3x3)XYZ"), 9);
        assert_eq!(decompress("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG");
        assert_eq!(decompress_length("A(2x2)BCD(2x2)EFG"), 11);
        assert_eq!(decompress("(6x1)(1x3)A"), "(1x3)A");
        assert_eq!(decompress_length("(6x1)(1x3)A"), 6);
        assert_eq!(decompress("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY");
        assert_eq!(decompress_length("X(8x2)(3x3)ABCY"), 18);
    }

    #[test]
    fn test_decompress_length_v2() {
        assert_eq!(decompress_length_v2("(3x3)XYZ"), "XYZXYZXYZ".len());
        assert_eq!(
            decompress_length_v2("X(8x2)(3x3)ABCY"),
            "XABCABCABCABCABCABCY".len()
        );
        assert_eq!(
            decompress_length_v2("(27x12)(20x12)(13x14)(7x10)(1x12)A"),
            241920
        );
        assert_eq!(
            decompress_length_v2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            445
        );
    }
}
