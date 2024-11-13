use std::io::{self, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Position {
    y: usize,
    x: usize,
}

impl Position {
    fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }
}

// The universe is a vast amount of emptyness with a few galaxies.
// So instead of storing it as a grid with galaxies and empty space,
// we store only the positions of the galaxies.
#[derive(Debug, PartialEq)]
struct Universe {
    galaxies: Vec<Position>,
    width: usize,
    height: usize,
}

impl Universe {
    fn new(width: usize, height: usize) -> Self {
        Self {
            galaxies: Vec::new(),
            width,
            height,
        }
    }

    fn build(input: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut galaxies: Vec<Position> = Vec::new();
        for (y, r) in input.lines().enumerate() {
            width = r.len();
            height += 1;
            for (x, el) in r.chars().enumerate() {
                if el == '#' {
                    galaxies.push(Position::new(y, x));
                }
            }
        }
        galaxies.sort();
        Universe {
            galaxies,
            width,
            height,
        }
    }

    fn find(&self, pos: Position) -> Option<&Position> {
        self.galaxies.iter().find(|p| **p == pos)
    }

    fn get(&self, y: usize, x: usize) -> Option<&Position> {
        let pos = Position::new(y, x);
        self.galaxies.iter().find(|p| **p == pos)
    }

    fn set(&mut self, y: usize, x: usize) {
        let pos = Position::new(y, x);
        if self.find(pos).is_none() {
            self.galaxies.push(pos);
        }
    }

    fn is_row_empty(&self, y: usize) -> bool {
        !self.galaxies.iter().any(|p| p.y == y)
    }

    fn is_col_empty(&self, x: usize) -> bool {
        !self.galaxies.iter().any(|p| p.x == x)
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("---");
        for y in 0..self.height {
            for x in 0..self.width {
                if self.find(Position::new(y, x)).is_some() {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

// any rows or columns that contain no galaxies should all actually be twice as big
fn expand_universe(image: &Universe, expansion_factor: usize) -> Universe {
    let mut expanded_hor: Universe = Universe::new(
        image.width * expansion_factor,
        image.height * expansion_factor,
    );
    // Expand horizontally
    let mut ye = 0;
    for y in 0..image.height {
        if image.is_row_empty(y) {
            // expand
            ye += expansion_factor;
        } else {
            for x in 0..image.width {
                if image.get(y, x).is_some() {
                    expanded_hor.set(ye, x);
                }
            }
            ye += 1;
        }
    }
    expanded_hor.height = ye;
    // expanded_hor.print();

    let mut expanded: Universe = Universe::new(image.width * expansion_factor, expanded_hor.height);
    // Expand vertically
    let mut xe = 0;
    for x in 0..image.width {
        if image.is_col_empty(x) {
            // expand
            xe += expansion_factor;
        } else {
            for y in 0..expanded_hor.height {
                if expanded_hor.get(y, x).is_some() {
                    expanded.set(y, xe);
                }
            }
            xe += 1;
        }
    }
    expanded.width = xe;
    // expanded.print();

    // This makes them comparable
    expanded.galaxies.sort();

    expanded
}

fn shortest_path(g1: Position, g2: Position) -> usize {
    g1.x.abs_diff(g2.x) + g1.y.abs_diff(g2.y)
}

fn sum_of_shortest_paths(image: &Universe) -> usize {
    let mut galaxy_pairs: Vec<(Position, Position)> = Vec::new();
    for g1 in 0..image.galaxies.len() {
        for g2 in g1 + 1..image.galaxies.len() {
            galaxy_pairs.push((image.galaxies[g1], image.galaxies[g2]));
        }
    }
    galaxy_pairs
        .iter()
        .map(|pair| shortest_path(pair.0, pair.1))
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let universe: Universe = Universe::build(&input);
    // universe.print();

    let expanded = expand_universe(&universe, 2);
    println!("Part 1: {}", sum_of_shortest_paths(&expanded));

    let expanded_massive = expand_universe(&universe, 1_000_000);
    println!("Part 2: {}", sum_of_shortest_paths(&expanded_massive));
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test1");
    const TEST_EXPANDED: &str = include_str!("../resources/test1_expanded");

    #[test]
    fn test_expand_universe() {
        let image: Universe = Universe::build(INPUT_TEST);
        let image_expanded: Universe = Universe::build(TEST_EXPANDED);

        assert_eq!(expand_universe(&image, 2), image_expanded);
    }

    fn part1(expansion_factor: usize) -> usize {
        let image: Universe = Universe::build(INPUT_TEST);
        let expanded = expand_universe(&image, expansion_factor);
        sum_of_shortest_paths(&expanded)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(2), 374);
        assert_eq!(part1(10), 1030);
        assert_eq!(part1(100), 8410);
    }
}
