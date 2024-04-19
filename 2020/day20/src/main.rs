use std::io::{self, Read};

// Convert a vector of booleans representing a binary number to an integer.
fn bin_to_int(bits: &[bool]) -> u32 {
    bits.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, b)| acc | u32::from(*b) << i)
}

// Reverses the bits of the 10-bits integer.
fn reverse_bits(v: u32) -> u32 {
    // We cannot use Rust reverse_bits directly because we need to work on 10 bits.
    v.reverse_bits() >> (32 - Tile::SIZE)
}

struct Tile {
    id: u64,
    grid: Vec<bool>,
}

impl Tile {
    const SIZE: usize = 10;

    #[allow(dead_code)]
    fn print(&self) {
        for row in 0..Tile::SIZE {
            for p in row * Tile::SIZE..(row + 1) * Tile::SIZE {
                let c = self.grid[p];
                print!("{}", if c { '#' } else { '.' });
            }
            println!();
        }
    }

    fn get_top_id(&self) -> u32 {
        bin_to_int(&self.grid[0..Tile::SIZE])
    }

    fn get_bottom_id(&self) -> u32 {
        bin_to_int(&self.grid[self.grid.len() - Tile::SIZE..self.grid.len()])
    }

    fn get_left_id(&self) -> u32 {
        let bits: Vec<bool> = self.grid.iter().step_by(Tile::SIZE).copied().collect();
        bin_to_int(&bits)
    }

    fn get_right_id(&self) -> u32 {
        let bits: Vec<bool> = self
            .grid
            .iter()
            .skip(Tile::SIZE - 1)
            .step_by(Tile::SIZE)
            .copied()
            .collect();
        bin_to_int(&bits)
    }

    // Get IDs of the borders, starting from top one and going clockwise.
    fn get_border_ids(&self) -> [u32; 4] {
        [
            self.get_top_id(),
            self.get_right_id(),
            self.get_bottom_id(),
            self.get_left_id(),
        ]
    }

    // All numbers representing the tiles borders.
    // To find which tile can connect to which, we only need 8 numbers:
    // The ID of each border and their reverse.
    fn get_border_options(&self) -> [u32; 8] {
        let border_ids = self.get_border_ids();
        [
            border_ids[0],
            border_ids[1],
            border_ids[2],
            border_ids[3],
            reverse_bits(border_ids[0]),
            reverse_bits(border_ids[1]),
            reverse_bits(border_ids[2]),
            reverse_bits(border_ids[3]),
        ]
    }
}

fn build(input: &str) -> Vec<Tile> {
    let mut tiles: Vec<Tile> = Vec::new();
    let it = input.lines();

    let mut id = 0;
    let mut grid = Vec::new();
    for line in it {
        if line.is_empty() {
            tiles.push(Tile { id, grid });
            id = 0;
            grid = Vec::new();
            continue;
        }
        if line.starts_with("Tile ") {
            assert_eq!(id, 0);
            id = line
                .trim_start_matches("Tile ")
                .trim_end_matches(':')
                .parse()
                .unwrap();
            continue;
        }
        grid.extend(line.chars().map(|c| c == '#'));
    }
    tiles.push(Tile { id, grid });
    tiles
}

// Builds the graph representing how the tiles are connected.
fn build_image_graph(tiles: &[Tile]) -> Vec<Vec<usize>> {
    let border_options: Vec<[u32; 8]> =
        tiles.iter().map(Tile::get_border_options).collect();

    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); tiles.len()];

    for (i, tile_ids) in border_options.iter().enumerate() {
        for (j, others) in border_options.iter().enumerate() {
            if i == j {
                continue;
            }
            if others.iter().any(|t| tile_ids.contains(t)) {
                graph[i].push(j);
            }
        }
        // println!("{}: [{}] - {:?}", i, graph[i].len(), graph[i]);
    }
    graph
}

// The graph is nice with following:
//     dot -Tpdf -Kneato resources/input.gv > input.pdf
#[allow(dead_code)]
fn print_graphviz(graph: &[Vec<usize>]) {
    println!("digraph {{");
    for (i, connections) in graph.iter().enumerate() {
        for c in connections {
            println!("\t{} -> {};", i, c);
        }
    }
    println!("}}");
}

fn find_assembled_image_result(tiles: &[Tile]) -> u64 {
    let graph = build_image_graph(tiles);

    let corners: Vec<usize> = graph
        .iter()
        .enumerate()
        .filter(|(_, conns)| conns.len() == 2)
        .map(|(i, _)| i)
        .collect();
    corners.iter().map(|&i| tiles[i].id).product()
}

fn part2(tiles: &[Tile]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tiles = build(&input);
    // for t in &tiles {
    //     println!("{} {}", t.id, t.grid.len());
    //     t.print();
    // }

    println!("Part 1: {}", find_assembled_image_result(&tiles));
    println!("Part 2: {}", part2(&tiles));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_id_calculation() {
        let input = build(INPUT_TEST);
        let tile = input.first().unwrap();
        assert_eq!(tile.get_top_id(), 210);
        assert_eq!(tile.get_bottom_id(), 231);
        assert_eq!(tile.get_left_id(), 498);
        assert_eq!(tile.get_right_id(), 89);
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            find_assembled_image_result(&build(INPUT_TEST)),
            20899048083289
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
