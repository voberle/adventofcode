use std::io::{self, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn build(line: &str) -> Self {
        let parts: Vec<i32> = line[1..line.len() - 1]
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();
        Self {
            x: parts[0],
            y: parts[1],
        }
    }

    fn ship() -> Self {
        Self { x: 0, y: 0 }
    }

    fn dist_from_ship(self) -> u32 {
        u32::try_from(self.x.abs() + self.y.abs()).unwrap()
    }

    fn dist_from(self, other: Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

fn build(input: &str) -> Vec<Coord> {
    input.lines().map(Coord::build).collect()
}

fn diff_closest_furthest(islands: &[Coord]) -> u32 {
    let closest = islands.iter().min_by_key(|c| c.dist_from_ship()).unwrap();
    let furthest = islands.iter().max_by_key(|c| c.dist_from_ship()).unwrap();
    furthest.dist_from_ship() - closest.dist_from_ship()
}

fn sort_by_dist_from(islands: &[Coord], ref_island: Coord) -> Vec<Coord> {
    let mut sorted = islands.to_vec();
    sorted.sort_by(|a, b| {
        let a_dist = ref_island.dist_from(*a);
        let b_dist = ref_island.dist_from(*b);
        a_dist.cmp(&b_dist).then(a.x.cmp(&b.x)).then(a.y.cmp(&b.y))
    });
    sorted
}

fn dist_closest_next_closests(islands: &[Coord]) -> u32 {
    let sorted_from_ship = sort_by_dist_from(islands, Coord::ship());
    let closest = sorted_from_ship.first().unwrap();

    let islands_minus_closest: Vec<_> = islands.iter().filter(|&c| c != closest).copied().collect();
    let sorted_from_closest = sort_by_dist_from(&islands_minus_closest, *closest);
    let closest_from_closest = sorted_from_closest.first().unwrap();

    closest.dist_from(*closest_from_closest)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let islands = build(&input);

    println!("Part 1: {}", diff_closest_furthest(&islands));
    println!("Part 2: {}", dist_closest_next_closests(&islands));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let islands = build(&INPUT_TEST);
        assert_eq!(diff_closest_furthest(&islands), 226);
    }

    #[test]
    fn test_part2() {
        let islands = build(&INPUT_TEST);
        assert_eq!(dist_closest_next_closests(&islands), 114);
    }
}
