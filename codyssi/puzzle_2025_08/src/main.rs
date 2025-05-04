use std::io::{self, Read};

fn build(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn alphabetical_composition(file: &[Vec<char>]) -> usize {
    file.iter()
        .flatten()
        .filter(|c| c.is_ascii_alphabetic())
        .count()
}

// If a numerical char is next to a char identified by is_reductible_neighbour_fn, they are both removed.
fn reduce(s: &[char], is_reductible_neighbour_fn: fn(char) -> bool) -> Vec<char> {
    let mut reduced = s.to_vec();

    'main_loop: loop {
        for pos in 0..reduced.len() {
            if !reduced[pos].is_numeric() {
                continue;
            }

            if pos > 0 && is_reductible_neighbour_fn(reduced[pos - 1]) {
                reduced.remove(pos);
                reduced.remove(pos - 1);
                // If we reduced one, start from the beginning.
                continue 'main_loop;
            } else if pos < reduced.len() - 1 && is_reductible_neighbour_fn(reduced[pos + 1]) {
                reduced.remove(pos + 1);
                reduced.remove(pos);
                continue 'main_loop;
            }
        }

        // If we get here, it means we looked at all numerical chars and found none that could be reduced.
        break;
    }

    reduced
}

fn reduce_and_count(file: &[Vec<char>], is_reductible_neighbour_fn: fn(char) -> bool) -> usize {
    file.iter()
        .map(|line| reduce(line, is_reductible_neighbour_fn).len())
        .sum()
}

fn chars_count_after_reduction1(file: &[Vec<char>]) -> usize {
    reduce_and_count(file, |c| c.is_ascii_alphabetic() || c == '-')
}

fn chars_count_after_reduction2(file: &[Vec<char>]) -> usize {
    reduce_and_count(file, |c| c.is_ascii_alphabetic())
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let file = build(&input);

    println!("Part 1: {}", alphabetical_composition(&file));
    println!("Part 2: {}", chars_count_after_reduction1(&file));
    println!("Part 3: {}", chars_count_after_reduction2(&file));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let file = build(&INPUT_TEST);
        assert_eq!(alphabetical_composition(&file), 52);
    }

    fn str2vec(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    #[test]
    fn test_reduce() {
        fn check(c: char) -> bool {
            c.is_ascii_alphabetic() || c == '-'
        }

        assert_eq!(reduce(&str2vec("baa3"), check), str2vec("ba"));
        assert_eq!(reduce(&str2vec("321ab"), check), str2vec("3"));
        // or "a", as reductions can be performed in any order
        assert_eq!(reduce(&str2vec("a7b"), check), str2vec("b"));
        assert_eq!(reduce(&str2vec("z-4"), check), str2vec("z"));
    }

    #[test]
    fn test_part2() {
        let file = build(&INPUT_TEST);
        assert_eq!(chars_count_after_reduction1(&file), 18);
    }

    #[test]
    fn test_part3() {
        let file = build(&INPUT_TEST);
        assert_eq!(chars_count_after_reduction2(&file), 26);
    }
}
