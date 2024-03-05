use std::io::{self, Read};

fn build(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.split(',').map(|v| v.parse().unwrap()).collect())
        .collect()
}

// Distance between points
fn distance(p1: &[i32], p2: &[i32]) -> u32 {
    p1.iter()
        .zip(p2.iter())
        .map(|pair| pair.0.abs_diff(*pair.1))
        .sum()
}

// Checks if the two constellations are the same and should be merged.
fn are_same_constellations(c1: &[Vec<i32>], c2: &[Vec<i32>]) -> bool {
    for p1 in c1 {
        if c2.iter().any(|p2| distance(p1, p2) <= 3) {
            return true;
        }
    }
    false
}

fn create_constellations(points: &[Vec<i32>]) -> Vec<Vec<Vec<i32>>> {
    let mut constellations: Vec<Vec<Vec<i32>>> = //Vec::with_capacity(points.len());
        points.iter().map(|p| vec![p.clone()]).collect();
    // Go through all constellations, see if any of the followings can be merged with each.
    loop {
        let mut i = 0;
        let mut any_merge = false;
        while i < constellations.len() {
            let current = &constellations[i];
            let to_merge: Vec<usize> = (i + 1..constellations.len())
                .filter(|other| are_same_constellations(current, &constellations[*other]))
                .collect();

            for c in to_merge.iter().rev() {
                let other = constellations.remove(*c);
                constellations[i].extend(other);
                any_merge = true;
            }

            i += 1;
        }
        if !any_merge {
            break;
        }
    }
    constellations
}

fn constellations_count(points: &[Vec<i32>]) -> usize {
    let constellations = create_constellations(points);
    constellations.len()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let points = build(&input);

    println!("Part 1: {}", constellations_count(&points));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");
    const INPUT_TEST_3: &str = include_str!("../resources/input_test_3");
    const INPUT_TEST_4: &str = include_str!("../resources/input_test_4");

    #[test]
    fn test_constellations_count() {
        assert_eq!(constellations_count(&build(INPUT_TEST_1)), 2);
        assert_eq!(constellations_count(&build(INPUT_TEST_2)), 4);
        assert_eq!(constellations_count(&build(INPUT_TEST_3)), 3);
        assert_eq!(constellations_count(&build(INPUT_TEST_4)), 8);
    }
}
