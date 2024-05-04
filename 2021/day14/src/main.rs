mod brute_force;

use std::io::{self, Read};

use fxhash::FxHashMap;
use itertools::{Itertools, MinMaxResult::MinMax};

type Rules = FxHashMap<(char, char), char>;

fn build(input: &str) -> (Vec<char>, Rules) {
    let template = input.lines().next().unwrap().chars().collect();
    let rules = input
        .lines()
        .skip(2)
        .map(|line| {
            let (f, t) = line.split_once(" -> ").unwrap();
            (
                f.chars().collect_tuple().unwrap(),
                t.chars().next().unwrap(),
            )
        })
        .collect();
    (template, rules)
}

fn result_after(template: &[char], rules: &Rules, steps: usize) -> usize {
    // Divide the template into pairs
    let mut pairs: FxHashMap<(char, char), usize> = FxHashMap::default();
    for pair in template.windows(2).map(|p| (p[0], p[1])) {
        pairs.entry(pair).and_modify(|e| *e += 1).or_insert(1);
    }

    for _ in 0..steps {
        // Replace each pair with two pairs formed with the middle.
        let mut new_pairs: FxHashMap<(char, char), usize> = FxHashMap::default();
        for (pair, count) in &pairs {
            let middle = *rules.get(pair).unwrap();
            new_pairs
                .entry((pair.0, middle))
                .and_modify(|e| *e += count)
                .or_insert(*count);
            new_pairs
                .entry((middle, pair.1))
                .and_modify(|e| *e += count)
                .or_insert(*count);
        }

        std::mem::swap(&mut pairs, &mut new_pairs);
    }

    // Count the frequencies of each char in the pairs.
    let mut freq: FxHashMap<char, usize> = FxHashMap::default();
    for ((a, b), count) in pairs {
        *freq.entry(a).or_default() += count;
        *freq.entry(b).or_default() += count;
    }

    // The actual frequency of the characters is the frequency in the pairs divided by 2,
    // plus 1 for the char at the end.
    match freq.values().map(|c| c / 2 + c % 2).minmax() {
        MinMax(least, most) => most - least,
        _ => panic!("Didn't find most/least common"),
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (template, rules) = build(&input);

    println!("Part 1: {}", result_after(&template, &rules, 10));
    println!("Part 2: {}", result_after(&template, &rules, 40));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (template, rules) = build(INPUT_TEST);
        assert_eq!(result_after(&template, &rules, 10), 1588);
    }

    #[test]
    fn test_part2() {
        let (template, rules) = build(INPUT_TEST);
        assert_eq!(result_after(&template, &rules, 40), 2188189693529);
    }
}
