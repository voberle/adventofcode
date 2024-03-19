use fxhash::FxHashMap;

/// Small module to get the list of portals and their positions from the input.

// Grid to help parse the input.
struct Grid {
    values: Vec<char>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn build(input: &str) -> Self {
        let mut rows = 0;
        let values: Vec<_> = input
            .lines()
            .flat_map(|l| {
                rows += 1;
                l.chars().collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }
}

fn is_portal(c: char) -> bool {
    c.is_ascii_uppercase()
}

fn is_open(c: char) -> bool {
    c == '.'
}

pub fn get_portals_from_input(input: &str) -> FxHashMap<usize, String> {
    let mut portals = FxHashMap::default();
    let grid = Grid::build(input);

    // Horizontal ones
    for row in 0..grid.rows {
        // Note the -2 at the end
        for p in row * grid.cols..(row + 1) * grid.cols - 2 {
            let p0 = p;
            let p1 = p + 1;
            let p2 = p + 2;
            let c0 = grid.values[p0];
            let c1 = grid.values[p1];
            let c2 = grid.values[p2];
            if is_portal(c0) && is_portal(c1) && is_open(c2) {
                portals.insert(p2, format!("{}{}", c0, c1));
            }
            if is_open(c0) && is_portal(c1) && is_portal(c2) {
                portals.insert(p0, format!("{}{}", c1, c2));
            }
        }
    }

    // Vertical ones
    for col in 0..grid.cols {
        // Note the -2 at the end
        for p in (col..(col + grid.cols * (grid.rows - 2))).step_by(grid.cols) {
            let p0 = p;
            let p1 = p + grid.cols;
            let p2 = p + 2 * grid.cols;
            let c0 = grid.values[p0];
            let c1 = grid.values[p1];
            let c2 = grid.values[p2];
            if is_portal(c0) && is_portal(c1) && is_open(c2) {
                portals.insert(p2, format!("{}{}", c0, c1));
            }
            if is_open(c0) && is_portal(c1) && is_portal(c2) {
                portals.insert(p0, format!("{}{}", c1, c2));
            }
        }
    }

    portals
}
