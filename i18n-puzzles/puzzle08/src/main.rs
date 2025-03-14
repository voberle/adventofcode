use std::hash::Hash;
use std::io::{self, Read};

use deunicode::deunicode;
use fxhash::FxHashSet;
use is_vowel::IsRomanceVowel;
use unicode_normalization::UnicodeNormalization;

fn build(input: &str) -> Vec<String> {
    input.lines().map(ToString::to_string).collect()
}

fn is_vowel(c: char) -> bool {
    // From is_vowel crate.
    c.is_romance_vowel()
}

fn is_consonant(c: char) -> bool {
    // A consonant is an alphabetic character that is not a vowel.
    c.is_alphabetic() && !is_vowel(c)
}

// Makes all characters lowercase and removes all accents.
fn ignore_accents_and_case(s: &str) -> String {
    let lowercase = s.to_lowercase();
    // From deunicode crate: Replaces all Unicode chars with ASCII equivalents, which has the effect or removing the accents.
    deunicode(&lowercase)
}

// Checks if the iterator has only unique elements.
// From <https://stackoverflow.com/a/46767732>
fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = FxHashSet::default();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn is_valid(input: &str) -> bool {
    // First we need to normalize the password, see <https://dietcode.io/p/unicode-normalization/>
    let password = input.nfc().collect::<String>();

    // A length of at least 4 and at most 12.
    // We don't use len() as it would return the byte length, not the characters one.
    let len = password.chars().count();
    if !(4..=12).contains(&len) {
        return false;
    }

    // At least one digit.
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return false;
    }

    // At least one accented or unaccented vowel (a, e, i, o, u) (examples: i, Á or ë).
    if !password.chars().any(is_vowel) {
        return false;
    }

    // At least one accented or unaccented consonant, examples: s, ñ or ŷ.
    if !password.chars().any(is_consonant) {
        return false;
    }

    // No recurring letters in any form. Ignoring accents and case, letters should not recur.
    // For example, in 'niña' the 'n' occurs twice, one time with accent and one time without.
    // 'Usul' is out because the 'u' occurs twice, first uppercase and then lowercase.
    let simplified = ignore_accents_and_case(&password);
    if !has_unique_elements(simplified.chars()) {
        return false;
    }

    true
}

fn answer(lines: &[String]) -> usize {
    // Valid passwords count
    lines.iter().filter(|password| is_valid(password)).count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines = build(&input);

    println!("Answer: {}", answer(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_answer() {
        assert_eq!(answer(&build(INPUT_TEST)), 2);
    }
}
