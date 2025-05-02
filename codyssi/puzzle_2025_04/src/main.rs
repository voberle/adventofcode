use std::io::{self, Read};

fn str_to_vec(line: &str) -> Vec<char> {
    line.chars().collect()
}

fn build(input: &str) -> Vec<Vec<char>> {
    input.lines().map(str_to_vec).collect()
}

fn char_to_memory_units(c: char) -> usize {
    match c {
        '0'..='9' => c.to_digit(10).unwrap() as usize,
        'A'..='Z' => c as usize - 'A' as usize + 1,
        _ => panic!("Unknown char"),
    }
}

fn memory_units_count(message: &[Vec<char>], transform_fn: fn(&[char]) -> Vec<char>) -> usize {
    message
        .iter()
        .map(|line| {
            let transformed = transform_fn(line);
            // println!("{} =>  {}", line.iter().collect::<String>(), compressed.iter().collect::<String>());
            transformed
                .iter()
                .map(|&c| char_to_memory_units(c))
                .sum::<usize>()
        })
        .sum()
}

fn nop(line: &[char]) -> Vec<char> {
    line.to_vec()
}

fn lossy_compress(line: &[char]) -> Vec<char> {
    let to_keep_at_sides = line.len() / 10;
    let len_middle = line.len() - to_keep_at_sides * 2;

    let mut new_line = Vec::new();
    new_line.extend_from_slice(&line[0..to_keep_at_sides]);
    new_line.extend(len_middle.to_string().chars());
    new_line.extend_from_slice(&line[line.len() - to_keep_at_sides..]);
    new_line
}

fn lossless_compress(line: &[char]) -> Vec<char> {
    let mut new_line = Vec::new();

    let mut cnt = 1;
    for i in 0..line.len() {
        if i < line.len() - 1 && line[i] == line[i + 1] {
            cnt += 1;
        } else {
            new_line.extend(cnt.to_string().chars());
            new_line.push(line[i]);
            cnt = 1;
        }
    }
    new_line
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let message = build(&input);

    println!("Part 1: {}", memory_units_count(&message, nop));
    println!("Part 2: {}", memory_units_count(&message, lossy_compress));
    println!(
        "Part 3: {}",
        memory_units_count(&message, lossless_compress)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_lossy_compress() {
        assert_eq!(lossy_compress(&str_to_vec("ABCDEFGHIJ")), str_to_vec("A8J"));
        assert_eq!(
            lossy_compress(&str_to_vec("OONNHHHHHANNNHHHHHHHH")),
            str_to_vec("OO17HH")
        );
        assert_eq!(
            lossy_compress(&str_to_vec("BDGGGSCLUUVLCBBBQNUUUFFFFFXXXXXXXXX")),
            str_to_vec("BDG29XXX")
        );
    }

    #[test]
    fn test_lossless_compress() {
        assert_eq!(
            lossless_compress(&str_to_vec("OONNHHHHHANNNHHHHHHHH")),
            str_to_vec("2O2N5H1A3N8H")
        );
        assert_eq!(
            lossless_compress(&str_to_vec("BDGGGSCLUUVLCBBBQNUUUFFFFFXXXXXXXXX")),
            str_to_vec("1B1D3G1S1C1L2U1V1L1C3B1Q1N3U5F9X")
        );
    }

    #[test]
    fn test_part1() {
        let message = build(&INPUT_TEST);
        assert_eq!(memory_units_count(&message, nop), 1247);
    }

    #[test]
    fn test_part2() {
        let message = build(&INPUT_TEST);
        assert_eq!(memory_units_count(&message, lossy_compress), 219);
    }

    #[test]
    fn test_part3() {
        let message = build(&INPUT_TEST);
        assert_eq!(memory_units_count(&message, lossless_compress), 539);
    }
}
