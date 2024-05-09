use std::{
    fmt,
    io::{self, Read},
    ops::Add,
};

use itertools::Itertools;

// Helper struct for explosion support.
#[derive(Debug)]
struct ExplosionData {
    pair_index: usize,
    number_index: usize,
    left_number_to_add: u32,
    right_number_to_add: u32,
}

// Recursive data structure representing an Snailfish number.
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

    fn get_regular_number(&self) -> u32 {
        match self {
            SnailfishNb::Number(v) => *v,
            SnailfishNb::Pair(_) => panic!("Requesting regular number of a pair"),
        }
    }

    // Finds what to explode.
    fn find_explosion_data(
        &self,
        level: usize,
        number_index: &mut usize,
        pair_index: &mut usize,
    ) -> Option<ExplosionData> {
        if level >= 4 {
            match self {
                SnailfishNb::Number(_) => {}
                SnailfishNb::Pair(p) => {
                    if matches!(p.0, SnailfishNb::Number(_))
                        && matches!(p.1, SnailfishNb::Number(_))
                    {
                        // BOOM!
                        return Some(ExplosionData {
                            pair_index: *pair_index,
                            number_index: *number_index,
                            left_number_to_add: p.0.get_regular_number(),
                            right_number_to_add: p.1.get_regular_number(),
                        });
                    }
                }
            }
        }

        match self {
            SnailfishNb::Number(_) => {
                *number_index += 1;
            }
            SnailfishNb::Pair(p) => {
                *pair_index += 1;
                let explosion_data = p.0.find_explosion_data(level + 1, number_index, pair_index);
                if explosion_data.is_some() {
                    return explosion_data;
                }
                let explosion_data = p.1.find_explosion_data(level + 1, number_index, pair_index);
                if explosion_data.is_some() {
                    return explosion_data;
                }
            }
        }
        None
    }

    // Executes the explosion based on the explosion data.
    fn exec_explosion(
        &self,
        explosion_data: &ExplosionData,
        number_index: &mut usize,
        pair_index: &mut usize,
    ) -> SnailfishNb {
        match self {
            SnailfishNb::Number(v) => {
                *number_index += 1;
                if *number_index == explosion_data.number_index {
                    SnailfishNb::Number(*v + explosion_data.left_number_to_add)
                } else if *number_index == explosion_data.number_index + 1 {
                    SnailfishNb::Number(*v + explosion_data.right_number_to_add)
                } else {
                    SnailfishNb::Number(*v)
                }
            }
            SnailfishNb::Pair(p) => {
                if *pair_index == explosion_data.pair_index {
                    *pair_index += 1;
                    SnailfishNb::Number(0)
                } else {
                    let mut elements: Vec<SnailfishNb> = Vec::new();
                    *pair_index += 1;
                    elements.push(p.0.exec_explosion(explosion_data, number_index, pair_index));
                    elements.push(p.1.exec_explosion(explosion_data, number_index, pair_index));
                    assert_eq!(elements.len(), 2);
                    SnailfishNb::Pair(Box::new((elements.swap_remove(0), elements.swap_remove(0))))
                }
            }
        }
    }

    // Explodes the number, if necessary.
    fn explode(&self) -> Option<SnailfishNb> {
        // Rust won't let us explore and modify the structure in place.
        // So we proceed in two phases: We first find what needs to be exploded, and then we do it by
        // building a new structure.

        let mut number_index = 0;
        let mut pair_index = 0;
        if let Some(explosion_data) =
            self.find_explosion_data(0, &mut number_index, &mut pair_index)
        {
            number_index = 0;
            pair_index = 0;
            let exploded_number =
                self.exec_explosion(&explosion_data, &mut number_index, &mut pair_index);
            Some(exploded_number)
        } else {
            None
        }
    }

    // Goes through the structure, and executes a split if necessary.
    // split_done indicates if we did already one split, as we must do only the left most one.
    fn exec_split(&self, split_done: &mut bool) -> SnailfishNb {
        match self {
            SnailfishNb::Number(v) => {
                if *v >= 10 && !*split_done {
                    // Split
                    *split_done = true;
                    SnailfishNb::Pair(Box::new((
                        SnailfishNb::Number(v / 2),
                        SnailfishNb::Number(v / 2 + v % 2),
                    )))
                } else {
                    SnailfishNb::Number(*v)
                }
            }
            SnailfishNb::Pair(p) => {
                let mut elements: Vec<SnailfishNb> = Vec::new();
                elements.push(p.0.exec_split(split_done));
                elements.push(p.1.exec_split(split_done));
                assert_eq!(elements.len(), 2);
                SnailfishNb::Pair(Box::new((elements.swap_remove(0), elements.swap_remove(0))))
            }
        }
    }

    // Splits the number if necessary.
    fn split(&self) -> Option<SnailfishNb> {
        let mut split_done = false;
        let split_number = self.exec_split(&mut split_done);
        if split_number == *self {
            None
        } else {
            // println!("{}", split_number);
            Some(split_number)
        }
    }

    fn reduce(&self) -> SnailfishNb {
        let mut number = self.clone();
        // println!("after addition: {}", number);
        loop {
            if let Some(exploded) = number.explode() {
                number = exploded;
                // println!("after explode:  {}", number);
                continue;
            }
            if let Some(split) = number.split() {
                number = split;
                // println!("after split:    {}", number);
                continue;
            }
            break;
        }
        number
    }

    fn magnitude(&self) -> u32 {
        match self {
            SnailfishNb::Number(v) => *v,
            SnailfishNb::Pair(p) => 3 * p.0.magnitude() + 2 * p.1.magnitude(),
        }
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
        let added = SnailfishNb::Pair(Box::new((self, other)));
        added.reduce()
    }
}

fn build(input: &str) -> Vec<SnailfishNb> {
    input.lines().map(SnailfishNb::new).collect()
}

fn final_sum(numbers: &[SnailfishNb]) -> SnailfishNb {
    numbers
        .iter()
        .skip(1)
        .fold(numbers[0].clone(), |acc, e| acc + e.clone())
    // numbers.iter().skip(1).fold(numbers[0].clone(), |acc, e| {
    //     println!("  {}", acc);
    //     println!("+ {}", e);
    //     let s = acc + e.clone();
    //     println!("= {}", s);
    //     println!();
    //     s
    // })
}

fn magnitude_final_sum(numbers: &[SnailfishNb]) -> u32 {
    let sum = final_sum(numbers);
    sum.magnitude()
}

fn largest_magnitude_any_sum(numbers: &[SnailfishNb]) -> u32 {
    numbers
        .iter()
        .permutations(2)
        .map(|p| (p[0].clone() + p[1].clone()).magnitude())
        .max()
        .unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let numbers = build(input.trim());
    // println!("{:#?}", numbers);

    // println!("{}", input.trim());
    // for n in &numbers {
    //     println!("{}", n);
    // }

    println!("Part 1: {}", magnitude_final_sum(&numbers));
    println!("Part 2: {}", largest_magnitude_any_sum(&numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_build() {
        for line in INPUT_TEST_2.lines() {
            assert_eq!(SnailfishNb::new(line).to_string(), line);
        }
    }

    #[test]
    fn test_add_no_reduction() {
        let a = SnailfishNb::new("[1,2]");
        let b = SnailfishNb::new("[[3,4],5]");
        assert_eq!(a + b, SnailfishNb::new("[[1,2],[[3,4],5]]"));
    }

    #[test]
    fn test_explode() {
        assert_eq!(
            SnailfishNb::new("[[[[[9,8],1],2],3],4]").explode(),
            Some(SnailfishNb::new("[[[[0,9],2],3],4]"))
        );
        assert_eq!(
            SnailfishNb::new("[7,[6,[5,[4,[3,2]]]]]").explode(),
            Some(SnailfishNb::new("[7,[6,[5,[7,0]]]]"))
        );
        assert_eq!(
            SnailfishNb::new("[[6,[5,[4,[3,2]]]],1]").explode(),
            Some(SnailfishNb::new("[[6,[5,[7,0]]],3]"))
        );
        assert_eq!(
            SnailfishNb::new("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").explode(),
            Some(SnailfishNb::new("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"))
        );
        assert_eq!(
            SnailfishNb::new("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").explode(),
            Some(SnailfishNb::new("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"))
        );
    }

    #[test]
    fn test_split() {
        assert_eq!(SnailfishNb::Number(1).split(), None);
        assert_eq!(
            SnailfishNb::Number(10).split(),
            Some(SnailfishNb::new("[5,5]"))
        );
        assert_eq!(
            SnailfishNb::Number(11).split(),
            Some(SnailfishNb::new("[5,6]"))
        );
        assert_eq!(
            SnailfishNb::Number(12).split(),
            Some(SnailfishNb::new("[6,6]"))
        );
        assert_eq!(SnailfishNb::new("[[3,4],5]").split(), None);
    }

    #[test]
    fn test_reduce() {
        assert_eq!(
            SnailfishNb::new("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").reduce(),
            SnailfishNb::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        );
    }

    #[test]
    fn test_add_with_reduction() {
        let a = SnailfishNb::new("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let b = SnailfishNb::new("[1,1]]");
        assert_eq!(a + b, SnailfishNb::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    }

    #[test]
    fn test_final_sum() {
        assert_eq!(
            final_sum(&build("[1,1]\n[2,2]\n[3,3]\n[4,4]")),
            SnailfishNb::new("[[[[1,1],[2,2]],[3,3]],[4,4]]")
        );
        assert_eq!(
            final_sum(&build("[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]")),
            SnailfishNb::new("[[[[3,0],[5,3]],[4,4]],[5,5]]")
        );
        assert_eq!(
            final_sum(&build("[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]")),
            SnailfishNb::new("[[[[5,0],[7,4]],[5,5]],[6,6]]")
        );
    }

    #[test]
    fn test_final_sum_large_1() {
        assert_eq!(
            final_sum(&build(INPUT_TEST_1)),
            SnailfishNb::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        );
    }

    #[test]
    fn test_final_sum_large_2() {
        assert_eq!(
            final_sum(&build(INPUT_TEST_2)),
            SnailfishNb::new("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
        );
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(SnailfishNb::new("[[1,2],[[3,4],5]]").magnitude(), 143);
        assert_eq!(
            SnailfishNb::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude(),
            1384
        );
        assert_eq!(
            SnailfishNb::new("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude(),
            445
        );
        assert_eq!(
            SnailfishNb::new("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude(),
            791
        );
        assert_eq!(
            SnailfishNb::new("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude(),
            1137
        );
        assert_eq!(
            SnailfishNb::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(),
            3488
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(magnitude_final_sum(&build(INPUT_TEST_2)), 4140);
    }

    #[test]
    fn test_part2() {
        assert_eq!(largest_magnitude_any_sum(&build(INPUT_TEST_2)), 3993);
    }
}
