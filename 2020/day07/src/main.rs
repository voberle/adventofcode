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

fn shiny_gold_bags_count(rule_list: &RuleList) -> usize {
    let shiny_gold_idx = rule_list
        .colors
        .iter()
        .position(|c| c == "shiny gold")
        .unwrap();

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

fn part2(rule_list: &RuleList) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let rule_list = build(&input);
    // println!("{:?}", rule_list);

    println!("Part 1: {}", shiny_gold_bags_count(&rule_list));
    println!("Part 2: {}", part2(&rule_list));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(shiny_gold_bags_count(&build(INPUT_TEST)), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
