use std::error::Error;
use std::io::{self, BufRead, BufReader};

fn compute(program: &mut [u32]) {
    let mut pos = 0;
    loop {
        match program[pos] {
            1 => program[program[pos+3] as usize] = program[program[pos+1] as usize] + program[program[pos+2] as usize],
            2 => program[program[pos+3] as usize] = program[program[pos+1] as usize] * program[program[pos+2] as usize],
            99 => break,
            _ => panic!("unknown opcode {} at position {}", program[pos], pos),
        }
        pos += 4;
    }
}

#[test]
fn example0() {
    let mut program = vec![1,9,10,3,2,3,11,0,99,30,40,50];
    compute(&mut program);
    assert_eq!(program, vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
}

#[test]
fn example1() {
    let mut program = vec![1,0,0,0,99];
    compute(&mut program);
    assert_eq!(program, vec![2,0,0,0,99]);
}

#[test]
fn example2() {
    let mut program = vec![2,3,0,3,99];
    compute(&mut program);
    assert_eq!(program, vec![2,3,0,6,99]);
}

#[test]
fn example3() {
    let mut program = vec![2,4,4,5,99,0];
    compute(&mut program);
    assert_eq!(program, vec![2,4,4,5,99,9801]);
}

#[test]
fn example4() {
    let mut program = vec![1,1,1,4,99,5,6,0,99];
    compute(&mut program);
    assert_eq!(program, vec![30,1,1,4,2,5,6,0,99]);
}

fn main() -> Result<(), Box<dyn Error>> {
    let buffered = BufReader::new(io::stdin());
    let mut program = vec![];

    for line in buffered.lines() {
        for number in line?.trim().split(',') {
            program.push(number.parse::<u32>()?);
        }
    }

    let original_program = program.to_vec();

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            program = original_program.to_vec();
            program[1] = noun;
            program[2] = verb;
            compute(&mut program);
            if program[0] == 19690720 {
                println!("Found matching program. Noun = {}, verb = {}, answer is therefore: {}", noun, verb, 100 * noun + verb);
                break 'outer;
            }
        }
    }

    Ok(())
}
