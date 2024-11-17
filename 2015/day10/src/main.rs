use std::io::{self, Read};

fn look_and_say(s: &str) -> String {
    let mut result = String::new();
    let mut count = 0;
    let mut iter = s.chars().peekable();
    while let Some(c) = iter.next() {
        count += 1;
        if let Some(&next) = iter.peek() {
            if next != c {
                result.push_str(&format!("{count}{c}"));
                count = 0;
            }
        } else {
            result.push_str(&format!("{count}{c}"));
        }
    }
    result
}

fn recursive_look_and_say<const COUNT: usize>(input: &str) -> usize {
    assert!(!input.is_empty());
    (0..COUNT)
        .fold(input.to_string(), |acc, _| look_and_say(&acc))
        .len()
}

fn part1(input: &str) -> usize {
    recursive_look_and_say::<40>(input)
}

fn part2(input: &str) -> usize {
    recursive_look_and_say::<50>(input)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_look_and_say() {
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");
    }

    #[test]
    fn test_recursive() {
        assert_eq!(recursive_look_and_say::<5>("1"), 6);
    }
}
