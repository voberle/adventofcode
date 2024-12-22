use std::{collections::VecDeque, fmt};

use fxhash::FxHashMap;

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
#[derive(Debug, Clone, Copy)]
pub enum DirKey {
    Up,
    Down,
    Left,
    Right,
    A,
}

impl DirKey {
    // Returns the shortest set of keys to press to go press next starting from self.
    // Always 1, 2 or 3 moves, plus A.
    fn go_press(self, next: DirKey) -> Vec<Vec<DirKey>> {
        use DirKey::{Down, Left, Right, Up, A};
        let mut moves = match self {
            Up => match next {
                Up => vec![],
                Down => vec![vec![Down]],
                Left => vec![vec![Down, Left]],
                Right => vec![vec![Down, Right], vec![Right, Down]],
                A => vec![vec![Right]],
            },
            Down => match next {
                Up => vec![vec![Up]],
                Down => vec![],
                Left => vec![vec![Left]],
                Right => vec![vec![Right]],
                A => vec![vec![Up, Right], vec![Right, Up]],
            },
            Left => match next {
                Up => vec![vec![Right, Up]],
                Down => vec![vec![Right]],
                Left => vec![],
                Right => vec![vec![Right, Right]],
                A => vec![vec![Right, Up, Right], vec![Right, Right, Up]],
            },
            Right => match next {
                Up => vec![vec![Up, Left], vec![Left, Up]],
                Down => vec![vec![Left]],
                Left => vec![vec![Left, Left]],
                Right => vec![],
                A => vec![vec![Up]],
            },
            A => match next {
                Up => vec![vec![Left]],
                Down => vec![vec![Left, Down], vec![Down, Left]],
                Left => vec![vec![Left, Down, Left], vec![Down, Left, Left]],
                Right => vec![vec![Down]],
                A => vec![],
            },
        };
        // After the moves we still need to press A each time.
        moves.iter_mut().for_each(|m| m.push(A));
        moves
    }
}

impl fmt::Display for DirKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                DirKey::Up => '^',
                DirKey::Down => 'v',
                DirKey::Left => '<',
                DirKey::Right => '>',
                DirKey::A => 'A',
            }
        )
    }
}

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumKey {
    K7,
    K8,
    K9,
    K4,
    K5,
    K6,
    K1,
    K2,
    K3,
    K0,
    KA,
}

impl NumKey {
    fn index(self) -> usize {
        match self {
            NumKey::K7 => 7,
            NumKey::K8 => 8,
            NumKey::K9 => 9,
            NumKey::K4 => 4,
            NumKey::K5 => 5,
            NumKey::K6 => 6,
            NumKey::K1 => 1,
            NumKey::K2 => 2,
            NumKey::K3 => 3,
            NumKey::K0 => 0,
            NumKey::KA => 10,
        }
    }

    // Returns the key in that direction, if any.
    #[allow(clippy::match_same_arms)]
    fn next_key(self, dir: DirKey) -> Option<NumKey> {
        use NumKey::{K0, K1, K2, K3, K4, K5, K6, K7, K8, K9, KA};
        match self {
            K7 => match dir {
                DirKey::Up => None,
                DirKey::Down => Some(K4),
                DirKey::Left => None,
                DirKey::Right => Some(K8),
                DirKey::A => panic!("A isn't a direction"),
            },
            K8 => match dir {
                DirKey::Up => None,
                DirKey::Down => Some(K5),
                DirKey::Left => Some(K7),
                DirKey::Right => Some(K9),
                DirKey::A => panic!("A isn't a direction"),
            },
            K9 => match dir {
                DirKey::Up => None,
                DirKey::Down => Some(K6),
                DirKey::Left => Some(K8),
                DirKey::Right => None,
                DirKey::A => panic!("A isn't a direction"),
            },
            K4 => match dir {
                DirKey::Up => Some(K7),
                DirKey::Down => Some(K1),
                DirKey::Left => None,
                DirKey::Right => Some(K5),
                DirKey::A => panic!("A isn't a direction"),
            },
            K5 => match dir {
                DirKey::Up => Some(K8),
                DirKey::Down => Some(K2),
                DirKey::Left => Some(K4),
                DirKey::Right => Some(K6),
                DirKey::A => panic!("A isn't a direction"),
            },
            K6 => match dir {
                DirKey::Up => Some(K9),
                DirKey::Down => Some(K3),
                DirKey::Left => Some(K5),
                DirKey::Right => None,
                DirKey::A => panic!("A isn't a direction"),
            },
            K1 => match dir {
                DirKey::Up => Some(K4),
                DirKey::Down => None,
                DirKey::Left => None,
                DirKey::Right => Some(K2),
                DirKey::A => panic!("A isn't a direction"),
            },
            K2 => match dir {
                DirKey::Up => Some(K5),
                DirKey::Down => Some(K0),
                DirKey::Left => Some(K1),
                DirKey::Right => Some(K3),
                DirKey::A => panic!("A isn't a direction"),
            },
            K3 => match dir {
                DirKey::Up => Some(K6),
                DirKey::Down => Some(KA),
                DirKey::Left => Some(K2),
                DirKey::Right => None,
                DirKey::A => panic!("A isn't a direction"),
            },
            K0 => match dir {
                DirKey::Up => Some(K2),
                DirKey::Down => None,
                DirKey::Left => None,
                DirKey::Right => Some(KA),
                DirKey::A => panic!("A isn't a direction"),
            },
            KA => match dir {
                DirKey::Up => Some(K3),
                DirKey::Down => None,
                DirKey::Left => Some(K0),
                DirKey::Right => None,
                DirKey::A => panic!("A isn't a direction"),
            },
        }
    }

    // Returns the direction needed to go from two neighbour numeric keys.
    pub fn dir(self, next: NumKey) -> DirKey {
        use DirKey::{Down, Left, Right, Up};
        use NumKey::{K0, K1, K2, K3, K4, K5, K6, K7, K8, K9, KA};
        // println!("{} => {}", self, next);
        match self {
            K7 => match next {
                K8 => Right,
                K4 => Down,
                _ => panic!("Keys are not neighours."),
            },
            K8 => match next {
                K7 => Left,
                K9 => Right,
                K5 => Down,
                _ => panic!("Keys are not neighours."),
            },
            K9 => match next {
                K8 => Left,
                K6 => Down,
                _ => panic!("Keys are not neighours."),
            },
            K4 => match next {
                K7 => Up,
                K5 => Right,
                K1 => Down,
                _ => panic!("Keys are not neighours."),
            },
            K5 => match next {
                K8 => Up,
                K4 => Left,
                K6 => Right,
                K2 => Down,
                _ => panic!("Keys are not neighours."),
            },
            K6 => match next {
                K9 => Up,
                K5 => Left,
                K3 => Down,
                _ => panic!("Keys are not neighours."),
            },
            K1 => match next {
                K4 => Up,
                K2 => Right,
                _ => panic!("Keys are not neighours."),
            },
            K2 => match next {
                K5 => Up,
                K1 => Left,
                K3 => Right,
                K0 => Down,
                _ => panic!("Keys are not neighours."),
            },
            K3 => match next {
                K6 => Up,
                K2 => Left,
                KA => Down,
                _ => panic!("Keys are not neighours."),
            },
            K0 => match next {
                K2 => Up,
                KA => Right,
                _ => panic!("Keys are not neighours."),
            },
            KA => match next {
                K0 => Left,
                K3 => Up,
                _ => panic!("Keys are not neighours."),
            },
        }
    }

    fn neighbour_keys_iter(self) -> impl Iterator<Item = Self> + 'static {
        [DirKey::Up, DirKey::Down, DirKey::Left, DirKey::Right]
            .into_iter()
            .filter_map(move |dir| self.next_key(dir))
    }

    // Finds all shortest path from one key to another.
    // Uses BFS.
    pub fn find_all_paths_to(self, to: NumKey) -> Vec<Vec<NumKey>> {
        let mut paths = Vec::new();
        // Queue for BFS, storing (current key, current path)
        let mut queue: VecDeque<(NumKey, Vec<NumKey>)> = VecDeque::new();
        // Use a map to track the shortest known distance to each node
        let mut shortest_distances: FxHashMap<NumKey, usize> = FxHashMap::default();

        queue.push_back((self, vec![self]));
        shortest_distances.insert(self, 1);

        let mut shortest_path_length = usize::MAX;

        while let Some((current_key, current_path)) = queue.pop_front() {
            let current_path_length = current_path.len();

            if current_key == to {
                #[allow(clippy::comparison_chain)]
                if current_path_length < shortest_path_length {
                    shortest_path_length = current_path_length;
                    paths.clear();
                    paths.push(current_path);
                } else if current_path_length == shortest_path_length {
                    paths.push(current_path);
                }
                continue;
            }

            if current_path_length >= shortest_path_length {
                continue;
            }

            for next_key in current_key.neighbour_keys_iter() {
                let next_dist = shortest_distances.entry(next_key).or_insert(usize::MAX);
                if current_path_length < *next_dist {
                    *next_dist = current_path_length + 1;
                    let mut next_path = current_path.clone();
                    next_path.push(next_key);
                    queue.push_back((next_key, next_path));
                }
            }
        }

        paths
    }
}

impl From<char> for NumKey {
    fn from(c: char) -> Self {
        match c {
            '7' => Self::K7,
            '8' => Self::K8,
            '9' => Self::K9,
            '4' => Self::K4,
            '5' => Self::K5,
            '6' => Self::K6,
            '1' => Self::K1,
            '2' => Self::K2,
            '3' => Self::K3,
            '0' => Self::K0,
            'A' => Self::KA,
            _ => panic!("Invalid num key"),
        }
    }
}

impl fmt::Display for NumKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                NumKey::K7 => '7',
                NumKey::K8 => '8',
                NumKey::K9 => '9',
                NumKey::K4 => '4',
                NumKey::K5 => '5',
                NumKey::K6 => '6',
                NumKey::K1 => '1',
                NumKey::K2 => '2',
                NumKey::K3 => '3',
                NumKey::K0 => '0',
                NumKey::KA => 'A',
            }
        )
    }
}

#[rustfmt::skip]
pub fn print_numeric_keypad(highlight_keys: &[NumKey]) {
    const RED: &str = "\x1b[31m";
    const RESET: &str = "\x1b[0m";
    let p = |c: char| if highlight_keys.contains(&c.into()) {
        print!("{RED}{c}{RESET}");
    } else {
        print!("{c}");
    };

    println!("+---+---+---+");
    print!("| "); p('7'); print!(" | "); p('8'); print!(" | "); p('9'); println!(" |");
    println!("+---+---+---+");
    print!("| "); p('4'); print!(" | "); p('5'); print!(" | "); p('6'); println!(" |");
    println!("+---+---+---+");
    print!("| "); p('1'); print!(" | "); p('2'); print!(" | "); p('3'); println!(" |");
    println!("+---+---+---+");
    print!("    | ");                    p('0'); print!(" | "); p('A'); println!(" |");
    println!("    +---+---+");
}
