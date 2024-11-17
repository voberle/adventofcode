use std::io::{self, Read};

use fxhash::FxHashMap;
use regex::Regex;

#[derive(Debug)]
struct Instruction {
    reg: String,
    inc: bool, // true to increase, false to decrease
    amount: i32,
    cond_reg: String,
    cond: fn(i32, i32) -> bool,
    cond_val: i32,
}

fn get_cond_fn(cond: &str) -> fn(i32, i32) -> bool {
    match cond {
        "<" => |a, b| a < b,
        "<=" => |a, b| a <= b,
        ">" => |a, b| a > b,
        ">=" => |a, b| a >= b,
        "==" => |a, b| a == b,
        "!=" => |a, b| a != b,
        _ => panic!("Invalid condition {cond}"),
    }
}

fn build(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"(\w+) (inc|dec) (\-?\d+) if (\w+) (<|<=|>|>=|==|!=) (\-?\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let parts = re.captures(line).unwrap();
            Instruction {
                reg: parts[1].to_string(),
                inc: if &parts[2] == "inc" {
                    true
                } else if &parts[2] == "dec" {
                    false
                } else {
                    panic!("Invalid input")
                },
                amount: parts[3].parse().unwrap(),
                cond_reg: parts[4].to_string(),
                cond: get_cond_fn(&parts[5]),
                cond_val: parts[6].parse().unwrap(),
            }
        })
        .collect()
}

fn largest_value_any_register(instructions: &[Instruction]) -> (i32, i32) {
    let mut registers: FxHashMap<String, i32> = FxHashMap::default();
    let mut ir = 0;
    let mut largest_val = 0;
    while let Some(ins) = instructions.get(ir) {
        let cond_reg_val = registers.get(&ins.cond_reg).copied().unwrap_or_default();
        if (ins.cond)(cond_reg_val, ins.cond_val) {
            let reg = registers.entry(ins.reg.clone()).or_default();
            if ins.inc {
                *reg += ins.amount;
            } else {
                *reg -= ins.amount;
            }
            if *reg > largest_val {
                largest_val = *reg;
            }
        }
        ir += 1;
    }
    (*registers.values().max().unwrap(), largest_val)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);
    // println!("{:#?}", instructions);

    println!("Part 1: {}", largest_value_any_register(&instructions).0);
    println!("Part 2: {}", largest_value_any_register(&instructions).1);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(largest_value_any_register(&build(INPUT_TEST)).0, 1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(largest_value_any_register(&build(INPUT_TEST)).1, 10);
    }
}
