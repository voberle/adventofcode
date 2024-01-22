use std::io::{self, Read};

fn value_after_last_insert(steps: usize) -> usize {
    const REPEAT_COUNT: usize = 2017;

    let mut buf: Vec<usize> = Vec::with_capacity(REPEAT_COUNT + 1);
    buf.push(0);

    let mut pos = 0;
    for i in 1..=REPEAT_COUNT {
        pos = (pos + steps).rem_euclid(buf.len()) + 1;
        buf.insert(pos, i);
    }
    buf[pos + 1]
}

fn part2(steps: usize) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let steps = input.trim().parse().unwrap();

    println!("Part 1: {}", value_after_last_insert(steps));
    println!("Part 2: {}", part2(steps));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(value_after_last_insert(3), 638);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(3), 0);
    }
}
