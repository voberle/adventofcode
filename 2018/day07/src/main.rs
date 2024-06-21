use std::{
    fmt,
    io::{self, Read},
};

use regex::Regex;

#[inline]
fn char(s: &str) -> char {
    s.chars().next().unwrap()
}

// Returns a list of dependencies:
// Index corresponds to the step that needs to wait (A is 0, B is 1 etc).
// Values are the index of the steps that it depends on.
fn build<const STEP_COUNT: usize>(input: &str) -> Vec<Vec<usize>> {
    let mut result = vec![Vec::new(); STEP_COUNT];
    let re = Regex::new(r"Step (\w) must be finished before step (\w) can begin.").unwrap();
    for line in input.lines() {
        let p = re.captures(line).unwrap();
        let left_idx = char2idx(char(&p[1]));
        let right_idx = char2idx(char(&p[2]));
        result[right_idx].push(left_idx);
    }
    result
}

#[inline]
fn char2idx(c: char) -> usize {
    (c as u8 - b'A') as usize
}

#[inline]
#[allow(clippy::cast_possible_truncation)]
fn idx2char(idx: usize) -> char {
    char::from(b'A' + idx as u8)
}

// State of the steps
#[derive(Debug, Clone, Copy)]
enum State {
    NotReady,
    Completed(usize),
    InProgress(usize), // used in part 2
    Ready,
}
use State::{Completed, InProgress, NotReady, Ready};

impl State {
    fn get_id(&self) -> usize {
        match self {
            Completed(id) | InProgress(id) => *id,
            NotReady | Ready => panic!("No ID"),
        }
    }
}

// Since we just deal with up to 26 steps (letters in alphabetical order),
// tracking them in an array is convenient.
//  0: Not ready to exec.
//  1-26: Executed, in that order.
//  READY: Not executed, but ready.
fn build_steps_array<const STEP_COUNT: usize>(deps: &[Vec<usize>]) -> [State; STEP_COUNT] {
    let mut steps: [State; STEP_COUNT] = [Ready; STEP_COUNT];
    // Mark all that cannot be executed initially.
    for (i, d) in deps.iter().enumerate() {
        if !d.is_empty() {
            steps[i] = NotReady;
        }
    }
    steps
}

// Mark the steps ready to be executed.
fn mark_ready<const STEP_COUNT: usize>(deps: &[Vec<usize>], steps: &mut [State]) {
    for idx in 0..STEP_COUNT {
        if let NotReady = steps[idx] {
            // If it's on the right side of a dependency and not ready, we can only do it if we have done all the prerequisites.
            if deps[idx].iter().all(|v| matches!(steps[*v], Completed(_))) {
                steps[idx] = Ready;
            }
        }
    }
}

fn steps_array_to_string<const STEP_COUNT: usize>(steps: &[State]) -> String {
    (1..=STEP_COUNT)
        .map(|idx| {
            let c_as_int = steps
                .iter()
                .map(|s| {
                    if let Completed(i) = s {
                        i
                    } else {
                        panic!("Not all steps completed")
                    }
                })
                .position(|i| *i == idx)
                .unwrap();
            idx2char(c_as_int)
        })
        .collect()
}

fn steps_in_order<const STEP_COUNT: usize>(deps: &[Vec<usize>]) -> String {
    let mut steps: [State; STEP_COUNT] = build_steps_array(deps);

    let mut pos = 1;
    while pos <= STEP_COUNT {
        mark_ready::<STEP_COUNT>(deps, &mut steps);

        // Do first in alphabetical order.
        if let Some(to_exec_idx) = steps.iter().position(|v| matches!(*v, Ready)) {
            steps[to_exec_idx] = Completed(pos);
            pos += 1;
        }
    }

    steps_array_to_string::<STEP_COUNT>(&steps)
}

#[derive(Debug, Clone)]
struct Worker {
    task_id: Option<usize>,
    time_remaining: usize,
}

impl Worker {
    fn new() -> Self {
        Self {
            task_id: None,
            time_remaining: 0,
        }
    }

    fn is_free(&self) -> bool {
        self.task_id.is_none()
    }

    fn take_task(&mut self, task_id: usize, step_duration_offset: usize) {
        self.task_id = Some(task_id);
        self.time_remaining = task_id + 1 + step_duration_offset;
    }

    fn next_second(&mut self) {
        self.time_remaining -= 1;
        if self.time_remaining == 0 {
            self.task_id = None;
        }
    }
}

impl fmt::Display for Worker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(task_id) = self.task_id {
            write!(f, "{}", idx2char(task_id))
        } else {
            write!(f, ".")
        }
    }
}

#[allow(dead_code)]
fn print_workers_state(workers: &[Worker], seconds: usize) {
    print!("{}\t", seconds);
    for w in workers {
        print!("{}\t", w);
    }
    println!();
}

fn time_to_complete<const STEP_COUNT: usize>(
    deps: &[Vec<usize>],
    workers_count: usize,
    step_duration_offset: usize,
) -> usize {
    // Workers array: They can be doing nothing or working on a task.
    let mut workers: Vec<Worker> = vec![Worker::new(); workers_count];

    let mut steps: [State; STEP_COUNT] = build_steps_array(deps);

    let mut completed_steps = 0;
    let mut seconds = 0;

    while completed_steps < STEP_COUNT {
        mark_ready::<STEP_COUNT>(deps, &mut steps);

        // Find free worker
        while let Some(free_worker_idx) = workers.iter().position(Worker::is_free) {
            // Do next in alphabetical order.
            if let Some(task_id) = steps.iter().position(|v| matches!(*v, Ready)) {
                // Task started
                steps[task_id] = InProgress(free_worker_idx);
                workers[free_worker_idx].take_task(task_id, step_duration_offset);
            } else {
                // We have a free worker but nothing to do for him
                break;
            }
        }

        seconds += 1;
        // print_workers_state(&workers, seconds);

        // Update each worker
        for worker in workers.iter_mut().take(workers_count) {
            if !worker.is_free() {
                let task_id = worker.task_id.unwrap();
                worker.next_second();
                if worker.is_free() {
                    // Mark task completed
                    steps[task_id] = Completed(steps[task_id].get_id());
                    completed_steps += 1;
                }
            }
        }
    }

    seconds
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let dependencies = build::<26>(input.trim());

    println!("Part 1: {}", steps_in_order::<26>(&dependencies));
    println!("Part 2: {}", time_to_complete::<26>(&dependencies, 5, 60));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(steps_in_order::<6>(&build::<6>(INPUT_TEST)), "CABDFE");
    }

    #[test]
    fn test_part2() {
        assert_eq!(time_to_complete::<6>(&build::<6>(INPUT_TEST), 2, 0), 15);
    }
}
