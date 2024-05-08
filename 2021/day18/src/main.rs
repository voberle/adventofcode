use std::{
    fmt,
    io::{self, Read}, ops::Add,
};

#[derive(Debug, Clone, PartialEq)]
enum SnailfishNb {
    Number(u32),
    Pair(Box<(SnailfishNb, SnailfishNb)>),
}

impl SnailfishNb {
    fn new(s: &str) -> Self {
        // Putting this in a vector of chars, as it's easier to manipulate by index.
        let chars: Vec<_> = s.chars().collect();
        let mut i = 0;
        SnailfishNb::build(&chars, &mut i)
    }

    fn build(s: &[char], i: &mut usize) -> Self {
        assert_eq!(s[*i], '[');
        *i += 1;

        let mut elements: Vec<SnailfishNb> = Vec::new();

        while *i < s.len() {
            match s[*i] {
                // Assumption: All numbers are single-digits.
                '0'..='9' => {
                    elements.push(SnailfishNb::Number(s[*i].to_digit(10).unwrap()));
                    *i += 1;
                }
                '[' => {
                    elements.push(SnailfishNb::build(s, i));
                }
                ']' => {
                    *i += 1;
                    break;
                }
                ',' => {
                    *i += 1;
                }
                _ => panic!("Invalid char"),
            }
        }
        assert_eq!(elements.len(), 2);
        SnailfishNb::Pair(Box::new((elements.swap_remove(0), elements.swap_remove(0))))
    }
}

impl fmt::Display for SnailfishNb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SnailfishNb::Number(v) => write!(f, "{}", v),
            SnailfishNb::Pair(p) => write!(f, "[{},{}]", p.0, p.1),
        }
    }
}

impl Add for SnailfishNb {
    type Output = SnailfishNb;

    fn add(self, other: SnailfishNb) -> SnailfishNb {
        SnailfishNb::Pair(Box::new(
            (
                self,
                other
            )
        ))
    }
}

fn build(input: &str) -> Vec<SnailfishNb> {
    input.lines().map(SnailfishNb::new).collect()
}

fn magnitude_final_sum(numbers: &[SnailfishNb]) -> i64 {
    0
}

fn part2(numbers: &[SnailfishNb]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let numbers = build(input.trim());
    // println!("{:#?}", numbers);

    println!("{}", input.trim());
    for n in &numbers {
        println!("{}", n);
    }

    println!("Part 1: {}", magnitude_final_sum(&numbers));
    println!("Part 2: {}", part2(&numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_build() {
        for line in INPUT_TEST.lines() {
            assert_eq!(SnailfishNb::new(line).to_string(), line);
        }
    }

    #[test]
    fn test_add() {
        let a = SnailfishNb::new("[1,2]");
        let b = SnailfishNb::new("[[3,4],5]");
        assert_eq!(a + b, SnailfishNb::new("[[1,2],[[3,4],5]]"));
    }

    #[test]
    fn test_magnitude_final_sum() {
        assert_eq!(magnitude_final_sum(&build("[[1,2],[[3,4],5]]")), 143);
        assert_eq!(
            magnitude_final_sum(&build("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")),
            1384
        );
        assert_eq!(
            magnitude_final_sum(&build("[[[[1,1],[2,2]],[3,3]],[4,4]]")),
            445
        );
        assert_eq!(
            magnitude_final_sum(&build("[[[[3,0],[5,3]],[4,4]],[5,5]]")),
            791
        );
        assert_eq!(
            magnitude_final_sum(&build("[[[[5,0],[7,4]],[5,5]],[6,6]]")),
            1137
        );
        assert_eq!(
            magnitude_final_sum(&build(
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            )),
            3488
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(magnitude_final_sum(&build(INPUT_TEST)), 4140);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
