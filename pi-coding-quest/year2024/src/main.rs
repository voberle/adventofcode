const INPUT: &str = "Wii kxtszof ova fsegyrpm d lnsrjkujvq roj! Kdaxii svw vnwhj pvugho buynkx tn vwh-gsvw ruzqia. Mrq'x kxtmjw bx fhlhlujw cjoq! Hmg tyhfa gx dwd fdqu bsm osynbn oulfrex, kahs con vjpmd qtjv bx whwxssp cti hmulkudui yqg f Miywh Sj Efh!";

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn left_shift_lowercase(c: char, shift: i32) -> char {
    ('a' as i32 + (c as i32 - 'a' as i32 - shift).rem_euclid(26)) as u8 as char
}

fn left_shift(c: char, shift: i32) -> char {
    if c.is_ascii_lowercase() {
        left_shift_lowercase(c, shift)
    } else {
        left_shift_lowercase(c.to_ascii_lowercase(), shift).to_ascii_uppercase()
    }
}

#[allow(unused)]
fn right_shift(c: char, shift: i32) -> char {
    left_shift(c, -shift)
}

#[allow(clippy::cast_possible_wrap)]
fn decipher(input: &str) -> String {
    // We need to take only the first 16 digits of Pi, and then start from the beginning again.
    const PI: &str = "3141592653589793";
    let pi_digits = PI.chars().map(|d| d.to_digit(10).unwrap() as i32).cycle();

    let mut message = String::with_capacity(input.len());
    for (c, shift) in INPUT.chars().zip(pi_digits) {
        let decoded_char = if c.is_ascii_alphabetic() {
            left_shift(c, shift)
        } else {
            c
        };
        message.push(decoded_char);
    }
    message
}

fn main() {
    let message = decipher(INPUT);
    println!("{message}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_shift() {
        assert_eq!(left_shift('d', 3), 'a');
        assert_eq!(left_shift('a', 3), 'x');
        assert_eq!(left_shift('D', 3), 'A');
    }

    #[test]
    fn test_right_shift() {
        assert_eq!(right_shift('a', 3), 'd');
        assert_eq!(right_shift('x', 3), 'a');
        assert_eq!(right_shift('A', 3), 'D');
    }
}
