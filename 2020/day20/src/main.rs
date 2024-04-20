use std::{
    io::{self, Read},
    vec,
};

mod square_grid;
use square_grid::SquareGrid;

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
    grid: SquareGrid,
}

impl Tile {
    const SIZE: usize = 10;

    fn get_top_id(&self) -> u32 {
        bin_to_int(&self.grid.values[0..Tile::SIZE])
    }

    fn get_bottom_id(&self) -> u32 {
        bin_to_int(&self.grid.values[self.grid.values.len() - Tile::SIZE..self.grid.values.len()])
    }

    fn get_left_id(&self) -> u32 {
        let bits: Vec<bool> = self
            .grid
            .values
            .iter()
            .step_by(Tile::SIZE)
            .copied()
            .collect();
        bin_to_int(&bits)
    }

    fn get_right_id(&self) -> u32 {
        let bits: Vec<bool> = self
            .grid
            .values
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
            tiles.push(Tile {
                id,
                grid: SquareGrid::new(grid, Tile::SIZE),
            });
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
    tiles.push(Tile {
        id,
        grid: SquareGrid::new(grid, Tile::SIZE),
    });
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

// Assembles the image.
// Returns the list of tiles in correct orientation and their positions in the puzzle.
fn assemble_image(tiles: &[Tile], graph: &[Vec<usize>]) -> (Vec<Tile>, Vec<Vec<usize>>) {
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
                    tile.grid.next_orientation(tile_orientation);
                }
            }
        }
    }
    // println!("{:?}", puzzle);
    (tiles, puzzle)
}

// Removes tile borders and merges them.
#[allow(clippy::needless_range_loop)]
fn merge_tiles(tiles: &[Tile], puzzle: &[Vec<usize>]) -> SquareGrid {
    let picture_tiles_count = sqrt(tiles.len());
    let picture_size = picture_tiles_count * (Tile::SIZE - 2);
    let picture_len = picture_size * picture_size;

    let mut picture: SquareGrid = SquareGrid::new(vec![false; picture_len], picture_size);

    for row in 0..picture_tiles_count {
        for col in 0..picture_tiles_count {
            let tid = puzzle[row][col];
            let tile = &tiles[tid];
            for r in 1..Tile::SIZE - 1 {
                for c in 1..Tile::SIZE - 1 {
                    let rp = row * (Tile::SIZE - 2) + r - 1;
                    let rc = col * (Tile::SIZE - 2) + c - 1;
                    let rpos = square_grid::pos(picture_size, rp, rc);

                    let p = square_grid::pos(Tile::SIZE, r, c);
                    picture.values[rpos] = tile.grid.values[p];
                }
            }
        }
    }
    picture
}

const SEA_MONSTER: &str = r"                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";
const SEA_MONSTER_WIDTH: usize = 20;
const SEA_MONSTER_HEIGHT: usize = 3;

fn sea_monster_offsets() -> Vec<(usize, usize)> {
    SEA_MONSTER
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, c)| if c == '#' { Some((row, col)) } else { None })
        })
        .collect()
}

fn is_monster(picture: &SquareGrid, row: usize, col: usize, offsets: &[(usize, usize)]) -> bool {
    offsets.iter().all(|(r_off, c_off)| {
        let pos = square_grid::pos(picture.size, row + r_off, col + c_off);
        picture.values[pos]
    })
}

#[allow(dead_code)]
fn print_picture_with_monsters(
    picture: &mut SquareGrid,
    monsters_locations: &[(usize, usize)],
    monster_offsets: &[(usize, usize)],
) {
    println!("Found {} monsters in:", monsters_locations.len());
    let locations: Vec<(usize, usize)> = monsters_locations
        .iter()
        .flat_map(|(r, c)| {
            monster_offsets
                .iter()
                .map(move |(r_off, c_off)| (r + r_off, c + c_off))
        })
        .collect();
    picture.print_with_position(&locations);
}

// Finds the correct orientations of the picture and returns the number of sea monsters in it.
fn count_sea_monsters(picture: &mut SquareGrid, monster_offsets: &[(usize, usize)]) -> usize {
    for tile_orientation in 0..12 {
        let mut monsters_locations: Vec<(usize, usize)> = Vec::new();
        for row in 0..picture.size - SEA_MONSTER_HEIGHT {
            for col in 0..picture.size - SEA_MONSTER_WIDTH {
                if is_monster(picture, row, col, monster_offsets) {
                    monsters_locations.push((row, col));
                }
            }
        }
        if !monsters_locations.is_empty() {
            // print_picture_with_monsters(picture, &monsters_locations, monster_offsets);
            return monsters_locations.len();
        }
        // If no monsters found, try next tile orientation.
        picture.next_orientation(tile_orientation);
    }
    panic!("No monsters found");
}

fn count_vals_not_in_sea_monster(tiles: &[Tile]) -> usize {
    let graph = build_image_graph(tiles);

    let (tiles, puzzle) = assemble_image(tiles, &graph);
    let mut picture = merge_tiles(&tiles, &puzzle);
    // println!("Assembled picture:");
    // picture.print();

    let sea_monster_offsets = sea_monster_offsets();

    let monsters_count = count_sea_monsters(&mut picture, &sea_monster_offsets);

    // Number of '#' in picture.
    let picture_hash_count = picture.values.iter().filter(|v| **v).count();
    // Number of '#' in one monster.
    let monster_hash_count = sea_monster_offsets.len();

    picture_hash_count - monsters_count * monster_hash_count
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
    println!("Part 2: {}", count_vals_not_in_sea_monster(&tiles));
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
        assert_eq!(count_vals_not_in_sea_monster(&build(INPUT_TEST)), 273);
    }
}
