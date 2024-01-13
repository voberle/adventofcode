use std::io::{self, Read};

fn build(input: &str) -> Vec<bool> {
    input.chars().map(|c| c == '^').collect()
}

fn next_row(row: &[bool]) -> Vec<bool> {
    // Configurations that cause traps:
    const TRAP_CONFIGS: [[bool; 3]; 4] = [
        [true, true, false],
        [false, true, true],
        [true, false, false],
        [false, false, true],
    ];
    let mut next = Vec::with_capacity(row.len());
    next.push(row[1]);
    next.extend(
        row.windows(3)
            .map(|w| TRAP_CONFIGS.contains(&[w[0], w[1], w[2]])),
    );
    next.push(row[row.len() - 2]);
    next
}

fn count_safe_tiles(row: &[bool]) -> usize {
    row.iter().filter(|&&e| !e).count()
}

fn safe_tiles_count(first_row: &[bool], rows_count: usize) -> usize {
    let mut row = first_row.to_vec();
    let mut total_safe_tiles = count_safe_tiles(&row);
    for _ in 1..rows_count {
        row = next_row(&row);
        total_safe_tiles += count_safe_tiles(&row);
    }
    total_safe_tiles
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let first_row = build(&input);

    println!("Part 1: {}", safe_tiles_count(&first_row, 40));
    println!("Part 2: {}", safe_tiles_count(&first_row, 400_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_string(row: &[bool]) -> String {
        row.iter().map(|&e| if e { '^' } else { '.' }).collect()
    }

    #[test]
    fn test_next_row() {
        assert_eq!(to_string(&next_row(&build("..^^."))), ".^^^^");
        assert_eq!(to_string(&next_row(&build(".^^^^"))), "^^..^");
    }

    #[test]
    fn test_part1() {
        assert_eq!(safe_tiles_count(&build("..^^."), 3), 6);
        assert_eq!(safe_tiles_count(&build(".^^.^.^^^^"), 10), 38);
    }
}
