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

fn pos(cols: usize, row: usize, col: usize) -> usize {
    row * cols + col
}

fn transpose(from: &[bool], size: usize) -> Vec<bool> {
    let mut to = vec![false; from.len()];
    for i in 0..size {
        for j in 0..size {
            to[pos(size, i, j)] = from[pos(size, j, i)];
        }
    }
    to
}

fn reverse_rows(from: &[bool], size: usize) -> Vec<bool> {
    let mut to = vec![false; from.len()];
    for i in 0..size {
        for j in 0..size {
            to[pos(size, i, j)] = from[pos(size, i, size - 1 - j)];
        }
    }
    to
}

fn reverse_columns(from: &[bool], size: usize) -> Vec<bool> {
    let mut to = vec![false; from.len()];
    for j in 0..size {
        for i in 0..size {
            to[pos(size, i, j)] = from[pos(size, size - 1 - i, j)];
        }
    }
    to
}

#[allow(
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation
)]
fn sqrt(v: usize) -> usize {
    (v as f64).sqrt() as usize
}

#[derive(Clone)]
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

    // Rotate by +90 degres
    fn rotate(&self) -> Self {
        let g = transpose(&self.grid, Self::SIZE);
        let result = reverse_rows(&g, Self::SIZE);
        Self {
            id: self.id,
            grid: result,
        }
    }

    fn flip_horizontally(&self) -> Self {
        let result = reverse_columns(&self.grid, Self::SIZE);
        Self {
            id: self.id,
            grid: result,
        }
    }

    fn flip_vertically(&self) -> Self {
        let result = reverse_rows(&self.grid, Self::SIZE);
        Self {
            id: self.id,
            grid: result,
        }
    }

    fn next_orientation(&mut self, orientation: usize) {
        match orientation % 3 {
            0 => {
                *self = self.flip_horizontally();
            }
            1 => {
                *self = self.flip_horizontally();
                *self = self.flip_vertically();
            }
            2 => {
                *self = self.flip_vertically();
                *self = self.rotate();
            }
            _ => panic!("Invalid orientation"),
        }
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
    let border_options: Vec<[u32; 8]> = tiles.iter().map(Tile::get_border_options).collect();

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

fn get_corners(graph: &[Vec<usize>]) -> Vec<usize> {
    graph
        .iter()
        .enumerate()
        .filter(|(_, conns)| conns.len() == 2)
        .map(|(i, _)| i)
        .collect()
}

fn find_assembled_image_corners_result(tiles: &[Tile]) -> u64 {
    let graph = build_image_graph(tiles);

    let corners = get_corners(&graph);
    corners.iter().map(|&i| tiles[i].id).product()
}

const TILE_UNKNOWN: u32 = u32::MAX;
const TILE_AT_BORDER: u32 = u32::MAX - 1;

// Compares the border IDs of a tile with the ones around,
fn are_border_ids_matching(border_ids: &[u32], surrounding_ids: &[u32]) -> bool {
    border_ids
        .iter()
        .zip(surrounding_ids.iter())
        .filter(|p| *p.0 != TILE_UNKNOWN && *p.1 != TILE_UNKNOWN)
        .all(|p| p.0 == p.1)
}

fn assemble_image(tiles: &[Tile], graph: &[Vec<usize>]) {
    // We know which tiles connect to which.
    // Also, there should be only one way to connect a tile to another one.

    // Make a copy of the tiles as we will need to rotate and flip them.
    let mut tiles = tiles.to_vec();

    let picture_size = sqrt(tiles.len());
    let mut puzzle: Vec<Vec<usize>> = vec![vec![usize::MAX; picture_size]; picture_size];

    // Start with a corner. We don't have to, but easier to nagivage the puzzle grid.
    let corners = get_corners(graph);
    let mut connections_to_explore: Vec<usize> = vec![corners[0]];

    // We need all borders again to handle border tiles.
    let border_options: Vec<[u32; 8]> = tiles.iter().map(Tile::get_border_options).collect();

    for row in 0..picture_size {
        for col in 0..picture_size {
            // The IDs of the tiles surrounding the one we are trying to place.
            let mut surrounding_ids = [TILE_UNKNOWN; 4];
            if row > 0 {
                let tid = puzzle[row - 1][col];
                if tid != usize::MAX {
                    surrounding_ids[0] = tiles[tid].get_bottom_id();
                }
            } else {
                surrounding_ids[0] = TILE_AT_BORDER;
            }
            if col < picture_size - 1 {
                let tid = puzzle[row][col + 1];
                if tid != usize::MAX {
                    surrounding_ids[1] = tiles[tid].get_left_id();
                }
            } else {
                surrounding_ids[1] = TILE_AT_BORDER;
            }
            if row < picture_size - 1 {
                let tid = puzzle[row + 1][col];
                if tid != usize::MAX {
                    surrounding_ids[2] = tiles[tid].get_top_id();
                }
            } else {
                surrounding_ids[2] = TILE_AT_BORDER;
            }
            if col > 0 {
                let tid = puzzle[row][col - 1];
                if tid != usize::MAX {
                    surrounding_ids[3] = tiles[tid].get_right_id();
                }
            } else {
                surrounding_ids[3] = u32::MAX - 1;
            }

            // Take the possible connections and modify them until they fit.
            'conn_loop: for conn in &connections_to_explore {
                // Exclude tiles already placed.
                if puzzle.iter().flatten().any(|i| i == conn) {
                    continue;
                }

                let tile = &mut tiles[*conn];
                // Try all orientations.
                for tile_orientation in 0..12 {
                    // println!("{}/{}: {} -----", row, col, tile.id); tile.print();

                    // Find the border IDs of this tile, and clear the ones that don't
                    // have a match because they are at the border.
                    let border_ids: Vec<u32> = tile
                        .get_border_ids()
                        .iter()
                        .map(|id| {
                            if border_options
                                .iter()
                                .enumerate()
                                .filter(|(i, _)| *i != *conn)
                                .any(|(_, ig)| ig.contains(id))
                            {
                                *id
                            } else {
                                u32::MAX - 1
                            }
                        })
                        .collect();

                    // Check if the tile fits now.
                    if are_border_ids_matching(&border_ids, &surrounding_ids) {
                        puzzle[row][col] = *conn;

                        if col < picture_size - 1 {
                            connections_to_explore = graph[*conn].clone();
                        } else {
                            connections_to_explore = graph[puzzle[row][0]].clone();
                        }
                        break 'conn_loop;
                    }

                    // If it doesn't fit, try next tile orientation.
                    tile.next_orientation(tile_orientation);
                }
            }
        }
    }
    println!("{:?}", puzzle);
}

fn part2(tiles: &[Tile]) -> i64 {
    let graph = build_image_graph(tiles);

    assemble_image(tiles, &graph);
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

    println!("Part 1: {}", find_assembled_image_corners_result(&tiles));
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
            find_assembled_image_corners_result(&build(INPUT_TEST)),
            20899048083289
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
