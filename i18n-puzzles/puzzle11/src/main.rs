use std::io::{self, Read};

fn build(input: &str) -> Vec<String> {
    input.lines().map(ToString::to_string).collect()
}

fn shift_uppercase(letter: char, shift: usize) -> char {
    static UPPERCASE_LETTERS: &[char] = &[
        'Α', 'Β', 'Γ', 'Δ', 'Ε', 'Ζ', 'Η', 'Θ', 'Ι', 'Κ', 'Λ', 'Μ', 'Ν', 'Ξ', 'Ο', 'Π', 'Ρ', 'Σ',
        'Τ', 'Υ', 'Φ', 'Χ', 'Ψ', 'Ω',
    ];

    // Is there a more efficient way to do this?
    if let Some(index) = UPPERCASE_LETTERS.iter().position(|c| *c == letter) {
        let shifted_index = (index + shift) % UPPERCASE_LETTERS.len();
        UPPERCASE_LETTERS[shifted_index]
    } else {
        letter
    }
}

fn shift_lowercase(letter: char, shift: usize) -> char {
    static LOWERCASE_LETTERS: &[char] = &[
        'α', 'β', 'γ', 'δ', 'ε', 'ζ', 'η', 'θ', 'ι', 'κ', 'λ', 'μ', 'ν', 'ξ', 'ο', 'π', 'ρ', 'σ',
        'τ', 'υ', 'φ', 'χ', 'ψ', 'ω',
    ];

    // Handle sigma special case
    // 'ς' isn't in the table, but 'σ' is.
    let fixed_letter = if letter == 'ς' { 'σ' } else { letter };

    if let Some(index) = LOWERCASE_LETTERS.iter().position(|c| *c == fixed_letter) {
        let shifted_index = (index + shift) % LOWERCASE_LETTERS.len();
        LOWERCASE_LETTERS[shifted_index]
    } else {
        letter
    }
}

fn shift_char(c: char, shift: usize) -> char {
    if c.is_uppercase() {
        shift_uppercase(c, shift)
    } else {
        shift_lowercase(c, shift)
    }
}

fn shift_str(line: &str, shift: usize) -> String {
    line.chars().map(|c| shift_char(c, shift)).collect()
}

const ODYSSEUS_VARIANTS: &[&str] = &[
    "Οδυσσευς",
    "Οδυσσευ",
    "Οδυσσευς",
    "Οδυσσεως",
    "Οδυσσει",
    "Οδυσσεα",
    "Οδυσσευ",
];

fn odysseus_shift_count(line: &str) -> Option<usize> {
    for shift in 1..line.len() {
        let shifted_str = shift_str(line, shift);
        for variant in ODYSSEUS_VARIANTS {
            // If we would need the actual index of the variant, we should use match_indices(),
            // since find() returns the byte index. But we just need to know of it's there or not.
            if shifted_str.find(variant).is_some() {
                return Some(shift);
            }
        }
    }
    None
}

fn answer(lines: &[String]) -> usize {
    lines
        .iter()
        .map(|line| odysseus_shift_count(line).unwrap_or_default())
        .sum()
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
    fn test_shift() {
        // Uppercase
        assert_eq!(shift_char('Α', 3), 'Δ');
        assert_eq!(shift_char('Χ', 3), 'Α');
        // Lowercase
        assert_eq!(shift_char('α', 3), 'δ');
        assert_eq!(shift_char('χ', 3), 'α');
        // Sigma
        assert_eq!(shift_char('σ', 1), 'τ');
        assert_eq!(shift_char('σ', 1), shift_char('ς', 1));
        // 23 is like shifting 1 to the left.
        assert_eq!(shift_char('σ', 23), 'ρ');
        assert_eq!(shift_char('σ', 23), shift_char('ς', 23));
    }

    #[test]
    fn test_odysseus_shift_count() {
        assert_eq!(
            odysseus_shift_count("σζμ γ' ωοωλδθαξλδμξρ οπξρδυζ οξκτλζσθρ Ξγτρρδτρ."),
            Some(1)
        );
        assert_eq!(
            odysseus_shift_count("αφτ κ' λαλψφτ ωπφχλρφτ δξησηρζαλψφτ φελο, Φκβωωλβ."),
            Some(18)
        );
        assert_eq!(
            odysseus_shift_count("γ βρφαγζ ωνψν ωγφ πγχρρφ δρδαθωραγζ ρφανφ."),
            None
        );
    }

    #[test]
    fn test_answer() {
        let lines = build(INPUT_TEST);
        assert_eq!(answer(&lines), 19);
    }
}
