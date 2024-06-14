use std::{
    fmt::Display,
    io::{self, Read},
};

use fxhash::FxHashMap;
use itertools::Itertools;

// Mapping of monkey names to their values.
type MonkeyMap = FxHashMap<String, u64>;

#[derive(Debug, Clone)]
enum Element {
    Monkey(String),
    Number(u64),
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Monkey(m) => write!(f, "{}", m),
            Element::Number(n) => write!(f, "{}", n),
        }
    }
}

impl Element {
    fn get_monkey(&self) -> String {
        match self {
            Element::Monkey(m) => m.to_string(),
            Element::Number(_) => panic!("Not a monkey"),
        }
    }

    // Returns the value from this element, either directly if it's a value, or from
    // monkeys list if it's there.
    fn get_val(&self, monkeys: &MonkeyMap) -> Option<u64> {
        match self {
            Element::Monkey(n) => monkeys.get(n).copied(),
            Element::Number(n) => Some(*n),
        }
    }

    // Returns this element with monkey replaced with value, if possible.
    fn get_elt(&self, monkeys: &MonkeyMap) -> Element {
        if let Some(v1) = self.get_val(monkeys) {
            Element::Number(v1)
        } else {
            self.clone()
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Value(Element),
    Addition(Element, Element),
    Subtraction(Element, Element),
    Multiplication(Element, Element),
    Division(Element, Element),
}

impl From<&str> for Operation {
    #[allow(clippy::match_on_vec_items)]
    fn from(value: &str) -> Self {
        if let Ok(n) = value.parse() {
            Operation::Value(Element::Number(n))
        } else {
            let p: Vec<_> = value.split_ascii_whitespace().collect();
            let m1 = Element::Monkey(p[0].to_string());
            let m2 = Element::Monkey(p[2].to_string());
            match p[1] {
                "+" => Operation::Addition(m1, m2),
                "-" => Operation::Subtraction(m1, m2),
                "*" => Operation::Multiplication(m1, m2),
                "/" => Operation::Division(m1, m2),
                _ => panic!("Unknown operation"),
            }
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Value(v) => write!(f, "{}", v),
            Operation::Addition(m1, m2) => write!(f, "{} + {}", m1, m2),
            Operation::Subtraction(m1, m2) => write!(f, "{} - {}", m1, m2),
            Operation::Multiplication(m1, m2) => write!(f, "{} * {}", m1, m2),
            Operation::Division(m1, m2) => write!(f, "{} / {}", m1, m2),
        }
    }
}

impl Operation {
    // Calculates the operation if possible.
    fn calculate(&self, monkeys: &MonkeyMap) -> Option<u64> {
        match self {
            Operation::Value(v) => match v {
                Element::Number(n) => Some(*n),
                Element::Monkey(_) => None,
            },
            Operation::Addition(m1, m2) => {
                if let (Some(v1), Some(v2)) = (m1.get_val(monkeys), m2.get_val(monkeys)) {
                    Some(v1 + v2)
                } else {
                    None
                }
            }
            Operation::Subtraction(m1, m2) => {
                if let (Some(v1), Some(v2)) = (m1.get_val(monkeys), m2.get_val(monkeys)) {
                    Some(v1 - v2)
                } else {
                    None
                }
            }
            Operation::Multiplication(m1, m2) => {
                if let (Some(v1), Some(v2)) = (m1.get_val(monkeys), m2.get_val(monkeys)) {
                    Some(v1 * v2)
                } else {
                    None
                }
            }
            Operation::Division(m1, m2) => {
                if let (Some(v1), Some(v2)) = (m1.get_val(monkeys), m2.get_val(monkeys)) {
                    Some(v1 / v2)
                } else {
                    None
                }
            }
        }
    }

    // Replaces the monkeys with the values.
    fn replace(&self, monkeys: &MonkeyMap) -> Self {
        match self {
            Operation::Value(v) => Operation::Value(v.clone()),
            Operation::Addition(m1, m2) => {
                Operation::Addition(m1.get_elt(monkeys), m2.get_elt(monkeys))
            }
            Operation::Subtraction(m1, m2) => {
                Operation::Subtraction(m1.get_elt(monkeys), m2.get_elt(monkeys))
            }
            Operation::Multiplication(m1, m2) => {
                Operation::Multiplication(m1.get_elt(monkeys), m2.get_elt(monkeys))
            }
            Operation::Division(m1, m2) => {
                Operation::Division(m1.get_elt(monkeys), m2.get_elt(monkeys))
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Equation {
    monkey: Element,
    operation: Operation,
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let (m, op) = value.split(": ").collect_tuple().unwrap();
        Equation {
            monkey: Element::Monkey(m.to_string()),
            operation: op.into(),
        }
    }
}

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.monkey, self.operation)
    }
}

impl Equation {
    // Replaces the monkeys with the values.
    fn replace(&self, monkeys: &MonkeyMap) -> Self {
        let monkey = self.monkey.get_elt(monkeys);
        let operation = self.operation.replace(monkeys);
        Self { monkey, operation }
    }

    // Tries to solve the equation.
    fn solve(&self, monkeys: &mut MonkeyMap) -> bool {
        // Try to calculate the operation.
        if let Some(op_result) = self.operation.calculate(monkeys) {
            monkeys.insert(self.monkey.get_monkey(), op_result);
            return true;
        }

        // If the monkey is a number, and one of the operation element is a number too,
        // we can solve the equation as well.
        if let Element::Number(monkey_number) = self.monkey {
            match &self.operation {
                Operation::Value(_) => {}
                Operation::Addition(e1, e2) => {
                    if let Some(v) = e1.get_val(monkeys) {
                        monkeys.insert(e2.get_monkey(), monkey_number - v);
                        return true;
                    }
                    if let Some(v) = e2.get_val(monkeys) {
                        monkeys.insert(e1.get_monkey(), monkey_number - v);
                        return true;
                    }
                }
                Operation::Subtraction(e1, e2) => {
                    if let Some(v) = e1.get_val(monkeys) {
                        monkeys.insert(e2.get_monkey(), v - monkey_number);
                        return true;
                    }
                    if let Some(v) = e2.get_val(monkeys) {
                        monkeys.insert(e1.get_monkey(), monkey_number + v);
                        return true;
                    }
                }
                Operation::Multiplication(e1, e2) => {
                    if let Some(v) = e1.get_val(monkeys) {
                        monkeys.insert(e2.get_monkey(), monkey_number / v);
                        return true;
                    }
                    if let Some(v) = e2.get_val(monkeys) {
                        monkeys.insert(e1.get_monkey(), monkey_number / v);
                        return true;
                    }
                }
                Operation::Division(e1, e2) => {
                    if let Some(v) = e1.get_val(monkeys) {
                        monkeys.insert(e2.get_monkey(), v / monkey_number);
                        return true;
                    }
                    if let Some(v) = e2.get_val(monkeys) {
                        monkeys.insert(e1.get_monkey(), monkey_number * v);
                        return true;
                    }
                }
            }
        }

        false
    }
}

fn build(input: &str) -> Vec<Equation> {
    input.lines().map(Into::into).collect()
}

#[allow(dead_code)]
fn print_equations(equations: &[Equation]) {
    for eq in equations {
        println!("{}", eq);
    }
}

// Go through all equations once and try to solve them.
// Solved ones are removed from the list, and monkey values are added to the monkey map.
fn solve(monkeys: &mut MonkeyMap, equations: &mut Vec<Equation>) {
    let mut next_set: Vec<Equation> = Vec::new();
    while let Some(eq) = equations.pop() {
        if !eq.solve(monkeys) {
            next_set.push(eq);
        }
    }
    std::mem::swap(equations, &mut next_set);
}

// Solve the equations as much as possible.
fn solve_max(monkeys: &mut MonkeyMap, equations: &mut Vec<Equation>) {
    let mut cnt = equations.len();
    while cnt > 0 {
        solve(monkeys, equations);
        if equations.len() == cnt {
            break;
        }
        cnt = equations.len();
    }
}

// Replace known monkeys in the equations.
fn replace(monkeys: &mut MonkeyMap, equations: &mut [Equation]) {
    for eq in equations.iter_mut() {
        *eq = eq.replace(monkeys);
    }
}

fn root_number(equations: &[Equation]) -> u64 {
    let mut monkeys: MonkeyMap = FxHashMap::default();
    let mut equations: Vec<Equation> = equations.to_vec();

    solve_max(&mut monkeys, &mut equations);

    *monkeys.get("root").expect("Didn't find root")
}

// Remove humn entry.
fn remove_human_entry(equations: &mut Vec<Equation>) {
    let humn_pos = equations
        .iter()
        .position(|eq| eq.monkey.get_monkey() == "humn")
        .unwrap();
    equations.remove(humn_pos);
}

// Find the root operation. One of its part should be a value.
// We remove the root operation from the list of equations, and add the value to the monkey map.
fn extract_root_operation(monkeys: &mut MonkeyMap, equations: &mut Vec<Equation>) {
    // Find the root operation.
    let root_pos = equations
        .iter()
        .position(|eq| eq.monkey.get_monkey() == "root")
        .unwrap();
    let root_op = equations.remove(root_pos).operation;

    // Extract the value and monkey from it.
    let (val, name) = match root_op {
        Operation::Value(_) => panic!("Root shouldn't be a value"),
        Operation::Addition(m1, m2)
        | Operation::Subtraction(m1, m2)
        | Operation::Multiplication(m1, m2)
        | Operation::Division(m1, m2) => match (m1, m2) {
            (Element::Monkey(mo1), Element::Number(n2)) => (n2, mo1),
            (Element::Number(n1), Element::Monkey(mo2)) => (n1, mo2),
            _ => panic!("Cannot extract this root operation"),
        },
    };

    monkeys.insert(name, val);
}

fn number_passing_eq_test(equations: &[Equation]) -> u64 {
    let mut monkeys: MonkeyMap = FxHashMap::default();
    let mut equations: Vec<Equation> = equations.to_vec();

    remove_human_entry(&mut equations);

    // Simplify as much as possible.
    solve_max(&mut monkeys, &mut equations);
    replace(&mut monkeys, &mut equations);

    extract_root_operation(&mut monkeys, &mut equations);

    // Solve and replace until we found the human value.
    while !monkeys.contains_key("humn") {
        replace(&mut monkeys, &mut equations);
        solve(&mut monkeys, &mut equations);
    }

    *monkeys.get("humn").expect("Didn't find human")
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let operations = build(&input);

    println!("Part 1: {}", root_number(&operations));
    println!("Part 2: {}", number_passing_eq_test(&operations));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(root_number(&build(INPUT_TEST)), 152);
    }

    #[test]
    fn test_part2() {
        assert_eq!(number_passing_eq_test(&build(INPUT_TEST)), 301);
    }
}
