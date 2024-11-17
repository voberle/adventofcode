use std::fmt::Write;
use std::io::{self, Read};

fn hexa2bool(c: char) -> [bool; 4] {
    // A bit ugly, any better way?
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
        _ => panic!("Invalid char {c}"),
    }
}

fn get_grid(input: &str) -> Vec<Vec<bool>> {
    (0..128)
        .map(|row| {
            let khash = knot_hash(&format!("{}-{}", input.trim(), row));
            khash.chars().flat_map(|c| hexa2bool(c).to_vec()).collect()
        })
        .collect()
}

fn squares_used(grid: &[Vec<bool>]) -> usize {
    grid.iter().flatten().filter(|v| **v).count()
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}
use Dir::{Down, Left, Right, Up};

impl Dir {
    fn can_go(&self, pos: (usize, usize)) -> bool {
        match self {
            Up => pos.0 > 0,
            Down => pos.0 < 127,
            Left => pos.1 > 0,
            Right => pos.1 < 127,
        }
    }

    fn next_pos(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Up => (pos.0 - 1, pos.1),
            Down => (pos.0 + 1, pos.1),
            Left => (pos.0, pos.1 - 1),
            Right => (pos.0, pos.1 + 1),
        }
    }
}

fn regions_count(grid: &[Vec<bool>]) -> usize {
    let mut regions_cnt = 0;
    // We visit all the used squares of the grid, for each unvisited square, we find the corresponding region.
    let mut visited: Vec<Vec<bool>> = vec![vec![false; 128]; 128];
    for row in 0..128 {
        for col in 0..128 {
            if visited[row][col] {
                // Already been there.
                continue;
            }
            if !grid[row][col] {
                // If square is free, just mark it visited and continue.
                visited[row][col] = true;
                continue;
            }
            // We are on an unvisited used square. It's the beginning of a region.
            regions_cnt += 1;
            // Visit the whole region
            let mut queue: Vec<(usize, usize)> = Vec::new();
            queue.push((row, col));
            while let Some(pos) = queue.pop() {
                visited[pos.0][pos.1] = true;
                queue.extend([Up, Down, Left, Right].iter().filter_map(|dir| {
                    if !dir.can_go(pos) {
                        return None;
                    }
                    let next_pos = dir.next_pos(pos);
                    if visited[next_pos.0][next_pos.1] {
                        return None;
                    }
                    if !grid[next_pos.0][next_pos.1] {
                        return None;
                    }
                    Some(next_pos)
                }));
            }
        }
    }
    regions_cnt
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let grid = get_grid(&input);

    println!("Part 1: {}", squares_used(&grid));
    println!("Part 2: {}", regions_count(&grid));
}

// Below is exact code from From Day 10

fn reverse(list: &mut [usize], length: usize, current_pos: usize) {
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
        let _ = write!(output, "{xored:02x?}");
        output
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let grid = get_grid("flqrgnkx");
        assert_eq!(squares_used(&grid), 8108);
    }

    #[test]
    fn test_part2() {
        let grid = get_grid("flqrgnkx");
        assert_eq!(regions_count(&grid), 1242);
    }
}
