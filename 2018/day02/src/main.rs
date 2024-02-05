use std::{
    cell::Cell,
    io::{self, Read},
};

fn build(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

// Returns if the id has letters with 2 occurences and with 3 occurences.
fn count_occurences(id: &[char]) -> (bool, bool) {
    // We only have letter from the alphabet, so we can simply use
    // an array of 26 chars to count the number of occurences of each.
    let mut occs: [u8; 26] = [0; 26];
    for c in id {
        occs[*c as usize - 'a' as usize] += 1;
    }
    (occs.contains(&2), occs.contains(&3))
}

fn checksum(box_ids: &[Vec<char>]) -> usize {
    let res = box_ids.iter().fold((0, 0), |acc: (usize, usize), ids| {
        let (c2, c3) = count_occurences(ids);
        (
            acc.0 + if c2 { 1 } else { 0 },
            acc.1 + if c3 { 1 } else { 0 },
        )
    });
    res.0 * res.1
}

// Returns true of the two id differ by only one char at the same position.
fn cmp_ids(id1: &[char], id2: &[char]) -> bool {
    let diff_count = Cell::new(0);
    id1.iter()
        .zip(id2.iter())
        .take_while(|_| diff_count.get() < 2)
        .for_each(|p| {
            if p.0 != p.1 {
                diff_count.set(diff_count.get() + 1);
            }
        });
    diff_count.get() == 1
}

// Returns a string made of the common letters of the two slices (common and at the same position).
fn common_letters(id1: &[char], id2: &[char]) -> String {
    id1.iter()
        .zip(id2.iter())
        .flat_map(|p| if p.0 == p.1 { Some(p.0) } else { None })
        .collect()
}

fn correct_ids_common_letters(box_ids: &[Vec<char>]) -> String {
    // A simple way to iterate over all pairs we can do with the vector, without using itertools.
    let mut iter = box_ids.iter();
    for id1 in box_ids {
        for id2 in iter.clone() {
            if cmp_ids(id1, id2) {
                return common_letters(id1, id2);
            }
        }
        iter.next();
    }
    panic!("No common ID found");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let box_ids = build(&input);

    println!("Part 1: {}", checksum(&box_ids));
    println!("Part 2: {}", correct_ids_common_letters(&box_ids));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(checksum(&build(INPUT_TEST_1)), 12);
    }

    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part2() {
        assert_eq!(correct_ids_common_letters(&build(INPUT_TEST_2)), "fgij");
    }
}
