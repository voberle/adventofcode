use std::io::{self, Read};

use fxhash::{FxHashMap, FxHashSet};

trait Is {
    fn is(c: char) -> bool;
}

struct IsSymbol;

impl Is for IsSymbol {
    fn is(c: char) -> bool {
        !c.is_ascii_digit() && c != '.'
    }
}

#[allow(dead_code)]
struct IsGear;

impl Is for IsGear {
    fn is(c: char) -> bool {
        c == '*'
    }
}

struct Schematic(Vec<Vec<char>>);

impl From<&str> for Schematic {
    fn from(value: &str) -> Self {
        Self(value.lines().map(|line| line.chars().collect()).collect())
    }
}

impl Schematic {
    fn is_adj_line<I: Is>(&self, i: usize, j: usize) -> bool {
        I::is(self.0[i][j])
            || I::is(self.0[i][j.saturating_sub(1)])
            || I::is(self.0[i][usize::min(j + 1, self.0[i].len() - 1)])
    }

    fn is_adjacent<I: Is>(&self, i: usize, j: usize) -> bool {
        self.is_adj_line::<I>(i, j)
            || self.is_adj_line::<I>(i.saturating_sub(1), j)
            || self.is_adj_line::<I>(usize::min(i + 1, self.0.len() - 1), j)
    }

    fn add_if_gear(&self, i: usize, j: usize, adj_gears: &mut Vec<(usize, usize)>) {
        if self.0[i][j] == '*' {
            adj_gears.push((i, j));
        }
    }

    // Finds all the '*' adjacent to this position
    fn find_adjacent_gears(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut adj_gears = Vec::new();
        self.add_if_gear(i, j.saturating_sub(1), &mut adj_gears);
        self.add_if_gear(i, usize::min(j + 1, self.0[i].len() - 1), &mut adj_gears);
        for k in [i.saturating_sub(1), usize::min(i + 1, self.0.len() - 1)] {
            self.add_if_gear(k, j.saturating_sub(1), &mut adj_gears);
            self.add_if_gear(k, j, &mut adj_gears);
            self.add_if_gear(k, usize::min(j + 1, self.0[i].len() - 1), &mut adj_gears);
        }
        adj_gears
    }

    fn analyze(&self) -> (u32, u32) {
        let mut total = 0;
        // Maps the position of the '*' to the numbers around it.
        let mut gears: FxHashMap<(usize, usize), Vec<u32>> = FxHashMap::default();

        for (i, line) in self.0.iter().enumerate() {
            let mut n = 0;
            let mut include = false;
            let mut adj_gears: FxHashSet<(usize, usize)> = FxHashSet::default();
            line.iter().enumerate().for_each(|(j, c)| {
                if let Some(d) = c.to_digit(10) {
                    n = n * 10 + d;
                    if self.is_adjacent::<IsSymbol>(i, j) {
                        include = true;
                    }
                    adj_gears.extend(self.find_adjacent_gears(i, j));
                } else {
                    if include {
                        total += n;
                        for k in &adj_gears {
                            let list = gears.entry((k.0, k.1)).or_default();
                            list.push(n);
                            // println!("{} added for gear {}:{}", n, k.0, k.1);
                        }
                    }
                    n = 0;
                    include = false;
                    adj_gears.clear();
                }
            });
            // This is to handle the case of a number being last on the line
            if include {
                total += n;
                for k in &adj_gears {
                    let list = gears.entry((k.0, k.1)).or_default();
                    list.push(n);
                    // println!("{} added for gear {}:{}", n, k.0, k.1);
                }
            }
        }

        let gear_ratio_sum: u32 = gears
            .values()
            .filter(|v| v.len() == 2)
            .map(|v| v[0] * v[1])
            .sum();

        (total, gear_ratio_sum)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let schematic: Schematic = input.as_str().into();

    let (total, gear_ratio_sum) = schematic.analyze();

    println!("Part 1: {total}");
    println!("Part 2: {gear_ratio_sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test");

    #[test]
    fn test_part1() {
        let (total, _) = Into::<Schematic>::into(INPUT_TEST).analyze();
        assert_eq!(total, 4361);
    }

    #[test]
    fn test_part2() {
        let (_, gear_ratio_sum) = Into::<Schematic>::into(INPUT_TEST).analyze();
        assert_eq!(gear_ratio_sum, 467835);
    }
}
