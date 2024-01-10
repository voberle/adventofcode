use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

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

fn decompress_length(input: &str) -> usize {
    decompress(input).len()
}

fn part2(input: &str) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", decompress_length(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompress() {
        assert_eq!(decompress("ADVENT"), "ADVENT");
        assert_eq!(decompress("A(1x5)BC"), "ABBBBBC");
        assert_eq!(decompress("(3x3)XYZ"), "XYZXYZXYZ");
        assert_eq!(decompress("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG");
        assert_eq!(decompress("(6x1)(1x3)A"), "(1x3)A");
        assert_eq!(decompress("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(""), 0);
    }
}
