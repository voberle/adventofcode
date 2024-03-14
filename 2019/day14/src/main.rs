use fxhash::FxHashMap;
use std::{
    fmt,
    io::{self, Read},
};

type Chemical = String;

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

// We store the reactions into a map
// "element created" => "number of elements created" and "elements needed"
type Reactions = FxHashMap<Chemical, (usize, Vec<Material>)>;

fn build(input: &str) -> Reactions {
    let mut line_count = 0;
    let reactions: Reactions = input
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
            line_count += 1;
            (from_chemical, (from_amount, to))
        })
        .collect();
    // Check that each chemical is produced by only one reaction.
    assert_eq!(reactions.len(), line_count);
    reactions
}

// Add that material to the amounts.
fn add(chem: &Chemical, count: usize, amounts: &mut FxHashMap<Chemical, usize>) {
    amounts
        .entry(chem.clone())
        .and_modify(|e| *e += count)
        .or_insert(count);
}

#[allow(
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation
)]
fn div_round_up(a: usize, b: usize) -> usize {
    (a as f64 / b as f64).ceil() as usize
}

fn min_ore_amount(reactions: &Reactions, fuel_amount: usize) -> usize {
    // Since each chemical can only be produced by one reaction,
    // if we need a certain chemical, it's sure we will have to produce that reaction.
    // If we have leftovers from the reaction, we park them and possibly use them later.

    // Chemicals and their quantities we need.
    let mut needed: FxHashMap<Chemical, usize> = FxHashMap::default();

    // What we have produced already and may reuse.
    let mut available: FxHashMap<Chemical, usize> = FxHashMap::default();

    needed.insert("FUEL".to_string(), fuel_amount);

    loop {
        // Use a second map for putting needed stuff on each loop, as it's not practical
        // to iterator and modify the same map.
        // We could use vectors as well, especially if we would replace
        // the chemical strings with just integers, but it's fast enough as it is.
        let mut next_needed: FxHashMap<Chemical, usize> = FxHashMap::default();

        for (chem, amount) in &needed {
            // How much of this chemical do we need
            let mut to_produce = *amount;

            // If we have some available from a previous reaction, use it.
            if let Some(avail_amount) = available.get(chem) {
                assert!(*avail_amount > 0);
                if *avail_amount <= to_produce {
                    // Use all saved amount.
                    to_produce -= avail_amount;
                    available.remove(chem);
                } else {
                    // Use some of the saved amount only.
                    available.insert(chem.clone(), avail_amount - to_produce);
                    to_produce = 0;
                }

                if to_produce == 0 {
                    // There was enough already, we don't need to produce anything new.
                    continue;
                }
            }

            // ORE cannot be converted anymore, just skipping it.
            if chem == "ORE" {
                add(chem, *amount, &mut next_needed);
                continue;
            }

            // Find the reaction needed for conversion.
            let (reaction_prod_count, reaction_needed_materials) = reactions.get(chem).unwrap();

            // If reaction produces reaction_prod_count, but we need amount,
            // find how many times we need to run the reaction
            let react_times = div_round_up(to_produce, *reaction_prod_count);

            for src in reaction_needed_materials {
                // Split the quantity produced into what we need and what we have extra.
                let min_to_produce = src.unit * react_times;
                let leftover = reaction_prod_count * react_times - to_produce;

                add(&src.chemical, min_to_produce, &mut next_needed);

                if leftover > 0 {
                    // we can just insert, we had no previous leftover at this stage
                    available.insert(chem.clone(), leftover);
                }
            }
        }

        std::mem::swap(&mut needed, &mut next_needed);

        if needed.len() == 1 {
            let amount_ore = needed.get("ORE").expect("No ORE in needed list");
            return *amount_ore;
        }
    }
}

fn fuel_for_trillion_ore(reactions: &Reactions) -> usize {
    const ONE_TRILLION: usize = 1_000_000_000_000;

    // Producing FUEL for one trillion ORE will result in a number of fuel
    // bigger than with 1 ORE. Since min_ore_amount is quite fast, a binary search approach works.

    // Initial low and high FUEL amounts.
    let mut low_fuel = ONE_TRILLION / min_ore_amount(reactions, 1);
    let mut high_fuel = 6 * low_fuel;

    // Check that we picked low and high correctly.
    assert!(min_ore_amount(reactions, low_fuel) < ONE_TRILLION);
    assert!(min_ore_amount(reactions, high_fuel) > ONE_TRILLION);

    // We search when ORE amount is closest to one trillion.
    while low_fuel <= high_fuel {
        let mid_fuel = ((high_fuel - low_fuel) / 2) + low_fuel;
        let val = min_ore_amount(reactions, mid_fuel);

        if val == ONE_TRILLION {
            return mid_fuel;
        }
        if val < ONE_TRILLION {
            low_fuel = mid_fuel + 1;
        }
        if val > ONE_TRILLION {
            high_fuel = mid_fuel - 1;
        }
    }

    assert!(min_ore_amount(reactions, high_fuel) < ONE_TRILLION);
    assert!(min_ore_amount(reactions, high_fuel + 1) > ONE_TRILLION);

    high_fuel
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let reactions = build(&input);

    println!("Part 1: {}", min_ore_amount(&reactions, 1));
    println!("Part 2: {}", fuel_for_trillion_ore(&reactions));
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
        assert_eq!(min_ore_amount(&build(INPUT_TEST_1), 1), 31);
        assert_eq!(min_ore_amount(&build(INPUT_TEST_2), 1), 165);
        assert_eq!(min_ore_amount(&build(INPUT_TEST_3), 1), 13312);
        assert_eq!(min_ore_amount(&build(INPUT_TEST_4), 1), 180697);
        assert_eq!(min_ore_amount(&build(INPUT_TEST_5), 1), 2210736);
    }

    #[test]
    fn test_fuel_for_trillion_ore() {
        assert_eq!(fuel_for_trillion_ore(&build(INPUT_TEST_3)), 82892753);
        assert_eq!(fuel_for_trillion_ore(&build(INPUT_TEST_4)), 5586022);
        assert_eq!(fuel_for_trillion_ore(&build(INPUT_TEST_5)), 460664);
    }
}
