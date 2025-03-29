use std::io::{self, Read};

use codepage_437::{CP437_CONTROL, FromCp437};

fn cp437_to_str(input: Vec<u8>) -> String {
    String::from_cp437(input, &CP437_CONTROL)
}

fn rotations_count(screencap: &str) -> usize {
    0
}

fn main() {
    // Reading as binary.
    let mut input = Vec::new();
    let _ = io::stdin().read_to_end(&mut input);

    let screencap = cp437_to_str(input);
    println!("{screencap}");

    println!("Answer: {}", rotations_count(&screencap));
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_answer() {
        let mut f = File::open("../resources/input_test_1").unwrap();
        let mut input = Vec::new();
        let _ = f.read_to_end(&mut input);

        let screencap = cp437_to_str(input);

        assert_eq!(rotations_count(&screencap), 3030);
    }
}
