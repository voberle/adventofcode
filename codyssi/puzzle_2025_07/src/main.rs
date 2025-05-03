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
    let test_index = parts[2].parse().unwrap();
    (frequencies, instructions, test_index)
}

fn test_track_freq(frequencies: &[u32], instructions: &[(usize, usize)], test_index: usize) -> u32 {
    let mut freqs = frequencies.to_vec();
    for ins in instructions {
        freqs.swap(ins.0 - 1, ins.1 - 1);
    }
    freqs[test_index - 1]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (frequencies, instructions, test_index) = build(&input);

    println!(
        "Part 1: {}",
        test_track_freq(&frequencies, &instructions, test_index)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (frequencies, instructions, test_index) = build(&INPUT_TEST);
        assert_eq!(test_track_freq(&frequencies, &instructions, test_index), 45);
    }
}
