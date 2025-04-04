use std::io::{self, Read};

use itertools::Itertools;

fn line_to_u8(line: &str) -> Vec<u8> {
    (0..line.len())
        .step_by(2)
        .map(|i| {
            let s = &line[i..=i + 1];
            u8::from_str_radix(s, 16).unwrap()
        })
        .collect()
}

fn bloc_to_u8(bloc: &str) -> Vec<Vec<u8>> {
    bloc.lines().map(line_to_u8).collect()
}

// Parses the input as a list of fragments, each fragment being a list of bytes.
fn build(input: &str) -> Vec<Vec<Vec<u8>>> {
    let fragments: Vec<_> = input.split("\n\n").collect();
    fragments
        .iter()
        .map(|fragment| bloc_to_u8(fragment))
        .collect()
}

fn fragment_to_string(fragment: &[Vec<u8>]) -> String {
    fragment
        .iter()
        .map(|line| {
            // println!("{:02x?}", line);
            String::from_utf8_lossy(line)
        })
        .join("\n")
}

fn x_product(map_sample: &[Vec<Vec<u8>>]) -> usize {
    for fragment in map_sample {
        println!("{}", fragment_to_string(fragment));
        println!();
    }
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map_sample = build(&input);

    println!("Answer: {}", x_product(&map_sample));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_hex_to_utf8() {
        let bin = line_to_u8("e29594");
        let string_utf8_lossy = String::from_utf8_lossy(&bin);
        println!("{}", string_utf8_lossy);
        assert_eq!(string_utf8_lossy, "╔");
    }

    #[test]
    fn test_fragment_to_string() {
        let fragment = bloc_to_u8(INPUT_TEST_2);
        let expected = r"╔-═-═-═-
|~≋≋ññ≋~
║ññ≋~~≋�
|~ññ𑀍ñ≋�";
        let s = fragment_to_string(&fragment);
        assert_eq!(s, expected)
    }

    #[test]
    fn test_answer() {
        let map_sample = build(INPUT_TEST_1);
        assert_eq!(x_product(&map_sample), 132);
    }
}
