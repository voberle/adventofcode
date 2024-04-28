use std::io::{self, Read};

fn build(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[allow(clippy::unreadable_literal)]
const TRANSFORM_CONST: u64 = 20201227;

fn transform_subject_number(subject_number: u64, loop_size: usize) -> u64 {
    let mut value: u64 = 1;
    for _ in 0..loop_size {
        value *= subject_number;
        value %= TRANSFORM_CONST;
    }
    value
}

fn find_loop_size(public_keys: &[u64]) -> (usize, usize) {
    const SUBJECT_NUMBER: u64 = 7;

    let mut value: u64 = 1;
    for loop_size in 1.. {
        value *= SUBJECT_NUMBER;
        value %= TRANSFORM_CONST;
        if public_keys[0] == value {
            return (0, loop_size);
        }
        if public_keys[1] == value {
            return (1, loop_size);
        }
    }
    panic!("No loop size found")
}

fn encryption_key(public_keys: &[u64]) -> u64 {
    let (idx, loop_size) = find_loop_size(public_keys);

    transform_subject_number(public_keys[1 - idx], loop_size)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let public_keys = build(&input);

    println!("Part 1: {}", encryption_key(&public_keys));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_KEYS: [u64; 2] = [5764801, 17807724];

    #[test]
    fn test_transform_subject_number() {
        assert_eq!(transform_subject_number(7, 8), 5764801);
        assert_eq!(transform_subject_number(7, 11), 17807724);
    }

    #[test]
    fn test_part1() {
        assert_eq!(encryption_key(&TEST_KEYS), 14897079);
    }
}
