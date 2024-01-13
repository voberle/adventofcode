use std::{
    collections::VecDeque,
    io::{self, Read},
};

fn elf_getting_all_v1(elf_count: usize) -> usize {
    // This array has the index of the next elf that has presents
    let mut next_with_presents = vec![0; elf_count];
    for (i, e) in next_with_presents.iter_mut().enumerate().take(elf_count) {
        *e = i + 1;
    }
    next_with_presents[elf_count - 1] = 0;

    let mut elf_with_presents = elf_count;
    let mut stealer = 0;
    while elf_with_presents > 1 {
        // next looser
        let looser = next_with_presents[stealer];

        next_with_presents[stealer] = next_with_presents[looser];
        elf_with_presents -= 1;

        stealer = next_with_presents[stealer];
    }
    stealer + 1
}

const fn wrapping_index(i: usize, len: usize) -> usize {
    (i % len + len) % len
}

// Horrible brute-force implementation, but I couldn't find a smarter way
fn elf_getting_all_v2(elf_count: usize) -> usize {
    // This array has the elf number
    let mut next_with_presents: Vec<i32> = vec![0; elf_count];
    for (i, e) in next_with_presents.iter_mut().enumerate().take(elf_count) {
        *e = i as i32 + 1;
    }

    let mut stealer_idx = 0;
    while next_with_presents.len() > 1 {
        let stealer = next_with_presents[stealer_idx];

        let looser_idx = wrapping_index(
            stealer_idx + next_with_presents.len() / 2,
            next_with_presents.len(),
        );

        next_with_presents.remove(looser_idx);

        stealer_idx = wrapping_index(
            next_with_presents
                .iter()
                .position(|&e| e == stealer)
                .unwrap()
                + 1,
            next_with_presents.len(),
        );
    }
    next_with_presents[0] as usize
}

// Fast implementation inspired by an idea on Reddit.
// https://www.reddit.com/r/adventofcode/comments/5j4lp1/comment/dbdj8jh/?utm_source=share&utm_medium=web2x&context=3
#[allow(dead_code)]
fn elf_getting_all_v2_fast(elf_count: usize) -> usize {
    let mut left: VecDeque<usize> = (1..=elf_count / 2).collect();
    let mut right: VecDeque<usize> = (elf_count / 2 + 1..=elf_count).collect();

    while left.len() + right.len() > 1 {
        right.pop_front();
        right.push_back(left.pop_front().unwrap());
        if (left.len() + right.len()) % 2 == 0 {
            left.push_back(right.pop_front().unwrap());
        }
    }
    right[0]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let elf_count = input.trim().parse().unwrap();

    println!("Part 1: {}", elf_getting_all_v1(elf_count));
    println!("Part 2: {}", elf_getting_all_v2(elf_count));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(elf_getting_all_v1(3), 3);
        assert_eq!(elf_getting_all_v1(4), 1);
        assert_eq!(elf_getting_all_v1(5), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(elf_getting_all_v2(3), 3);
        assert_eq!(elf_getting_all_v2(4), 1);
        assert_eq!(elf_getting_all_v2(5), 2);
    }
}
