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
            panic!("other_port: Port {} invalid for {}", port, self);
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

// fn bridge_strength(bridge: &[&Component]) -> usize {
//     bridge.iter().map(|c| c.strength()).sum()
// }

// Convert the component list into a structure easier to walk through:
// A vector where at each index are the components that match that index.
fn convert_components_list(components: &[Component]) -> Vec<Vec<&Component>> {
    let max_component_nb = components.iter().flat_map(|c| [c.p1, c.p2]).max().unwrap();
    let mut mapped: Vec<Vec<&Component>> = vec![Vec::new(); max_component_nb + 1];
    for c in components {
        mapped[c.p1].push(c);
        mapped[c.p2].push(c);
    }
    mapped
}

// Recursive function
fn build_bridge(
    components: &[Vec<&Component>],
    port_to_connect: usize,
    current_bridge: &[&Component],
    current_strength: usize,
) -> usize {
    // Find all components that could be connected to the last one and that are not yet in the bridge
    let possible_connections: Vec<_> = components[port_to_connect]
        .iter()
        .filter(|c| !current_bridge.contains(c))
        .collect();

    if possible_connections.is_empty() {
        return current_strength;
    }

    // Try to build a bridge with each possibility.
    let mut max_strength = 0;
    for c in possible_connections {
        let mut new_bridge = current_bridge.to_vec();
        new_bridge.push(c);
        let new_strength = current_strength + c.strength();
        // println!("Bridge s={}: {}", new_strength, bridge_to_string(&new_bridge));

        let strength = build_bridge(
            components,
            c.other_port(port_to_connect),
            &new_bridge,
            new_strength,
        );
        max_strength = max_strength.max(strength);
    }
    max_strength
}

fn strongest_bridge_strength(components: &[Vec<&Component>]) -> usize {
    let mut max_strength = 0;
    for c in &components[0] {
        let strength = build_bridge(components, c.other_port(0), &[*c], c.strength());
        max_strength = max_strength.max(strength);
    }
    max_strength
}

fn part2(components: &[Component]) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let components = build(&input);

    let mapped = convert_components_list(&components);

    println!("Part 1: {}", strongest_bridge_strength(&mapped));
    println!("Part 2: {}", part2(&components));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let components = build(INPUT_TEST);
        let mapped = convert_components_list(&components);
        assert_eq!(strongest_bridge_strength(&mapped), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
