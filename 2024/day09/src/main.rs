use std::{
    fmt,
    io::{self, Read},
};

mod fs_ind_block;

fn build(input: &str) -> Vec<u8> {
    input.chars().map(|c| c as u8 - b'0').collect()
}

// Implementation that tracks each block as a group.
#[derive(Clone, Copy)]
enum Block {
    Free(usize),
    File(u32, usize),
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Free(cnt) => write!(f, "{}", ".".repeat(cnt)),
            Self::File(id, cnt) => {
                if id < 10 {
                    write!(f, "{}", id.to_string().repeat(cnt))
                } else {
                    // Not bothering to really support it.
                    write!(f, "{}", "#".repeat(cnt))
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
        .map(|d| {
            let b = if is_file {
                let file = Block::File(id, *d as usize);
                id += 1;
                file
            } else {
                Block::Free(*d as usize)
            };
            is_file ^= true; // toggle
            b
        })
        .collect()
}

#[allow(dead_code)]
fn block_list_to_string(blocks: &[Block]) -> String {
    blocks
        .iter()
        .map(std::string::ToString::to_string)
        .collect()
}

fn move_blocks(blocks: &mut Vec<Block>) {
    let mut initial_file_block_pos = blocks.len() - 1;
    loop {
        // Take the right most file, and try to move it to the left most free space.
        let mut file_block_pos = initial_file_block_pos;
        while !matches!(blocks[file_block_pos], Block::File(_, _)) && file_block_pos > 0 {
            file_block_pos -= 1;
        }
        // Find left most free spot that would fit this block
        let size = match blocks[file_block_pos] {
            Block::Free(_) => panic!("bug"),
            Block::File(_, cnt) => cnt,
        };
        let mut free_space_block_pos = 0;
        while free_space_block_pos < file_block_pos
            && !matches!(blocks[free_space_block_pos], Block::Free(c) if size <= c)
        {
            free_space_block_pos += 1;
        }

        if free_space_block_pos >= file_block_pos {
            initial_file_block_pos -= 1;
            if initial_file_block_pos == 0 {
                break;
            }
            continue;
        }

        // for (i, b) in blocks.iter().enumerate() {
        //     println!("{i}: {b}");
        // }
        // println!("Moving {file_block_pos} to {free_space_block_pos}");

        let b = blocks.remove(file_block_pos);
        blocks.insert(file_block_pos, Block::Free(size));
        let cnt = match blocks[free_space_block_pos] {
            Block::Free(cnt) => cnt,
            Block::File(_, _) => panic!("bug"),
        };
        blocks[free_space_block_pos] = Block::Free(cnt - size);
        blocks.insert(free_space_block_pos, b);

        initial_file_block_pos = blocks.len() - 1;

        // println!("{}", block_list_to_string(&blocks));
    }
}

fn calc_checksum(blocks: &[Block]) -> u64 {
    let mut pos: u64 = 0;
    let mut checksum = 0;
    for b in blocks {
        match b {
            Block::Free(cnt) => pos += *cnt as u64,
            Block::File(id, cnt) => {
                let id = u64::from(*id);
                let end_pos = pos + *cnt as u64;
                while pos < end_pos {
                    checksum += pos * id;
                    pos += 1;
                }
            }
        }
    }
    checksum
}

fn individual_frag_checksum(disk_map: &[u8]) -> u64 {
    fs_ind_block::checksum(disk_map)
}

fn whole_file_frag_checksum(disk_map: &[u8]) -> u64 {
    let mut blocks = make_block_list(disk_map);
    // println!("{}", block_list_to_string(&blocks));

    move_blocks(&mut blocks);
    // println!("{}", block_list_to_string(&blocks));

    // for (i, b) in blocks.iter().enumerate() {
    //     let p = match b {
    //         Block::Free(_) => 0,
    //         Block::File(id, cnt) => u64::try_from(*cnt).unwrap() * u64::from(*id),
    //     };
    //     println!("{p}\t{i}: {b}");
    // }

    calc_checksum(&blocks)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let disk_map = build(input.trim());

    println!("Part 1: {}", individual_frag_checksum(&disk_map));
    println!("Part 2: {}", whole_file_frag_checksum(&disk_map));
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
        assert_eq!(whole_file_frag_checksum(&build(INPUT_TEST)), 2858);
    }
}
