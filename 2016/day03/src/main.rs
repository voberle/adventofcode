use std::io::{self, Read};

fn build(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_triangle(t: &[u32]) -> bool {
    let (mut a, mut b, mut c) = (t[0], t[1], t[2]);
    if a < c {
        std::mem::swap(&mut a, &mut c);
    }
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    // Now the biggest element is the 1st one.
    a < b + c
}

fn count_triangles(triangles: &[Vec<u32>]) -> usize {
    triangles.iter().filter(|t| is_triangle(t)).count()
}

fn part2(triangles: &[Vec<u32>]) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let triangles = build(&input);

    println!("Part 1: {}", count_triangles(&triangles));
    println!("Part 2: {}", part2(&triangles));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_triangle() {
        assert_eq!(is_triangle(&[5, 10, 25]), false);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build("")), 0);
    }
}
