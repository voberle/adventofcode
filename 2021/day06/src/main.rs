use std::io::{self, Read};

fn build(input: &str) -> Vec<usize> {
    input.split(',').map(|line| line.parse().unwrap()).collect()
}

// Converts the list of fishes age into an array counting how many fish of each age there is.
fn convert(fishes: &[usize]) -> [usize; 9] {
    let mut ages = [0; 9];
    for fish in fishes {
        ages[*fish] += 1;
    }
    ages
}

fn step(fish_ages: &[usize]) -> Vec<usize> {
    let mut new_ages = vec![0; 9];
    // Slides all the ages left by 1.
    new_ages[..(fish_ages.len() - 1)].copy_from_slice(&fish_ages[1..]);

    new_ages[6] += fish_ages[0];
    new_ages[8] += fish_ages[0];
    new_ages
}

fn fish_count(fish_ages: &[usize], days: usize) -> usize {
    let mut fish_ages = fish_ages.to_vec();
    for _ in 0..days {
        fish_ages = step(&fish_ages);
    }
    fish_ages.iter().sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let fishes = build(&input);

    let fish_ages = convert(&fishes);

    println!("Part 1: {}", fish_count(&fish_ages, 80));
    println!("Part 2: {}", fish_count(&fish_ages, 256));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = "3,4,3,1,2";

    #[test]
    fn test_part1() {
        let fish_ages = convert(&build(INPUT_TEST));
        assert_eq!(fish_count(&fish_ages, 18), 26);
        assert_eq!(fish_count(&fish_ages, 80), 5934);
    }

    #[test]
    fn test_part2() {
        let fish_ages = convert(&build(INPUT_TEST));
        assert_eq!(fish_count(&fish_ages, 256), 26984457539);
    }
}
