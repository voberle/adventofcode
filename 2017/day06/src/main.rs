use std::io::{self, Read};

use fxhash::FxHashMap;

fn build(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect()
}

const fn wrapping_index(i: usize, len: usize) -> usize {
    (i % len + len) % len
}

fn most_blocks_idx(mem_banks: &[u32]) -> usize {
    // We cannot use iterator max, since if there are several max, it would return the last one,
    // while we want the first.
    mem_banks
        .iter()
        .enumerate()
        .fold(
            (0, 0),
            |(im, vm), (i, v)| if *v > vm { (i, *v) } else { (im, vm) },
        )
        .0
}

fn redistribution_cycles_count(original: &[u32]) -> (usize, usize) {
    let mut mem_banks = original.to_vec();
    let mut seen_at: FxHashMap<Vec<u32>, usize> = FxHashMap::default();
    let mut count = 0;
    while !seen_at.contains_key(&mem_banks) {
        seen_at.insert(mem_banks.clone(), count);

        // maybe we could optimize by distributing more than one if we have a lot
        let mut i = most_blocks_idx(&mem_banks);
        let mut blocks = mem_banks[i];
        // take all
        mem_banks[i] = 0;
        // and distribute it
        while blocks > 0 {
            i = wrapping_index(i + 1, mem_banks.len());
            mem_banks[i] += 1;
            blocks -= 1;
        }
        count += 1;
    }
    (count, count - *seen_at.get(&mem_banks).unwrap())
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mem_banks = build(&input);

    let (since_beginning, since_seen) = redistribution_cycles_count(&mem_banks);
    println!("Part 1: {}", since_beginning);
    println!("Part 2: {}", since_seen);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(redistribution_cycles_count(&[0, 2, 7, 0]).0, 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(redistribution_cycles_count(&[0, 2, 7, 0]).1, 4);
    }
}
