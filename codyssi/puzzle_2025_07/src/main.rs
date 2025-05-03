use std::io::{self, Read};

fn build(input: &str) -> (Vec<u32>, Vec<(usize, usize)>, usize) {
    let parts: Vec<_> = input.split("\n\n").collect();
    let frequencies = parts[0].lines().map(|n| n.parse().unwrap()).collect();
    let instructions = parts[1]
        .lines()
        .map(|ins| {
            let p: Vec<usize> = ins.split('-').map(|i| i.parse().unwrap()).collect();
            (p[0], p[1])
        })
        .collect();
    let index = parts[2].parse().unwrap();
    (frequencies, instructions, index)
}

fn track_freq_1(frequencies: &[u32], instructions: &[(usize, usize)], index: usize) -> u32 {
    let mut freqs = frequencies.to_vec();
    for (x, y) in instructions {
        freqs.swap(x - 1, y - 1);
    }
    freqs[index - 1]
}

fn track_freq_2(frequencies: &[u32], instructions: &[(usize, usize)], index: usize) -> u32 {
    let mut freqs = frequencies.to_vec();
    for index in 0..instructions.len() {
        let (x, y, z) = (
            instructions[index].0 - 1,
            instructions[index].1 - 1,
            instructions[(index + 1).rem_euclid(instructions.len())].0 - 1,
        );
        let z_val = freqs[z];
        freqs[z] = freqs[y];
        freqs[y] = freqs[x];
        freqs[x] = z_val;
    }
    freqs[index - 1]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (freqs, instructions, index) = build(&input);

    println!("Part 1: {}", track_freq_1(&freqs, &instructions, index));
    println!("Part 2: {}", track_freq_2(&freqs, &instructions, index));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (freqs, instructions, index) = build(&INPUT_TEST);
        assert_eq!(track_freq_1(&freqs, &instructions, index), 45);
    }

    #[test]
    fn test_part2() {
        let (freqs, instructions, index) = build(&INPUT_TEST);
        assert_eq!(track_freq_2(&freqs, &instructions, index), 796);
    }
}
