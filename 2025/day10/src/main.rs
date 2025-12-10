use std::io::{self, Read};

#[derive(Debug, PartialEq)]
struct LightDiagram(u32);

impl LightDiagram {
    fn build(s: &str) -> Self {
        Self(s[1..s.len() - 1].chars().fold(0, |acc, c| {
            println!("acc={acc}, c={c}");
            acc * 2
                + match c {
                    '.' => 0,
                    '#' => 1,
                    _ => panic!("Invalid light char"),
                }
        }))
    }
}

#[derive(Debug)]
struct WiringSchematic(Vec<Vec<usize>>);

impl WiringSchematic {
    fn build(v: &[&str]) -> Self {
        Self(
            v.iter()
                .map(|s| {
                    s[1..s.len() - 1]
                        .split(',')
                        .map(|p| p.parse().unwrap())
                        .collect()
                })
                .collect(),
        )
    }
}

#[derive(Debug)]
struct JoltageReqs(Vec<u32>);

impl JoltageReqs {
    fn build(s: &str) -> Self {
        Self(
            s[1..s.len() - 1]
                .split(',')
                .map(|p| p.parse().unwrap())
                .collect(),
        )
    }
}

fn build(input: &str) -> (Vec<LightDiagram>, Vec<WiringSchematic>, Vec<JoltageReqs>) {
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split_ascii_whitespace().collect();
            (
                LightDiagram::build(parts[0]),
                WiringSchematic::build(&parts[1..parts.len() - 1]),
                JoltageReqs::build(parts[parts.len() - 1]),
            )
        })
        .collect()
}

fn fewest_presses(light_diagrams: &[LightDiagram], wiring_schematics: &[WiringSchematic]) -> usize {
    0
}

fn part2(
    light_diagrams: &[LightDiagram],
    wiring_schematics: &[WiringSchematic],
    joltage_reqs: &[JoltageReqs],
) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (light_diagrams, wiring_schematics, joltage_reqs) = build(&input);

    println!(
        "Part 1: {}",
        fewest_presses(&light_diagrams, &wiring_schematics)
    );
    println!(
        "Part 2: {}",
        part2(&light_diagrams, &wiring_schematics, &joltage_reqs)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_light_diagrams() {
        assert_eq!(LightDiagram::build("[.##.]"), LightDiagram(6));
    }

    #[test]
    fn test_part1() {
        let (light_diagrams, wiring_schematics, _) = build(&INPUT_TEST);
        assert_eq!(fewest_presses(&light_diagrams, &wiring_schematics), 7);
    }

    #[test]
    fn test_part2() {
        let (light_diagrams, wiring_schematics, joltage_reqs) = build(&INPUT_TEST);
        assert_eq!(part2(&light_diagrams, &wiring_schematics, &joltage_reqs), 0);
    }
}
