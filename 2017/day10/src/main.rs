use std::io::{self, Read};

fn build(input: &str) -> Vec<usize> {
    input.split(',').map(|v| v.parse().unwrap()).collect()
}

fn hash(lengths: &[usize], size: usize) -> Vec<usize> {
    let wrapping = |i| (i % size + size) % size;

    let mut list: Vec<usize> = (0..size).collect();
    let mut current_pos = 0;

    for (skip_size, &length) in lengths.iter().enumerate() {
        for i in 0..length / 2 {
            let fi = wrapping(current_pos + i);
            let li = wrapping(current_pos + length - 1 - i);
            // println!("Swap {fi} - {li}");
            list.swap(fi, li);
        }

        current_pos = wrapping(current_pos + length + skip_size);
        // println!("{length}: {:?}; pos={current_pos}, skip_size={skip_size}", list);
    }
    list
}

fn part2(lengths: &[usize]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input_parsed = build(&input);

    let final_list = hash(&input_parsed, 256);
    println!("Part 1: {}", final_list[0] * final_list[1]);
    println!("Part 2: {}", part2(&input_parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let final_list = hash(&[3, 4, 1, 5], 5);
        assert_eq!(final_list, &[3, 4, 2, 1, 0]);
        assert_eq!(final_list[0] * final_list[1], 12);
        assert!(false);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build("1,2")), 0);
    }
}
