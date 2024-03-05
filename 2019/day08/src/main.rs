use std::io::{self, Read};

fn layer_fewest_0_result(picture: &[u8], width: usize, heigth: usize) -> usize {
    let layer_size = width * heigth;
    let layer = picture
        .chunks(layer_size)
        .min_by_key(|c| bytecount::count(c, b'0'))
        .unwrap();
    bytecount::count(layer, b'1') * bytecount::count(layer, b'2')
}

const BLACK: u8 = b'0';
const WHITE: u8 = b'1';
const TRANSPARENT: u8 = b'2';

fn print_image(picture: &[u8], width: usize, heigth: usize) {
    for row in 0..heigth {
        for c in picture.iter().take((row + 1) * width).skip(row * width) {
            if *c == BLACK {
                print!("\u{2B1B}");
            } else if *c == WHITE {
                print!("\u{2B1C}");
            } else {
                panic!("Image not fully decoded");
            }
        }
        println!();
    }
}

fn get_message(picture: &[u8], width: usize, heigth: usize) -> Vec<u8> {
    let layer_size = width * heigth;
    let mut decoded = vec![TRANSPARENT; layer_size];
    picture.chunks(layer_size).for_each(|c| {
        c.iter().enumerate().for_each(|(i, v)| {
            // If layer pixel is black or white, and it's not yet decoded, then use it.
            if decoded[i] == TRANSPARENT && *v != TRANSPARENT {
                decoded[i] = *v;
            }
        });
    });
    decoded
}

fn main() {
    let mut picture = Vec::new();
    io::stdin().read_to_end(&mut picture).unwrap();

    println!("Part 1: {}", layer_fewest_0_result(&picture, 25, 6));

    let message = get_message(&picture, 25, 6);
    println!("Part 2:");
    print_image(&message, 25, 6);
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const INPUT_TEST: &[u8; 16] = include_bytes!("../resources/input_test");

    #[test]
    fn test_get_message() {
        let msg = get_message(INPUT_TEST, 2, 2);
        assert_eq!(msg, [BLACK, WHITE, WHITE, BLACK]);
    }
}
