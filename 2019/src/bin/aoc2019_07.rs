use std::error::Error;
use aoc2019::{IntcodeMachine,MachineState,Value};
use itertools::Itertools;

fn find_best_phase_setting(run: fn(&IntcodeMachine, &Vec<Value>) -> Value, range: std::ops::Range<Value>, template: &IntcodeMachine) -> (Value, Vec<Value>) {
    let mut max = 0;
    let mut best_phase_setting = vec![];
    for perm in range.permutations(5) {
        let output = run(&template, &perm);
        if output > max {
            max = output;
            best_phase_setting = perm;
        }
    }
    (max, best_phase_setting)
}

fn run_chain(template: &IntcodeMachine, phase_settings: &Vec<Value>) -> Value {
    let mut previous_output = 0;
    for phase in phase_settings {
        let mut amplifier = template.clone();
        amplifier.set_input(vec![*phase, previous_output]);
        amplifier.compute();
        previous_output = amplifier.get_output();
    }
    previous_output
}

fn run_chain_feedback(template: &IntcodeMachine, phase_settings: &Vec<Value>) -> Value {
    let mut amplifiers = vec![];
    for phase in phase_settings {
        let mut amplifier = template.clone();
        amplifier.push_input(*phase);
        amplifiers.push(amplifier);
    }
    amplifiers[0].push_input(0);

    let count = phase_settings.len();
    'outer: loop {
        for pos in 0..count {
            let state = amplifiers[pos].compute();
            let output = amplifiers[pos].get_output();
            amplifiers[(pos+1) % count].push_input(output);
            if pos == count - 1 && state == MachineState::Done {
                break 'outer output;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let template = IntcodeMachine::from_stdin();

    let output = find_best_phase_setting(run_chain, 0..5, &template);
    println!("Output value without feedback is {} for phase setting {:?}", output.0, output.1);

    let output = find_best_phase_setting(run_chain_feedback, 5..10, &template);
    println!("Output value _with_  feedback is {} for phase setting {:?}", output.0, output.1);

    Ok(())
}

#[test]
fn example1a() {
    let template = IntcodeMachine::from_string("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
    assert_eq!(find_best_phase_setting(run_chain, 0..5, &template), (43210, vec![4,3,2,1,0]));
}

#[test]
fn example2a() {
    let template = IntcodeMachine::from_string("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
    assert_eq!(find_best_phase_setting(run_chain, 0..5, &template), (54321, vec![0,1,2,3,4]));
}

#[test]
fn example3a() {
    let template = IntcodeMachine::from_string("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
    assert_eq!(find_best_phase_setting(run_chain, 0..5, &template), (65210, vec![1,0,4,3,2]));
}

#[test]
fn example1b() {
    let template = IntcodeMachine::from_string("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
    assert_eq!(find_best_phase_setting(run_chain_feedback, 5..10, &template), (139629729, vec![9,8,7,6,5]));
}

#[test]
fn example2b() {
    let template = IntcodeMachine::from_string("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10");
    assert_eq!(find_best_phase_setting(run_chain_feedback, 5..10, &template), (18216, vec![9,7,8,5,6]));
}
