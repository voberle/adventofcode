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

fn fish_count(fish_ages: &[usize], days: usize) -> usize {
    let mut ages = [0; 9];
    ages.clone_from_slice(fish_ages);

    for _ in 0..days {
        // Slides all the ages left by 1.
        ages.rotate_left(1);
        // The ones with timer 0 reproduced: New ones got age 8 (so at index 8),
        // and existing ones get age 6 (so to be added to index 6)/
        ages[6] += ages[8];
    }
    ages.iter().sum()
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
