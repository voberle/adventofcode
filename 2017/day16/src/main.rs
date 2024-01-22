use std::io::{self, Read};

#[derive(Debug)]
enum Moves {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}
use Moves::{Exchange, Partner, Spin};

impl Moves {
    fn exec(&self, programs: &mut [char]) {
        match self {
            Spin(x) => programs.rotate_right(*x),
            Exchange(pa, pb) => programs.swap(*pa, *pb),
            Partner(a, b) => {
                if let Some(pa) = programs.iter().position(|c| c == a) {
                    if let Some(pb) = programs.iter().position(|c| c == b) {
                        programs.swap(pa, pb);
                    }
                }
            }
        }
    }
}

#[inline]
fn char(s: &str) -> char {
    s.chars().next().unwrap()
}

#[inline]
fn int<T>(s: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    s.parse::<T>().unwrap()
}

fn build(input: &str) -> Vec<Moves> {
    input
        .split(',')
        .map(|s| match &s[0..1] {
            "s" => Spin(int(&s[1..])),
            "x" => {
                let p: Vec<_> = s[1..].split('/').collect();
                Exchange(int(p[0]), int(p[1]))
            }
            "p" => {
                let p: Vec<_> = s[1..].split('/').collect();
                Partner(char(p[0]), char(p[1]))
            }
            _ => panic!("Invalid input"),
        })
        .collect()
}

fn programs_to_string(programs: &[char]) -> String {
    programs.iter().collect()
}

fn string_to_programs(s: &str) -> Vec<char> {
    s.chars().collect()
}

fn dance(moves: &[Moves], programs: &[char]) -> String {
    let mut programs = programs.to_vec();
    for m in moves {
        m.exec(&mut programs);
    }
    programs_to_string(&programs)
}

fn part2(moves: &[Moves]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let moves = build(&input);

    let programs = string_to_programs("abcdefghijklmnop");

    println!("Part 1: {}", dance(&moves, &programs));
    println!("Part 2: {}", part2(&moves));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(
            dance(&build(INPUT_TEST), &string_to_programs("abcde")),
            "baedc"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
