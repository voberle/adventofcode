/// All previous days tasks and tests.
///
/// Run real inputs with:
///     cargo t --features `previous_days`

#[cfg(test)]
fn get_input_results(day: &str) -> (String, String, String) {
    let input = std::fs::read_to_string(format!("../{day}/resources/input"))
        .expect("Unable to read input file");
    let result1 = std::fs::read_to_string(format!("../{day}/resources/part1.answer"))
        .expect("Unable to read part 1 answer file");
    let result2 = std::fs::read_to_string(format!("../{day}/resources/part2.answer"))
        .expect("Unable to read part 2 answer file");
    (
        input,
        result1.trim().to_string(),
        result2.trim().to_string(),
    )
}

#[cfg(test)]
mod day02 {
    use crate::{previous_days::get_input_results, IntcodeComputer, Param};

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

    fn run_noun_verb(computer: &IntcodeComputer, noun: i64, verb: i64) -> i64 {
        let mut computer = computer.clone();
        computer.set(&Param::from(1), noun);
        computer.set(&Param::from(2), verb);
        computer.exec();
        computer.mem[0]
    }

    fn find_noun_verb(computer: &IntcodeComputer) -> i64 {
        const TARGET: i64 = 19_690_720;
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
        let (input, result1, result2) = get_input_results("day02");
        let computer = IntcodeComputer::build(&input);

        let part1 = run_noun_verb(&computer, 12, 2);
        assert_eq!(part1.to_string(), result1);

        let part2 = find_noun_verb(&computer);
        assert_eq!(part2.to_string(), result2);
    }
}

#[cfg(test)]
mod day05 {
    use crate::{previous_days::get_input_results, IntcodeComputer};

    fn run_io(code: &str, input: i64) -> i64 {
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

    fn run_diagnostic_test(computer: &IntcodeComputer, system_to_test_id: i64) -> i64 {
        let mut computer = computer.clone();
        computer.input.push_back(system_to_test_id);
        computer.exec();
        *computer.output.last().unwrap()
    }

    #[test]
    #[cfg_attr(not(feature = "previous_days"), ignore)]
    fn real_input() {
        let (input, result1, result2) = get_input_results("day05");
        let computer = IntcodeComputer::build(&input);

        let part1 = run_diagnostic_test(&computer, 1);
        assert_eq!(part1.to_string(), result1.trim());

        let part2 = run_diagnostic_test(&computer, 5);
        assert_eq!(part2.to_string(), result2.trim());
    }
}

#[cfg(test)]
mod day07 {
    use crate::{previous_days::get_input_results, IntcodeComputer};
    use itertools::Itertools;

    fn build_amp(computer: &IntcodeComputer, phase_setting: i64) -> IntcodeComputer {
        let mut amp = computer.clone();
        amp.input.push_back(phase_setting);
        amp
    }

    fn exec_amp(amp: &mut IntcodeComputer, input: i64) -> i64 {
        amp.input.push_back(input);
        amp.exec();
        amp.output.pop().unwrap()
    }

    fn build_and_exec(computer: &IntcodeComputer, input: i64, phase_setting: i64) -> i64 {
        let mut amp = build_amp(computer, phase_setting);
        exec_amp(&mut amp, input)
    }

    fn get_thruster_signal(computer: &IntcodeComputer, phase_settings: &[i64]) -> i64 {
        let a_output = build_and_exec(computer, 0, phase_settings[0]);
        let b_output = build_and_exec(computer, a_output, phase_settings[1]);
        let c_output = build_and_exec(computer, b_output, phase_settings[2]);
        let d_output = build_and_exec(computer, c_output, phase_settings[3]);
        build_and_exec(computer, d_output, phase_settings[4])
    }

    fn max_thruster_signal(computer: &IntcodeComputer) -> i64 {
        (0..=4)
            .permutations(5)
            .map(|phase_settings| get_thruster_signal(computer, &phase_settings))
            .max()
            .unwrap()
    }

    fn get_thruster_signal_with_feedback(
        computer: &IntcodeComputer,
        phase_settings: &[i64],
    ) -> i64 {
        let mut amp_a = build_amp(computer, phase_settings[0]);
        let mut amp_b = build_amp(computer, phase_settings[1]);
        let mut amp_c = build_amp(computer, phase_settings[2]);
        let mut amp_d = build_amp(computer, phase_settings[3]);
        let mut amp_e = build_amp(computer, phase_settings[4]);

        let mut e_output = 0;
        while !amp_e.halted {
            let a_output = exec_amp(&mut amp_a, e_output);
            let b_output = exec_amp(&mut amp_b, a_output);
            let c_output = exec_amp(&mut amp_c, b_output);
            let d_output = exec_amp(&mut amp_d, c_output);
            e_output = exec_amp(&mut amp_e, d_output);
        }
        e_output
    }

    fn max_thruster_signal_with_feedback(computer: &IntcodeComputer) -> i64 {
        (5..=9)
            .permutations(5)
            .map(|phase_settings| get_thruster_signal_with_feedback(computer, &phase_settings))
            .max()
            .unwrap()
    }

    #[test]
    fn test_max_thruster_signal() {
        let computer = IntcodeComputer::build("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        assert_eq!(max_thruster_signal(&computer), 43210);

        let computer = IntcodeComputer::build(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
        );
        assert_eq!(max_thruster_signal(&computer), 54321);

        let computer = IntcodeComputer::build("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
        assert_eq!(max_thruster_signal(&computer), 65210);
    }

    #[test]
    fn test_max_thruster_signal_with_feedback() {
        let computer = IntcodeComputer::build(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        );
        assert_eq!(max_thruster_signal_with_feedback(&computer), 139629729);

        let computer = IntcodeComputer::build("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10");
        assert_eq!(max_thruster_signal_with_feedback(&computer), 18216);
    }

    #[test]
    #[cfg_attr(not(feature = "previous_days"), ignore)]
    fn real_input() {
        let (input, result1, result2) = get_input_results("day07");
        let computer = IntcodeComputer::build(&input);

        let part1 = max_thruster_signal(&computer);
        assert_eq!(part1.to_string(), result1.trim());

        let part2 = max_thruster_signal_with_feedback(&computer);
        assert_eq!(part2.to_string(), result2.trim());
    }
}
