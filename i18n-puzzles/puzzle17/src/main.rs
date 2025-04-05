use std::{
    char::REPLACEMENT_CHARACTER,
    io::{self, Read},
};

use itertools::Itertools;

type Fragment = Vec<Vec<u8>>;

// Converts a string with hexadecimals to a vector of bytes.
fn hex_line_to_u8(line: &str) -> Vec<u8> {
    (0..line.len())
        .step_by(2)
        .map(|i| {
            let s = &line[i..=i + 1];
            u8::from_str_radix(s, 16).unwrap()
        })
        .collect()
}

// Converts a string of hexadecimals lines to a fragment (a vec of vec of bytes).
fn hexa_bloc_to_fragment(bloc: &str) -> Fragment {
    bloc.lines().map(hex_line_to_u8).collect()
}

// Parses the input as a list of fragments, each fragment being a list of bytes.
fn build(input: &str) -> Vec<Fragment> {
    let fragments: Vec<_> = input.split("\n\n").collect();
    fragments
        .iter()
        .map(|fragment| hexa_bloc_to_fragment(fragment))
        .collect()
}

// Converts the bloc of u8 into a String bloc of text (with new lines).
// Invalid UTF-8 blocs are replaced with replacement character.
fn fragment_to_string_lossy(fragment: &Fragment) -> String {
    fragment
        .iter()
        .map(|line| String::from_utf8_lossy(line))
        .join("\n")
}

// Checks if the fragment (a partial map) is valid UTF-8.
// This is done by ignoring any possible replacement chars at the end of each line.
fn is_valid_utf8_inside(fragment: &Fragment) -> bool {
    fragment.iter().all(|line| {
        let mut lossy = String::from_utf8_lossy(line.trim_ascii_end()).into_owned();
        if let Some(last) = lossy.chars().last() {
            if last == REPLACEMENT_CHARACTER {
                lossy.pop();
            }
        }
        !lossy.contains(REPLACEMENT_CHARACTER)
    })
}

// Finds the left corners and returns their indexes.
fn find_left_corners(map_samples: &[Fragment]) -> (usize, usize) {
    let (mut top_left, mut bottom_left) = (None, None);
    // Go through each fragment and check if they contain the corner characters.
    for (index, fragment) in map_samples.iter().enumerate() {
        let first_line = String::from_utf8_lossy(fragment.first().unwrap());
        if let Some(first) = first_line.chars().next() {
            if first == '╔' {
                assert!(top_left.is_none());
                top_left = Some(index);
            }
        }

        let last_line = String::from_utf8_lossy(fragment.last().unwrap());
        if let Some(first) = last_line.chars().next() {
            if first == '╚' {
                assert!(bottom_left.is_none());
                bottom_left = Some(index);
            }
        }
    }

    (top_left.unwrap(), bottom_left.unwrap())
}

// Check if this string fragment is part of the left border.
fn is_left_border_str(s: &str) -> bool {
    s.lines().any(|line| {
        let f = line.chars().next().unwrap();
        ['╔', '|', '║', '╚'].contains(&f)
    })
}

// Find all the fragments that are on the left border.
fn find_left_border(map_samples: &[Fragment]) -> Vec<usize> {
    map_samples
        .iter()
        .enumerate()
        .filter_map(|(index, fragment)| {
            let s = fragment_to_string_lossy(fragment);
            if is_left_border_str(&s) {
                Some(index)
            } else {
                None
            }
        })
        .collect()
}

// Add the specified fragment to the bigger map in construction.
// The fragment is added at the specified y position, after the most right fragment.
fn add_fragment_at(big_map: &mut Fragment, fragment: &Fragment, y_pos: usize) {
    for y in 0..fragment.len() {
        big_map[y_pos + y].extend(fragment[y].clone());
    }
}

// Find the ╳.
fn find_x_position_product(map: &Fragment) -> usize {
    let map_as_str = fragment_to_string_lossy(map);
    for (y, line) in map_as_str.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '╳' {
                return x * y;
            }
        }
    }
    panic!("X not found")
}

// Finds the position of the X.
fn x_product(map_samples: &[Fragment]) -> usize {
    // The samples have all the same width but different heights.

    // We can find all the fragments that go on the left.
    // We know the top one and bottom one. There are not that many in the middle, so
    // not that many permutations.
    // So we pick a permutation, and try to fill the next column.

    let (top_left, bottom_left) = find_left_corners(map_samples);

    let left_border = find_left_border(map_samples);

    let left_border_without_corner: Vec<usize> = left_border
        .iter()
        .filter(|i| ![top_left, bottom_left].contains(i))
        .copied()
        .collect();

    let mut big_map = Vec::new();

    for perm in left_border_without_corner
        .iter()
        .permutations(left_border_without_corner.len())
    {
        // For this permutation, build the left column.
        big_map.clone_from(&map_samples[top_left]);
        for i in perm {
            big_map.extend(map_samples[*i].clone());
        }
        big_map.extend(map_samples[bottom_left].clone());

        assert!(is_valid_utf8_inside(&big_map));

        // Indexes of the fragments we still need to locate.
        let mut remaining: Vec<usize> = (0..map_samples.len()).collect();
        remaining.retain(|i| !left_border.contains(i));

        while !remaining.is_empty() {
            // Try to build the next column
            let mut current_y = 0;

            'outer_col: while current_y < big_map.len() {
                let mut added_to_this_col = Vec::new();
                for fragment_index in &remaining {
                    // The fragment may be too high to fit.
                    if current_y + map_samples[*fragment_index].len() > big_map.len() {
                        continue;
                    }

                    // Create a copy of the map, add the fragment and check if it's valid.
                    let mut map_copy = big_map.clone();

                    add_fragment_at(&mut map_copy, &map_samples[*fragment_index], current_y);

                    if is_valid_utf8_inside(&map_copy) {
                        // If it fits, we take the copy of the map as new reference, and look for next one.
                        big_map = map_copy;
                        added_to_this_col.push(*fragment_index);
                        current_y += map_samples[*fragment_index].len();
                    }
                }

                if added_to_this_col.is_empty() {
                    // None found, we have wrong permutation
                    break 'outer_col;
                }
                remaining.retain(|i| !added_to_this_col.contains(i));
            }

            if current_y == big_map.len() {
                // Filled the column
                continue;
            }

            // We couldn't fill the column with this permutation, trying next one.
            break;
        }
    }

    find_x_position_product(&big_map)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map_samples = build(&input);

    println!("Answer: {}", x_product(&map_samples));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_hex_to_utf8() {
        let bin = hex_line_to_u8("e29594");
        let string_utf8_lossy = String::from_utf8_lossy(&bin);
        println!("{}", string_utf8_lossy);
        assert_eq!(string_utf8_lossy, "╔");
    }

    #[test]
    fn test_fragment_to_string() {
        let fragment = hexa_bloc_to_fragment(INPUT_TEST_2);
        let expected = r"╔-═-═-═-
|~≋≋ññ≋~
║ññ≋~~≋�
|~ññ𑀍ñ≋�";
        let s = fragment_to_string_lossy(&fragment);
        assert_eq!(s, expected)
    }

    #[test]
    fn test_answer() {
        let map_samples = build(INPUT_TEST_1);
        assert_eq!(x_product(&map_samples), 132);
    }
}
