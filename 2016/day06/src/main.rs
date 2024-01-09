use std::io::{self, Read};

use fxhash::FxHashMap;

fn build(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn error_corrected_msg<const MODIFIED: bool>(input: &[Vec<char>]) -> String {
    let line_len = input[0].len();
    let mut message = String::with_capacity(line_len);
    for i in 0..line_len {
        let frequencies =
            input
                .iter()
                .map(|line| line[i])
                .fold(FxHashMap::default(), |mut map, val| {
                    map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
                    map
                });
        message.push(if MODIFIED {
            *frequencies.iter().min_by_key(|(_, v)| *v).unwrap().0
        } else {
            *frequencies.iter().max_by_key(|(_, v)| *v).unwrap().0
        });
    }
    message
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input_parsed = build(&input);

    println!("Part 1: {}", error_corrected_msg::<false>(&input_parsed));
    println!("Part 2: {}", error_corrected_msg::<true>(&input_parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(error_corrected_msg::<false>(&build(INPUT_TEST)), "easter");
    }

    #[test]
    fn test_part2() {
        assert_eq!(error_corrected_msg::<true>(&build(INPUT_TEST)), "advent");
    }
}
