#[cfg(test)]
mod day02 {
    use std::fs;

    use crate::{Address, IntcodeComputer};

    fn exec(code: &str) -> String {
        let mut computer = IntcodeComputer::build(code);
        computer.exec();
        computer.dump_memory()
    }

    #[test]
    fn test_exec() {
        assert_eq!(
            exec("1,9,10,3,2,3,11,0,99,30,40,50"),
            "3500,9,10,70,2,3,11,0,99,30,40,50"
        );
        assert_eq!(exec("1,0,0,0,99"), "2,0,0,0,99");
        assert_eq!(exec("2,3,0,3,99"), "2,3,0,6,99");
        assert_eq!(exec("2,4,4,5,99,0"), "2,4,4,5,99,9801");
        assert_eq!(exec("1,1,1,4,99,5,6,0,99"), "30,1,1,4,2,5,6,0,99");
    }

    fn run_noun_verb(computer: &IntcodeComputer, noun: i32, verb: i32) -> i32 {
        let mut computer = computer.clone();
        computer.set(Address::from(1), noun);
        computer.set(Address::from(2), verb);
        computer.exec();
        computer.mem[0]
    }

    fn find_noun_verb(computer: &IntcodeComputer) -> i32 {
        const TARGET: i32 = 19_690_720;
        for noun in 0..=99 {
            for verb in 0..=99 {
                let output = run_noun_verb(computer, noun, verb);
                if output == TARGET {
                    return 100 * noun + verb;
                }
            }
        }
        panic!("Target output not found")
    }

    #[test]
    #[cfg_attr(not(feature = "previous_days"), ignore)]
    fn real_input() {
        let input =
            fs::read_to_string("../day02/resources/input").expect("Unable to read input file");
        let result1 = fs::read_to_string("../day02/resources/part1.answer")
            .expect("Unable to read input file");
        let result2 = fs::read_to_string("../day02/resources/part2.answer")
            .expect("Unable to read input file");

        let computer = IntcodeComputer::build(&input);

        let part1 = run_noun_verb(&computer, 12, 2);
        assert_eq!(part1.to_string(), result1.trim());

        let part2 = find_noun_verb(&computer);
        assert_eq!(part2.to_string(), result2.trim());
    }
}

#[cfg(test)]
mod day05 {
    use std::fs;

    use crate::IntcodeComputer;

    fn run_io(code: &str, input: i32) -> i32 {
        let mut computer = IntcodeComputer::build(code);
        computer.input.push_back(input);
        computer.exec();
        *computer.output.last().unwrap()
    }

    #[test]
    fn test_cmp() {
        let c = "3,9,8,9,10,9,4,9,99,-1,8";
        assert_eq!(run_io(c, 8), 1);
        assert_eq!(run_io(c, 3), 0);
        let c = "3,9,7,9,10,9,4,9,99,-1,8";
        assert_eq!(run_io(c, 3), 1);
        assert_eq!(run_io(c, 9), 0);
        let c = "3,3,1108,-1,8,3,4,3,99";
        assert_eq!(run_io(c, 8), 1);
        assert_eq!(run_io(c, 3), 0);
        let c = "3,3,1107,-1,8,3,4,3,99";
        assert_eq!(run_io(c, 3), 1);
        assert_eq!(run_io(c, 9), 0);
    }

    #[test]
    fn test_jump() {
        let c = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        assert_eq!(run_io(c, 0), 0);
        assert_eq!(run_io(c, 4), 1);
        let c = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        assert_eq!(run_io(c, 0), 0);
        assert_eq!(run_io(c, 4), 1);
    }

    #[test]
    fn test_larger_program() {
        let c = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        assert_eq!(run_io(c, 1), 999);
        assert_eq!(run_io(c, 8), 1000);
        assert_eq!(run_io(c, 45), 1001);
    }

    fn run_diagnostic_test(computer: &IntcodeComputer, system_to_test_id: i32) -> i32 {
        let mut computer = computer.clone();
        computer.input.push_back(system_to_test_id);
        computer.exec();
        *computer.output.last().unwrap()
    }

    #[test]
    #[cfg_attr(not(feature = "previous_days"), ignore)]
    fn real_input() {
        let input =
            fs::read_to_string("../day05/resources/input").expect("Unable to read input file");
        let result1 = fs::read_to_string("../day05/resources/part1.answer")
            .expect("Unable to read input file");
        let result2 = fs::read_to_string("../day05/resources/part2.answer")
            .expect("Unable to read input file");

        let computer = IntcodeComputer::build(&input);
        let part1 = run_diagnostic_test(&computer, 1);
        assert_eq!(part1.to_string(), result1.trim());

        let part2 = run_diagnostic_test(&computer, 5);
        assert_eq!(part2.to_string(), result2.trim());
    }
}
