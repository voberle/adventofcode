use std::io::{self, Read};

use regex::Regex;

#[derive(Debug, PartialEq)]
struct Node {
    x: usize,
    y: usize,
    size: u32,
    used: u32,
    avail: u32,
    use_perc: u32,
}

#[inline]
fn int<T>(s: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    s.parse::<T>().unwrap()
}

fn build(input: &str) -> Vec<Node> {
    // Filesystem              Size  Used  Avail  Use%
    let re =
        Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%").unwrap();
    input
        .lines()
        .skip(2)
        .map(|line| {
            if let Some(parts) = re.captures(line) {
                Node {
                    x: int(&parts[1]),
                    y: int(&parts[2]),
                    size: int(&parts[3]),
                    used: int(&parts[4]),
                    avail: int(&parts[5]),
                    use_perc: int(&parts[6]),
                }
            } else {
                panic!("Invalid input {}", line)
            }
        })
        .collect()
}

fn is_viable_pair(a: &Node, b: &Node) -> bool {
    a.used != 0 && a != b && a.used <= b.avail
}

fn viable_pairs_count(nodes: &[Node]) -> usize {
    nodes
        .iter()
        .map(|a| nodes.iter().filter(move |b| is_viable_pair(a, b)).count())
        .sum()
}

fn part2(nodes: &[Node]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let nodes = build(&input);

    println!("Part 1: {}", viable_pairs_count(&nodes));
    println!("Part 2: {}", part2(&nodes));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(viable_pairs_count(&build("")), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build("")), 0);
    }
}
