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
    signal
        .iter()
        .map(|i| char::from_digit(*i as u32, 10).unwrap())
        .take(8)
        .collect()
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

// Take the first 7 digits of the input and convert them to a number.
#[allow(clippy::cast_sign_loss)]
fn get_message_offset(input: &[i32]) -> usize {
    input[0..7]
        .iter()
        .rev()
        .enumerate()
        .fold(0_i32, |acc, (i, d)| {
            acc + *d * 10_i32.pow(u32::try_from(i).unwrap())
        }) as usize
}

fn get_real_message(input: &[i32]) -> String {
    // Considering following:
    // If we want the nth digit, we only need to calculate things for t [n..],
    // and we can ignore all the digits before.
    // Also, if the offset is bigger than half the signal,
    // then all pattern digits we need to use are 1.

    // So we can find the answer as following:
    // 1) Find all the digits in the signal after the offset.
    // 2) As many times as there are digits:
    //    - Skip one more at the beginning each time.
    //    - Add them.
    //    - Take the last digit.
    // 3) Repeat 100 times.
    // 4) Take the first 8.

    const REPEAT_COUNT: usize = 10_000;
    const PHASES_COUNT: usize = 100;

    // Use the initial string for the offset.
    let offset = get_message_offset(input);

    // We use this assumption to avoid dealing with the pattern.
    assert!(offset > input.len() * REPEAT_COUNT / 2);

    // We should optimize this vec creation, to avoid allocating such a big vector for nothing.
    let mut signal = input.repeat(REPEAT_COUNT)[offset..].to_vec();

    for _phase in 0..PHASES_COUNT {
        // Initial more naive version.
        // signal = (0..signal.len())
        //     .map(|pos| {
        //         let sum: i32 = signal.iter().skip(pos).sum();
        //         // Keep last digit only
        //         sum.abs() % 10
        //     })
        //     .collect();

        // Optimized version that computes them from the end.
        for pos in (0..signal.len() - 1).rev() {
            let sum = signal[pos] + signal[pos + 1];
            signal[pos] = sum % 10;
        }
    }

    signal_to_string(&signal)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input_parsed = build(&input);

    println!("Part 1: {}", final_first_eight(&input_parsed));
    println!("Part 2: {}", get_real_message(&input_parsed));
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
    fn test_get_message_offset() {
        assert_eq!(get_message_offset(&build("0303673257721294")), 303673)
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            get_real_message(&build("03036732577212944063491565474664")),
            "84462026"
        );
        assert_eq!(
            get_real_message(&build("02935109699940807407585447034323")),
            "78725270"
        );
        assert_eq!(
            get_real_message(&build("03081770884921959731165446850517")),
            "53553731"
        );
    }
}
