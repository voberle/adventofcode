use std::io::{self, Read};

use itertools::Itertools;
use regex::Regex;

struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

fn build(input: &str) -> Vec<Ingredient> {
    let re = Regex::new(r"(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Ingredient {
                capacity: caps[2].parse().unwrap(),
                durability: caps[3].parse().unwrap(),
                flavor: caps[4].parse().unwrap(),
                texture: caps[5].parse().unwrap(),
                calories: caps[6].parse().unwrap(),
            }
        })
        .collect()
}

fn score(ingredients: &[Ingredient], spoons: &[i64]) -> i64 {
    [
        |i: &Ingredient| i.capacity,
        |i: &Ingredient| i.durability,
        |i: &Ingredient| i.flavor,
        |i: &Ingredient| i.texture,
    ]
    .iter()
    .map(|f| {
        ingredients
            .iter()
            .zip(spoons)
            .map(|(i, s)| f(i) * s)
            .sum::<i64>()
            .max(0) // if a property makes a negative total, we take 0
    })
    .product()
}

fn calories_count(ingredients: &[Ingredient], spoons: &[i64]) -> i64 {
    ingredients
        .iter()
        .zip(spoons)
        .map(|(i, s)| i.calories * s)
        .sum::<i64>()
}

// This method supports only 2 or 4 ingredients recipees.
fn highest_score<const CALORIES: i64>(ingredients: &[Ingredient]) -> i64 {
    if ingredients.len() == 2 {
        (0..100)
            .filter_map(|i| {
                let spoons = &[i, 100 - i];
                if CALORIES == 0 || calories_count(ingredients, spoons) == CALORIES {
                    Some(score(ingredients, spoons))
                } else {
                    None
                }
            })
            .max()
            .unwrap()
    } else if ingredients.len() == 4 {
        (0..100)
            .permutations(3)
            .filter_map(|perm| {
                let sum = perm.iter().sum::<i64>();
                if sum > 100 {
                    return None;
                }
                let spoons = &[perm[0], perm[1], perm[2], 100 - sum];
                if CALORIES == 0 || calories_count(ingredients, spoons) == CALORIES {
                    Some(score(ingredients, spoons))
                } else {
                    None
                }
            })
            .max()
            .unwrap()
    } else {
        panic!("Unsupported number of ingredients {}", ingredients.len())
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let ingredients = build(&input);
    println!("Part 1: {}", highest_score::<0>(&ingredients));
    println!("Part 2: {}", highest_score::<500>(&ingredients));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_score() {
        assert_eq!(score(&build(INPUT_TEST), &vec![44, 56]), 62842880);
    }

    #[test]
    fn test_part1() {
        assert_eq!(highest_score::<0>(&build(INPUT_TEST)), 62842880);
    }

    #[test]
    fn test_part2() {
        assert_eq!(highest_score::<500>(&build(INPUT_TEST)), 57600000);
    }
}
