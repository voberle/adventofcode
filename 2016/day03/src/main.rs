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

fn count_triangles_vertically(triangles: &[Vec<u32>]) -> usize {
    triangles
        .chunks(3)
        .flat_map(|c| {
            vec![
                [c[0][0], c[1][0], c[2][0]],
                [c[0][1], c[1][1], c[2][1]],
                [c[0][2], c[1][2], c[2][2]],
            ]
        })
        .filter(|t| is_triangle(t))
        .count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let triangles = build(&input);

    println!("Part 1: {}", count_triangles(&triangles));
    println!("Part 2: {}", count_triangles_vertically(&triangles));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_triangle() {
        assert!(!is_triangle(&[5, 10, 25]));
    }

    #[test]
    fn test_part2() {
        const INPUT: &str = r"101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603";
        assert_eq!(count_triangles_vertically(&build(INPUT)), 6);
    }
}
