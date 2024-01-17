use std::io::{self, Read};

fn build(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_truncation)]
fn steps_to_exit(original_offsets: &[i32]) -> usize {
    let mut offsets = original_offsets.to_vec();
    let mut ir: i32 = 0; // need to use signed integer as the offset can make us go negative
    let mut steps = 0;
    while ir >= 0 && ir < offsets.len() as i32 {
        let o = offsets[ir as usize];
        offsets[ir as usize] += 1;
        ir += o;
        steps += 1;
    }
    steps
}

fn part2(offsets: &[i32]) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let offsets = build(&input);

    println!("Part 1: {}", steps_to_exit(&offsets));
    println!("Part 2: {}", part2(&offsets));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(steps_to_exit(&build(INPUT_TEST)), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
