// Implementation of the filesystem using individual blocks.

use std::{fmt, iter::repeat_n};

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

#[allow(dead_code)]
fn block_list_to_string(blocks: &[Block]) -> String {
    blocks
        .iter()
        .map(std::string::ToString::to_string)
        .collect()
}

fn move_blocks(blocks: &mut [Block]) {
    let mut free_space_pos = 0;
    let mut file_pos = blocks.len() - 1;
    while free_space_pos <= file_pos {
        while !matches!(blocks[free_space_pos], Block::Free) {
            free_space_pos += 1;
        }
        while !matches!(blocks[file_pos], Block::File(_)) {
            file_pos -= 1;
        }
        if free_space_pos >= file_pos {
            break;
        }
        blocks.swap(free_space_pos, file_pos);

        // println!("{} ({},{})", block_list_to_string(&blocks), free_space_pos, file_pos);
    }
}

fn calc_checksum(blocks: &[Block]) -> u64 {
    blocks
        .iter()
        .enumerate()
        .map(|(i, block)| match block {
            Block::Free => 0,
            Block::File(id) => i as u64 * u64::from(*id),
        })
        .sum()
}

pub fn checksum(disk_map: &[u8]) -> u64 {
    let mut blocks = make_block_list(disk_map);
    // println!("{}", block_list_to_string(&blocks));

    move_blocks(&mut blocks);
    // println!("{}", block_list_to_string(&blocks));

    calc_checksum(&blocks)
}
