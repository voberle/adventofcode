use std::{
    collections::BinaryHeap,
    io::{self, Read},
};

use fxhash::{FxHashMap, FxHashSet};

// Lowest bit is on the left (aka first light).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct LightDiagram(u32);

impl LightDiagram {
    fn build(s: &str) -> Self {
        Self(s[1..s.len() - 1].chars().rev().fold(0, |acc, c| {
            acc * 2
                + match c {
                    '.' => 0,
                    '#' => 1,
                    _ => panic!("Invalid light char"),
                }
        }))
    }

    fn toggle(self, buttons: Buttons) -> Self {
        Self(self.0 ^ buttons.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Buttons(u32);

impl Buttons {
    fn build(press: &str) -> Self {
        let mut value = 0;
        for p in press[1..press.len() - 1].split(',') {
            value |= 1u32 << p.parse::<u32>().unwrap();
        }
        Self(value)
    }
}

#[derive(Debug, PartialEq)]
struct WiringSchematic(Vec<Buttons>);

impl WiringSchematic {
    fn build(presses: &[&str]) -> Self {
        Self(presses.iter().map(|press| Buttons::build(press)).collect())
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

struct Machine {
    light_diagrams: LightDiagram,
    wiring_schematics: WiringSchematic,
    joltage_reqs: JoltageReqs,
}

impl Machine {
    fn build(line: &str) -> Machine {
        let parts: Vec<_> = line.split_ascii_whitespace().collect();
        Machine {
            light_diagrams: LightDiagram::build(parts[0]),
            wiring_schematics: WiringSchematic::build(&parts[1..parts.len() - 1]),
            joltage_reqs: JoltageReqs::build(parts[parts.len() - 1]),
        }
    }
}

fn build(input: &str) -> Vec<Machine> {
    input.lines().map(Machine::build).collect()
}

// Node we are exploring with Dijkstra.
#[derive(Debug, PartialEq, Eq)]
struct Node {
    lights: LightDiagram,
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra shortest path.
fn find_shortest_path(end: LightDiagram, wiring_schematic: &WiringSchematic) -> usize {
    let start = LightDiagram(0);

    let mut visited: FxHashSet<LightDiagram> = FxHashSet::default();
    let mut distance: FxHashMap<LightDiagram, usize> = FxHashMap::default();
    let mut shortest_distance = usize::MAX;

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(Node {
        lights: start,
        cost: 0,
    });

    while let Some(Node { lights, cost }) = queue.pop() {
        visited.insert(lights);

        if lights == end {
            shortest_distance = shortest_distance.min(cost);
            continue;
        }

        queue.extend(wiring_schematic.0.iter().filter_map(|buttons| {
            let next_lights = lights.toggle(*buttons);

            if visited.contains(&next_lights) {
                return None;
            }

            let next_cost = cost + 1;
            if let Some(prevcost) = distance.get(&next_lights)
                && *prevcost <= next_cost
            {
                return None;
            }

            distance.insert(next_lights, next_cost);
            Some(Node {
                lights: next_lights,
                cost: next_cost,
            })
        }));
    }
    shortest_distance
}

fn fewest_presses_for_machine(machine: &Machine) -> usize {
    find_shortest_path(machine.light_diagrams, &machine.wiring_schematics)
}

fn fewest_presses(machines: &[Machine]) -> usize {
    machines.iter().map(fewest_presses_for_machine).sum()
}

fn part2(machines: &[Machine]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let machines = build(&input);

    println!("Part 1: {}", fewest_presses(&machines));
    println!("Part 2: {}", part2(&machines));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_light_diagrams() {
        assert_eq!(LightDiagram::build("[.##.]"), LightDiagram(6));
        assert_eq!(LightDiagram::build("[...#.]"), LightDiagram(8));
    }

    #[test]
    fn test_buttons() {
        assert_eq!(Buttons::build("(1,3)"), Buttons(10));
    }

    #[test]
    fn test_wiring_schematic() {
        assert_eq!(
            WiringSchematic::build(&["(1,3)"]),
            WiringSchematic(vec![Buttons(10)])
        );
    }

    #[test]
    fn test_toggle() {
        let lights = LightDiagram::build("[#.....]");
        let buttons = Buttons::build("(0,3,4)");
        assert_eq!(lights.toggle(buttons), LightDiagram::build("[...##.]"));
    }

    #[test]
    fn test_part1() {
        assert_eq!(fewest_presses(&build(INPUT_TEST)), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
