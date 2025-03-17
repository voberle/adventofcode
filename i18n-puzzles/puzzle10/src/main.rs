use std::{
    collections::HashMap,
    fmt::Display,
    io::{self, Read},
};

use itertools::Itertools;
use unicode_normalization::char::compose;
use unicode_normalization::char::decompose_canonical;

type AuthDatabase = HashMap<String, String>;

fn verify(password: &str, hash: &str) -> bool {
    if let Ok(val) = bcrypt::verify(password, hash) {
        val
    } else {
        println!("Error calling bcrypt verify");
        false
    }
}

// Decomposes the string into a list of chars, either ASCII or accents.
fn decompose(s: &str) -> Vec<char> {
    s.chars()
        .flat_map(|letter| {
            let mut chars = Vec::new();
            decompose_canonical(letter, |c| {
                chars.push(c);
            });
            chars
        })
        .collect()
}

#[derive(Debug)]
enum Item {
    Ascii(char),
    Pair(char, char),
}

// Given a list of decomposed chars, group all ASCII + Non-ASCII as pairs
fn itemize(chars: &[char]) -> Vec<Item> {
    let mut items: Vec<Item> = chars
        .windows(2)
        .filter_map(|w| {
            if w[0].is_ascii() {
                if w[1].is_ascii() {
                    // Raw ascii in source, just one char.
                    Some(Item::Ascii(w[0]))
                } else {
                    // Pair.
                    Some(Item::Pair(w[0], w[1]))
                }
            } else {
                // Accent, was already handled on previous iteration.
                None
            }
        })
        .collect();
    // If last item was ascii, we need to also the next one as ascii, it's missing for now
    if matches!(items.last().unwrap(), Item::Ascii(_)) {
        items.push(Item::Ascii(*chars.last().unwrap()));
    }
    items
}

// Try all combinations.
fn try_all_combinations(items: &[Item], hash: &str) -> bool {
    // There are items.len() combinations to try.
    for combi in 0..items.len() {
        let mut pair_index = 0;
        let chars: Vec<char> = items
            .iter()
            .flat_map(|item| {
                match item {
                    Item::Ascii(c) => vec![*c],
                    Item::Pair(c1, c2) => {
                        let res = if combi & (1 << pair_index) != 0 {
                            // Try to merge.
                            if let Some(merged) = compose(*c1, *c2) {
                                vec![merged]
                            } else {
                                vec![*c1, *c2]
                            }
                        } else {
                            vec![*c1, *c2]
                        };
                        pair_index += 1;
                        res
                    }
                }
            })
            .collect();

        let password_to_try: String = chars.iter().collect();
        if verify(&password_to_try, hash) {
            return true;
        }
    }
    false
}

#[derive(Debug)]
struct LoginAttempt {
    username: String,
    password: String,
}

impl LoginAttempt {
    fn new(username: &str, password: &str) -> Self {
        LoginAttempt {
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    fn is_valid(&self, auth_entries: &AuthDatabase) -> bool {
        if let Some(hash) = auth_entries.get(&self.username) {
            // We need to try all possible normalization combinations.

            let decomposed = decompose(&self.password);
            let items = itemize(&decomposed);

            if try_all_combinations(&items, hash) {
                println!("{self}\t is valid.");
                true
            } else {
                println!("{self}\t is not valid");
                false
            }
        } else {
            println!("No username '{}' in DB", self.username);
            false
        }
    }
}

impl Display for LoginAttempt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.username, self.password)
    }
}

fn build(input: &str) -> (AuthDatabase, Vec<LoginAttempt>) {
    let mut it = input.lines();

    let mut auth_entries: AuthDatabase = HashMap::new();
    for line in it.by_ref() {
        if line.is_empty() {
            break;
        }
        let (username, password_bcrypted) = line.split_whitespace().collect_tuple().unwrap();
        auth_entries.insert(username.to_string(), password_bcrypted.to_string());
    }

    let mut login_attempts = Vec::new();
    for line in it {
        let (username, password) = line.split_whitespace().collect_tuple().unwrap();
        login_attempts.push(LoginAttempt::new(username, password));
    }

    (auth_entries, login_attempts)
}

fn valid_login_count(auth_entries: &AuthDatabase, login_attempts: &[LoginAttempt]) -> usize {
    login_attempts
        .iter()
        .filter(|attempt| attempt.is_valid(auth_entries))
        .count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (auth_entries, login_attempts) = build(&input);
    // println!("{:?}", auth_entries);
    // println!("{:?}", login_attempts);

    println!(
        "Answer: {}",
        valid_login_count(&auth_entries, &login_attempts)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_bcrypt_verify() {
        assert!(
            bcrypt::verify(
                "secret",
                "$2b$10$v3I80pwHtgxp2ampg4Opy.hehc03wCR.JBZE6WHsrSQtxred57/PG"
            )
            .unwrap()
        );
    }

    #[test]
    fn test_decompose_and_try() {
        // let password = ".pM?XÑ0i7ÈÌ";
        // let password = r"3+sÍkÜLg._";
        let password = r"3+sÍkÜLg?_";
        let hash = "$2b$07$4F7o9sxNeaPe..........l1ZfgXdJdYtpfyyUYXN/HQA1lhpuldO";

        // for letter in password.chars() {
        //     let mut chars = Vec::new();
        //     decompose_canonical(letter, |c| { chars.push(c); });
        //     dbg!(chars);
        // }

        let decomposed = decompose(&password);
        let items = itemize(&decomposed);
        for i in &items {
            println!("{:?}", i);
        }

        assert!(try_all_combinations(&items, hash));
    }

    #[test]
    fn test_valid_attempt() {
        let mut auth_entries: AuthDatabase = HashMap::new();
        auth_entries.insert(
            "etasche".to_string(),
            "$2b$07$0EBrxS4iHy/aHAhqbX/ao.n7305WlMoEpHd42aGKsG21wlktUQtNu".to_string(),
        );
        let valid_attempt = LoginAttempt::new("etasche", ".pM?XÑ0i7ÈÌ");
        assert!(valid_attempt.is_valid(&auth_entries));
    }

    #[test]
    fn test_invalid_attempt() {
        let mut auth_entries: AuthDatabase = HashMap::new();
        auth_entries.insert(
            "mpataki".to_string(),
            "$2b$07$bVWtf3J7xLm5KfxMLOFLiu8Mq64jVhBfsAwPf8/xx4oc5aGBIIHxO".to_string(),
        );
        let invalid_attempt = LoginAttempt::new("mpataki", "2ö$p3ÄÌgÁüy");
        assert!(!invalid_attempt.is_valid(&auth_entries));
    }

    #[test]
    fn test_answer() {
        let (auth_entries, login_attempts) = build(INPUT_TEST);

        for (attempt, result) in login_attempts.iter().zip([
            true, false, false, true, false, false, false, false, true, true, false, false,
        ]) {
            assert_eq!(attempt.is_valid(&auth_entries), result);
        }

        // assert_eq!(valid_login_count(&auth_entries, &login_attempts), 4);
    }
}
