use std::io::{self, Read};

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

#[derive(Debug)]
struct Food<'a> {
    ingredients: Vec<&'a str>,
    allergens: Vec<&'a str>,
}

impl<'a> Food<'a> {
    fn new(line: &'a str) -> Self {
        let mut tokens_it = line.split_ascii_whitespace();
        let mut ingredients: Vec<&'a str> = Vec::new();
        loop {
            let token = tokens_it.next().unwrap();
            if token.starts_with('(') {
                // skip "(contains" and move on to allergens
                break;
            }
            ingredients.push(token);
        }

        let mut allergens: Vec<&'a str> = Vec::new();
        for mut token in tokens_it {
            token = token.strip_suffix(',').unwrap_or(token);
            token = token.strip_suffix(')').unwrap_or(token);
            allergens.push(token);
        }

        Food {
            ingredients,
            allergens,
        }
    }
}

fn build(input: &str) -> Vec<Food> {
    input.lines().map(Food::new).collect()
}

fn find_ingredients_with_allergens<'a>(foods: &'a [Food]) -> FxHashMap<&'a str, &'a str> {
    // Rules:
    // - Each allergen is found in exactly one ingredient.
    // - Each ingredient contains zero or one allergen.

    // For each allergen, have the list of foods that contain it.
    let mut allergens_in: FxHashMap<&str, Vec<FxHashSet<&str>>> = FxHashMap::default();
    for food in foods {
        for allergen in &food.allergens {
            allergens_in
                .entry(allergen)
                .and_modify(|ingredients| {
                    let ingredients_set: FxHashSet<&str> =
                        food.ingredients.iter().copied().collect();
                    ingredients.push(ingredients_set);
                })
                .or_insert({
                    let ingredients_set: FxHashSet<&str> =
                        food.ingredients.iter().copied().collect();
                    vec![ingredients_set]
                });
        }
    }
    // println!("{:#?}", allergens_in);

    // Ingredients candidates that may contain the allergens.
    // This is the interestion of the foods that contain the allergen.
    let mut allergens_in_reduced: FxHashMap<&str, FxHashSet<&str>> = FxHashMap::default();
    for (allergens, ingredients_sets) in &mut allergens_in {
        let mut iter = ingredients_sets.iter();
        let intersection = iter
            .next()
            .map(|set| {
                iter.fold(set.clone(), |set1, set2| {
                    set1.intersection(set2).copied().collect()
                })
            })
            .unwrap();
        allergens_in_reduced.insert(allergens, intersection);
    }
    // println!("{:#?}", allergens_in_reduced);

    // Now just iterate over the list by removing from candidates the ones we know for sure.
    while allergens_in_reduced
        .values()
        .any(|candidates| candidates.len() > 1)
    {
        // Get all ingredients we know.
        let ingredients_we_know: Vec<&str> = allergens_in_reduced
            .values()
            .filter_map(|ingredients| {
                if ingredients.len() == 1 {
                    Some(*ingredients.iter().next().unwrap())
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
        .map(|(allergen, ingredients)| (*allergen, *ingredients.iter().next().unwrap()))
        .collect()
}

fn ingredients_without_allergens_count(
    foods: &[Food],
    ingredients_with_allergens: &FxHashMap<&str, &str>,
) -> usize {
    // Extract the list of ingredients that have allergens.
    let ingredients: FxHashSet<&str> = ingredients_with_allergens.values().copied().collect();

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

fn canonical_dangerous_ingredient(ingredients_with_allergens: &FxHashMap<&str, &str>) -> String {
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
