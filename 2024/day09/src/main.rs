use std::io::{self, Read};

mod fs_ind_block;

fn build(input: &str) -> Vec<u8> {
    input.chars().map(|c| c as u8 - b'0').collect()
}

fn individual_frag_checksum(disk_map: &[u8]) -> u64 {
    fs_ind_block::checksum(disk_map)
}

fn part2(disk_map: &[u8]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let disk_map = build(input.trim());

    println!("Part 1: {}", individual_frag_checksum(&disk_map));
    println!("Part 2: {}", part2(&disk_map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(individual_frag_checksum(&build(INPUT_TEST)), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
