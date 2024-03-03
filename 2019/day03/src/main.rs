use std::io::{self, Read};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use fxhash::FxHashSet;
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

fn follow_steps(path: &Path) -> FxHashSet<(i32, i32)> {
    let mut steps: FxHashSet<(i32, i32)> = FxHashSet::default();
    let mut pos = (0, 0);
    for ins in path {
        for _ in 0..ins.1 {
            match ins.0 {
                Up => pos.1 -= 1,
                Down => pos.1 += 1,
                Left => pos.0 -= 1,
                Right => pos.0 += 1,
            }
            steps.insert(pos);
        }
    }
    steps
}

#[allow(clippy::cast_sign_loss)]
fn distance(x: i32, y: i32) -> u32 {
    (x.abs() + y.abs()) as u32
}

fn closest_crossing_point(path1: &Path, path2: &Path) -> u32 {
    let steps1 = follow_steps(path1);
    let steps2 = follow_steps(path2);
    let closest = steps1
        .intersection(&steps2)
        .min_by(|a, b| distance(a.0, a.1).cmp(&distance(b.0, b.1)))
        .unwrap();
    distance(closest.0, closest.1)
}

fn part2(path1: &Path, path2: &Path) -> u32 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (path1, path2) = build(&input);

    println!("Part 1: {}", closest_crossing_point(&path1, &path2));
    println!("Part 2: {}", part2(&path1, &path2));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");
    const INPUT_TEST_3: &str = include_str!("../resources/input_test_3");

    #[test]
    fn test_part1() {
        let (path1, path2) = build(INPUT_TEST_1);
        assert_eq!(closest_crossing_point(&path1, &path2), 6);
        let (path1, path2) = build(INPUT_TEST_2);
        assert_eq!(closest_crossing_point(&path1, &path2), 159);
        let (path1, path2) = build(INPUT_TEST_3);
        assert_eq!(closest_crossing_point(&path1, &path2), 135);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(&build(INPUT_TEST_1)), 0);
    }
}
