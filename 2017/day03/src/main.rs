use std::io::{self, Read};

fn target_position(target_square: i32) -> (i32, i32) {
    assert!(target_square > 1);

    let mut square = 1;
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut side_len = 0; // 2, 4, 6 etc

    // println!("Target square {}", target_square);
    while square < target_square {
        // move to next circle
        x += 1;
        y += 1;
        side_len += 2;

        // Right vertical side
        if target_square <= square + side_len {
            y -= target_square - square;
            return (x, y);
        }
        y -= side_len;
        square += side_len;
        // println!("  After right vertical: {x},{y}, square={square}. Side={side_len}");

        // Top horizontal side
        if target_square <= square + side_len {
            x -= target_square - square;
            return (x, y);
        }
        x -= side_len;
        square += side_len;
        // println!("  After top horizontal: {x},{y}, square={square}. Side={side_len}");

        // Left vertical side
        if target_square <= square + side_len {
            y += target_square - square;
            return (x, y);
        }
        y += side_len;
        square += side_len;
        // println!("  After left vertical: {x},{y}, square={square}. Side={side_len}");

        // Bottom horizontal side
        if target_square <= square + side_len {
            x += target_square - square;
            return (x, y);
        }
        x += side_len;
        square += side_len;
        // println!("  After bottom horizontal: {x},{y}, square={square}. Side={side_len}");
    }

    panic!("Should never get here: square={square}, side_len={side_len}, target_square={target_square}")
}

fn steps_count(target_square: usize) -> usize {
    if target_square == 1 {
        return 0;
    }
    let (x, y) = target_position(i32::try_from(target_square).expect("Numbers don't fit in i32"));
    // println!("Target pos {x},{y}");
    (x.abs_diff(0) + y.abs_diff(0)) as usize
}

fn part2(square: usize) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let square = input.trim().parse().unwrap();

    println!("Part 1: {}", steps_count(square));
    println!("Part 2: {}", part2(square));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_position() {
        assert_eq!(target_position(12), (2, -1));
        assert_eq!(target_position(23), (0, 2));
    }

    #[test]
    fn test_part1() {
        assert_eq!(steps_count(1), 0);
        assert_eq!(steps_count(12), 3);
        assert_eq!(steps_count(23), 2);
        assert_eq!(steps_count(1024), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(0), 0);
    }
}
