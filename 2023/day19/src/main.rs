use std::{
    collections::HashMap,
    io::{self, Read},
};

use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn new(s: &str) -> Self {
        match s {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!("Invalid category char: {}", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Rule {
    Bigger(Category, u32, String),
    Smaller(Category, u32, String),
    Rejected,
    Accepted,
    Next(String),
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn new(name: String) -> Self {
        Self {
            name,
            rules: Vec::new(),
        }
    }

    fn build_next(s: &str) -> Rule {
        if s == "A" {
            Rule::Accepted
        } else if s == "R" {
            Rule::Rejected
        } else {
            Rule::Next(s.to_string())
        }
    }

    // Next, Accepted or Rejected
    fn find_decision(&self, rating: &Rating) -> Rule {
        for rule in &self.rules {
            match rule {
                Rule::Bigger(category, value, next) => {
                    if rating.get(category) > *value {
                        return Self::build_next(next);
                    }
                }
                Rule::Smaller(category, value, next) => {
                    if rating.get(category) < *value {
                        return Self::build_next(next);
                    }
                }
                Rule::Next(next) => return Rule::Next(next.to_string()),
                Rule::Rejected => return Rule::Rejected,
                Rule::Accepted => return Rule::Accepted,
            }
        }
        panic!("Workflow find_decision got stuck")
    }
}

#[derive(Debug)]
struct Rating {
    // Indexed by Category enum values
    values: [u32; 4],
}

impl Rating {
    fn new() -> Self {
        Self { values: [0; 4] }
    }

    fn set(&mut self, category: Category, value: u32) {
        self.values[category as usize] = value;
    }

    fn get(&self, category: &Category) -> u32 {
        self.values[category.clone() as usize]
    }

    fn sum_ratings(&self) -> u32 {
        self.values.iter().sum()
    }
}

const FIRST_INS: &str = "in";

// Part 1.
fn sum_ratings_all_accepted_parts(
    workflows: &HashMap<String, Workflow>,
    ratings: &[Rating],
) -> u32 {
    ratings
        .iter()
        .map(|rating| {
            let mut ins_name = FIRST_INS.to_string();
            while let Some(workflow) = workflows.get(&ins_name) {
                let decision = workflow.find_decision(rating);
                match decision {
                    Rule::Rejected => return 0,
                    Rule::Accepted => return rating.sum_ratings(),
                    Rule::Next(next) => ins_name = next,
                    _ => panic!("Unsupported decision"),
                };
            }
            panic!("Workflow walking failed")
        })
        .sum()
}

// Needed for part 2.
#[derive(Debug, Clone, PartialEq)]
struct RatingRange {
    // Indexed by Category enum values
    // Pair first value is beginning, second is end. Both are inclusive.
    values: [(u32, u32); 4],
}

impl RatingRange {
    const ZERO: RatingRange = RatingRange {
        values: [(0, 0); 4],
    };

    fn new() -> Self {
        Self {
            values: [(1, 4000); 4],
        }
    }

    fn combinations_count(&self) -> u64 {
        self.values
            .iter()
            .map(|r| u64::from(r.1 - r.0 + 1))
            .product()
    }

    // Split the ranges for this category.
    fn split(
        &self,
        category: &Category,
        limit: u32,
        put_limit_in_higher: bool,
    ) -> (RatingRange, RatingRange) {
        let r = self.values[category.clone() as usize];
        // if limit is on the border itself, there could be an issue
        if limit < r.0 {
            return (Self::ZERO, self.clone());
        } else if r.1 < limit {
            return (self.clone(), Self::ZERO);
        }
        let mut lower = self.clone();
        let mut higher = self.clone();
        lower.values[category.clone() as usize].1 = if put_limit_in_higher {
            limit - 1
        } else {
            limit
        };
        higher.values[category.clone() as usize].0 = if put_limit_in_higher {
            limit
        } else {
            limit + 1
        };
        (lower, higher)
    }
}

fn get_rating_range(
    workflows: &HashMap<String, Workflow>,
    name: &str,
    range: &RatingRange,
) -> Vec<RatingRange> {
    let mut rating_ranges = Vec::new();
    if let Some(workflow) = workflows.get(name) {
        let mut new_range = range.clone();
        for rule in &workflow.rules {
            match rule {
                Rule::Bigger(category, value, next) => {
                    // Once the range is split, one part is used for this token (next match)
                    // and the other part is used for the rest of this workflow line
                    let higher: RatingRange;
                    (new_range, higher) = new_range.split(category, *value, false);

                    if higher != RatingRange::ZERO {
                        match Workflow::build_next(next) {
                            Rule::Rejected => {}
                            Rule::Accepted => {
                                rating_ranges.push(higher.clone());
                            }
                            Rule::Next(_) => {
                                rating_ranges.extend(get_rating_range(workflows, next, &higher));
                            }
                            _ => panic!("Unsupported decision"),
                        }
                    }
                }
                Rule::Smaller(category, value, next) => {
                    let lower: RatingRange;
                    (lower, new_range) = new_range.split(category, *value, true);

                    if lower != RatingRange::ZERO {
                        match Workflow::build_next(next) {
                            Rule::Rejected => {}
                            Rule::Accepted => {
                                rating_ranges.push(lower.clone());
                            }
                            Rule::Next(_) => {
                                rating_ranges.extend(get_rating_range(workflows, next, &lower));
                            }
                            _ => panic!("Unsupported decision"),
                        }
                    }
                }
                Rule::Next(next) => {
                    if new_range != RatingRange::ZERO {
                        rating_ranges.extend(get_rating_range(workflows, next, &new_range));
                    }
                }
                Rule::Rejected => {}
                Rule::Accepted => {
                    if new_range != RatingRange::ZERO {
                        rating_ranges.push(new_range.clone());
                    }
                }
            };
        }
    }
    rating_ranges
}

fn distinct_combinations(workflows: &HashMap<String, Workflow>) -> u64 {
    let rating_ranges = get_rating_range(workflows, FIRST_INS, &RatingRange::new());
    // for r in &rating_ranges {
    //     println!("{:?}", r);
    // }

    rating_ranges
        .iter()
        .map(RatingRange::combinations_count)
        .sum()
}

fn build_workflows_ratings(input: &str) -> (HashMap<String, Workflow>, Vec<Rating>) {
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut ratings: Vec<Rating> = Vec::new();

    // workflows
    // ex{x>10:one,m<20:two,a>30:R,A}
    let workflow_re = Regex::new(r"(\w+)\{(.+)\}").unwrap();
    let instruction_re = Regex::new(r"(\w+)([<>])(\d+):(\w+)").unwrap();
    // ratings
    // {x=787,m=2655,a=1222,s=2876}
    let rating_re = Regex::new(r"([xmas])=(\d+)").unwrap();

    for line in input.lines() {
        if line.starts_with('{') {
            // Ratings
            let mut rating = Rating::new();
            let ratings_str: Vec<&str> = line.trim_end().trim_start().split(',').collect();
            for rating_str in ratings_str {
                let rating_cap = rating_re.captures(rating_str).unwrap();
                rating.set(
                    Category::new(&rating_cap[1]),
                    rating_cap[2].to_string().parse().unwrap(),
                );
            }
            ratings.push(rating);
        } else if line.trim().is_empty() {
            continue;
        } else {
            // Workflows
            let workflow_cap = workflow_re.captures(line).unwrap();
            let (name, instructions_str) = (&workflow_cap[1], &workflow_cap[2]);
            let mut workflow = Workflow::new(name.to_string());

            let instructions_str_list: Vec<&str> = instructions_str.split(',').collect();
            for ins_str in instructions_str_list {
                if let Some(instruction_cap) = instruction_re.captures(ins_str) {
                    let more_or_less = &instruction_cap[2];
                    if more_or_less == ">" {
                        workflow.rules.push(Rule::Bigger(
                            Category::new(&instruction_cap[1]),
                            instruction_cap[3].to_string().parse().unwrap(),
                            instruction_cap[4].to_string(),
                        ));
                    } else if more_or_less == "<" {
                        workflow.rules.push(Rule::Smaller(
                            Category::new(&instruction_cap[1]),
                            instruction_cap[3].to_string().parse().unwrap(),
                            instruction_cap[4].to_string(),
                        ));
                    } else {
                        panic!("Invalid instruction sign: {}", more_or_less);
                    }
                } else if ins_str == "A" {
                    workflow.rules.push(Rule::Accepted);
                } else if ins_str == "R" {
                    workflow.rules.push(Rule::Rejected);
                } else {
                    workflow.rules.push(Rule::Next(ins_str.to_string()));
                }
            }
            workflows.insert(name.to_string(), workflow);
        }
    }
    (workflows, ratings)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (workflows, ratings) = build_workflows_ratings(&input);

    println!(
        "Part 1: {}",
        sum_ratings_all_accepted_parts(&workflows, &ratings)
    );

    println!("Part 2: {}", distinct_combinations(&workflows));
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test");

    #[test]
    fn test_part1_and_2() {
        let (workflows, ratings) = build_workflows_ratings(INPUT_TEST);

        assert_eq!(sum_ratings_all_accepted_parts(&workflows, &ratings), 19114);

        assert_eq!(distinct_combinations(&workflows), 167409079868000);
    }
}
