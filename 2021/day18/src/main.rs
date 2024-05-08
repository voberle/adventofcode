use std::{
    fmt,
    io::{self, Read},
    ops::Add,
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

    fn get_regular_number(&self) -> u32 {
        match self {
            SnailfishNb::Number(v) => *v,
            SnailfishNb::Pair(_) => panic!("Requesting regular number of a pair"),
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
        SnailfishNb::Pair(Box::new((self, other)))
    }
}

fn build(input: &str) -> Vec<SnailfishNb> {
    input.lines().map(SnailfishNb::new).collect()
}

#[derive(Debug)]
struct ExplosionData {
    pair_index: usize,
    number_index: usize,
    left_number_to_add: u32,
    right_number_to_add: u32,
}

fn find_explosion_data(
    current: &SnailfishNb,
    level: usize,
    number_index: &mut usize,
    pair_index: &mut usize,
) -> Option<ExplosionData> {
    if level >= 4 {
        match current.clone() {
            SnailfishNb::Number(_) => {}
            SnailfishNb::Pair(p) => {
                if matches!(p.0, SnailfishNb::Number(_)) && matches!(p.1, SnailfishNb::Number(_)) {
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

    match current {
        SnailfishNb::Number(_) => {
            *number_index += 1;
        }
        SnailfishNb::Pair(p) => {
            *pair_index += 1;
            let explosion_data = find_explosion_data(&p.0, level + 1, number_index, pair_index);
            if explosion_data.is_some() {
                return explosion_data;
            }
            let explosion_data = find_explosion_data(&p.1, level + 1, number_index, pair_index);
            if explosion_data.is_some() {
                return explosion_data;
            }
        }
    }
    None
}

fn exec_explosion(
    current: &SnailfishNb,
    explosion_data: &ExplosionData,
    number_index: &mut usize,
    pair_index: &mut usize,
) -> SnailfishNb {
    match current {
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
                elements.push(exec_explosion(
                    &p.0,
                    explosion_data,
                    number_index,
                    pair_index,
                ));
                elements.push(exec_explosion(
                    &p.1,
                    explosion_data,
                    number_index,
                    pair_index,
                ));
                assert_eq!(elements.len(), 2);
                SnailfishNb::Pair(Box::new((elements.swap_remove(0), elements.swap_remove(0))))
            }
        }
    }
}

fn explode(number: &SnailfishNb) -> Option<SnailfishNb> {
    // Rust won't let us explore and modify the structure in place.
    // So we proceed in two phases: We first find what needs to be exploded, and then we do it by
    // building a new structure.

    let mut number_index = 0;
    let mut pair_index = 0;
    if let Some(explosion_data) = find_explosion_data(number, 0, &mut number_index, &mut pair_index)
    {
        number_index = 0;
        pair_index = 0;
        let exploded_number =
            exec_explosion(number, &explosion_data, &mut number_index, &mut pair_index);
        // println!("{}", exploded_number);
        Some(exploded_number)
    } else {
        None
    }
}

// split_done indicates if we did already one split, as we must do only the left most one.
fn exec_split(current: &SnailfishNb, split_done: &mut bool) -> SnailfishNb {
    match current {
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
            elements.push(exec_split(&p.0, split_done));
            elements.push(exec_split(&p.1, split_done));
            assert_eq!(elements.len(), 2);
            SnailfishNb::Pair(Box::new((elements.swap_remove(0), elements.swap_remove(0))))
        }
    }
}

fn split(number: &SnailfishNb) -> Option<SnailfishNb> {
    let mut split_done = false;
    let split_number = exec_split(number, &mut split_done);
    if split_number == *number {
        None
    } else {
        // println!("{}", split_number);
        Some(split_number)
    }
}

fn magnitude_final_sum(numbers: &[SnailfishNb]) -> i64 {
    for n in numbers {
        // n.explore(0);

        split(n);
        // explode(n);
    }
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
    fn test_explode() {
        assert_eq!(
            explode(&SnailfishNb::new("[[[[[9,8],1],2],3],4]")),
            Some(SnailfishNb::new("[[[[0,9],2],3],4]"))
        );
        assert_eq!(
            explode(&SnailfishNb::new("[7,[6,[5,[4,[3,2]]]]]")),
            Some(SnailfishNb::new("[7,[6,[5,[7,0]]]]"))
        );
        assert_eq!(
            explode(&SnailfishNb::new("[[6,[5,[4,[3,2]]]],1]")),
            Some(SnailfishNb::new("[[6,[5,[7,0]]],3]"))
        );
        assert_eq!(
            explode(&SnailfishNb::new("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")),
            Some(SnailfishNb::new("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"))
        );
        assert_eq!(
            explode(&SnailfishNb::new("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")),
            Some(SnailfishNb::new("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"))
        );
    }

    #[test]
    fn test_split() {
        assert_eq!(split(&SnailfishNb::Number(1)), None);
        assert_eq!(
            split(&SnailfishNb::Number(10)),
            Some(SnailfishNb::new("[5,5]"))
        );
        assert_eq!(
            split(&SnailfishNb::Number(11)),
            Some(SnailfishNb::new("[5,6]"))
        );
        assert_eq!(
            split(&SnailfishNb::Number(12)),
            Some(SnailfishNb::new("[6,6]"))
        );
        assert_eq!(split(&SnailfishNb::new("[[3,4],5]")), None);
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
