use std::io::{self, Read};

// Return the total calories per elf, sorted in ascending order.
fn build_elf_calories(input: &str) -> Vec<u32> {
    // Is there a better way to parse the input?
    let mut all_calories = Vec::new();
    let mut elf_calories = 0;
    for line in input.lines() {
        if line.is_empty() {
            all_calories.push(elf_calories);
            elf_calories = 0;
            continue;
        }
        elf_calories += line.parse::<u32>().unwrap();
    }
    all_calories.push(elf_calories);

    all_calories.sort_unstable();
    all_calories
}

fn most_calories(calories: &[u32]) -> u32 {
    *calories.last().unwrap()
}

fn top_3_calories_sum(calories: &[u32]) -> u32 {
    calories[calories.len() - 3..].iter().sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let calories = build_elf_calories(&input);

    println!("Part 1: {}", most_calories(&calories));
    println!("Part 2: {}", top_3_calories_sum(&calories));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(most_calories(&build_elf_calories(INPUT_TEST)), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(top_3_calories_sum(&build_elf_calories(INPUT_TEST)), 45000);
    }
}
