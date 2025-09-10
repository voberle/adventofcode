use std::io::{self, Read};

#[derive(Debug)]
enum Instruction {
    Face(u64),
    Row(usize, u64),
    Col(usize, u64),
}

impl Instruction {
    fn build(line: &str) -> Self {
        let parts: Vec<_> = line.split(" - ").collect();
        let value = parts[1].trim_start_matches("VALUE ").parse().unwrap();
        if parts[0].starts_with("FACE") {
            Instruction::Face(value)
        } else if parts[0].starts_with("ROW") {
            // Instruction row numbers start at 1, but we prefer to start at 0.
            let index = parts[0]
                .trim_start_matches("ROW ")
                .parse::<usize>()
                .unwrap()
                - 1;
            Instruction::Row(index, value)
        } else if parts[0].starts_with("COL") {
            let index = parts[0]
                .trim_start_matches("COL ")
                .parse::<usize>()
                .unwrap()
                - 1;
            Instruction::Col(index, value)
        } else {
            panic!("Invalid instruction")
        }
    }
}

#[derive(Debug)]
enum Twist {
    L,
    R,
    D,
    U,
}

impl Twist {
    fn build(c: char) -> Twist {
        match c {
            'L' => Twist::L,
            'R' => Twist::R,
            'D' => Twist::D,
            'U' => Twist::U,
            _ => panic!("Invalid Twist char"),
        }
    }
}

fn build(input: &str) -> (Vec<Instruction>, Vec<Twist>) {
    let parts: Vec<_> = input.split("\n\n").collect();
    let instructions: Vec<_> = parts[0].lines().map(Instruction::build).collect();
    let twists: Vec<Twist> = parts[1].chars().map(Twist::build).collect();

    assert_eq!(instructions.len(), twists.len() + 1);

    (instructions, twists)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Face<const SIDE: usize>(Vec<u64>);

impl<const SIDE: usize> Face<SIDE> {
    fn new() -> Self {
        // A cube is initialized with all its faces set to 1.
        Self(vec![1; SIDE * SIDE])
    }

    fn rotate_90_clockwise(&self) -> Self {
        let mut rotated = Self::new();
        for row in 0..SIDE {
            for col in 0..SIDE {
                let original_index = row * SIDE + col;
                let new_row = col;
                let new_col = SIDE - 1 - row;
                let new_index = new_row * SIDE + new_col;
                rotated.0[new_index] = self.0[original_index];
            }
        }
        rotated
    }

    fn rotate_90_counter_clockwise(&self) -> Self {
        let mut rotated = Self::new();
        for row in 0..SIDE {
            for col in 0..SIDE {
                let original_index = row * SIDE + col;
                let new_row = SIDE - 1 - col;
                let new_col = row;
                let new_index = new_row * SIDE + new_col;
                rotated.0[new_index] = self.0[original_index];
            }
        }
        rotated
    }

    fn mirror_vertical(&self) -> Self {
        let mut mirrored = Self::new();
        for row in 0..SIDE {
            for col in 0..SIDE {
                let original_index = row * SIDE + col;
                let new_row = row;
                let new_col = SIDE - 1 - col;
                let new_index = new_row * SIDE + new_col;
                mirrored.0[new_index] = self.0[original_index];
            }
        }
        mirrored
    }

    fn mirror_horizontal(&self) -> Self {
        let mut mirrored = Self::new();
        for row in 0..SIDE {
            for col in 0..SIDE {
                let original_index = row * SIDE + col;
                let new_row = SIDE - 1 - row;
                let new_col = col;
                let new_index = new_row * SIDE + new_col;
                mirrored.0[new_index] = self.0[original_index];
            }
        }
        mirrored
    }

    fn apply_face(&mut self, value: u64) {
        for number in &mut self.0 {
            *number = (*number + value) % 100;
        }
    }

    fn apply_row(&mut self, row: usize, value: u64) {
        for number in &mut self.0.iter_mut().skip(row * SIDE).take(SIDE) {
            *number = (*number + value) % 100;
        }
    }

    fn apply_col(&mut self, col: usize, value: u64) {
        for number in &mut self.0.iter_mut().skip(col).step_by(SIDE) {
            *number = (*number + value) % 100;
        }
    }
}

// Cube faces
#[derive(Debug, Clone, PartialEq, Eq)]
struct Cube<const SIDE: usize> {
    up: Face<SIDE>,
    down: Face<SIDE>,
    front: Face<SIDE>,
    back: Face<SIDE>,
    right: Face<SIDE>,
    left: Face<SIDE>,
}

impl<const SIDE: usize> Cube<SIDE> {
    fn new() -> Self {
        Self {
            up: Face::new(),
            down: Face::new(),
            front: Face::new(),
            back: Face::new(),
            right: Face::new(),
            left: Face::new(),
        }
    }

    // “L” rotates the cube one face to the right (so that the new ‘current face’ is
    // one face to the left of the previous ‘current face’).
    fn rotate_left(&self) -> Self {
        Self {
            up: self.up.rotate_90_counter_clockwise(),
            down: self.down.rotate_90_counter_clockwise(),
            front: self.left.clone(),
            back: self.right.clone(),
            right: self.front.mirror_vertical(),
            left: self.back.mirror_vertical(),
        }
    }

    // “R” rotates the cube one face to the left (so that the new ‘current face’ is
    // one face to the right of the previous ‘current face’).
    fn rotate_right(&self) -> Self {
        Self {
            up: self.up.rotate_90_clockwise(),
            down: self.down.rotate_90_clockwise(),
            front: self.right.mirror_vertical(),
            back: self.left.mirror_vertical(),
            right: self.back.clone(),
            left: self.left.clone(),
        }
    }

    // “D” rotates the cube one face upwards (so that the new ‘current face’
    // is one face below the previous ‘current face’).
    fn rotate_down(&self) -> Self {
        Self {
            up: self.front.mirror_horizontal(),
            down: self.back.mirror_horizontal(),
            front: self.down.clone(),
            back: self.up.clone(),
            right: self.right.rotate_90_clockwise(),
            left: self.left.rotate_90_clockwise(),
        }
    }

    // “U” rotates the cube one face downwards (so that the new ‘current face’ is
    // one face above the previous ‘current face’).
    fn rotate_up(&self) -> Self {
        Self {
            up: self.back.clone(),
            down: self.front.clone(),
            front: self.up.mirror_horizontal(),
            back: self.down.mirror_horizontal(),
            right: self.right.rotate_90_counter_clockwise(),
            left: self.left.rotate_90_counter_clockwise(),
        }
    }
}

fn rotate_cube<const SIDE: usize>(cube: &Cube<SIDE>, twist: &Twist) -> Cube<SIDE> {
    match twist {
        Twist::L => cube.rotate_left(),
        Twist::R => cube.rotate_right(),
        Twist::D => cube.rotate_down(),
        Twist::U => cube.rotate_up(),
    }
}

fn apply_instruction_to_face<const SIDE: usize>(
    instruction: &Instruction,
    face: &Face<SIDE>,
) -> Face<SIDE> {
    let mut new_face = face.clone();
    match instruction {
        Instruction::Face(val) => new_face.apply_face(*val),
        Instruction::Row(row, val) => new_face.apply_row(*row, *val),
        Instruction::Col(col, val) => new_face.apply_col(*col, *val),
    }
    new_face
}

// Calculate power.
fn calc_power(side: usize, instruction: &Instruction) -> u64 {
    let s = side as u64;
    match instruction {
        Instruction::Face(val) => val * s * s,
        Instruction::Row(_, val) | Instruction::Col(_, val) => val * s,
    }
}

fn highest_absorptions<const SIDE: usize>(instructions: &[Instruction], twists: &[Twist]) -> u64 {
    const UP: usize = 0;
    const DOWN: usize = 1;
    const FRONT: usize = 2;
    const BACK: usize = 3;
    const RIGHT: usize = 4;
    const LEFT: usize = 5;

    // Absorptions
    let mut abs: [u64; 6] = [0; 6];

    // Apply first instruction.
    let power = calc_power(SIDE, &instructions[0]);
    abs[FRONT] += power;

    for (instruction, twist) in instructions.iter().skip(1).zip(twists) {
        // Rotate cube.
        abs = match twist {
            Twist::L => [
                abs[UP], abs[DOWN], abs[LEFT], abs[RIGHT], abs[FRONT], abs[BACK],
            ],
            Twist::R => [
                abs[UP], abs[DOWN], abs[RIGHT], abs[LEFT], abs[BACK], abs[FRONT],
            ],
            Twist::D => [
                abs[FRONT], abs[BACK], abs[DOWN], abs[UP], abs[RIGHT], abs[LEFT],
            ],
            Twist::U => [
                abs[BACK], abs[FRONT], abs[UP], abs[DOWN], abs[RIGHT], abs[LEFT],
            ],
        };

        let power = calc_power(SIDE, instruction);
        abs[FRONT] += power;
    }

    // Find two biggest absorptions.
    abs.sort_unstable();
    abs[abs.len() - 1] * abs[abs.len() - 2]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (instructions, twists) = build(&input);

    println!(
        "Part 1: {}",
        highest_absorptions::<80>(&instructions, &twists)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    fn init_face_with(m: u64) -> Face<3> {
        Face(vec![
            1 * m,
            2 * m,
            3 * m,
            4 * m,
            5 * m,
            6 * m,
            7 * m,
            8 * m,
            9 * m,
        ])
    }

    fn init_face() -> Face<3> {
        Face(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
    }

    #[test]
    fn test_rotate_90_clockwise() {
        // 1 2 3      7 4 1
        // 4 5 6  =>  8 5 2
        // 7 8 9      9 6 3
        let face = init_face();
        let rotated = face.rotate_90_clockwise();
        assert_eq!(rotated.0, vec![7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }

    #[test]
    fn test_rotate_90_counter_clockwise() {
        // 1 2 3      3 6 9
        // 4 5 6  =>  2 5 8
        // 7 8 9      1 4 7
        let face = init_face();
        let rotated = face.rotate_90_counter_clockwise();
        assert_eq!(rotated.0, vec![3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn test_mirror_vertical() {
        let face = init_face();
        let rotated = face.mirror_vertical();
        assert_eq!(rotated.0, vec![3, 2, 1, 6, 5, 4, 9, 8, 7]);
    }

    #[test]
    fn test_mirror_horizontal() {
        let face = init_face();
        let rotated = face.mirror_horizontal();
        assert_eq!(rotated.0, vec![7, 8, 9, 4, 5, 6, 1, 2, 3]);
    }

    fn init_cube() -> Cube<3> {
        Cube {
            up: init_face_with(1),
            down: init_face_with(10),
            front: init_face_with(100),
            back: init_face_with(1000),
            right: init_face_with(10000),
            left: init_face_with(100000),
        }
    }

    #[test]
    fn test_cube_rotate_left() {
        let cube = init_cube();
        let mut c = cube.clone();
        c = c.rotate_left();
        c = c.rotate_left();
        c = c.rotate_left();
        c = c.rotate_left();
        assert_eq!(c, cube);
    }

    #[test]
    fn test_part1_1() {
        let (instructions, twists) = build(&INPUT_TEST_1);
        assert_eq!(highest_absorptions::<3>(&instructions, &twists), 201474);
    }

    #[test]
    fn test_part1_2() {
        let (instructions, twists) = build(&INPUT_TEST_2);
        assert_eq!(
            highest_absorptions::<80>(&instructions, &twists),
            6902016000
        );
    }
}
