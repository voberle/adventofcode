use std::io::{self, Read};

fn build(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[allow(clippy::unreadable_literal)]
fn transform_subject_number(subject_number: u64, loop_size: usize) -> u64 {
    let mut value: u64 = 1;
    for _ in 0..loop_size {
        value *= subject_number;
        value %= 20201227;
    }
    value
}

fn encryption_key(public_keys: &[u64]) -> u64 {
    let mut loop_size = 1;
    loop {
        let t = transform_subject_number(7, loop_size);
        if t == public_keys[0] {
            break;
        }
        loop_size += 1;
    }

    transform_subject_number(public_keys[1], loop_size)
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
