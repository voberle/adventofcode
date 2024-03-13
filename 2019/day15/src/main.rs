use std::io::{self, Read};

use fxhash::FxHashMap;
use intcode::IntcodeComputer;

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}
use Direction::{East, North, South, West};

impl Direction {
    fn get_val(self) -> i64 {
        match self {
            North => 1,
            South => 2,
            West => 3,
            East => 4,
        }
    }

    fn reverse(self) -> Self {
        match self {
            North => South,
            South => North,
            West => East,
            East => West,
        }
    }
}

const ALL_DIRECTIONS: [Direction; 4] = [North, South, West, East];

#[derive(Debug, PartialEq)]
enum Status {
    HitWall,
    Moved,
    MovedAndFound,
}

impl Status {
    fn new(code: i64) -> Self {
        match code {
            0 => Self::HitWall,
            1 => Self::Moved,
            2 => Self::MovedAndFound,
            _ => panic!("Invalid status code {}", code),
        }
    }
}

fn run(computer: &mut IntcodeComputer, movement_command: Direction) -> Status {
    computer.io.add_input(movement_command.get_val());
    computer.exec();
    Status::new(computer.io.get_output().unwrap())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32, // from west to east
    y: i32, // from north to south
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    fn move_towards(&self, dir: Direction) -> Self {
        match dir {
            North => Self::new(self.x, self.y - 1),
            South => Self::new(self.x, self.y + 1),
            West => Self::new(self.x - 1, self.y),
            East => Self::new(self.x + 1, self.y),
        }
    }
}

enum Element {
    Wall,
    Empty,
    OxygenSystem,
}

struct Maze(FxHashMap<Pos, Element>);

impl Maze {
    fn new() -> Self {
        Self(FxHashMap::default())
    }

    fn borders(&self) -> (Pos, Pos) {
        let mut min_pos = Pos::new(i32::MAX, i32::MAX);
        let mut max_pos = Pos::new(i32::MIN, i32::MIN);
        for pos in self.0.keys() {
            min_pos.x = min_pos.x.min(pos.x);
            max_pos.x = max_pos.x.max(pos.x);
            min_pos.y = min_pos.y.min(pos.y);
            max_pos.y = max_pos.y.max(pos.y);
        }
        (min_pos, max_pos)
    }

    fn print_with_droid(&self, droid: Option<Pos>) {
        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";
        let (min_pos, max_pos) = self.borders();
        for y in min_pos.y..=max_pos.y {
            for x in min_pos.x..=max_pos.x {
                let pos = Pos::new(x, y);
                if let Some(droid_pos) = droid {
                    if droid_pos == pos {
                        print!("D");
                        continue;
                    }
                }
                if let Some(elt) = self.0.get(&pos) {
                    match elt {
                        Element::Wall => print!("#"),
                        Element::Empty => print!("."),
                        Element::OxygenSystem => print!("{RED}O{RESET}"),
                    }
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    fn print(&self) {
        self.print_with_droid(None);
    }
}

// Build the maze.
fn discover_maze(computer: &IntcodeComputer) -> Maze {
    // Building the maze cannot be done recursively, otherwise the computer wouldn't be in the right state.
    // So we need to move back once we have reached a dead-end.

    let mut computer = computer.clone();
    let mut maze = Maze::new();
    let mut pos = Pos::zero();
    // We need to save position and direction, as computer requires both.
    let mut path: Vec<(Pos, Direction)> = Vec::new();

    'outer: loop {
        let unexplored_directions: Vec<(Direction, Pos)> = ALL_DIRECTIONS
            .iter()
            .map(|&d| (d, pos.move_towards(d)))
            .filter(|(_, pos)| !maze.0.contains_key(pos))
            .collect();

        for (dir, next_pos) in unexplored_directions {
            let status = run(&mut computer, dir);
            match status {
                Status::HitWall => {
                    maze.0.insert(next_pos, Element::Wall);
                    // position hasn't changed
                }
                Status::Moved => {
                    // We move into the first unexplored empty space we found.
                    // There might be more empty spaces around this position, we will explore them on our way back.
                    maze.0.insert(next_pos, Element::Empty);
                    path.push((pos, dir)); // we add the position to the path before leaving it
                    pos = next_pos;
                    continue 'outer;
                }
                Status::MovedAndFound => {
                    maze.0.insert(next_pos, Element::OxygenSystem);
                    // println!("!!! Found the system at {:?}", next_pos);
                    path.push((pos, dir));
                    pos = next_pos;
                    continue 'outer;
                }
            }
        }

        // Dead end, so need to move back.
        if let Some((back_pos, back_dir)) = path.pop() {
            pos = back_pos;
            let status = run(&mut computer, back_dir.reverse());
            assert_eq!(status, Status::Moved);
        } else {
            break;
        }
    }
    maze.print_with_droid(Some(Pos::zero()));

    maze
}

fn shortest_path_to_droid(computer: &IntcodeComputer) -> usize {
    0
}

fn part2(computer: &IntcodeComputer) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let computer = IntcodeComputer::build(&input);

    let maze = discover_maze(&computer);
    // maze.print();

    println!("Part 1: {}", shortest_path_to_droid(&computer));
    println!("Part 2: {}", part2(&computer));
}
