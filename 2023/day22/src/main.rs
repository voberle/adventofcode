// https://adventofcode.com/2023/day/22

use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

#[derive(Debug, Clone, PartialEq)]
enum Dir {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

impl Coord {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    fn val_dir(&self, dir: &Dir) -> usize {
        match dir {
            Dir::X => self.x,
            Dir::Y => self.y,
            Dir::Z => self.z,
        }
    }

    // Returns a copy of this coord with z set to 0.
    fn flatten(&self) -> Coord {
        Coord::new(self.x, self.y, 0)
    }
}

// Each brick is made up of a single straight line of cubes,
// which means that no more than one value is different between the two coordinates.
#[derive(Debug, Clone, PartialEq)]
struct Brick {
    // The coordinates are ordered, meaning c1 is the lowest one
    p1: Coord,
    p2: Coord,
    // Is it a brick that goes on x, y or z line?
    // By convention, single cube bricks will be marked on x.
    dir: Dir,
}

impl Brick {
    // Construct a Brick, having the coords ordered and the direction of the cube saved in dir.
    fn new(c1: Coord, c2: Coord) -> Self {
        if c1 == c2 {
            return Self {
                p1: c1,
                p2: c2,
                dir: Dir::X,
            };
        }
        if c1.x != c2.x {
            if c1.x < c2.x {
                return Self {
                    p1: c1,
                    p2: c2,
                    dir: Dir::X,
                };
            } else {
                return Self {
                    p1: c2,
                    p2: c1,
                    dir: Dir::X,
                };
            }
        }
        if c1.y != c2.y {
            if c1.y < c2.y {
                return Self {
                    p1: c1,
                    p2: c2,
                    dir: Dir::Y,
                };
            } else {
                return Self {
                    p1: c2,
                    p2: c1,
                    dir: Dir::Y,
                };
            }
        }
        if c1.z != c2.z {
            if c1.z < c2.z {
                return Self {
                    p1: c1,
                    p2: c2,
                    dir: Dir::Z,
                };
            } else {
                return Self {
                    p1: c2,
                    p2: c1,
                    dir: Dir::Z,
                };
            }
        }
        panic!("Something went wrong building Brick")
    }

    fn from(c1x: usize, c1y: usize, c1z: usize, c2x: usize, c2y: usize, c2z: usize) -> Self {
        Brick::new(Coord::new(c1x, c1y, c1z), Coord::new(c2x, c2y, c2z))
    }

    fn flatten(&self) -> Brick {
        Brick::new(self.p1.flatten(), self.p2.flatten())
    }

    fn xy_overlap(&self, other: &Brick) -> bool {
        let a = self.flatten();
        let b = other.flatten();
        assert_ne!(a.dir, Dir::Z);
        assert_ne!(b.dir, Dir::Z);
        if a.dir == b.dir {
            // Parallel bricks
            let d = &a.dir;
            // println!("Parallel: {:?}", d);
            let other_d = &if d == &Dir::X { Dir::Y } else { Dir::X };
            if a.p1.val_dir(other_d) != b.p1.val_dir(other_d) {
                // but not on same line
                return false;
            }
            // https://stackoverflow.com/a/3269471
            return a.p1.val_dir(d) <= b.p2.val_dir(d) && b.p1.val_dir(d) <= a.p2.val_dir(d);
        } else {
            // Perpendicular bricks
            // println!("Perpendicular");
            return a.p1.val_dir(&a.dir) <= b.p1.val_dir(&a.dir)
                && b.p1.val_dir(&a.dir) <= a.p2.val_dir(&a.dir)
                && b.p1.val_dir(&b.dir) <= a.p1.val_dir(&b.dir)
                && a.p1.val_dir(&b.dir) <= b.p2.val_dir(&b.dir);
        }
    }

    fn move_down(&mut self) {
        self.p1.z -= 1;
        self.p2.z -= 1;
    }
}

#[test]
fn test_xy_overlap() {
    let b = Brick::from(0, 0, 2, 2, 0, 2);
    assert!(b.xy_overlap(&b));
    let c = Brick::from(0, 2, 3, 2, 2, 3);
    assert!(!b.xy_overlap(&c));
    assert!(!c.xy_overlap(&b));
    let a = Brick::from(1, 0, 1, 1, 2, 1);
    assert!(a.xy_overlap(&b));
    assert!(a.xy_overlap(&c));
    let d = Brick::from(3, 0, 2, 4, 0, 2);
    assert!(!d.xy_overlap(&b));
    let e = Brick::from(1, 2, 1, 1, 4, 1);
    assert!(!e.xy_overlap(&b));
    assert!(e.xy_overlap(&c));
    let z = Brick::from(1, 1, 8, 1, 1, 9);
    println!("g: {:?}", z);
    assert!(z.xy_overlap(&z));
    assert!(z.xy_overlap(&a));
    assert!(!z.xy_overlap(&b));
    assert!(!z.xy_overlap(&c));
    assert!(!b.xy_overlap(&z));
    let z1 = Brick::from(1, 1, 2, 1, 1, 3);
    assert!(z1.xy_overlap(&z));
    let z2 = Brick::from(1, 2, 8, 1, 2, 9);
    assert!(!z2.xy_overlap(&z));
}

#[derive(Debug, Clone)]
struct Snapshot {
    bricks: Vec<Brick>,
    // These fields are helper for analyzing the snapshot.
    levels_bottom: HashMap<usize, Vec<usize>>,
    levels_top: HashMap<usize, Vec<usize>>,
    max_z: usize,
}

impl Snapshot {
    fn build<R>(reader: &mut R) -> Self
    where
        R: BufRead,
    {
        let mut bricks: Vec<Brick> = Vec::new();
        for l in reader.lines() {
            let line = l.unwrap();
            let p: Vec<Vec<usize>> = line
                .split('~')
                .map(|c| c.split(',').map(|i| i.parse().unwrap()).collect())
                .collect();
            bricks.push(Brick::new(
                Coord::new(p[0][0], p[0][1], p[0][2]),
                Coord::new(p[1][0], p[1][1], p[1][2]),
            ));
        }

        // Maybe bricks need to be ordered by their z level
        // bricks.sort_by_key(|b| b.p1.z);

        let max_z = bricks.iter().map(|b| b.p1.z.max(b.p2.z)).max().unwrap();

        let mut s = Snapshot {
            bricks,
            levels_bottom: HashMap::new(),
            levels_top: HashMap::new(),
            max_z,
        };
        s.order_bricks_by_level();
        s
    }

    fn order_bricks_by_level(&mut self) {
        self.levels_bottom.clear();
        self.levels_top.clear();
        for (i, b) in self.bricks.iter().enumerate() {
            self.levels_bottom
                .entry(b.p1.z)
                .and_modify(|v| v.push(i))
                .or_insert(vec![i]);
            self.levels_top
                .entry(b.p2.z)
                .and_modify(|v| v.push(i))
                .or_insert(vec![i]);
        }

        assert_eq!(
            self.levels_bottom.values().map(|v| v.len()).sum::<usize>(),
            self.bricks.len()
        );
        assert_eq!(
            self.levels_top.values().map(|v| v.len()).sum::<usize>(),
            self.bricks.len()
        );
        // println!("Bottom({}): {:?}", self.levels_bottom.len(), self.levels_bottom);
        // println!("Top({}): {:?}", self.levels_top.len(), self.levels_top);
    }

    fn move_brick_down(&mut self, i: usize) {
        // println!("Move brick {} one level down", i);
        self.bricks[i].move_down();
        self.order_bricks_by_level();
        // could also recheck z_max
    }

    fn disintegrate_brick(&mut self, i: usize) {
        self.bricks.remove(i);
        self.order_bricks_by_level();
    }

    fn print(&self) {
        for b in &self.bricks {
            println!("{:?}", b);
        }
    }
}

// Indicates if this brick has any brick directly under.
fn brick_has_support(snapshot: &Snapshot, level: usize, brick: &Brick) -> bool {
    if level == 1 {
        return true;
    }
    // get bricks the level under
    if let Some(indexes) = snapshot.levels_top.get(&(level - 1)) {
        indexes.iter().any(|b_index| {
            let brick_under = &snapshot.bricks[*b_index];
            brick.xy_overlap(brick_under)
        })
    } else {
        // No bricks under so no support
        false
    }
}

fn move_bricks_downward(snapshot: &Snapshot, z_to_start_at: usize) -> Snapshot {
    let mut s = snapshot.clone();

    let mut index_to_move_down = None;
    loop {
        'outer: for z in z_to_start_at..s.max_z {
            // bricks that have their botton at this level
            if let Some(indexes) = s.levels_bottom.get(&z) {
                for b_index in indexes {
                    let brick = &s.bricks[*b_index];
                    if !brick_has_support(&s, z, brick) {
                        // println!("No support: Brick {}: {:?} at level {}", *b_index, brick, z);
                        index_to_move_down = Some(*b_index);
                        break 'outer;
                    }
                }
            }
        }
        if let Some(i) = index_to_move_down {
            s.move_brick_down(i);
            index_to_move_down = None
        } else {
            break;
        }
    }
    s
}

// Requires a snapshot with bricks moved downwards
fn safely_disintegrate(snapshot: &Snapshot) -> u32 {
    let mut safe_to_disintegrate = 0;
    // Brute force: We try disintegrating each brick, and see if any remaining brick would move downwards.
    for i in 0..snapshot.bricks.len() {
        let mut s = snapshot.clone();
        let z_of_brick_disintegrated = s.bricks[i].p1.z;
        s.disintegrate_brick(i);
        let d = move_bricks_downward(&s, z_of_brick_disintegrated);
        // Compare bricks only, not full snapshots (the hashmaps don't need to be compared)
        if d.bricks == s.bricks {
            // println!("Brick at {} IS SAFE to disintegrate", i);
            safe_to_disintegrate += 1;
        }
    }
    safe_to_disintegrate
}

fn safely_disintegrated_count(snapshot: &Snapshot) -> u32 {
    let final_snapshot = move_bricks_downward(&snapshot, 1);
    // final_snapshot.print();
    // println!("{:#?}", final_snapshot);
    safely_disintegrate(&final_snapshot)
}

fn main() {
    let stdin = stdin();
    let snapshot = Snapshot::build(&mut stdin.lock());

    println!("Part 1: {}", safely_disintegrated_count(&snapshot));
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_part1() {
        let mut reader = BufReader::new(File::open("resources/input_test").unwrap());
        let snapshot = Snapshot::build(&mut reader);

        assert_eq!(safely_disintegrated_count(&snapshot), 5);
    }
}
