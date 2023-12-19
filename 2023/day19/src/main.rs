// https://adventofcode.com/2023/day/19

use std::{
    collections::HashMap,
    fmt,
    io::{self, BufRead},
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

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::X => 'x',
                Self::M => 'm',
                Self::A => 'a',
                Self::S => 's',
            }
        )
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

fn sum_ratings_all_accepted_parts(
    workflows: &HashMap<String, Workflow>,
    ratings: &Vec<Rating>,
) -> u32 {
    ratings
        .iter()
        .map(|rating| {
            let mut ins_name = FIRST_INS.to_string();
            while let Some(workflow) = workflows.get(&ins_name) {
                let decision = workflow.find_decision(rating);
                // println!("Name: {}; Workflow: {:?}, Decision: {:?}", ins_name, workflow, decision);
                match decision {
                    Rule::Rejected => return 0,
                    Rule::Accepted => return rating.sum_ratings(),
                    Rule::Next(next) => ins_name = next,
                    _ => panic!("Unsupported decision"),
                };
            }
            panic!("Workflow walking failed")
        })
        // .map(|i| { println!("{}", i); i })
        .sum()
}

#[derive(Debug, Clone, PartialEq)]
struct RatingRange {
    // Indexed by Category enum values
    // Pair first value is beginning, second is end. Both are inclusive.
    values: [(u32, u32); 4],
}


impl RatingRange {
    const ZERO: RatingRange = RatingRange { values: [(0, 0); 4] };

    fn new() -> Self {
        Self { values: [(1, 4000); 4] }
    }

    fn combinations_count(&self) -> u64 {
        println!("Calc count for range X={:?} M={:?} A={:?} S={:?}", self.values[0], self.values[1], self.values[2], self.values[3]);
        self.values.iter().map(|r| {
            assert!(r.1 > r.0);
            (r.1 - r.0 + 1) as u64
        }).product()
    }

    // fn range_counts(&self) -> [u32; 4] {
    //     [
    //         self.values[0].1 - self.values[0].0,
    //         self.values[1].1 - self.values[1].0,
    //         self.values[2].1 - self.values[2].0,
    //         self.values[3].1 - self.values[3].0,
    //     ]
    // }

    // Split the ranges for this category.
    fn split(&self, category: &Category, limit: u32, put_limit_in_higher: bool) -> (RatingRange, RatingRange) {
        let r = self.values[category.clone() as usize];
        // TODO if limit is on the border itself, there could be an issue
        if limit < r.0 {
            return (Self::ZERO, self.clone());
        } else if r.1 < limit {
            return (self.clone(), Self::ZERO);
        }
        let mut lower = self.clone();
        let mut higher = self.clone();
        lower.values[category.clone() as usize].1 = if put_limit_in_higher { limit - 1 } else { limit };
        higher.values[category.clone() as usize].0 = if put_limit_in_higher { limit } else { limit + 1 };
        (lower, higher)
    }

    fn trim_begin(&mut self, category: &Category, limit: u32) {
        let r = self.values[category.clone() as usize];
        if r.0 < limit && limit < r.1 {
            self.values[category.clone() as usize].0 = limit;
        }
    }

    fn trim_end(&mut self, category: &Category, limit: u32) {
        let r = self.values[category.clone() as usize];
        if r.0 < limit && limit < r.1 {
            self.values[category.clone() as usize].1 = limit;
        }
    }
}

// fn add_range_counts(a: &mut [u32; 4], b: &[u32; 4]) {
//     a[0] = a[0] + b[0];
//     a[1] = a[1] + b[1];
//     a[2] = a[2] + b[2];
//     a[3] = a[3] + b[3];
// }

// const ZERO_RANGE_COUNT: [u32; 4] = [0; 4];

fn get_rating_range(workflows: &HashMap<String, Workflow>, name: &str, range: &RatingRange) -> Vec<RatingRange> {
    let mut rating_ranges = Vec::new();
    if let Some(workflow) = workflows.get(name) {
        let mut new_range = range.clone();
        println!("{}: {:?}", name, new_range);
        for rule in &workflow.rules {
            match rule {
                Rule::Bigger(category, value, next) => {
                    let higher: RatingRange;
                    (new_range, higher) = new_range.split(category, *value, false);
                    println!("{}: after {}>{}: {:?} {:?}", name, category, value, new_range, higher);

                    match Workflow::build_next(next) {
                        Rule::Rejected => {},
                        Rule::Accepted => {
                            if higher != RatingRange::ZERO {
                                println!("{}: > ADD: {:?}", name, higher);
                                rating_ranges.push(higher.clone());
                            }
                        },
                        Rule::Next(_) => {
                            if higher != RatingRange::ZERO {
                                rating_ranges.extend(
                                    get_rating_range(workflows, next, &higher)
                                );
                            }
                        },
                        _ => panic!("Unsupported decision"),
                    }
                }
                Rule::Smaller(category, value, next) => {
                    let lower: RatingRange;
                    (lower, new_range) = new_range.split(category, *value, true);
                    println!("{}: after {}<{}: {:?} {:?}", name, category, value, lower, new_range);

                    match Workflow::build_next(next) {
                        Rule::Rejected => {},
                        Rule::Accepted => {
                            if lower != RatingRange::ZERO {
                                println!("{}: < ADD: {:?}", name, lower);
                                rating_ranges.push(lower.clone());
                            }
                        },
                        Rule::Next(_) => {
                            if lower != RatingRange::ZERO {
                                rating_ranges.extend(
                                    get_rating_range(workflows, next, &lower)
                                );
                            }
                        },
                        _ => panic!("Unsupported decision"),
                    }
                }
                Rule::Next(next) => {
                    if new_range != RatingRange::ZERO {
                        rating_ranges.extend(
                            get_rating_range(workflows, next, &new_range)
                        );
                    }
                },
                Rule::Rejected => {},
                Rule::Accepted => {
                    if new_range != RatingRange::ZERO {
                        rating_ranges.push(new_range.clone());
                        println!("{}: Solo ADD: {:?}", name, new_range);
                    }
                },
            };
        }
    }
    rating_ranges
}

fn distinct_combinations(workflows: &HashMap<String, Workflow>) -> u64 {
    let initial_range = RatingRange::new();
    let rating_ranges = get_rating_range(workflows, FIRST_INS, &initial_range);
    // result_range.combinations_count()

    for r in &rating_ranges {
        println!("{:?}", r);
        // let c = r.combinations_count();
    }

    let mut comb_counts: Vec<u64> = rating_ranges.iter().map(RatingRange::combinations_count).collect();
    comb_counts.sort();
    comb_counts.reverse();

    println!("{:?}", comb_counts);

    comb_counts.iter().sum()
    // comb_counts.iter().skip(1).fold(comb_counts[0], |acc, c| acc + (c - acc))
    // 0
    // range_count.iter().map(|c| *c as u64).product()
}

fn build_workflows_ratings<R>(reader: &mut R) -> (HashMap<String, Workflow>, Vec<Rating>)
where
    R: BufRead,
{
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut ratings: Vec<Rating> = Vec::new();

    // workflows
    // ex{x>10:one,m<20:two,a>30:R,A}
    let workflow_re = Regex::new(r"(\w+)\{(.+)\}").unwrap();
    let instruction_re = Regex::new(r"(\w+)([<>])(\d+):(\w+)").unwrap();
    let rating_re = Regex::new(r"([xmas])=(\d+)").unwrap();

    // ratings
    // {x=787,m=2655,a=1222,s=2876}

    for l in reader.lines() {
        let line = l.unwrap();
        if line.starts_with("{") {
            // Ratings
            let mut rating = Rating::new();
            let ratings_str: Vec<&str> = line.trim_end().trim_start().split(",").collect();
            for rating_str in ratings_str {
                let rating_cap = rating_re.captures(&rating_str).unwrap();
                // println!("1={}, 2={}", &rating_cap[1], &rating_cap[2]);
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
            // println!("line='{}'", line);
            let workflow_cap = workflow_re.captures(&line).unwrap();
            let (name, instructions_str) = (&workflow_cap[1], &workflow_cap[2]);
            // println!("name:{name} => instructions_str={instructions_str}");
            let mut workflow = Workflow::new(name.to_string());

            let instructions_str_list: Vec<&str> = instructions_str.split(",").collect();
            for ins_str in instructions_str_list {
                if let Some(instruction_cap) = instruction_re.captures(ins_str) {
                    // println!("1={}, 2={}, 3={}, 4={}", &instruction_cap[1], &instruction_cap[2], &instruction_cap[3], &instruction_cap[4]);
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
                } else {
                    // println!("F={}", ins_str);
                    if ins_str == "A" {
                        workflow.rules.push(Rule::Accepted);
                    } else if ins_str == "R" {
                        workflow.rules.push(Rule::Rejected);
                    } else {
                        workflow.rules.push(Rule::Next(ins_str.to_string()));
                    }
                }
            }
            // println!("Workflow: {:?}", workflow);
            workflows.insert(name.to_string(), workflow);
        }
    }

    // println!("Workflows: {:?}", workflows);
    // println!("Ratings: {:?}", ratings);
    (workflows, ratings)
}

fn main() {
    let stdin = io::stdin();
    let (workflows, ratings) = build_workflows_ratings(&mut stdin.lock());

    println!(
        "Part 1: {}",
        sum_ratings_all_accepted_parts(&workflows, &ratings)
    );

    println!("Part 2: {}", distinct_combinations(&workflows));
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_part1_and_2() {
        let mut reader = BufReader::new(File::open("resources/input_test").unwrap());
        let (workflows, ratings) = build_workflows_ratings(&mut reader);

        assert_eq!(sum_ratings_all_accepted_parts(&workflows, &ratings), 19114);
    }
}
