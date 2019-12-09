use std::error::Error;
use aoc2019::IntcodeMachine;

fn main() -> Result<(), Box<dyn Error>> {
    let mut test_machine = IntcodeMachine::from_stdin();
    let mut sensor_machine = test_machine.clone();
    test_machine.push_input(1);
    sensor_machine.push_input(2);

    test_machine.compute();
    let outputs = test_machine.get_outputs();

    if outputs.len() != 1 {
        println!("BOOST found errors: {:?}", outputs);
    } else {
        println!("BOOST keycode: {}", outputs[0]);
        println!("Boosting sensors...");
        sensor_machine.compute();
        println!("Distress signal coordinates: {}", sensor_machine.get_output());
    }

    Ok(())
}
