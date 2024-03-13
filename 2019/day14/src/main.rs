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
    loop {
        let mut new_amounts: FxHashMap<String, usize> = FxHashMap::default();
        println!("----");
        let mut something_converted = false;
        for (chem, count) in &amounts {
            println!("{}, {}", chem, count);
            if chem == "ORE" {
                new_amounts
                    .entry(chem.clone())
                    .and_modify(|e| *e += count)
                    .or_insert(*count);
                continue;
            }

            let (prod_count, needed_materials) = reactions.get(chem).unwrap();
            for m in needed_materials {
                // let t = ((count * m.unit) as f32 / *prod_count as f32).ceil() as usize;
                if count % prod_count == 0 {
                    let t = count / prod_count * m.unit;
                    new_amounts
                        .entry(m.chemical.clone())
                        .and_modify(|e| *e += t)
                        .or_insert(t);
                    something_converted = true;
                } else {
                    let t = *count;
                    new_amounts
                        .entry(chem.clone())
                        .and_modify(|e| *e += t)
                        .or_insert(t);
                }
            }
        }

        if !something_converted {
            println!("- Nothing converted precisely, going extended");
            std::mem::swap(&mut amounts, &mut new_amounts);
            new_amounts = FxHashMap::default();
            for (chem, count) in &amounts {
                println!("  For {} need at least {}", chem, count);
                if chem == "ORE" {
                    new_amounts
                        .entry(chem.clone())
                        .and_modify(|e| *e += count)
                        .or_insert(*count);
                    continue;
                }

                let (prod_count, needed_materials) = reactions.get(chem).unwrap();
                let unit_of_material = (*count as f32 / *prod_count as f32).ceil() as usize;
                println!(
                    "    For {} can produce {} with [{}], so need {} unit of material",
                    chem,
                    prod_count,
                    needed_materials.iter().join(", "),
                    unit_of_material
                );
                for m in needed_materials {
                    let t = m.unit * unit_of_material;
                    println!("    Need to produce {} of {}", t, m.chemical);
                    new_amounts
                        .entry(m.chemical.clone())
                        .and_modify(|e| *e += t)
                        .or_insert(t);
                }
            }
        }
        std::mem::swap(&mut amounts, &mut new_amounts);

        if amounts.len() == 1 {
            break;
        }
    }
    println!("{:?}", amounts);
    *amounts.get("ORE").unwrap()
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
