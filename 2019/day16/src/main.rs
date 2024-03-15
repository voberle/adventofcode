use std::{
    io::{self, Read},
    iter,
};

fn build(input: &str) -> Vec<i32> {
    input
        .chars()
        // to_digits is an alternative.
        .map(|c| c as i32 - 0x30) // 0x30 is '0'. 
        .collect()
}

#[allow(clippy::cast_sign_loss)]
fn signal_to_string(signal: &[i32]) -> String {
    // First 8 digits only.
    signal.iter().map(|i| char::from_digit(*i as u32, 10).unwrap()).take(8).collect()
}

fn final_first_eight(input: &[i32]) -> String {
    const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];

    let mut signal = input.to_vec();
    // Could have used iter::successors, but it made things harder to read.
    for _phase in 0..100 {
        signal = (1..=signal.len())
            .map(|repeat_factor| {
                let mut pattern_it = BASE_PATTERN
                    .iter()
                    .flat_map(|i| iter::repeat(i).take(repeat_factor))
                    .cycle()
                    .skip(1);

                let sum: i32 = signal.iter().map(|i| i * pattern_it.next().unwrap()).sum();
                // Keep last digit only
                sum.abs() % 10
            })
            .collect();
    }
    signal_to_string(&signal)
}

fn part2(input: &[i32]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input_parsed = build(&input);

    println!("Part 1: {}", final_first_eight(&input_parsed));
    println!("Part 2: {}", part2(&input_parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_final_first_eight() {
        assert_eq!(
            final_first_eight(&build("80871224585914546619083218645595")),
            "24176176"
        );
        assert_eq!(
            final_first_eight(&build("19617804207202209144916044189917")),
            "73745418"
        );
        assert_eq!(
            final_first_eight(&build("69317163492948606335995924319873")),
            "52432133"
        );
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
