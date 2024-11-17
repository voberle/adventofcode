use std::io::{self, Read};

use fxhash::FxHashSet;
use rand::{seq::SliceRandom, thread_rng};

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
        });
    }
    set.len()
}

fn recursive_min_find(
    dest_to_source: &Vec<(String, String)>,
    input: &str,
    steps_so_far: usize,
) -> Option<usize> {
    for (d, s) in dest_to_source {
        if let Some(idx) = input.find(d) {
            let mut new_mol = input.to_string();
            new_mol.replace_range(idx..idx + d.len(), s);
            if new_mol == "e" {
                return Some(steps_so_far + 1);
            }

            // We just try one more replacement down
            return recursive_min_find(dest_to_source, &new_mol, steps_so_far + 1);
        }
    }
    None
}

fn min_steps_for_medicine(replacements: &[(String, String)], molecule: &str) -> usize {
    let mut dest_to_source_vec: Vec<(String, String)> = replacements
        .iter()
        .map(|(s, d)| (d.to_string(), s.to_string()))
        .collect();
    // That works also:
    // dest_to_source_vec.sort_by_key(|k| k.0.clone());
    // v.reverse();

    // That's ugly but somehow finds the right answer :-(
    // With the input in different order, in some case we get it, in some we don't.
    // But in practice we always get the same one.
    (0..1000)
        .filter_map(|_| {
            dest_to_source_vec.shuffle(&mut thread_rng());
            recursive_min_find(&dest_to_source_vec, molecule, 0)
        })
        // .inspect(|v| println!("Result {v}"))
        .min()
        .unwrap()
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
