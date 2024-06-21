use std::io::{self, Read};

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coords {
    x: i32,
    y: i32,
}

impl Coords {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    // Returns all the positions at the specified Manhattan distance.
    fn generate_manhattan_points_at(&self, dist: i32) -> Vec<Coords> {
        if dist == 0 {
            return vec![self.clone()];
        }
        let mut points: Vec<Coords> = Vec::new();
        for offset in 0..dist {
            let inv_offset = dist - offset;
            points.push(Coords::new(self.x + offset, self.y + inv_offset));
            points.push(Coords::new(self.x + inv_offset, self.y - offset));
            points.push(Coords::new(self.x - offset, self.y - inv_offset));
            points.push(Coords::new(self.x - inv_offset, self.y + offset));
        }
        points
    }

    fn distance(&self, p: &Coords) -> u32 {
        self.x.abs_diff(p.x) + self.y.abs_diff(p.y)
    }
}

fn build(input: &str) -> Vec<Coords> {
    input
        .lines()
        .map(|line| {
            let p: Vec<_> = line.split(", ").collect();
            Coords::new(p[0].parse().unwrap(), p[1].parse().unwrap())
        })
        .collect()
}

// Find the coordinates that are furthest in each corner.
fn corner_coordinates(coords: &[Coords]) -> (i32, i32, i32, i32) {
    if let itertools::MinMaxResult::MinMax(min_x, max_x) = coords.iter().minmax_by_key(|c| c.x) {
        if let itertools::MinMaxResult::MinMax(min_y, max_y) = coords.iter().minmax_by_key(|c| c.y)
        {
            return (max_x.x, min_x.x, max_y.y, min_y.y);
        }
    }
    panic!("Couldn't find grid size");
}

// Rough estimation how big distances we need to compute.
fn max_dist_to_compute(coords: &[Coords]) -> i32 {
    let (max_x, min_x, max_y, min_y) = corner_coordinates(coords);
    (max_x - min_x).max(max_y - min_y)
}

fn largest_finite_area_size(coords: &[Coords]) -> usize {
    // Grid storing which coordinate is closest for each point, and the corresponding distance.
    // MAX means the location is equally close from two or more coordinates.
    // NB: A vector would be more efficient, but the math coordinates is a pain to handle.
    // As the grid isn't that big, a map works fine.
    const MULTIPLE: usize = usize::MAX;
    let mut grid: FxHashMap<Coords, (usize, i32)> = FxHashMap::default();

    let mut finite_areas: FxHashSet<usize> = FxHashSet::default();

    // Progressively look for bigger distancs
    for d in 0..max_dist_to_compute(coords) {
        // For each coord, mark the points around with 1) who is closest; 2) the distance.
        for (idx, coord) in coords.iter().enumerate() {
            if finite_areas.contains(&idx) {
                // Already identified as a finite area, can skip.
                continue;
            }

            // All the positions around this coordinate at the specific distance.
            let all_pos = coord.generate_manhattan_points_at(d);
            let mut any_pos_set = false;
            for p in all_pos {
                grid.entry(p)
                    .and_modify(|e| {
                        if e.1 == d {
                            // Position is already set at this distance for another coord, setting it to multiple.
                            *e = (MULTIPLE, d);
                        }
                    })
                    .or_insert_with(|| {
                        // If position isn't in grid yet, claim it.
                        any_pos_set = true;
                        (idx, d)
                    });
            }
            // If no position was set, we have a finite area
            if !any_pos_set {
                finite_areas.insert(idx);
            }
        }
    }

    // print_grid_for_test(coords, &grid);
    // println!("finite_areas {:?}", finite_areas);

    finite_areas
        .iter()
        .map(|a| grid.iter().filter(|(_, (c, _))| *c == *a).count())
        .max()
        .unwrap()
}

#[allow(dead_code)]
fn print_grid_for_test(coords: &[Coords], grid: &FxHashMap<Coords, (usize, i32)>) {
    const ABC: [char; 6] = ['A', 'B', 'C', 'D', 'E', 'F'];
    for y in 0..10 {
        for x in 0..10 {
            let c = Coords::new(x, y);
            if let Some(i) = coords.iter().position(|v| *v == c) {
                print!("{}", ABC[i]);
            } else if let Some(v) = grid.get(&c) {
                print!(
                    "{}",
                    if v.0 == usize::MAX {
                        '.'.to_string()
                    } else {
                        ABC[v.0].to_ascii_lowercase().to_string()
                    }
                );
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
fn region_total_dist_to_all_less(coords: &[Coords], max_dist: u32) -> usize {
    let mut count = 0;

    let (max_x, min_x, max_y, min_y) = corner_coordinates(coords);
    // Optimization later found on Reddit:
    // Since the sum of the distances must be less than 10000, the farthest a point could be is 10000/number_of_points
    let furthest_dist = max_dist as i32 / coords.len() as i32;

    for y in min_y - furthest_dist..max_y + furthest_dist {
        for x in min_x - furthest_dist..max_x + furthest_dist {
            let p = Coords::new(x, y);
            if coords.iter().map(|c| c.distance(&p)).sum::<u32>() < max_dist {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let coords = build(input.trim());

    println!("Part 1: {}", largest_finite_area_size(&coords));
    println!("Part 2: {}", region_total_dist_to_all_less(&coords, 10_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_generate_manhattan_points_at() {
        assert_eq!(
            Coords::new(1, 2).generate_manhattan_points_at(0),
            [Coords::new(1, 2)]
        );
        assert_eq!(
            Coords::new(1, 2).generate_manhattan_points_at(1),
            [
                Coords::new(1, 3),
                Coords::new(2, 2),
                Coords::new(1, 1),
                Coords::new(0, 2)
            ]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(largest_finite_area_size(&build(INPUT_TEST)), 17);
    }

    #[test]
    fn test_part2() {
        assert_eq!(region_total_dist_to_all_less(&build(INPUT_TEST), 32), 16);
    }
}
