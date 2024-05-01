use std::io::{self, Read};

fn build(input: &str) -> Vec<usize> {
    input.split(',').map(|line| line.parse().unwrap()).collect()
}

fn step(fishes: &mut Vec<usize>) {
    let mut new_fishes: Vec<usize> = Vec::new();
    for fish in &mut *fishes {
        if *fish == 0 {
            *fish = 6;
            new_fishes.push(8);
        } else {
            *fish -= 1;
        }
    }
    fishes.extend(new_fishes);
}

fn fish_count(fishes: &[usize], days: usize) -> usize {
    let mut fishes = fishes.to_vec();
    for _ in 0..days {
        step(&mut fishes);
    }
    fishes.len()
}

fn part2(fishes: &[usize]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let fishes = build(&input);

    println!("Part 1: {}", fish_count(&fishes, 80));
    println!("Part 2: {}", part2(&fishes));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = "3,4,3,1,2";

    #[test]
    fn test_part1() {
        assert_eq!(fish_count(&build(INPUT_TEST), 18), 26);
        assert_eq!(fish_count(&build(INPUT_TEST), 80), 5934);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
