use std::io::{self, Read};

fn value_after_last_insert(steps: usize, insert_count: usize) -> usize {
    let mut buf: Vec<usize> = Vec::with_capacity(insert_count + 1);
    buf.push(0);

    let mut pos = 0;
    for i in 1..=insert_count {
        pos = (pos + steps).rem_euclid(buf.len()) + 1;
        buf.insert(pos, i);
    }
    buf[pos + 1]
}

fn value_after_zero(steps: usize, insert_count: usize) -> usize {
    // Since 0 never moves, it means we want the value at index 1.
    // We don't need to actually have a vector and do inserts, we can just track
    // which value would be set at index 1.
    let mut val_at_1 = 0;

    let mut pos = 0;
    for i in 1..=insert_count {
        pos = (pos + steps).rem_euclid(i) + 1;
        if pos == 1 {
            val_at_1 = i;
        }
    }
    val_at_1
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let steps = input.trim().parse().unwrap();

    println!("Part 1: {}", value_after_last_insert(steps, 2017));
    println!("Part 2: {}", value_after_zero(steps, 50_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(value_after_last_insert(3, 2017), 638);
    }
}
