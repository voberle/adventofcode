use std::io::{self, Read};

fn straight_three_letters(password: &str) -> bool {
    password
        .as_bytes()
        .windows(3)
        .any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2])
}

fn no_iol(password: &str) -> bool {
    ['i', 'o', 'l'].iter().all(|c| !password.contains(*c))
}

fn two_different_non_overlapping_pairs(password: &str) -> bool {
    // Find all the pairs
    let mut pair_chars: Vec<_> = password
        .as_bytes()
        .windows(2)
        .filter_map(|w| if w[0] == w[1] { Some(w[0]) } else { None })
        .collect();
    if pair_chars.len() > 1 {
        // If there are at least two pairs, dedup them to see if they are not all the same
        pair_chars.sort();
        pair_chars.dedup();
        return pair_chars.len() > 1;
    }
    false
}

fn is_valid(password: &str) -> bool {
    straight_three_letters(password)
        && no_iol(password)
        && two_different_non_overlapping_pairs(password)
}

fn wrap_password(password: &str) -> String {
    // New password, generated inversed and as bytes
    let mut new: Vec<u8> = Vec::new();

    let mut inc = true; // Indicates if the char should be increased or not
                        // We start from the end
    for c in password.chars().rev() {
        if inc {
            if c == 'z' {
                new.push(b'a');
                inc = true;
            } else {
                new.push(c as u8 + 1);
                inc = false;
            }
        } else {
            new.push(c as u8);
        }
    }

    // Converting to right order and to String
    new.reverse();
    String::from_utf8(new).expect("Our bytes should be valid")
}

fn next_password(input: &str) -> String {
    let mut password = wrap_password(input);
    while !is_valid(&password) {
        password = wrap_password(&password);
    }
    password
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n = next_password(&input);
    println!("Part 1: {}", n);
    println!("Part 2: {}", next_password(&n));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        // hijklmmn meets the first requirement (because it contains the straight hij) but fails the second requirement requirement (because it contains i and l).
        assert!(straight_three_letters("hijklmmn"));
        assert!(!no_iol("hijklmmn"));
        assert!(!is_valid("hijklmmn"));
        // abbceffg meets the third requirement (because it repeats bb and ff) but fails the first requirement.
        assert!(!straight_three_letters("abbceffg"));
        assert!(two_different_non_overlapping_pairs("abbceffg"));
        assert!(!is_valid("abbceffg"));
        // abbcegjk fails the third requirement, because it only has one double letter (bb).
        assert!(!two_different_non_overlapping_pairs("abbcegjk"));
        assert!(!is_valid("abbcegjk"));

        assert!(is_valid("abcdffaa"));
        assert!(is_valid("ghjaabcc"));
    }

    #[test]
    fn test_next_password() {
        assert_eq!(next_password("abcdefgh"), "abcdffaa");
        assert_eq!(next_password("ghijklmn"), "ghjaabcc");
    }
}
