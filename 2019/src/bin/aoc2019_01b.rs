use std::error::Error;
use std::io::{self, BufRead, BufReader};

fn fuel(mass: u32) -> u32 {
    /* Fuel required to launch a given module is based on its mass.
       Specifically, to find the fuel required for a module, take its mass,
       divide by three, round down, and subtract 2. */
    (mass / 3).saturating_sub(2)
}

fn fuel_with_fuel_fuel(mass: u32) -> u32 {
    let fuel_for_mass = fuel(mass);

    let mut last_fuel_chunk = fuel_for_mass;
    let mut fuel_for_fuel = 0u32;
    loop {
        if (last_fuel_chunk == 0) {
            break;
        }
        last_fuel_chunk = fuel(last_fuel_chunk);
        fuel_for_fuel += last_fuel_chunk;
    }

    fuel_for_mass + fuel_for_fuel
}

fn main() -> Result<(), Box<dyn Error>> {
    let buffered = BufReader::new(io::stdin());
    let mut sum = 0u64;

    for line in buffered.lines() {
        let mass = line?.trim().parse::<u32>()?;
        sum += fuel_with_fuel_fuel(mass) as u64;
    }

    println!("{}", sum);

    Ok(())
}
