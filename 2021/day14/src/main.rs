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

fn apply_rules(template: &[char], rules: &Rules) -> Vec<char> {
    let mut result: Vec<_> = template
        .windows(2)
        .flat_map(|p| [p[0], *rules.get(&(p[0], p[1])).unwrap()])
        .collect();
    result.push(*template.last().unwrap());
    result
}

fn grow_template(template: &[char], rules: &Rules, steps: usize) -> Vec<char> {
    let mut template = template.to_vec();
    for _ in 0..steps {
        template = apply_rules(&template, rules);
    }
    template
}

fn most_and_least_common_occurences(template: &[char]) -> (usize, usize) {
    let mut freq: FxHashMap<char, usize> = FxHashMap::default();
    for c in template {
        *freq.entry(*c).or_default() += 1;
    }
    match freq.iter().minmax_by_key(|(_, v)| *v) {
        MinMax(min, max) => (*min.1, *max.1),
        _ => panic!("Didn't find most/least common"),
    }
}

fn result_after(template: &[char], rules: &Rules, steps: usize) -> usize {
    let template = grow_template(template, rules, steps);
    let (least, most) = most_and_least_common_occurences(&template);
    most - least
}

fn part2(template: &[char], rules: &Rules, steps: usize) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (template, rules) = build(&input);

    println!("Part 1: {}", result_after(&template, &rules, 10));
    println!("Part 2: {}", part2(&template, &rules, 10));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_grow_template() {
        let (template, rules) = build(INPUT_TEST);
        assert_eq!(
            grow_template(&template, &rules, 1),
            "NCNBCHB".chars().collect_vec()
        );
        assert_eq!(
            grow_template(&template, &rules, 2),
            "NBCCNBBBCBHCB".chars().collect_vec()
        );

        assert_eq!(grow_template(&template, &rules, 5).len(), 97);
        let after10 = grow_template(&template, &rules, 10);
        assert_eq!(after10.len(), 3073);
        assert_eq!(after10.iter().filter(|c| **c == 'B').count(), 1749);
        assert_eq!(after10.iter().filter(|c| **c == 'C').count(), 298);
        assert_eq!(after10.iter().filter(|c| **c == 'H').count(), 161);
        assert_eq!(after10.iter().filter(|c| **c == 'N').count(), 865);
    }

    #[test]
    fn test_part1() {
        let (template, rules) = build(INPUT_TEST);
        assert_eq!(result_after(&template, &rules, 10), 1588);
    }

    #[test]
    fn test_part2() {
        let (template, rules) = build(INPUT_TEST);
        assert_eq!(part2(&template, &rules, 10), 0);
    }
}
