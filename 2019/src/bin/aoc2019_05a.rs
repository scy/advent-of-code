use std::error::Error;
use aoc2019::IntcodeMachine;

fn main() -> Result<(), Box<dyn Error>> {
    let mut machine = IntcodeMachine::from_stdin();
    machine.set_input(1);
    machine.compute();
    println!("Diagnostic code: {}", machine.get_output());
    Ok(())
}

#[test]
fn test_input() {
    let mut machine = IntcodeMachine::from_string("3,0,99");
    machine.set_input(23);
    machine.compute();
    assert_eq!(machine.get_program(), "23,0,99");
}

#[test]
fn test_output() {
    let mut machine = IntcodeMachine::from_string("4,0,99");
    machine.compute();
    assert_eq!(machine.get_output(), 4);
}
