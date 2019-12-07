use std::error::Error;
use aoc2019::IntcodeMachine;

fn main() -> Result<(), Box<dyn Error>> {
    let mut machine = IntcodeMachine::from_stdin();
    machine.set_input(vec![5]);
    machine.compute();
    println!("Diagnostic code: {}", machine.get_output());
    Ok(())
}

#[test]
fn long_example() {
    let mut machine = IntcodeMachine::from_string("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
    machine.set_input(vec![5]);
    machine.compute();
    assert_eq!(machine.get_output(), 999);
}
