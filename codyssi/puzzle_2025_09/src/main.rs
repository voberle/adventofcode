use std::{
    collections::HashMap,
    fmt::Display,
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

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} => {}: {}", self.from, self.to, self.amount)
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

    let transactions = parts[1].lines().map(Transaction::from).collect();

    (balances, transactions)
}

fn three_highest_balances_sum(balances: &HashMap<String, i32>) -> i32 {
    let mut amounts: Vec<i32> = balances.values().copied().collect();
    amounts.sort_unstable();
    amounts[amounts.len() - 3..].iter().sum()
}

fn apply_transactions(balances: &mut HashMap<String, i32>, transactions: &[Transaction]) {
    for tr in transactions {
        *balances.get_mut(&tr.from).unwrap() -= tr.amount;
        *balances.get_mut(&tr.to).unwrap() += tr.amount;
    }
}

fn balances_sum(initial_balances: &HashMap<String, i32>, transactions: &[Transaction]) -> i32 {
    let mut balances = initial_balances.clone();
    apply_transactions(&mut balances, transactions);

    three_highest_balances_sum(&balances)
}

fn apply_transactions_limited(balances: &mut HashMap<String, i32>, transactions: &[Transaction]) {
    for tr in transactions {
        let amount = tr.amount.min(balances[&tr.from]);

        *balances.get_mut(&tr.from).unwrap() -= amount;
        *balances.get_mut(&tr.to).unwrap() += amount;
    }
}

fn limited_balances_sum(
    initial_balances: &HashMap<String, i32>,
    transactions: &[Transaction],
) -> i32 {
    let mut balances = initial_balances.clone();
    apply_transactions_limited(&mut balances, transactions);

    three_highest_balances_sum(&balances)
}

// Finds what debt can be reimbursed.
// Returns one debt to reimburse, but there might be more.
fn identify_reimbursement(
    balances: &HashMap<String, i32>,
    debts: &HashMap<String, Vec<(String, i32)>>,
) -> Option<(String, String, i32)> {
    for (debtor, debtor_balance) in balances {
        let debtor = debtor.as_str();
        let debtor_balance = *debtor_balance;

        if debtor_balance > 0 {
            if let Some(recv_debts) = debts.get(debtor) {
                if !recv_debts.is_empty() {
                    let reimbursing_to = &recv_debts[0].0;
                    let amount_owned = recv_debts[0].1;

                    // Enough money to pay in full.
                    if debtor_balance > amount_owned {
                        // println!("{debtor} pays {reimbursing_to} in full ({amount_owned})",);
                        return Some((
                            debtor.to_string(),
                            reimbursing_to.to_string(),
                            amount_owned,
                        ));
                    }

                    // Partial repayment only.
                    // println!("{debtor} pays {reimbursing_to} partially, {debtor_balance}",);
                    return Some((
                        debtor.to_string(),
                        reimbursing_to.to_string(),
                        debtor_balance,
                    ));
                }
            }
        }
    }
    None
}

fn apply_transactions_with_debt(balances: &mut HashMap<String, i32>, transactions: &[Transaction]) {
    // Debts that everybody owns, ranked. A Vec is used to rank the debts.
    let mut debts: HashMap<String, Vec<(String, i32)>> = HashMap::default();

    // println!("Initial Balances: {balances:?}\n");

    for tr in transactions {
        if tr.amount <= balances[&tr.from] {
            // Normal transaction.
            // println!("Normal transaction: {tr}");
            *balances.get_mut(&tr.from).unwrap() -= tr.amount;
            *balances.get_mut(&tr.to).unwrap() += tr.amount;
        } else {
            // Limited transaction.
            // println!("Limited transaction: {}", tr);
            let amount_to_transfer = balances[&tr.from];
            let debt_taken = tr.amount - amount_to_transfer;

            // Sender has 0 cash and debts.
            *balances.get_mut(&tr.from).unwrap() = 0;

            // Update sender's debts.
            debts
                .entry(tr.from.clone())
                .and_modify(|e| {
                    // If there are several debts to the same person, they are kept separate.
                    e.push((tr.to.clone(), debt_taken));
                })
                .or_insert(vec![(tr.to.clone(), debt_taken)]);

            // Receiver gets all cash of sender.
            *balances.get_mut(&tr.to).unwrap() += amount_to_transfer;
        }

        // After each transaction, see if some debts can be reimbursed.
        while let Some((debtor, reimbursing_to, amount_to_repay)) =
            identify_reimbursement(balances, &debts)
        {
            // Pay who is owned.
            *balances.get_mut(&reimbursing_to).unwrap() += amount_to_repay;

            // Update debtor balance.
            *balances.get_mut(&debtor).unwrap() -= amount_to_repay;

            // Update debtor debts.
            let recv_debts = debts.get_mut(&debtor).unwrap();
            recv_debts[0].1 -= amount_to_repay;
            if recv_debts[0].1 == 0 {
                // Debt is fully reimbursed.
                recv_debts.remove(0);
            }
        }

        // println!("Balances: {balances:?}",);
        // println!("Debts:");
        // for (who_owns, debts_owned) in &debts {
        //     println!("  {who_owns} owns {debts_owned:?}");
        // }
        // println!();
    }
}

fn balances_with_debt(
    initial_balances: &HashMap<String, i32>,
    transactions: &[Transaction],
) -> i32 {
    let mut balances = initial_balances.clone();
    apply_transactions_with_debt(&mut balances, transactions);

    three_highest_balances_sum(&balances)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (balances, transactions) = build(&input);

    println!("Part 1: {}", balances_sum(&balances, &transactions));
    println!("Part 2: {}", limited_balances_sum(&balances, &transactions));
    println!("Part 3: {}", balances_with_debt(&balances, &transactions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (balances, transactions) = build(&INPUT_TEST);
        assert_eq!(balances_sum(&balances, &transactions), 2870);
    }

    #[test]
    fn test_part2() {
        let (balances, transactions) = build(&INPUT_TEST);
        assert_eq!(limited_balances_sum(&balances, &transactions), 2542);
    }

    #[test]
    fn test_part3() {
        let (balances, transactions) = build(&INPUT_TEST);
        assert_eq!(balances_with_debt(&balances, &transactions), 2511);
    }
}
