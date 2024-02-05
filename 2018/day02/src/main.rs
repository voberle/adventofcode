use std::io::{self, Read};

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

fn part2(box_ids: &[Vec<char>]) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let box_ids = build(&input);

    println!("Part 1: {}", checksum(&box_ids));
    println!("Part 2: {}", part2(&box_ids));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(checksum(&build(INPUT_TEST)), 12);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
