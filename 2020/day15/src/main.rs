use std::io::{self, Read};

fn build(input: &str) -> Vec<u32> {
    input.split(',').map(|line| line.parse().unwrap()).collect()
}

fn number_at(starting_numbers: &[u32], pos: usize) -> u32 {
    let mut numbers = starting_numbers.to_vec();
    while numbers.len() < pos {
        let last_number = numbers.last().unwrap();
        numbers.push(if numbers[0..numbers.len() - 1].contains(last_number) {
            let last_nb_pos_rev = numbers
                .iter()
                .rev()
                .skip(1)
                .position(|v| v == last_number)
                .unwrap();
            u32::try_from(last_nb_pos_rev).unwrap() + 1
        } else {
            0
        });
    }
    *numbers.last().unwrap()
}

fn part2(starting_numbers: &[u32]) -> u32 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let starting_numbers = build(input.trim());

    println!("Part 1: {}", number_at(&starting_numbers, 2020));
    println!("Part 2: {}", part2(&starting_numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(number_at(&build("0,3,6"), 2020), 436);
        assert_eq!(number_at(&build("1,3,2"), 2020), 1);
        assert_eq!(number_at(&build("2,1,3"), 2020), 10);
        assert_eq!(number_at(&build("1,2,3"), 2020), 27);
        assert_eq!(number_at(&build("2,3,1"), 2020), 78);
        assert_eq!(number_at(&build("3,2,1"), 2020), 438);
        assert_eq!(number_at(&build("3,1,2"), 2020), 1836);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
