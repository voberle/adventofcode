use fxhash::FxHashMap;

/// Printing a 2D map that is stored in a HashMap

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32, // from west to east
    y: i32, // from north to south
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

// Gets the corners of the map
fn borders(map: &FxHashMap<Pos, char>) -> (Pos, Pos) {
    let mut min_pos = Pos::new(i32::MAX, i32::MAX);
    let mut max_pos = Pos::new(i32::MIN, i32::MIN);
    for pos in map.keys() {
        min_pos.x = min_pos.x.min(pos.x);
        max_pos.x = max_pos.x.max(pos.x);
        min_pos.y = min_pos.y.min(pos.y);
        max_pos.y = max_pos.y.max(pos.y);
    }
    (min_pos, max_pos)
}

fn print_with_positions(map: &FxHashMap<Pos, char>, positions: Vec<Pos>) {
    const RED: &str = "\x1b[31m";
    const BLUE: &str = "\x1b[94m";
    const RESET: &str = "\x1b[0m";
    let (min_pos, max_pos) = borders(map);
    for y in min_pos.y..=max_pos.y {
        for x in min_pos.x..=max_pos.x {
            let pos = Pos::new(x, y);
            if positions.contains(&pos) {
                print!("{RED}D{RESET}");
                continue;
            }
            if let Some(c) = map.get(&pos) {
                print!("{BLUE}{}{RESET}", c);
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[allow(dead_code)]
fn print(map: &FxHashMap<Pos, char>) {
    print_with_positions(map, vec![]);
}