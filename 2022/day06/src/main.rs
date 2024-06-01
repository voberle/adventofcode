use std::io::{self, Read};

fn build(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn all_different(w: &[char]) -> bool {
    w[0] != w[1] && w[0] != w[2] && w[0] != w[3] && w[1] != w[2] && w[1] != w[3] && w[2] != w[3]
}

fn start_packet_marker(ds: &[char]) -> usize {
    ds.windows(4).position(all_different).unwrap() + 4
}

fn part2(ds: &[char]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let ds = build(&input);

    println!("Part 1: {}", start_packet_marker(&ds));
    println!("Part 2: {}", part2(&ds));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = "";

    #[test]
    fn test_part1() {
        let ds = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(start_packet_marker(&build(ds)), 7);
        let ds = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(start_packet_marker(&build(ds)), 5);
        let ds = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(start_packet_marker(&build(ds)), 6);
        let ds = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(start_packet_marker(&build(ds)), 10);
        let ds = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(start_packet_marker(&build(ds)), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
