use std::io::{self, Read};

struct Functions {
    a: u64, // add
    b: u64, // multiply
    c: u32, // raise to the power of
}

impl Functions {
    fn build(input: &str) -> Self {
        let mut it = input.lines();
        let a = it
            .next()
            .unwrap()
            .strip_prefix("Function A: ADD ")
            .unwrap()
            .parse()
            .unwrap();
        let b = it
            .next()
            .unwrap()
            .strip_prefix("Function B: MULTIPLY ")
            .unwrap()
            .parse()
            .unwrap();
        let c = it
            .next()
            .unwrap()
            .strip_prefix("Function C: RAISE TO THE POWER OF ")
            .unwrap()
            .parse()
            .unwrap();
        Self { a, b, c }
    }

    fn apply(&self, n: u64) -> u64 {
        n.pow(self.c) * self.b + self.a
    }
}

fn build(input: &str) -> (Functions, Vec<u64>) {
    let parts: Vec<_> = input.split("\n\n").collect();
    (
        Functions::build(parts[0]),
        parts[1].lines().map(|n| n.parse().unwrap()).collect(),
    )
}

fn median_price(functions: &Functions, rooms: &[u64]) -> u64 {
    let mut sorted = rooms.to_vec();
    sorted.sort_unstable();

    let median_room = sorted[sorted.len() / 2];
    functions.apply(median_room)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (functions, rooms) = build(&input);

    println!("Part 1: {}", median_price(&functions, &rooms));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (functions, rooms) = build(&INPUT_TEST);
        assert_eq!(median_price(&functions, &rooms), 9130674516975);
    }
}
