use std::{
    fmt,
    io::{self, Read},
};

#[derive(Debug, Clone, PartialEq)]
struct Component {
    p1: usize,
    p2: usize,
}

impl Component {
    fn new(p1: usize, p2: usize) -> Self {
        Self { p1, p2 }
    }

    fn strength(&self) -> usize {
        self.p1 + self.p2
    }

    fn other_port(&self, port: usize) -> usize {
        if self.p1 == port {
            self.p2
        } else if self.p2 == port {
            self.p1
        } else {
            panic!("other_port: Port {port} invalid for {self}");
        }
    }
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.p1, self.p2)
    }
}

fn build(input: &str) -> Vec<Component> {
    input
        .lines()
        .map(|line| {
            let p: Vec<_> = line.split('/').collect();
            Component::new(p[0].parse().unwrap(), p[1].parse().unwrap())
        })
        .collect()
}

#[allow(dead_code)]
fn bridge_to_string(bridge: &[&Component]) -> String {
    bridge
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("--")
}

fn bridge_strength(bridge: &[&Component]) -> usize {
    bridge.iter().map(|c| c.strength()).sum()
}

// Convert the component list into a structure easier to walk through:
// A vector where at each index are the components that match that index.
//
// Note that we assume there are no duplicate in the list (there aren't in mine).
fn convert_components_list(components: &[Component]) -> Vec<Vec<&Component>> {
    let max_component_nb = components.iter().flat_map(|c| [c.p1, c.p2]).max().unwrap();
    let mut mapped: Vec<Vec<&Component>> = vec![Vec::new(); max_component_nb + 1];
    for c in components {
        mapped[c.p1].push(c);
        mapped[c.p2].push(c);
    }
    mapped
}

// Recursive function.
fn build_bridge<'a>(
    components: &[Vec<&'a Component>],
    port_to_connect: usize,
    current_bridge: &[&'a Component],
    current_strength: usize,
    bridges: &mut Vec<Vec<&'a Component>>, // All bridges we've found (as long as possible)
) {
    // Find all components that could be connected to the last one and that are not yet in the bridge
    let possible_connections: Vec<_> = components[port_to_connect]
        .iter()
        .filter(|c| !current_bridge.contains(c))
        .collect();

    if possible_connections.is_empty() {
        bridges.push(current_bridge.to_vec());
        return;
    }

    // Try to build a bridge with each possibility.
    for c in possible_connections {
        let mut new_bridge = current_bridge.to_vec();
        new_bridge.push(c);
        let new_strength = current_strength + c.strength();

        build_bridge(
            components,
            c.other_port(port_to_connect),
            &new_bridge,
            new_strength,
            bridges,
        );
    }
}

// First value is the strength of the strongest bridge.
// Second value is the strength of the longest bridge.
fn bridges_strength<'a>(components: &[Vec<&'a Component>]) -> (usize, usize) {
    let mut bridges: Vec<Vec<&'a Component>> = Vec::new();
    for c in &components[0] {
        build_bridge(
            components,
            c.other_port(0),
            &[*c],
            c.strength(),
            &mut bridges,
        );
    }

    let strongest = bridges.iter().map(|b| bridge_strength(b)).max().unwrap();
    let max_len = bridges.iter().map(Vec::len).max().unwrap();
    let longest = bridges
        .iter()
        .filter(|b| b.len() == max_len)
        .map(|b| bridge_strength(b))
        .max()
        .unwrap();

    (strongest, longest)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let components = build(&input);

    let mapped = convert_components_list(&components);

    let (strongest, longest) = bridges_strength(&mapped);
    println!("Part 1: {strongest}");
    println!("Part 2: {longest}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1_2() {
        let components = build(INPUT_TEST);
        let mapped = convert_components_list(&components);
        let (strongest, longest) = bridges_strength(&mapped);

        assert_eq!(strongest, 31);
        assert_eq!(longest, 19);
    }
}
