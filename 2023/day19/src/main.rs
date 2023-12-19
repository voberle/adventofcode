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
        .sum()
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
