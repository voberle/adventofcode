use std::io::{self, Read};

mod circuit;

// fn part2(wires: &FxHashMap<String, u8>, gates: &[Gate]) -> i64 {
//     0
// }

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (wires, gates) = circuit::build(&input);

    println!("Part 1: {}", circuit::z_output_number(&wires, &gates));
    // println!("Part 2: {}", part2(&wires, &gates));
}

#[cfg(test)]
mod tests {
    pub const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    pub const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");
}
