use std::error::Error;
use aoc2019::IntcodeMachine;
use itertools::Itertools;

fn find_best_phase_setting(template: &IntcodeMachine) -> (i32, Vec<i32>) {
    let mut max = 0;
    let mut best_phase_setting = vec![];
    for perm in (0..5).permutations(5) {
        let output = run_chain(&template, &perm);
        if output > max {
            max = output;
            best_phase_setting = perm;
        }
    }
    (max, best_phase_setting)
}

fn run_chain(template: &IntcodeMachine, phase_settings: &Vec<i32>) -> i32 {
    let mut previous_output = 0;
    for phase in phase_settings {
        let mut amplifier = IntcodeMachine::from_string(&template.get_program());
        amplifier.set_input(vec![*phase, previous_output]);
        amplifier.compute();
        previous_output = amplifier.get_output();
    }
    previous_output
}

fn main() -> Result<(), Box<dyn Error>> {
    let output = find_best_phase_setting(&IntcodeMachine::from_stdin());
    println!("Output value is {} for phase setting {:?}", output.0, output.1);

    Ok(())
}

#[test]
fn example1() {
    let template = IntcodeMachine::from_string("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
    assert_eq!(find_best_phase_setting(&template), (43210, vec![4,3,2,1,0]));
}

#[test]
fn example2() {
    let template = IntcodeMachine::from_string("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
    assert_eq!(find_best_phase_setting(&template), (54321, vec![0,1,2,3,4]));
}

#[test]
fn example3() {
    let template = IntcodeMachine::from_string("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
    assert_eq!(find_best_phase_setting(&template), (65210, vec![1,0,4,3,2]));
}
