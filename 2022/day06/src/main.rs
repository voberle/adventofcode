use std::io::{self, Read};

fn build(input: &str) -> Vec<char> {
    input.chars().collect()
}

// O(n^2) version.
fn _all_different(w: &[char]) -> bool {
    for i in 0..w.len() {
        if w[i + 1..].contains(&w[i]) {
            return false;
        }
    }
    true
}

// Input is limited to the 26 lowercase alphabet chars,
// so we can have such an optimized version in O(n).
fn all_different(w: &[char]) -> bool {
    let mut seen = 0u32; // 32-bit integer to represent 26 letters
    w.iter().all(|&ch| {
        let bit = 1 << (ch as u8 - b'a');
        if seen & bit != 0 {
            false
        } else {
            seen |= bit;
            true
        }
    })
}

fn start_marker(ds: &[char], distinct_cnt: usize) -> usize {
    ds.windows(distinct_cnt).position(all_different).unwrap() + distinct_cnt
}

fn start_packet_marker(ds: &[char]) -> usize {
    start_marker(ds, 4)
}

fn start_message_marker(ds: &[char]) -> usize {
    start_marker(ds, 14)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let ds = build(&input);

    println!("Part 1: {}", start_packet_marker(&ds));

    // let now = std::time::Instant::now();
    println!("Part 2: {}", start_message_marker(&ds));
    // println!("Execution time: {:.2?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const INPUT_TEST_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const INPUT_TEST_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const INPUT_TEST_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const INPUT_TEST_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_part1() {
        assert_eq!(start_packet_marker(&build(INPUT_TEST_1)), 7);
        assert_eq!(start_packet_marker(&build(INPUT_TEST_2)), 5);
        assert_eq!(start_packet_marker(&build(INPUT_TEST_3)), 6);
        assert_eq!(start_packet_marker(&build(INPUT_TEST_4)), 10);
        assert_eq!(start_packet_marker(&build(INPUT_TEST_5)), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(start_message_marker(&build(INPUT_TEST_1)), 19);
        assert_eq!(start_message_marker(&build(INPUT_TEST_2)), 23);
        assert_eq!(start_message_marker(&build(INPUT_TEST_3)), 23);
        assert_eq!(start_message_marker(&build(INPUT_TEST_4)), 29);
        assert_eq!(start_message_marker(&build(INPUT_TEST_5)), 26);
    }
}
