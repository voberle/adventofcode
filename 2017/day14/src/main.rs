use std::fmt::Write;
use std::io::{self, Read};

fn hexa2bool(c: char) -> [bool; 4] {
    match c {
        '0' => [false, false, false, false],
        '1' => [false, false, false, true],
        '2' => [false, false, true, false],
        '3' => [false, false, true, true],
        '4' => [false, true, false, false],
        '5' => [false, true, false, true],
        '6' => [false, true, true, false],
        '7' => [false, true, true, true],
        '8' => [true, false, false, false],
        '9' => [true, false, false, true],
        'a' => [true, false, true, false],
        'b' => [true, false, true, true],
        'c' => [true, true, false, false],
        'd' => [true, true, false, true],
        'e' => [true, true, true, false],
        'f' => [true, true, true, true],
        _ => panic!("Invalid char {}", c),
    }
}

fn squares_used(input: &str) -> usize {
    let map: Vec<Vec<bool>> = (0..128)
        .map(|row| {
            let khash = knot_hash(&format!("{}-{}", input.trim(), row));
            khash.chars().flat_map(|c| hexa2bool(c).to_vec()).collect()
        })
        .collect();

    map.iter().flatten().filter(|v| **v).count()
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", squares_used(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(squares_used("flqrgnkx"), 8108);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("flqrgnkx"), 0);
    }
}


// Below is exact code from From Day 10

fn reverse(list: &mut Vec<usize>, length: usize, current_pos: usize) {
    for i in 0..length / 2 {
        let fi = (current_pos + i).rem_euclid(list.len());
        let li = (current_pos + length - 1 - i).rem_euclid(list.len());
        list.swap(fi, li);
    }
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