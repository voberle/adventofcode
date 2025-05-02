use std::io::{self, Read};

fn build(input: &str) -> Vec<bool> {
    input.lines().map(|line| line == "TRUE").collect()
}

fn part1(values: &[bool]) -> usize {
    values
        .iter()
        .enumerate()
        .filter_map(|(id, val)| if *val { Some(id + 1) } else { None })
        .sum()
}

fn value_count(values: &[bool]) -> usize {
    values.iter().filter(|&&val| val).count()
}

fn convert_circuit(circuit: &[bool]) -> Vec<bool> {
    circuit
        .chunks(2)
        .enumerate()
        .map(|(i, gates)| {
            if i % 2 == 0 {
                gates[0] && gates[1]
            } else {
                gates[0] || gates[1]
            }
        })
        .collect()
}

fn gates_count(values: &[bool]) -> usize {
    let circuit = convert_circuit(values);
    value_count(&circuit)
}

#[allow(clippy::cast_possible_wrap)]
fn circuits_count(values: &[bool]) -> usize {
    let mut cnt = 0;
    let mut circuit = values.to_vec();
    while circuit.len() > 1 {
        cnt += value_count(&circuit);
        circuit = convert_circuit(&circuit);
    }
    cnt
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let values = build(&input);

    println!("Part 1: {}", part1(&values));
    println!("Part 2: {}", gates_count(&values));
    println!("Part 3: {}", circuits_count(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(part1(&build(INPUT_TEST)), 19);
    }

    #[test]
    fn test_part2() {
        assert_eq!(gates_count(&build(INPUT_TEST)), 2);
    }

    #[test]
    fn test_part3() {
        assert_eq!(circuits_count(&build(INPUT_TEST)), 7);
    }
}
