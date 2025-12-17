use std::io::{self, Read};

use fxhash::FxHashMap;

struct Rack {
    graph: Vec<Vec<usize>>,
    you: usize,
    out: usize,
}

impl Rack {
    fn build(input: &str) -> Self {
        let list: FxHashMap<String, Vec<String>> = input
            .lines()
            .map(|line| {
                let parts: Vec<_> = line.split(": ").collect();
                let conns: Vec<_> = parts[1]
                    .split_ascii_whitespace()
                    .map(std::string::ToString::to_string)
                    .collect();
                (parts[0].to_string(), conns)
            })
            .collect();
        // println!("{:#?}", list);

        let mut names_to_id: FxHashMap<String, usize> = FxHashMap::default();
        names_to_id.extend(
            list.iter()
                .flat_map(|(k, v)| std::iter::once(k).chain(v.iter()))
                .map(|s| (s.clone(), usize::MAX)),
        );

        let you = 0;
        let out = names_to_id.len() - 1;

        *names_to_id.get_mut("you").unwrap() = you;
        *names_to_id.get_mut("out").unwrap() = out;
        let mut i = 1;
        for val in &mut names_to_id {
            if *val.1 != usize::MAX {
                continue;
            }
            *val.1 = i;
            i += 1;
        }
        // println!("{:#?}", names_to_id);

        let mut graph: Vec<Vec<usize>> = vec![vec![]; names_to_id.len()];
        for (dev, conns) in list {
            let i = *names_to_id.get(&dev).unwrap();
            for conn in conns {
                let k = *names_to_id.get(&conn).unwrap();
                graph[i].push(k);
            }
        }
        // println!("{:#?}", graph);

        Self { graph, you, out }
    }
}

fn total_paths(rack: &Rack) -> usize {
    0
}

fn part2(rack: &Rack) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input_parsed = Rack::build(&input);

    println!("Part 1: {}", total_paths(&input_parsed));
    println!("Part 2: {}", part2(&input_parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(total_paths(&Rack::build(INPUT_TEST)), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&Rack::build(INPUT_TEST)), 0);
    }
}
