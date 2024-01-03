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

fn part2(replacements: &[(String, String)], molecule: &str) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (replacements, molecule) = build(&input);

    println!(
        "Part 1: {}",
        distinct_molecules_count(&replacements, &molecule)
    );
    println!("Part 2: {}", part2(&replacements, &molecule));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (replacements, molecule) = build(INPUT_TEST);
        assert_eq!(distinct_molecules_count(&replacements, &molecule), 4);
    }

    #[test]
    fn test_part2() {
        let (replacements, molecule) = build(INPUT_TEST);
        assert_eq!(part2(&replacements, &molecule), 0);
    }
}
