use std::error::Error;
use std::io::{self, BufRead, BufReader};

fn fuel(mass: u32) -> u32 {
    /* Fuel required to launch a given module is based on its mass.
       Specifically, to find the fuel required for a module, take its mass,
       divide by three, round down, and subtract 2. */
    mass / 3 - 2
}

fn main() -> Result<(), Box<dyn Error>> {
    let buffered = BufReader::new(io::stdin());
    let mut sum = 0u64;

    for line in buffered.lines() {
        let mass = line?.trim().parse::<u32>()?;
        sum += fuel(mass) as u64;
    }

    println!("{}", sum);

    Ok(())
}
