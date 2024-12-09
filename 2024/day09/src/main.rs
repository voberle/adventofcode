use std::{
    fmt,
    io::{self, Read},
    iter::repeat_n,
};

fn build(input: &str) -> Vec<u8> {
    input.chars().map(|c| c as u8 - b'0').collect()
}

#[derive(Clone, Copy)]
enum Block {
    Free,
    File(u32),
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Free => write!(f, ".",),
            Self::File(id) => {
                if id < 10 {
                    write!(f, "{id}")
                } else {
                    write!(f, "[{id}]")
                }
            }
        }
    }
}

fn make_block_list(disk_map: &[u8]) -> Vec<Block> {
    let mut is_file = true;
    let mut id = 0;
    disk_map
        .iter()
        .flat_map(|d| {
            let b = if is_file {
                let file = Block::File(id);
                id += 1;
                file
            } else {
                Block::Free
            };
            is_file ^= true; // toggle
            repeat_n(b, *d as usize)
        })
        .collect()
}

fn block_list_to_string(blocks: &[Block]) -> String {
    blocks
        .iter()
        .map(std::string::ToString::to_string)
        .collect()
}

fn checksum(disk_map: &[u8]) -> i64 {
    let blocks = make_block_list(disk_map);
    println!("{}", block_list_to_string(&blocks));
    0
}

fn part2(disk_map: &[u8]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let disk_map = build(&input);

    println!("Part 1: {}", checksum(&disk_map));
    println!("Part 2: {}", part2(&disk_map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(checksum(&build(INPUT_TEST)), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
