use std::io::{self, Read};

use fxhash::FxHashSet;

fn build(input: &str) -> (Vec<(String, String)>, String) {
    let mut replacements = Vec::new();
    let mut molecule = String::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<_> = line.split(" => ").collect();
        if parts.len() == 2 {
            replacements.push((parts[0].to_string(), parts[1].to_string()));
        } else {
            molecule = line.to_string();
            break;
        }
    }
    (replacements, molecule)
}

fn distinct_molecules_count(replacements: &[(String, String)], molecule: &str) -> usize {
    let mut set: FxHashSet<String> = FxHashSet::default();
    for r in replacements {
        let source = &r.0;
        let dest = &r.1;
        molecule.match_indices(source).for_each(|(idx, _)| {
            let mut new_mol = molecule.to_string();
            new_mol.replace_range(idx..idx + source.len(), dest);
            set.insert(new_mol);
        })
    }
    set.len()
}

fn possible_replacements(replacements: &[(String, String)], elt: &str) -> Vec<String> {
    replacements
        .iter()
        .filter(|(s, _)| s == elt)
        .map(|(_, d)| d)
        .cloned()
        .collect()
}

fn min_steps_for_medicine(replacements: &[(String, String)], molecule: &str) -> usize {
    let mut steps = 0;
    // This vector contains all molecules generated on each step
    let mut generated = vec!["e".to_string()];
    loop {
        steps += 1;
        let mut set: FxHashSet<String> = FxHashSet::default();
        generated.iter().for_each(|g| {
            for r in replacements {
                let source = &r.0;
                let dest = &r.1;
                g.match_indices(source).for_each(|(idx, _)| {
                    let mut new_mol = g.to_string();
                    new_mol.replace_range(idx..idx + source.len(), dest);
                    set.insert(new_mol);
                })
            }
        });
        println!("{steps}: {:?}", set);
        // println!("{steps}");
        if set.contains(molecule) {
            break;
        }
        generated = set.into_iter().collect();
    }
    steps
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (replacements, molecule) = build(&input);

    println!(
        "Part 1: {}",
        distinct_molecules_count(&replacements, &molecule)
    );
    println!(
        "Part 2: {}",
        min_steps_for_medicine(&replacements, &molecule)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (replacements, molecule) = build(INPUT_TEST_1);
        assert_eq!(distinct_molecules_count(&replacements, &molecule), 4);
    }

    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");
    const INPUT_TEST_3: &str = include_str!("../resources/input_test_3");

    #[test]
    fn test_part2_1() {
        let (replacements, molecule) = build(INPUT_TEST_2);
        assert_eq!(min_steps_for_medicine(&replacements, &molecule), 3);
    }

    #[test]
    fn test_part2_2() {
        let (replacements, molecule) = build(INPUT_TEST_3);
        assert_eq!(min_steps_for_medicine(&replacements, &molecule), 6);
    }
}
