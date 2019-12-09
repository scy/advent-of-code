use std::error::Error;
use aoc2019::IntcodeMachine;

fn main() -> Result<(), Box<dyn Error>> {
    let mut machine = IntcodeMachine::from_stdin();
    machine.push_input(1);

    machine.compute();
    let outputs = machine.get_outputs();

    if outputs.len() != 1 {
        println!("BOOST found errors: {:?}", outputs);
    } else {
        println!("BOOST keycode: {}", outputs[0]);
    }

    Ok(())
}
