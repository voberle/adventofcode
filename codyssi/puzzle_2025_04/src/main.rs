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

fn calc_memory_units(line: &[char]) -> usize {
    line.iter().map(|&c| char_to_memory_units(c)).sum()
}

fn memory_units_count(message: &[Vec<char>]) -> usize {
    message.iter().map(|line| calc_memory_units(line)).sum()
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

fn memory_units_after_lossy_compression(message: &[Vec<char>]) -> usize {
    message
        .iter()
        .map(|line| {
            let compressed = lossy_compress(line);
            // println!("{} =>  {}", line.iter().collect::<String>(), compressed.iter().collect::<String>());
            calc_memory_units(&compressed)
        })
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let message = build(&input);

    println!("Part 1: {}", memory_units_count(&message));
    println!("Part 2: {}", memory_units_after_lossy_compression(&message));
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
    fn test_part1() {
        let message = build(&INPUT_TEST);
        assert_eq!(memory_units_count(&message), 1247);
    }

    #[test]
    fn test_part2() {
        let message = build(&INPUT_TEST);
        assert_eq!(memory_units_after_lossy_compression(&message), 219);
    }
}
