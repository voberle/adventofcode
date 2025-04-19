use std::io::{self, Read};

use base64::prelude::*;
use itertools::Itertools;

fn from_base64(input: &str) -> Vec<u8> {
    BASE64_STANDARD.decode(input.replace('\n', "")).unwrap()
}

// Decodes the vector of bytes as a UTF-16 LE string.
// Borrowed from puzzle 13.
fn as_utf_16_le(bytes: &[u8]) -> Option<String> {
    fn has_bom(bytes: &[u8], bom_marker: &[u8]) -> bool {
        bytes.starts_with(bom_marker)
    }

    fn as_u16_le(bytes: &[u8]) -> Vec<u16> {
        bytes
            .chunks(2)
            .map(|w| (u16::from(w[1]) << 8) + u16::from(w[0]))
            .collect()
    }

    if bytes.len() % 2 == 1 {
        // We require an even number of bytes.
        return None;
    }

    // In UTF-16, a BOM (U+FEFF) may be placed as the first bytes of a file
    // or character stream to indicate the endianness (byte order) of all the 16-bit code units of the file or stream.
    if has_bom(bytes, &[0xFF, 0xFE]) {
        String::from_utf16(&as_u16_le(bytes)[1..]).ok()
    } else {
        String::from_utf16(&as_u16_le(bytes)).ok()
    }
}

// Convert this string of bits into a list of bytes.
fn string_bits_to_vec_bytes(bits_str: &str) -> Vec<u8> {
    bits_str
        .chars()
        .chunks(8)
        .into_iter()
        .map(|chunk| {
            let str = chunk.into_iter().join("");
            u8::from_str_radix(&str, 2).unwrap()
        })
        .collect()
}

// Each char value is converted to a binary string if length MAGIC_VAL (padded with leading zeros),
// and joined into a String.
fn decode_utf16_special<const MAGIC_VAL: usize>(utf16_str: &str) -> String {
    utf16_str
        .chars()
        .map(|c| {
            let s = format!("{:0width$b}", c as u32, width = MAGIC_VAL);

            // I'm not sure why, but if it starts with 8 zeros, remove them.
            if let Some(stripped) = s.strip_prefix("00000000") {
                // Not a surrogate pair.
                assert_eq!(c.len_utf16(), 1);

                stripped.to_string()
            } else {
                s
            }
        })
        .join("")
}

// The list of bytes is treated as UTF-8 with sequences of 5 and 6 bytes.
// Extract all the bits and return them as a big string.
fn decode_utf8_extended(bytes: &[u8]) -> String {
    fn is_6_bytes_char(first_byte: u8) -> bool {
        first_byte & 0b1111_1100 == 0b1111_1100
    }

    fn is_5_bytes_char(first_byte: u8) -> bool {
        first_byte & 0b1111_1000 == 0b1111_1000
    }

    fn is_2_bytes_char(first_byte: u8) -> bool {
        first_byte & 0b1100_0000 == 0b1100_0000
    }

    fn extract_6_bits(byte: u8) -> String {
        format!("{:06b}", byte & 0b0011_1111)
    }

    fn extract_4_bits(byte: u8) -> String {
        format!("{:04b}", byte & 0b0000_1111)
    }

    fn extract_2_bits(byte: u8) -> String {
        format!("{:02b}", byte & 0b0000_0011)
    }

    let mut result = String::new();

    let mut remaining_bytes = bytes.to_vec();
    while !remaining_bytes.is_empty() {
        let first_byte = remaining_bytes[0];

        if is_6_bytes_char(first_byte) {
            // Take last 2 bits of second byte, then last 6 bits of bytes 3-6.
            let mut seq = String::new();
            seq.push_str(&extract_4_bits(remaining_bytes[1]));
            seq.push_str(&extract_6_bits(remaining_bytes[2]));
            seq.push_str(&extract_6_bits(remaining_bytes[3]));
            seq.push_str(&extract_6_bits(remaining_bytes[4]));
            seq.push_str(&extract_6_bits(remaining_bytes[5]));
            assert_eq!(seq.len(), 28);
            result.push_str(&seq);

            remaining_bytes = remaining_bytes[6..].to_vec();
            // println!("Decoder: Handled 6-byte seq starting {first_byte}");
        } else if is_5_bytes_char(first_byte) {
            // Take last 2 bytes of first byte, last 6 of the rest.
            let mut seq = String::new();
            seq.push_str(&extract_2_bits(remaining_bytes[0]));
            seq.push_str(&extract_6_bits(remaining_bytes[1]));
            seq.push_str(&extract_6_bits(remaining_bytes[2]));
            seq.push_str(&extract_6_bits(remaining_bytes[3]));
            seq.push_str(&extract_6_bits(remaining_bytes[4]));
            assert_eq!(seq.len(), 26);
            seq = format!("{seq:0>28}"); // Padding to 28.
            result.push_str(&seq);

            remaining_bytes = remaining_bytes[5..].to_vec();
            // println!("Decoder: Handled 5-byte seq starting {first_byte}");
        } else if is_2_bytes_char(first_byte) {
            // Take last 2 bytes of first byte, last 6 of next one.
            // Actually happens only at end of test input.
            let mut seq = String::new();
            seq.push_str(&extract_2_bits(remaining_bytes[0]));
            seq.push_str(&extract_6_bits(remaining_bytes[1]));
            assert_eq!(seq.len(), 8);
            result.push_str(&seq);

            remaining_bytes = remaining_bytes[2..].to_vec();
            // println!("Decoder: Handled 2-byte seq starting {first_byte}");
        } else if first_byte == 0 && remaining_bytes.len() == 1 {
            // On real input we have a zero at the end, we ignore it and just stop.
            // println!("Decoder: Last byte is 0");
            break;
        } else {
            panic!("Not supported starting sequence");
        }
    }

    // println!("Length: {}", result.len());
    // println!("{result}");

    result
}

fn decode(input: &str) -> String {
    // Base64 decode.
    let decoded = from_base64(input);
    // println!("Decoded base 64. Length: {}\n{decoded:?}", decoded.len());

    // UTF-16 LE decoding
    let utf16_str = as_utf_16_le(&decoded).unwrap();
    // println!("UTF-16 string len: {}", utf16_str.len());

    // Each char value is converted to a 20-bit binary string (padded with leading zeros),
    // and joined into a String.
    let utf16_20_bits_str = decode_utf16_special::<20>(&utf16_str);

    // Convert this string of bits into a list of bytes.
    let bytes_from_utf16_20_bits: Vec<u8> = string_bits_to_vec_bytes(&utf16_20_bits_str);

    // println!(
    //     "Bytes from UTF-16, 20 bits limited. Length: {}\n{bytes_from_utf16_20_bits:?}",
    //     bytes_from_utf16_20_bits.len()
    // );

    // Decode the bytes as extended UTF-8 (with up to 6 bytes chars).
    let utf8_ext_str = decode_utf8_extended(&bytes_from_utf16_20_bits);

    // Convert this string of bits into a list of bytes.
    let bytes_from_utf8_ext: Vec<_> = string_bits_to_vec_bytes(&utf8_ext_str);

    // println!(
    //     "Bytes from UTF-16, 20 bits limited. Length: {}\n{bytes_from_utf8_ext:?}",
    //     bytes_from_utf8_ext.len()
    // );

    // Convert the bytes to UTF-8
    let final_str = String::from_utf8_lossy(&bytes_from_utf8_ext);

    // println!("Result:");
    // println!("{final_str}");
    // println!("Expected:");
    // println!("ꪪꪪꪪ This is a secret message. ꪪꪪꪪ Good luck decoding me! ꪪꪪꪪ");

    final_str.to_string()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Answer: {}", decode(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_answer() {
        assert_eq!(
            decode(INPUT_TEST),
            "ꪪꪪꪪ This is a secret message. ꪪꪪꪪ Good luck decoding me! ꪪꪪꪪ"
        );
    }
}
