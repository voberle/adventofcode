use std::io::{self, Read};

fn build(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn calibration_values_sum(
    calibration_values: &[&str],
    conversion_fn: Option<fn(&str) -> String>,
) -> u32 {
    calibration_values
        .iter()
        .filter_map(|value| {
            let value = if let Some(conv_fn) = conversion_fn {
                conv_fn(value)
            } else {
                // This to_string isn't needed if we run only part 1, but required to share the code.
                // Should hopefully be a fast optimized version.
                (*value).to_string()
            };

            // An iterator, not collecting.
            let mut d = value.chars().filter_map(|c| c.to_digit(10));
            if let Some(first) = d.next() {
                // chars() is a double-ended iterator, so we can use next_back.
                // The unwrap fallback handles the case when there is only 1 digit in the value.
                let last = d.next_back().unwrap_or(first);
                Some(first * 10 + last)
            } else {
                None
            }
        })
        .sum()
}

fn calibration_values_sum_v1(calibration_values: &[&str]) -> u32 {
    calibration_values_sum(calibration_values, None)
}

fn conversion(value: &str) -> String {
    const STRING_TO_DIGIT: [(&str, u32); 9] = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut s = (*value).to_string();

    // We need to make sure we replace the first number we find
    // The right calibration values for string "eighthree" is 83 and for "sevenine" is 79.
    let mut i = 0;
    while i < value.len() {
        for pair in STRING_TO_DIGIT {
            if s[i..].starts_with(pair.0) {
                s.replace_range(i..=i, &pair.1.to_string());
            }
        }
        i += 1;
    }
    s
}

fn calibration_values_sum_v2(calibration_values: &[&str]) -> u32 {
    calibration_values_sum(calibration_values, Some(conversion))
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let calibration_values = build(&input);

    println!("Part 1: {}", calibration_values_sum_v1(&calibration_values));
    println!("Part 2: {}", calibration_values_sum_v2(&calibration_values));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part1() {
        assert_eq!(calibration_values_sum_v1(&build(INPUT_TEST_1)), 142);
    }

    #[test]
    fn test_part2() {
        assert_eq!(calibration_values_sum_v2(&build(INPUT_TEST_2)), 281);
    }
}
