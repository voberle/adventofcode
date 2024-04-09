use std::io::{self, Read};

use regex::Regex;

type ColorId = usize;

#[derive(Debug)]
struct Rule {
    outer_bag: ColorId,
    inner_bags: Vec<(usize, ColorId)>,
}

#[derive(Debug)]
struct RuleList {
    colors: Vec<String>,
    rules: Vec<Rule>,
}

fn get_color_id(colors: &mut Vec<String>, col: &str) -> usize {
    colors.iter().position(|c| c == col).unwrap_or_else(|| {
        colors.push(col.to_string());
        colors.len() - 1
    })
}

fn build(input: &str) -> RuleList {
    let re = Regex::new(r"(\d+) (.+) bag(s?)").unwrap();
    let mut colors: Vec<String> = Vec::new();
    let rules = input
        .lines()
        .map(|line| {
            let p: Vec<_> = line.split(" bags contain ").collect();
            let outer_bag = get_color_id(&mut colors, p[0]);
            let inner_bags: Vec<_> = p[1]
                .trim_end_matches('.')
                .split(", ")
                .filter_map(|i| {
                    re.captures(i).map(|ip| {
                        let inner_color = get_color_id(&mut colors, &ip[2]);
                        (ip[1].parse().unwrap(), inner_color)
                    })
                })
                .collect();
            Rule {
                outer_bag,
                inner_bags,
            }
        })
        .collect();
    RuleList { colors, rules }
}

fn get_shiny_gold_index(rule_list: &RuleList) -> usize {
    rule_list
        .colors
        .iter()
        .position(|c| c == "shiny gold")
        .unwrap()
}

fn bags_containing_shiny_gold(rule_list: &RuleList) -> usize {
    let shiny_gold_idx = get_shiny_gold_index(rule_list);

    let mut bags_containing_gold = vec![false; rule_list.colors.len()];
    bags_containing_gold[shiny_gold_idx] = true;

    loop {
        let mut something_changed = false;
        for rule in &rule_list.rules {
            if rule
                .inner_bags
                .iter()
                .any(|(_, color)| bags_containing_gold[*color])
            {
                if !bags_containing_gold[rule.outer_bag] {
                    something_changed = true;
                }
                bags_containing_gold[rule.outer_bag] = true;
            }
        }
        if !something_changed {
            break;
        }
    }

    // -1 as we want to exclude the gold bag itself.
    bags_containing_gold.iter().filter(|v| **v).count() - 1
}

fn bags_inside_shiny_gold(rule_list: &RuleList) -> usize {
    let shiny_gold_idx = get_shiny_gold_index(rule_list);

    // List of bags to add on each round.
    let mut bags_to_add: Vec<usize> = vec![0; rule_list.colors.len()];
    bags_to_add[shiny_gold_idx] = 1;

    let mut total_bags = 0;
    loop {
        let mut add_this_round: Vec<usize> = vec![0; rule_list.colors.len()];

        for rule in &rule_list.rules {
            if bags_to_add[rule.outer_bag] > 0 {
                for (count, color_idx) in &rule.inner_bags {
                    add_this_round[*color_idx] += bags_to_add[rule.outer_bag] * count;
                }
            }
        }

        let add_this_round_count: usize = add_this_round.iter().sum();
        if add_this_round_count == 0 {
            break;
        }

        total_bags += add_this_round_count;
        std::mem::swap(&mut bags_to_add, &mut add_this_round);
    }
    total_bags
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let rule_list = build(&input);

    println!("Part 1: {}", bags_containing_shiny_gold(&rule_list));
    println!("Part 2: {}", bags_inside_shiny_gold(&rule_list));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part1() {
        assert_eq!(bags_containing_shiny_gold(&build(INPUT_TEST_1)), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(bags_inside_shiny_gold(&build(INPUT_TEST_1)), 32);
        assert_eq!(bags_inside_shiny_gold(&build(INPUT_TEST_2)), 126);
    }
}
