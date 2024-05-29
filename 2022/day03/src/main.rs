use std::io::{self, Read};

fn build(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

// Returns the index of the element, that can be used to index a vector.
// It turns out that the element priority is to index + 1.
fn elt_index(e: char) -> usize {
    let p = match e {
        'a'..='z' => e as u8 - b'a',
        'A'..='Z' => e as u8 - b'A' + 26,
        _ => panic!("Invalid element"),
    };
    usize::from(p)
}

fn index_to_priority(i: usize) -> usize {
    i + 1
}

fn presence_vec(v: &[char]) -> Vec<bool> {
    let mut presence = vec![false; 52];
    for e in v {
        presence[elt_index(*e)] = true;
    }
    presence
}

// Finds the intersection between two slices,
// using the fact that we have a maximum of 52 different elements.
// Returns the index of the element.
fn intersection_2(v1: &[char], v2: &[char]) -> Vec<usize> {
    let p1 = presence_vec(v1);
    let p2 = presence_vec(v2);
    (0..52).filter(|&i| p1[i] && p2[i]).collect()
}

fn intersection_3(v1: &[char], v2: &[char], v3: &[char]) -> Vec<usize> {
    let p1 = presence_vec(v1);
    let p2 = presence_vec(v2);
    let p3 = presence_vec(v3);
    (0..52).filter(|&i| p1[i] && p2[i] && p3[i]).collect()
}

fn common_elt_priorities_sum(rucksacks: &[Vec<char>]) -> usize {
    rucksacks
        .iter()
        .map(|r| {
            let mid = r.len() / 2;
            index_to_priority(intersection_2(&r[0..mid], &r[mid..])[0])
        })
        .sum()
}

fn badges_priorities_sum(rucksacks: &[Vec<char>]) -> usize {
    rucksacks
        .chunks(3)
        .map(|g| index_to_priority(intersection_3(&g[0], &g[1], &g[2])[0]))
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let now = std::time::Instant::now();
    let rucksacks = build(&input);

    println!("Part 1: {}", common_elt_priorities_sum(&rucksacks));
    println!("Part 2: {}", badges_priorities_sum(&rucksacks));

    println!("Execution time: {:.2?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(common_elt_priorities_sum(&build(INPUT_TEST)), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(badges_priorities_sum(&build(INPUT_TEST)), 70);
    }
}
