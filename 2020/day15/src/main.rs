use std::io::{self, Read};

use fxhash::FxHashMap;

fn build(input: &str) -> Vec<u32> {
    input.split(',').map(|line| line.parse().unwrap()).collect()
}

// Unoptimized version using a vector.
fn _number_at(starting_numbers: &[u32], pos: usize) -> u32 {
    let mut numbers = starting_numbers.to_vec();
    while numbers.len() < pos {
        let last_number = numbers.last().unwrap();
        numbers.push(if numbers[0..numbers.len() - 1].contains(last_number) {
            let last_nb_pos_rev = numbers
                .iter()
                .rev()
                .skip(1)
                .position(|v| v == last_number)
                .unwrap();
            u32::try_from(last_nb_pos_rev).unwrap() + 1
        } else {
            0
        });
    }
    *numbers.last().unwrap()
}

// Optimized version tracking the position of last occurence of a number.
fn number_at(starting_numbers: &[u32], target_pos: u32) -> u32 {
    let mut number_last_pos: FxHashMap<u32, u32> = FxHashMap::default();

    // Add starting numbers, except last one (it will be next spoken one).
    for (i, n) in starting_numbers[0..starting_numbers.len() - 1]
        .iter()
        .enumerate()
    {
        // Checking that numbers are unique in starting list.
        assert!(number_last_pos
            .insert(*n, u32::try_from(i).unwrap() + 1)
            .is_none());
    }

    let mut pos = u32::try_from(starting_numbers.len()).unwrap();
    let mut last_number = *starting_numbers.last().unwrap();

    while pos < target_pos {
        let next_nb = if let Some(last_pos) = number_last_pos.get(&last_number) {
            pos - *last_pos
        } else {
            0
        };

        number_last_pos.insert(last_number, pos);
        pos += 1;

        last_number = next_nb;
    }

    last_number
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let starting_numbers = build(input.trim());

    println!("Part 1: {}", number_at(&starting_numbers, 2020));
    println!("Part 2: {}", number_at(&starting_numbers, 30_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const POS: u32 = 2020;
        assert_eq!(number_at(&build("0,3,6"), POS), 436);
        assert_eq!(number_at(&build("1,3,2"), POS), 1);
        assert_eq!(number_at(&build("2,1,3"), POS), 10);
        assert_eq!(number_at(&build("1,2,3"), POS), 27);
        assert_eq!(number_at(&build("2,3,1"), POS), 78);
        assert_eq!(number_at(&build("3,2,1"), POS), 438);
        assert_eq!(number_at(&build("3,1,2"), POS), 1836);
    }

    #[test]
    fn test_part2() {
        const POS: u32 = 30_000_000;
        assert_eq!(number_at(&build("0,3,6"), POS), 175594);
        // Runs a bit slow, so commented those out.
        // assert_eq!(number_at(&build("1,3,2"), POS), 2578);
        // assert_eq!(number_at(&build("2,1,3"), POS), 3544142);
        // assert_eq!(number_at(&build("1,2,3"), POS), 261214);
        // assert_eq!(number_at(&build("2,3,1"), POS), 6895259);
        // assert_eq!(number_at(&build("3,2,1"), POS), 18);
        // assert_eq!(number_at(&build("3,1,2"), POS), 362);
    }
}
