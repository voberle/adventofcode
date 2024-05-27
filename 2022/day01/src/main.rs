use std::io::{self, Read};

fn build(input: &str) -> Vec<Vec<u32>> {
    let mut all_calories = Vec::new();
    let mut elf_calories = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            all_calories.push(elf_calories);
            elf_calories = Vec::new();
            continue;
        }
        elf_calories.push(line.parse().unwrap());
    }
    all_calories.push(elf_calories);
    all_calories
}

fn most_calories(calories: &[Vec<u32>]) -> u32 {
    calories.iter().map(|e| e.iter().sum()).max().unwrap()
}

fn part2(calories: &[Vec<u32>]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let calories = build(&input);

    println!("Part 1: {}", most_calories(&calories));
    println!("Part 2: {}", part2(&calories));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(most_calories(&build(INPUT_TEST)), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
