use std::{
    fmt,
    io::{self, Read},
};

use fxhash::FxHashMap;
use itertools::Itertools;

type Chemical = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Material {
    unit: usize,
    chemical: Chemical,
}

impl Material {
    fn new(unit: usize, chemical: &str) -> Self {
        Self {
            unit,
            chemical: chemical.to_string(),
        }
    }
}

impl fmt::Display for Material {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.unit, self.chemical)
    }
}

type Reactions = FxHashMap<Chemical, (usize, Vec<Material>)>;

fn build(input: &str) -> Reactions {
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(" => ").collect();
            let from_parts: Vec<_> = parts[1].split_whitespace().collect();
            let from_amount = from_parts[0].parse().unwrap();
            let from_chemical = from_parts[1].to_string();
            let to: Vec<_> = parts[0]
                .split(", ")
                .map(|s| {
                    let to_parts: Vec<_> = s.split_whitespace().collect();
                    Material::new(to_parts[0].parse().unwrap(), to_parts[1])
                })
                .collect();
            (from_chemical, (from_amount, to))
        })
        .collect()
}

fn min_ore_amount(reactions: &Reactions) -> usize {
    let mut amounts: FxHashMap<String, usize> = FxHashMap::default();
    amounts.insert("FUEL".to_string(), 1);

    convert_amounts(reactions, &mut amounts);

    println!("{:?}", amounts);
    *amounts.get("ORE").unwrap()
}

// Add that material to the amounts.
fn add(chem: &Chemical, count: usize, amounts: &mut FxHashMap<String, usize>) {
    amounts
    .entry(chem.clone())
    .and_modify(|e| *e += count)
    .or_insert(count);
}

// Add the list of needed materials to the amounts, for that amount of material unit.
fn convert_and_add(needed_materials: &[Material], unit_of_material: usize, amounts: &mut FxHashMap<String, usize>) {
    for m in needed_materials {
        let t = m.unit * unit_of_material;
        println!("    Need to produce {} of {}", t, m.chemical);
        amounts
            .entry(m.chemical.clone())
            .and_modify(|e| *e += t)
            .or_insert(t);
    }
}

// Convert as much as possible without any rounding.
fn convert_amounts_exactly(reactions: &Reactions, amounts: &mut FxHashMap<String, usize>) {
    loop {
        let mut new_amounts: FxHashMap<String, usize> = FxHashMap::default();
        let mut something_converted = false;
        println!("----");

        for (chem, count) in &mut *amounts {
            let count = *count;
            println!("{}, {}", chem, count);

            if chem == "ORE" {
                // Special handling for ORE, as it won't be found it the reaction list.
                add(chem, count, &mut new_amounts);
                continue;
            }

            let (prod_count, needed_materials) = reactions.get(chem).unwrap();
            if count % prod_count == 0 {
                // We can convert exactly, so do it.
                let unit_of_material = count / prod_count;
                convert_and_add(needed_materials, unit_of_material, &mut new_amounts);
                something_converted = true;
            } else {
                // Exact conversion not possible, leaving the chemical untouched.
                add(chem, count, &mut new_amounts);
            }
        }

        std::mem::swap(amounts, &mut new_amounts);

        if !something_converted {
            // Once we couldn't convert anything exactly, we stop trying.
            break;
        }
    }
}

fn convert_amounts(reactions: &Reactions, amounts: &mut FxHashMap<String, usize>) {
    loop {
        convert_amounts_exactly(reactions, amounts);

        if amounts.len() == 1 {
            assert!(amounts.contains_key("ORE"));
            break;
        }

        // At this stage, none of the amounts can be converted exactly.
        // So we have to try converting each with rounding, and go until the end
        // with each option to find the minimum.

        println!("- Nothing converted precisely, going extended");
        let mut new_amounts: FxHashMap<String, usize> = FxHashMap::default();
        for (chem, count) in &mut *amounts {
            let count = *count;
            println!("  For {} need at least {}", chem, count);

            if chem == "ORE" {
                add(chem, count, &mut new_amounts);
                continue;
            }

            let (prod_count, needed_materials) = reactions.get(chem).unwrap();
            let unit_of_material = (count as f32 / *prod_count as f32).ceil() as usize;
            println!(
                "    For {} can produce {} with [{}], so need {} unit of material",
                chem,
                prod_count,
                needed_materials.iter().join(", "),
                unit_of_material
            );
            convert_and_add(needed_materials, unit_of_material, &mut new_amounts);
        }

        std::mem::swap(amounts, &mut new_amounts);

        if amounts.len() == 1 {
            assert!(amounts.contains_key("ORE"));
            break;
        }
    }
}

fn part2(reactions: &Reactions) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let reactions = build(&input);
    // println!("{:?}", reactions);

    println!("Part 1: {}", min_ore_amount(&reactions));
    println!("Part 2: {}", part2(&reactions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");
    const INPUT_TEST_3: &str = include_str!("../resources/input_test_3");
    const INPUT_TEST_4: &str = include_str!("../resources/input_test_4");
    const INPUT_TEST_5: &str = include_str!("../resources/input_test_5");

    #[test]
    fn test_min_ore_amount() {
        assert_eq!(min_ore_amount(&build(INPUT_TEST_1)), 31);
        assert_eq!(min_ore_amount(&build(INPUT_TEST_2)), 165);
        assert_eq!(min_ore_amount(&build(INPUT_TEST_3)), 13312);
        assert_eq!(min_ore_amount(&build(INPUT_TEST_4)), 180697);
        assert_eq!(min_ore_amount(&build(INPUT_TEST_5)), 2210736);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
