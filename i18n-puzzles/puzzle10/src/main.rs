use std::{
    collections::HashMap,
    fmt::Display,
    io::{self, Read},
};

use itertools::Itertools;
use unicode_normalization::char::compose;
use unicode_normalization::char::decompose_canonical;

type AuthDatabase = HashMap<String, String>;

// Helper function to call bcrypt verify.
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

#[derive(Debug, PartialEq, Eq)]
enum Item {
    Ascii(char),
    Pair(char, char),
}

// Transforms the input list into a list where pairs of chars that match the condition
// are an Item::Pair, and other chars are an Item::Ascii.
fn process_chars(input: &[char], condition: fn(char, char) -> bool) -> Vec<Item> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < input.len() {
        let current_char = input[i];

        if i + 1 < input.len() {
            let next_char = input[i + 1];
            if condition(current_char, next_char) {
                result.push(Item::Pair(current_char, next_char));
                i += 2; // Skip the next char since it's part of the pair
            } else {
                result.push(Item::Ascii(current_char));
                i += 1;
            }
        } else {
            result.push(Item::Ascii(current_char));
            i += 1;
        }
    }

    result
}

// Given a list of decomposed chars, group all ASCII + Non-ASCII as pairs
fn itemize(chars: &[char]) -> Vec<Item> {
    process_chars(chars, |c1, c2| c1.is_alphabetic() && !c2.is_ascii())
}

fn pairs_count(items: &[Item]) -> usize {
    items
        .iter()
        .filter(|item| matches!(item, Item::Pair(_, _)))
        .count()
}

// Try all combinations until one matches the stored hash.
fn try_all_combinations(items: &[Item], hash: &str) -> bool {
    // There are 2 pow (number of pairs) combinations to try.
    let combi_count = 2u32.pow(u32::try_from(pairs_count(items)).unwrap());

    (0..combi_count).any(|combi| {
        let mut pair_index = 0;
        let password_to_try: String = items
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

        if verify(&password_to_try, hash) {
            return true;
        }
        false
    })
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
    fn test_empty_input() {
        let input: Vec<char> = vec![];
        let condition = |_, _| true;
        let expected: Vec<Item> = vec![];
        assert_eq!(process_chars(&input, condition), expected);
    }

    #[test]
    fn test_single_char() {
        let input = vec!['a'];
        let condition = |_, _| true;
        let expected = vec![Item::Ascii('a')];
        assert_eq!(process_chars(&input, condition), expected);
    }

    #[test]
    fn test_all_ascii() {
        let input = vec!['a', 'b', 'c'];
        let condition = |_, _| false; // Always false condition
        let expected = vec![Item::Ascii('a'), Item::Ascii('b'), Item::Ascii('c')];
        assert_eq!(process_chars(&input, condition), expected);
    }
    #[test]
    fn test_all_pairs() {
        let input = vec!['a', 'b', 'c', 'd', 'e', 'f'];
        let condition = |_, _| true; // Always true condition
        let expected = vec![
            Item::Pair('a', 'b'),
            Item::Pair('c', 'd'),
            Item::Pair('e', 'f'),
        ];
        assert_eq!(process_chars(&input, condition), expected);
    }
    #[test]
    fn test_mixed_ascii_pairs() {
        let input = vec!['a', 'b', 'c', 'd', 'e'];
        let condition = |_, c2| c2 == 'b' || c2 == 'd';
        let expected = vec![Item::Pair('a', 'b'), Item::Pair('c', 'd'), Item::Ascii('e')];
        assert_eq!(process_chars(&input, condition), expected);
    }

    #[test]
    fn test_mixed_ascii_pairs_2() {
        let input = vec!['a', 'b', 'c', 'd', 'e', 'f'];
        let condition = |_, c2| c2 == 'b' || c2 == 'f';
        let expected = vec![
            Item::Pair('a', 'b'),
            Item::Ascii('c'),
            Item::Ascii('d'),
            Item::Ascii('e'),
            Item::Pair('f', '\0'),
        ]; // incorrect - we have no logic to handle condition matching last char
        let result = process_chars(&input, condition);
        assert_ne!(result, expected); // we check that out result is *not* equal to bad logic
        let expected2 = vec![
            Item::Pair('a', 'b'),
            Item::Ascii('c'),
            Item::Ascii('d'),
            Item::Pair('e', 'f'),
        ];
        assert_eq!(process_chars(&input, condition), expected2);
    }

    #[test]
    fn test_pairs_at_end() {
        let input = vec!['a', 'b', 'c'];
        let condition = |_, c2| c2 == 'b' || c2 == 'c';
        let expected = vec![Item::Pair('a', 'b'), Item::Ascii('c')];
        assert_eq!(process_chars(&input, condition), expected);
    }

    #[test]
    fn test_only_pairs_odd_length() {
        let input = vec!['a', 'b', 'c', 'd', 'e'];
        let condition = |_, _| true;
        let expected = vec![Item::Pair('a', 'b'), Item::Pair('c', 'd'), Item::Ascii('e')];
        assert_eq!(process_chars(&input, condition), expected);
    }

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
