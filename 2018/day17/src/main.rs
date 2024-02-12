use std::{
    fmt,
    io::{self, Read},
};

mod parsing;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Material {
    Sand,
    Clay,
    Spring,
    WaterAtRest,
    WaterFlow,
}
use Material::{Clay, Sand, Spring, WaterAtRest, WaterFlow};

impl fmt::Display for Material {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Sand => '.',
                Clay => '#',
                Spring => '+',
                WaterAtRest => '~',
                WaterFlow => '|',
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::{Down, Left, Right, Up};

#[derive(Debug, Clone, PartialEq)]
struct Grid {
    values: Vec<Material>,
    rows: usize,
    cols: usize,
    min_y: usize,
    max_y: usize,
}

impl Grid {
    fn build(input: &str) -> Self {
        let lines = parsing::build(input);

        let (min_x, max_x, min_y, max_y) = parsing::borders(&lines);
        // println!("{:?}", (min_x, max_x, min_y, max_y));
        // leave an empty column on each side
        let (min_x, max_x) = (min_x - 1, max_x + 2);
        let cols = max_x - min_x;
        let rows = max_y + 1;
        let mut grid = Self {
            values: vec![Sand; rows * cols],
            rows,
            cols,
            min_y,
            max_y,
        };

        for line in lines {
            if line.x1 == line.x2 {
                let x = line.x1;
                for y in line.y1..=line.y2 {
                    let p = grid.pos(x - min_x, y);
                    grid.values[p] = Clay;
                }
            } else if line.y1 == line.y2 {
                let y = line.y1;
                for x in line.x1..=line.x2 {
                    let p = grid.pos(x - min_x, y);
                    grid.values[p] = Clay;
                }
            }
        }

        let p = grid.pos(500 - min_x, 0);
        grid.values[p] = Spring;

        grid
    }

    fn pos(&self, x: usize, y: usize) -> usize {
        y * self.cols + x
    }

    #[allow(dead_code)]
    fn pos_as_str(&self, index: usize) -> String {
        format!("({},{})", self.row(index), self.col(index))
    }

    fn col(&self, index: usize) -> usize {
        index % self.cols
    }

    fn row(&self, index: usize) -> usize {
        index / self.cols
    }

    fn allowed(&self, pos: usize, direction: Direction) -> bool {
        !match direction {
            Up => pos < self.cols,
            Down => pos / self.cols == self.rows - 1,
            Left => pos % self.cols == 0,
            Right => pos % self.cols == self.cols - 1,
        }
    }

    fn next_pos(&self, pos: usize, direction: Direction) -> usize {
        match direction {
            Up => pos - self.cols,
            Down => pos + self.cols,
            Left => pos - 1,
            Right => pos + 1,
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        const BLUE: &str = "\x1b[94m";
        const RESET: &str = "\x1b[0m";
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                if [WaterAtRest, WaterFlow].contains(&c) {
                    print!("{BLUE}{}{RESET}", c);
                } else {
                    print!("{}", c);
                }
            }
            println!();
        }
    }
}

fn find_flows(grid: &Grid) -> Vec<usize> {
    grid.values
        .iter()
        .enumerate()
        .filter_map(|(i, m)| {
            if [Spring, WaterFlow].contains(m) {
                Some(i)
            } else {
                None
            }
        })
        .collect()
}

fn move_flow_to_side(grid: &mut Grid, p: usize, direction: Direction) -> bool {
    assert!([Left, Right].contains(&direction));

    let mut something_happened = false;
    // We could optimize here and do a loop
    if grid.allowed(p, direction) {
        let side_pos = grid.next_pos(p, direction);
        if grid.values[side_pos] == Sand && grid.allowed(side_pos, Down) {
            let down = grid.next_pos(p, Down);
            match grid.values[down] {
                Clay | WaterAtRest => {
                    assert_eq!(grid.values[side_pos], Sand);
                    grid.values[side_pos] = WaterFlow;
                    something_happened = true;
                }
                _ => {}
            }
        }
    }
    something_happened
}

// Find all flows that have clay on both sides.
// Returns the position of the most left flow for such cases.
fn find_flows_with_clay_at_side(grid: &Grid) -> Vec<usize> {
    let flows = find_flows(grid);
    flows
        .iter()
        .filter(|&&p| {
            if grid.allowed(p, Left) && grid.values[grid.next_pos(p, Left)] == Clay {
                let mut r = p;
                loop {
                    if !grid.allowed(r, Right) {
                        break;
                    }
                    r = grid.next_pos(r, Right);
                    let v = grid.values[r];
                    if v == WaterFlow {
                        continue;
                    }
                    if v == Clay {
                        return true;
                    }
                    return false;
                }
                false
            } else {
                false
            }
        })
        .copied()
        .collect()
}

// For the flows with clay at side we found, fill them with water.
// Assumes `pos` points to the first '.' in "#||||#".
fn fill_space_with_water(grid: &mut Grid, pos: usize) {
    let mut p = pos;
    loop {
        grid.values[p] = WaterAtRest;
        if !grid.allowed(p, Right) {
            break;
        }
        p = grid.next_pos(p, Right);
        let v = grid.values[p];
        if v == WaterFlow {
            grid.values[p] = WaterAtRest;
            continue;
        } else if v == Clay {
            return;
        }
        panic!("Should never get here when filling a space with water");
    }
}

fn fill_water(grid: &mut Grid) {
    // Tracks if water moved in an iteration of the big loop.
    let mut something_happened = true;
    while something_happened {
        something_happened = false;

        // Find all flows and see if we can go down
        let flows = find_flows(grid);
        let mut went_down = false;
        let mut bottom_flows: Vec<usize> = Vec::new();
        for p in flows {
            if grid.allowed(p, Down) {
                let down = grid.next_pos(p, Down);
                match grid.values[down] {
                    Sand => {
                        // If we have sand below our water flow, flow goes down still.
                        grid.values[down] = WaterFlow;
                        something_happened = true;
                        went_down = true;
                    }
                    Clay | WaterAtRest => {
                        // If flow hit bottom, saving that position for next step.
                        bottom_flows.push(p);
                    }
                    _ => {}
                }
            }
        }
        if went_down {
            // If water could go down, we keep trying to go down.
            continue;
        }

        // If no water flow could go down, look if flow can go to the side.

        // Use the bottom flows positions we saved previously.
        for p in bottom_flows {
            // For each position left or right, add a water flow if there is solid (water or clay) under.
            if move_flow_to_side(grid, p, Left) {
                something_happened = true;
            }
            if move_flow_to_side(grid, p, Right) {
                something_happened = true;
            }
        }

        // Finally, find all flows that have clay on both sides, and replace them with water at rest.
        let flows_with_clay_at_side = find_flows_with_clay_at_side(grid);
        if !flows_with_clay_at_side.is_empty() {
            for p in flows_with_clay_at_side {
                fill_space_with_water(grid, p);
            }
            something_happened = true;
        }
    }
}

fn count_tiles(grid: &Grid, material: Material) -> usize {
    // While for the grid we start at y = 0 (as the spring is there), the instructions say:
    // "ignore tiles with a y coordinate smaller than the smallest y coordinate in your scan data or larger than the largest one"
    // and the smallest y isn't 1...
    grid.values
        .iter()
        .enumerate()
        .filter(|(i, _)| (grid.min_y..=grid.max_y).contains(&grid.row(*i)))
        .filter(|(_, m)| **m == material)
        .count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut grid = Grid::build(&input);
    fill_water(&mut grid);

    let param = std::env::args().nth(1).unwrap_or_default();
    if param == "visu" {
        grid.print();
    }

    let water_at_rest_count = count_tiles(&grid, WaterAtRest);
    let water_flow_count = count_tiles(&grid, WaterFlow);

    println!("Part 1: {}", water_at_rest_count + water_flow_count);
    println!("Part 2: {}", water_at_rest_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1_2() {
        let mut grid = Grid::build(INPUT_TEST);
        fill_water(&mut grid);

        let water_at_rest_count = count_tiles(&grid, WaterAtRest);
        let water_flow_count = count_tiles(&grid, WaterFlow);
        assert_eq!(water_at_rest_count + water_flow_count, 57);
        assert_eq!(water_at_rest_count, 29);
    }
}
