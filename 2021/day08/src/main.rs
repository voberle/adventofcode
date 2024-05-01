use std::io::{self, Read};

use fxhash::FxHashSet;
use itertools::Itertools;

struct Display {
    signal_patters: Vec<Vec<char>>,
    output_values: Vec<Vec<char>>,
}

impl Display {
    fn build(line: &str) -> Self {
        let (signal_patters, output_value) = line
            .split(" | ")
            .map(|s| s.split_whitespace().map(|p| p.chars().collect()).collect())
            .collect_tuple()
            .unwrap();
        Self {
            signal_patters,
            output_values: output_value,
        }
    }

    fn count_signal_patters_with(&self, c: char) -> usize {
        self.signal_patters
            .iter()
            .filter(|p| p.contains(&c))
            .count()
    }
}

fn build(input: &str) -> Vec<Display> {
    input.lines().map(Display::build).collect()
}

// How many times 1, 4, 7, or 8 appear.
fn subset_digits_count(displays: &[Display]) -> usize {
    const SEGMENT_COUNTS: [usize; 4] = [
        2, // 1
        3, // 7
        4, // 4
        7, // 8
    ];
    displays
        .iter()
        .map(|d| {
            d.output_values
                .iter()
                .filter(|v| SEGMENT_COUNTS.contains(&v.len()))
                .count()
        })
        .sum()
}

// Helper function that will ensure that `elt` is only in the set of index `pos_to_keep`.
fn keep_only_in(options: &mut [FxHashSet<char>], elt: char, pos_to_keep: usize) {
    for (i, option) in options.iter_mut().enumerate() {
        if i == pos_to_keep {
            continue;
        }
        option.remove(&elt);
    }

    options[pos_to_keep].clear();
    options[pos_to_keep].insert(elt);
}

// Finds the value for each display.
fn find_output_value(display: &Display) -> u32 {
    // We identify the digits elements with following indexes:
    //   0000
    //  1    2
    //  1    2
    //   3333
    //  4    5
    //  4    5
    //   6666

    // These are for each digit which elements are on.
    const POS_ON_PER_NUMBER: [[bool; 7]; 10] = [
        [true, true, true, false, true, true, true], // 0: 012.456
        [false, false, true, false, false, true, false], // 1: ..2..5.
        [true, false, true, true, true, false, true], // 2: 0.234.6
        [true, false, true, true, false, true, true], // 3: 0.23.56
        [false, true, true, true, false, true, false], // 4: .123.5.
        [true, true, false, true, false, true, true], // 5: 01.3.56
        [true, true, false, true, true, true, true], // 6: 01.3456
        [true, false, true, false, false, true, false], // 7: 0.2..5.
        [true, true, true, true, true, true, true],  // 8: 0123456
        [true, true, true, true, false, true, true], // 9: 0123.56
    ];

    // For each segment, this options arrays contains possible letters.
    // Initially, all letters are possible.
    let all_letters_set: FxHashSet<char> = ['a', 'b', 'c', 'd', 'e', 'f', 'g']
        .iter()
        .copied()
        .collect::<FxHashSet<_>>();
    let mut options: Vec<FxHashSet<char>> = vec![all_letters_set; 7];

    // So we start by using the numbers that have an unique count of elements (1, 4, 7).
    // If a signal pattern has such a length, then the pattern letters are on in the corresponding number.
    for val in &display.signal_patters {
        let val_set = val.iter().copied().collect::<FxHashSet<_>>();
        match val.len() {
            2 => {
                // Number 1
                for p in [2, 5] {
                    options[p] = options[p].intersection(&val_set).copied().collect();
                }
            }
            3 => {
                // Number 7
                for p in [0, 2, 5] {
                    options[p] = options[p].intersection(&val_set).copied().collect();
                }
            }
            4 => {
                // Number 4
                for p in [1, 2, 3, 5] {
                    options[p] = options[p].intersection(&val_set).copied().collect();
                }
            }
            // No need to bother with 8, no extra info from it.
            _ => {}
        }
    }

    // There are two entries with only two chars: Position 2 and 5, for the number "1".
    // We can remove these two from all other entries.
    let to_remove = &options[2].clone();
    for p in [0, 1, 3, 4, 6] {
        // not 2 and 5
        options[p] = options[p].difference(to_remove).copied().collect();
    }

    // Now we have the same for position 0.
    let to_remove = &options[0].clone();
    for p in [1, 2, 3, 4, 5, 6] {
        // not 0.
        options[p] = options[p].difference(to_remove).copied().collect();
    }

    // And also for position 1 and 3.
    let to_remove = &options[1].clone();
    for p in [0, 2, 4, 5, 6] {
        // not 1 and 3.
        options[p] = options[p].difference(to_remove).copied().collect();
    }

    // At this stage, except for position 0, for all others we just have two options now.

    // Now we use the fact that some elements are on for a specific count of numbers.

    // In position 5, the element is on in 9/10 numbers.
    let (c1, c2) = options[5].iter().copied().collect_tuple().unwrap();
    let pos_elt = if display.count_signal_patters_with(c1) == 9 {
        c1
    } else {
        c2
    };
    keep_only_in(&mut options, pos_elt, 5);

    // In position 3, the element is in 7/10 numbers.
    let (c1, c2) = options[3].iter().copied().collect_tuple().unwrap();
    let pos_elt = if display.count_signal_patters_with(c1) == 7 {
        c1
    } else {
        c2
    };
    keep_only_in(&mut options, pos_elt, 3);

    // In position 6, the element is in 7/10 numbers.
    let (c1, c2) = options[6].iter().copied().collect_tuple().unwrap();
    let pos_elt = if display.count_signal_patters_with(c1) == 7 {
        c1
    } else {
        c2
    };
    keep_only_in(&mut options, pos_elt, 6);

    // Now we have identified all positions.
    // Let's convert the output values.

    let mut output_val = 0;
    for val in &display.output_values {
        // Pattern of the digit.
        let mut pos = [false; 7];
        for c in val {
            let i = options.iter().position(|o| o.contains(c)).unwrap();
            pos[i] = true;
        }

        // Find the digit corresponding to the pattern.
        let digit = POS_ON_PER_NUMBER
            .iter()
            .position(|pattern| *pattern == pos)
            .unwrap();

        output_val = output_val * 10 + u32::try_from(digit).unwrap();
    }

    output_val
}

fn sum_output_values(displays: &[Display]) -> u32 {
    displays.iter().map(find_output_value).sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let displays = build(&input);

    println!("Part 1: {}", subset_digits_count(&displays));
    println!("Part 2: {}", sum_output_values(&displays));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(subset_digits_count(&build(INPUT_TEST)), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(sum_output_values(&build(INPUT_TEST)), 61229);
    }
}
