use std::io::{self, Read};

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

fn build(input: &str) -> Vec<Food> {
    let re = Regex::new(r"(.+)\(contains (.+)\)").unwrap();
    input
        .lines()
        .map(|line| {
            let parts = re.captures(line).unwrap();
            let ingredients: Vec<String> = parts[1]
                .split_whitespace()
                .map(ToString::to_string)
                .collect();
            let allergens: Vec<String> = parts[2].split(", ").map(ToString::to_string).collect();
            Food {
                ingredients,
                allergens,
            }
        })
        .collect()
}

fn find_ingredients_with_allergens(foods: &[Food]) -> FxHashMap<String, String> {
    // Rules:
    // - Each allergen is found in exactly one ingredient.
    // - Each ingredient contains zero or one allergen.

    // For each allergen, have the list of foods that contain it.
    let mut allergens_in: FxHashMap<String, Vec<FxHashSet<String>>> = FxHashMap::default();
    for food in foods {
        for allergen in &food.allergens {
            allergens_in
                .entry(allergen.clone())
                .and_modify(|ingredients| {
                    let ingredients_set: FxHashSet<String> =
                        food.ingredients.iter().cloned().collect();
                    ingredients.push(ingredients_set);
                })
                .or_insert({
                    let ingredients_set: FxHashSet<String> =
                        food.ingredients.iter().cloned().collect();
                    vec![ingredients_set]
                });
        }
    }
    // println!("{:#?}", allergens_in);

    // Ingredients candidates that may contain the allergens.
    // This is the interestion of the foods that contain the allergen.
    let mut allergens_in_reduced: FxHashMap<String, FxHashSet<String>> = FxHashMap::default();
    for (allergens, ingredients_sets) in &mut allergens_in {
        let mut iter = ingredients_sets.iter();
        let intersection = iter
            .next()
            .map(|set| {
                iter.fold(set.clone(), |set1, set2| {
                    set1.intersection(set2).cloned().collect()
                })
            })
            .unwrap();
        allergens_in_reduced.insert(allergens.clone(), intersection);
    }
    // println!("{:#?}", allergens_in_reduced);

    // Now just iterate over the list by removing from candidates the ones we know for sure.
    while allergens_in_reduced
        .values()
        .any(|candidates| candidates.len() > 1)
    {
        // Get all ingredients we know.
        let ingredients_we_know: Vec<String> = allergens_in_reduced
            .values()
            .filter_map(|ingredients| {
                if ingredients.len() == 1 {
                    Some(ingredients.iter().next().unwrap().clone())
                } else {
                    None
                }
            })
            .collect();

        // Remove the known ingredients from the lists that are bigger than 1.
        allergens_in_reduced
            .values_mut()
            .filter(|candidates| candidates.len() > 1)
            .for_each(|candidates| candidates.retain(|c| !ingredients_we_know.contains(c)));
    }
    // println!("{:#?}", allergens_in_reduced);

    allergens_in_reduced
        .iter()
        .map(|(allergen, ingredients)| {
            (allergen.clone(), ingredients.iter().next().unwrap().clone())
        })
        .collect()
}

fn ingredients_without_allergens_count(
    foods: &[Food],
    ingredients_with_allergens: &FxHashMap<String, String>,
) -> usize {
    // Extract the list of ingredients that have allergens.
    let ingredients: FxHashSet<String> = ingredients_with_allergens.values().cloned().collect();

    // Just count how many times we see ingredients without allergens.
    foods
        .iter()
        .map(|food| {
            food.ingredients
                .iter()
                .filter(|i| !ingredients.contains(*i))
                .count()
        })
        .sum()
}

fn canonical_dangerous_ingredient(
    ingredients_with_allergens: &FxHashMap<String, String>,
) -> String {
    // Sorted alphabetically by their allergen
    ingredients_with_allergens
        .iter()
        .sorted_unstable_by_key(|(allergen, _)| *allergen)
        .map(|(_, ingredient)| ingredient)
        .join(",")
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let foods = build(&input);

    let ingredients_with_allergens = find_ingredients_with_allergens(&foods);

    println!(
        "Part 1: {}",
        ingredients_without_allergens_count(&foods, &ingredients_with_allergens)
    );
    println!(
        "Part 2: {}",
        canonical_dangerous_ingredient(&ingredients_with_allergens)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let foods = build(INPUT_TEST);
        let ingredients_with_allergens = find_ingredients_with_allergens(&foods);
        assert_eq!(
            ingredients_without_allergens_count(&foods, &ingredients_with_allergens),
            5
        );
    }

    #[test]
    fn test_part2() {
        let foods = build(INPUT_TEST);
        let ingredients_with_allergens = find_ingredients_with_allergens(&foods);
        assert_eq!(
            canonical_dangerous_ingredient(&ingredients_with_allergens),
            "mxmxvkd,sqjhc,fvjkl"
        );
    }
}
