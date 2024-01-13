use std::io::{self, Read};

const fn wrapping_index(i: usize, len: usize) -> usize {
    (i % len + len) % len
}

fn elf_getting_all(elf_count: usize) -> usize {
    let mut has_presents = vec![true; elf_count];
    let mut elf_with_presents = elf_count;
    let mut i = 0;
    while elf_with_presents > 1 {
        // i is the stealer
        // find next looser
        i = wrapping_index(i + 1, elf_count);
        while !has_presents[i] {
            i = wrapping_index(i + 1, elf_count);
        }

        // now i is the looser, steal his present
        has_presents[i] = false;
        elf_with_presents -= 1;

        // find next elf with presents
        i = wrapping_index(i + 1, elf_count);
        while !has_presents[i] {
            i = wrapping_index(i + 1, elf_count);
        }
    }
    i + 1
}

fn part2(elf_count: usize) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let elf_count = input.trim().parse().unwrap();

    println!("Part 1: {}", elf_getting_all(elf_count));
    println!("Part 2: {}", part2(elf_count));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(elf_getting_all(3), 3);
        assert_eq!(elf_getting_all(4), 1);
        assert_eq!(elf_getting_all(5), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(5), 0);
    }
}
