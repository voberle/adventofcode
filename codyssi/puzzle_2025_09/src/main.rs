use std::{
    collections::HashMap,
    io::{self, Read},
};

struct Transaction {
    from: String,
    to: String,
    amount: i32,
}

impl From<&str> for Transaction {
    fn from(value: &str) -> Self {
        let parts: Vec<_> = value.split(" AMT ").collect();
        let sub_parts: Vec<_> = parts[0].trim_start_matches("FROM ").split(" TO ").collect();
        Self {
            from: sub_parts[0].to_string(),
            to: sub_parts[1].to_string(),
            amount: parts[1].parse().unwrap(),
        }
    }
}

fn build(input: &str) -> (HashMap<String, i32>, Vec<Transaction>) {
    let parts: Vec<_> = input.split("\n\n").collect();

    let mut balances = HashMap::default();
    for line in parts[0].lines() {
        let balances_parts: Vec<_> = line.split(" HAS ").collect();
        balances.insert(
            balances_parts[0].to_string(),
            balances_parts[1].parse().unwrap(),
        );
    }

    let transactions = parts[1]
        .lines()
        .map(Transaction::from)
        .collect();

    (balances, transactions)
}

fn three_highest_balances_sum(
    initial_balances: &HashMap<String, i32>,
    transactions: &[Transaction],
) -> i32 {
    let mut balances = initial_balances.clone();
    for tr in transactions {
        *balances.get_mut(&tr.from).unwrap() -= tr.amount;
        *balances.get_mut(&tr.to).unwrap() += tr.amount;
    }

    let mut amounts: Vec<i32> = balances.values().copied().collect();
    amounts.sort_unstable();
    amounts[amounts.len() - 3..].iter().sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (balances, transactions) = build(&input);

    println!(
        "Part 1: {}",
        three_highest_balances_sum(&balances, &transactions)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (balances, transactions) = build(&INPUT_TEST);
        assert_eq!(three_highest_balances_sum(&balances, &transactions), 2870);
    }
}
