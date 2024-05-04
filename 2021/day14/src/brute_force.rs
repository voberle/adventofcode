use fxhash::FxHashMap;
use itertools::{Itertools, MinMaxResult::MinMax};

use crate::Rules;

fn apply_rules(template: &[char], rules: &Rules) -> Vec<char> {
    // There are rules for all possible combinations, an insertion always happens.
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
        // println!("{}", template.iter().join(""));
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

#[allow(dead_code)]
pub fn result_after(template: &[char], rules: &Rules, steps: usize) -> usize {
    let template = grow_template(template, rules, steps);
    let (least, most) = most_and_least_common_occurences(&template);
    most - least
}

#[cfg(test)]
mod tests {
    use crate::build;

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
}
