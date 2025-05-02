use std::io::{self, Read};

fn build(input: &str) -> (Vec<u32>, Vec<char>) {
    let lines: Vec<_> = input.lines().collect();

    let numbers = lines
        .iter()
        .take(lines.len() - 1)
        .map(|n| n.parse().unwrap())
        .collect();
    let symbols = lines.last().unwrap().chars().collect();
    (numbers, symbols)
}

fn actual_compass_offset(numbers: &[u32], symbols: &[char]) -> i64 {
    numbers
        .iter()
        .skip(1)
        .zip(symbols.iter())
        .fold(i64::from(numbers[0]), |acc, (&n, s)| match s {
            '+' => acc + i64::from(n),
            '-' => acc - i64::from(n),
            _ => panic!("Unknown symbol"),
        })
}

fn new_actual_compass_offset(numbers: &[u32], symbols: &[char]) -> i64 {
    numbers
        .iter()
        .skip(1)
        .zip(symbols.iter().rev())
        .fold(i64::from(numbers[0]), |acc, (&n, s)| match s {
            '+' => acc + i64::from(n),
            '-' => acc - i64::from(n),
            _ => panic!("Unknown symbol"),
        })
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (numbers, symbols) = build(&input);

    println!("Part 1: {}", actual_compass_offset(&numbers, &symbols));
    println!("Part 2: {}", new_actual_compass_offset(&numbers, &symbols));
    // println!("Part 3: {}", part3(&numbers, &symbols));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (numbers, symbols) = build(&INPUT_TEST);
        assert_eq!(actual_compass_offset(&numbers, &symbols), 21);
    }

    #[test]
    fn test_part2() {
        let (numbers, symbols) = build(&INPUT_TEST);
        assert_eq!(new_actual_compass_offset(&numbers, &symbols), 23);
    }

    #[test]
    fn test_part3() {
        // let (numbers, symbols) = build(&INPUT_TEST);
        // assert_eq!(part3(&numbers, &symbols)), );
    }
}
