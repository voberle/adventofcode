use std::io::{self, Read};

fn build(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn resulting_frequency(changes: &[i64]) -> i64 {
    changes.iter().sum()
}

fn part2(changes: &[i64]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let changes = build(&input);

    println!("Part 1: {}", resulting_frequency(&changes));
    println!("Part 2: {}", part2(&changes));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            resulting_frequency(&build(&"+1, +1, +1".replace(", ", "\n"))),
            3
        );
        assert_eq!(
            resulting_frequency(&build(&"+1, +1, -2".replace(", ", "\n"))),
            0
        );
        assert_eq!(
            resulting_frequency(&build(&"-1, -2, -3".replace(", ", "\n"))),
            -6
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build("")), 0);
    }
}
