use std::io::{self, BufRead, Read};

use fxhash::FxHashMap;
use intcode::IntcodeComputer;

fn read_line() -> String {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin
        .lock()
        .read_line(&mut line)
        .expect("Could not read line");
    line.trim().to_string()
}

fn write_string(computer: &mut IntcodeComputer, s: &str) {
    const NEWLINE: i64 = 10;
    s.chars().map(|c| c as i64).for_each(|i| {
        computer.io.add_input(i);
    });
    computer.io.add_input(NEWLINE);
}

fn get_output(computer: &mut IntcodeComputer) -> String {
    let mut output: String = String::new();
    while let Some(i) = computer.io.get_output() {
        output.push(char::from_u32(u32::try_from(i).unwrap()).unwrap());
    }
    output
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn new(s: &str) -> Option<Self> {
        match s {
            "north" => Some(Direction::North),
            "south" => Some(Direction::South),
            "west" => Some(Direction::West),
            "east" => Some(Direction::East),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Output {
    room: Option<String>,
    next_dirs: Vec<Direction>,
    items: Vec<String>,
}

impl Output {
    fn parse(s: &str) -> Self {
        let mut room: Option<String> = None;
        let mut next_dirs: Vec<Direction> = Vec::new();
        let mut items: Vec<String> = Vec::new();
        let mut it = s.lines();

        while let Some(s) = it.next() {
            if s.is_empty() {
                continue;
            }

            if s.starts_with("==") {
                room = Some(s.trim_matches(['=', ' ']).to_string());
            }

            if s == "Doors here lead:" {
                loop {
                    let dir_str = it.next().unwrap();
                    if !dir_str.starts_with("- ") {
                        break;
                    }
                    next_dirs.push(Direction::new(&dir_str[2..]).expect("Not a direction string"));
                }
            }

            if s == "Items here:" {
                loop {
                    let item_str = it.next().unwrap();
                    if !item_str.starts_with("- ") {
                        break;
                    }
                    items.push(item_str[2..].to_string());
                }
            }
        }

        Output {
            room,
            next_dirs,
            items,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    fn move_towards(self, dir: &Direction) -> Self {
        match dir {
            Direction::North => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::East => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

// Gets the corners of the map
fn borders(map: &FxHashMap<Pos, String>) -> (Pos, Pos) {
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

#[allow(clippy::cast_sign_loss)]
fn print_line(min: i32, max: i32) {
    println!(
        "{:-<width$}",
        "",
        width = (max - min + 1) as usize * (25 + 3) + 2
    );
}

fn print_with_positions(map: &FxHashMap<Pos, String>, positions: &[Pos]) {
    const BLUE: &str = "\x1b[94m";
    const RESET: &str = "\x1b[0m";
    let (min_pos, max_pos) = borders(map);
    for y in min_pos.y..=max_pos.y {
        print_line(min_pos.x, max_pos.x);
        for x in min_pos.x..=max_pos.x {
            let pos = Pos::new(x, y);
            if let Some(c) = map.get(&pos) {
                if positions.contains(&pos) {
                    print!(" | {BLUE}{:>25}{RESET}", c);
                } else {
                    print!(" | {:>25}", c);
                }
            } else {
                print!(" | {:>25}", ' ');
            }
        }
        println!(" |");
    }
    print_line(min_pos.x, max_pos.x);
}

#[allow(dead_code)]
fn print(map: &FxHashMap<Pos, String>) {
    print_with_positions(map, &[]);
}

fn play(computer: &IntcodeComputer, saved_cmds: &str) {
    let mut computer = computer.clone();

    let mut replay_cmds: Vec<String> = saved_cmds.lines().map(ToString::to_string).collect();
    replay_cmds.reverse();

    let mut map: FxHashMap<Pos, String> = FxHashMap::default();
    let mut pos = Pos::zero();
    loop {
        computer.exec();
        let s = get_output(&mut computer);
        println!("{}", s);

        if computer.is_halted() {
            println!("Game over");
            break;
        }

        let output = Output::parse(&s);
        // println!("{:?}", output);

        if let Some(room) = output.room {
            map.insert(pos, room);
            print_with_positions(&map, &[pos]);
        }

        // for item in output.items {
        //     write_string(&mut computer, &format!("take {}", item));
        // }

        print!("> ");
        let input = if let Some(cmd) = replay_cmds.pop() {
            println!("{}", cmd);
            cmd
        } else {
            read_line()
        };

        if let Some(dir) = Direction::new(input.trim()) {
            pos = pos.move_towards(&dir);
        }

        write_string(&mut computer, &input);
    }
}

fn password_for_airlock(computer: &IntcodeComputer) -> i64 {
    0
}

fn main() {
    let param = std::env::args().nth(1).unwrap_or_default();
    if !param.is_empty() {
        // Not reading from stdin in this case, as it messes up with reading commands.
        let input = std::fs::read_to_string("resources/input").expect("Unable to read input file");
        let computer = IntcodeComputer::build(&input);

        if let Ok(saved_cmds) = std::fs::read_to_string(format!("resources/{}", param)) {
            play(&computer, &saved_cmds);
        } else {
            play(&computer, "");
        }

        return;
    }

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let computer = IntcodeComputer::build(&input);

    println!("Part 1: {}", password_for_airlock(&computer));
}
