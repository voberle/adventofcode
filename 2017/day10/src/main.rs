use std::fmt::Write;
use std::io::{self, Read};

fn build(input: &str) -> Vec<usize> {
    input.split(',').map(|v| v.parse().unwrap()).collect()
}

fn reverse(list: &mut Vec<usize>, length: usize, current_pos: usize) {
    for i in 0..length / 2 {
        let fi = (current_pos + i).rem_euclid(list.len());
        let li = (current_pos + length - 1 - i).rem_euclid(list.len());
        list.swap(fi, li);
    }
}

fn hash(lengths: &[usize], size: usize) -> Vec<usize> {
    let mut list: Vec<usize> = (0..size).collect();
    let mut current_pos = 0;

    for (skip_size, &length) in lengths.iter().enumerate() {
        reverse(&mut list, length, current_pos);
        current_pos = (current_pos + length + skip_size).rem_euclid(size);
    }
    list
}

fn knot_hash(input: &str) -> String {
    const FIXED_LENGTHS: [usize; 5] = [17, 31, 73, 47, 23];

    // Take the input as a string of bytes
    let mut lengths: Vec<usize> = input.as_bytes().iter().map(|v| *v as usize).collect();
    lengths.extend(FIXED_LENGTHS);

    let mut list: Vec<usize> = (0..256).collect();
    let mut current_pos: usize = 0;
    let mut skip_size: usize = 0;

    for _ in 0..64 {
        for length in &lengths {
            reverse(&mut list, *length, current_pos);
            current_pos = (current_pos + length + skip_size).rem_euclid(256);
            skip_size += 1;
        }
    }

    // list is now the sparse hash, convert to dense hash
    list.chunks(16).fold(String::new(), |mut output, block| {
        // copied() avoids us a lot of trouble with references in reduce().
        let xored = block.iter().copied().reduce(|acc, e| acc ^ e).unwrap();

        // fold() and write! is better than map and format!, less allocations
        let _ = write!(output, "{:02x?}", xored);
        output
    })
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let input_parsed = build(input.trim());
    let final_list = hash(&input_parsed, 256);
    println!("Part 1: {}", final_list[0] * final_list[1]);

    println!("Part 2: {}", knot_hash(input.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let final_list = hash(&[3, 4, 1, 5], 5);
        assert_eq!(final_list, &[3, 4, 2, 1, 0]);
        assert_eq!(final_list[0] * final_list[1], 12);
    }

    #[test]
    fn test_part2() {
        assert_eq!(knot_hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(knot_hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(knot_hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(knot_hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
