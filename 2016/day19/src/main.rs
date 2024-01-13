use std::io::{self, Read};

fn elf_getting_all_v1(elf_count: usize) -> usize {
    // This array has the index of the next elf that has presents
    let mut next_with_presents = vec![0; elf_count];
    for (i, e) in next_with_presents.iter_mut().enumerate().take(elf_count) {
        *e = i + 1;
    }
    next_with_presents[elf_count - 1] = 0;

    let mut elf_with_presents = elf_count;
    let mut stealer = 0;
    while elf_with_presents > 1 {
        // next looser
        let looser = next_with_presents[stealer];
        next_with_presents[stealer] = next_with_presents[looser];
        elf_with_presents -= 1;

        stealer = next_with_presents[stealer];
    }
    stealer + 1
}

fn elf_getting_all_v2(elf_count: usize) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let elf_count = input.trim().parse().unwrap();

    println!("Part 1: {}", elf_getting_all_v1(elf_count));
    println!("Part 2: {}", elf_getting_all_v2(elf_count));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(elf_getting_all_v1(3), 3);
        assert_eq!(elf_getting_all_v1(4), 1);
        assert_eq!(elf_getting_all_v1(5), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(elf_getting_all_v2(3), 3);
        assert_eq!(elf_getting_all_v2(4), 1);
        assert_eq!(elf_getting_all_v2(5), 2);
    }
}
