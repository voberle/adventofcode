use std::io::{self, Read};

fn build(input: &str) -> Vec<String> {
    input.lines().map(ToString::to_string).collect()
}

fn shift_with(letter: char, shift: usize, letters_array: &[char]) -> char {
    // Since letters are not contiguous as we would have with ASCII, we have to perform a search.
    // Alternative would be to use a hashmap, but it doesn't perform better here.
    if let Some(index) = letters_array.iter().position(|c| *c == letter) {
        let shifted_index = (index + shift) % letters_array.len();
        letters_array[shifted_index]
    } else {
        letter
    }
}

fn shift_char(letter: char, shift: usize) -> char {
    static UPPERCASE_LETTERS: &[char] = &[
        'Α', 'Β', 'Γ', 'Δ', 'Ε', 'Ζ', 'Η', 'Θ', 'Ι', 'Κ', 'Λ', 'Μ', 'Ν', 'Ξ', 'Ο', 'Π', 'Ρ', 'Σ',
        'Τ', 'Υ', 'Φ', 'Χ', 'Ψ', 'Ω',
    ];
    static LOWERCASE_LETTERS: &[char] = &[
        'α', 'β', 'γ', 'δ', 'ε', 'ζ', 'η', 'θ', 'ι', 'κ', 'λ', 'μ', 'ν', 'ξ', 'ο', 'π', 'ρ', 'σ',
        'τ', 'υ', 'φ', 'χ', 'ψ', 'ω',
    ];

    if letter.is_uppercase() {
        shift_with(letter, shift, UPPERCASE_LETTERS)
    } else {
        // Handle sigma special case
        // 'ς' isn't in the table, but 'σ' is.
        let letter = if letter == 'ς' { 'σ' } else { letter };

        shift_with(letter, shift, LOWERCASE_LETTERS)
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
            if shifted_str.contains(variant) {
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
