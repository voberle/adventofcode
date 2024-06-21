//! Very simple and working version adapted from a Reddit solution.
//! Works on specific input only.

use fxhash::FxHashMap;

#[allow(dead_code)]
pub fn run_both_parts(regex: &[u8]) {
    let mut grid: FxHashMap<(i32, i32), usize> = FxHashMap::default();
    let (mut dist, mut x, mut y) = (0, 0, 0);
    let mut stack: Vec<(usize, i32, i32)> = Vec::new();

    for c in &regex[1..regex.len() - 1] {
        match c {
            b'(' => stack.push((dist, x, y)),
            b')' => (dist, x, y) = stack.pop().unwrap(),
            b'|' => (dist, x, y) = *stack.last().unwrap(),
            _ => {
                x += i32::from(*c == b'E') - i32::from(*c == b'W');
                y += i32::from(*c == b'S') - i32::from(*c == b'N');
                dist += 1;
                grid.entry((x, y))
                    .and_modify(|e| {
                        if dist < *e {
                            *e = dist;
                        }
                    })
                    .or_insert(dist);
            }
        }
    }

    println!("Part 1: {}", grid.values().max().unwrap());
    println!("Part 2: {}", grid.values().filter(|v| **v >= 1000).count());
}
