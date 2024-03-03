use std::io::{self, Read};

use fxhash::FxHashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::{Down, Left, Right, Up};

impl Direction {
    fn new(s: &str) -> Self {
        match s {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => panic!("Invalid direction char"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[allow(clippy::cast_sign_loss)]
    fn distance(self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
}

type Path = Vec<(Direction, u32)>;

fn build_path(line: &str) -> Path {
    line.split(',')
        .map(|ins| (Direction::new(&ins[0..1]), ins[1..].parse::<u32>().unwrap()))
        .collect()
}

fn build(input: &str) -> (Path, Path) {
    let mut it = input.lines();
    (
        build_path(it.next().unwrap()),
        build_path(it.next().unwrap()),
    )
}

fn follow_steps(path: &Path) -> FxHashMap<Pos, usize> {
    let mut steps: FxHashMap<Pos, usize> = FxHashMap::default();
    let mut pos = Pos::new(0, 0);
    let mut distance = 0;
    for ins in path {
        for _ in 0..ins.1 {
            match ins.0 {
                Up => pos.y -= 1,
                Down => pos.y += 1,
                Left => pos.x -= 1,
                Right => pos.x += 1,
            }
            steps.insert(pos, distance);
            distance += 1;
        }
    }
    steps
}

fn closest_crossing_point(steps1: &FxHashMap<Pos, usize>, steps2: &FxHashMap<Pos, usize>) -> u32 {
    // For part 1, we could use a set and use intersection, but part 2 requires a map.
    steps1
        .iter()
        .filter(|(p, _)| steps2.contains_key(p))
        .min_by(|a, b| a.0.distance().cmp(&b.0.distance()))
        .unwrap()
        .0
        .distance()
}

fn fewest_comb_steps_to_inter(
    steps1: &FxHashMap<Pos, usize>,
    steps2: &FxHashMap<Pos, usize>,
) -> usize {
    steps1
        .iter()
        .filter(|(p, _)| steps2.contains_key(p))
        .map(|(p, d)| d + 1 + steps2.get(p).unwrap() + 1)
        .min()
        .unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (path1, path2) = build(&input);

    let steps1 = follow_steps(&path1);
    let steps2 = follow_steps(&path2);

    println!("Part 1: {}", closest_crossing_point(&steps1, &steps2));
    println!("Part 2: {}", fewest_comb_steps_to_inter(&steps1, &steps2));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");
    const INPUT_TEST_3: &str = include_str!("../resources/input_test_3");

    fn build_steps(input: &str) -> (FxHashMap<Pos, usize>, FxHashMap<Pos, usize>) {
        let (path1, path2) = build(input);
        (follow_steps(&path1), follow_steps(&path2))
    }

    #[test]
    fn test_part1() {
        let (steps1, steps2) = build_steps(INPUT_TEST_1);
        assert_eq!(closest_crossing_point(&steps1, &steps2), 6);
        let (steps1, steps2) = build_steps(INPUT_TEST_2);
        assert_eq!(closest_crossing_point(&steps1, &steps2), 159);
        let (steps1, steps2) = build_steps(INPUT_TEST_3);
        assert_eq!(closest_crossing_point(&steps1, &steps2), 135);
    }

    #[test]
    fn test_part2() {
        let (steps1, steps2) = build_steps(INPUT_TEST_1);
        assert_eq!(fewest_comb_steps_to_inter(&steps1, &steps2), 30);
        let (steps1, steps2) = build_steps(INPUT_TEST_2);
        assert_eq!(fewest_comb_steps_to_inter(&steps1, &steps2), 610);
        let (steps1, steps2) = build_steps(INPUT_TEST_3);
        assert_eq!(fewest_comb_steps_to_inter(&steps1, &steps2), 410);
    }
}
