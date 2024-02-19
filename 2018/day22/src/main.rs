use std::io::{self, Read};

use fxhash::{FxHashMap, FxHashSet};
use std::collections::BinaryHeap;

mod cave;

use cave::{Cave, RegionType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    const ZERO: Pos = Pos { x: 0, y: 0 };

    fn new(x: usize, y: usize) -> Self {
        Pos { x, y }
    }

    fn is_direction_allowed(&self, dir: Direction) -> bool {
        !(self.x == 0 && dir == West || self.y == 0 && dir == North)
    }

    fn next_pos(&self, dir: Direction) -> Pos {
        assert!(self.is_direction_allowed(dir));
        match dir {
            North => Pos::new(self.x, self.y - 1),
            South => Pos::new(self.x, self.y + 1),
            West => Pos::new(self.x - 1, self.y),
            East => Pos::new(self.x + 1, self.y),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::{East, North, South, West};

const ALL_DIRECTIONS: [Direction; 4] = [North, East, South, West];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither,
}

impl Tool {
    // Does our tool allow to enter this region?
    // The new equipment must be valid for the current region as well.
    fn allows_to_go(self, from: RegionType, region: RegionType) -> bool {
        tools_for_region(from).contains(&self) && tools_for_region(region).contains(&self)
    }
}

fn tools_for_region(region: RegionType) -> [Tool; 2] {
    match region {
        RegionType::Rocky(_) => [Tool::ClimbingGear, Tool::Torch],
        RegionType::Wet(_) => [Tool::ClimbingGear, Tool::Neither],
        RegionType::Narrow(_) => [Tool::Torch, Tool::Neither],
    }
}

// Node we are exploring with Dijkstra.
#[derive(Debug, PartialEq, Eq)]
struct Node {
    pos: Pos,
    tool: Tool,
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
fn find_shortest_path(cave: &Cave) -> usize {
    let start = Pos::ZERO;
    let target = cave.get_target();
    let starting_tool = Tool::Torch;

    let mut visited: FxHashSet<(Pos, Tool)> = FxHashSet::default();
    let mut distance: FxHashMap<(Pos, Tool), usize> = FxHashMap::default();
    let mut shortest_distance = usize::MAX;

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(Node {
        pos: start,
        tool: starting_tool,
        cost: 0,
    });

    while let Some(Node { pos, tool, cost }) = queue.pop() {
        visited.insert((pos, tool));

        if pos == target {
            assert!(matches!(cave.get_region(&pos), RegionType::Rocky(_)));
            // Make sure we have the torch when we reach the target.
            let cost_to_target = if tool == Tool::Torch { 0 } else { 7 };
            shortest_distance = usize::min(shortest_distance, cost + cost_to_target);
            continue;
        }

        queue.extend(
            ALL_DIRECTIONS
                .iter()
                .flat_map(|d| {
                    // For each direction, we will end up with 3 options, as we can go with any of the 3 tools.
                    if !pos.is_direction_allowed(*d) {
                        return vec![];
                    }
                    let next_pos = pos.next_pos(*d);

                    let current_region = cave.get_region(&pos);
                    let next_region = cave.get_region(&next_pos);

                    if tool.allows_to_go(current_region, next_region) {
                        vec![(next_pos, tool, 0)]
                    } else {
                        tools_for_region(next_region)
                            .iter()
                            .filter(|t| t.allows_to_go(current_region, next_region)) // Make sure the tool allows us to go from current to next
                            .map(|t| (next_pos, *t, 7))
                            .collect::<Vec<_>>()
                    }
                })
                .filter_map(|(next_pos, next_tool, tool_cost)| {
                    if visited.contains(&(next_pos, next_tool)) {
                        return None;
                    }

                    let next_cost = cost + 1 + tool_cost;

                    // No need to explore further than the shortest distance we have found.
                    // This is important since our map can grow indefinitely.
                    if next_cost > shortest_distance {
                        return None;
                    }

                    if let Some(prevcost) = distance.get(&(next_pos, next_tool)) {
                        if *prevcost <= next_cost {
                            return None;
                        }
                    }
                    distance.insert((next_pos, next_tool), next_cost);

                    Some(Node {
                        pos: next_pos,
                        tool: next_tool,
                        cost: next_cost,
                    })
                }),
        );
    }
    shortest_distance
}

fn target_reached_in(cave: &Cave) -> usize {
    find_shortest_path(cave)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let cave = Cave::new(&input);

    println!("Part 1: {}", cave.risk_level());
    println!("Part 2: {}", target_reached_in(&cave));
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let cave = Cave::new(INPUT_TEST);
        assert_eq!(cave.risk_level(), 114);
    }

    #[test]
    fn test_part2() {
        let cave = Cave::new(INPUT_TEST);
        assert_eq!(target_reached_in(&cave), 45);
    }
}
