use std::error::Error;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point(i32, i32);

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Wire(u8, u32),
    Intersection(u32, u32),
}

struct Field {
    num_wires: u8,
    contents: HashMap<Point, Cell>,
    intersections: Vec<u32>,
}

impl Field {
    fn new() -> Self {
        Field { num_wires: 0, contents: HashMap::new(), intersections: vec![] }
    }

    fn draw_wire(&mut self, instructions: &str) {
        if self.num_wires > 1 {
            panic!("Currently, only up to two wires are supported.");
        }
        self.num_wires += 1;

        let mut position = Point(0, 0);
        let mut steps = 0;
        for instruction in instructions.split(',') {
            self.draw_step(&instruction, self.num_wires, &mut position, &mut steps);
        }
    }

    fn draw_step(&mut self, instruction: &str, wire: u8, position: &mut Point, steps: &mut u32) {
        let direction = char::from(instruction.as_bytes()[0]);
        let length = (&instruction[1..]).parse::<i32>().unwrap();
        for _ in 0..length {
            match direction {
                'L' => position.0 -= 1,
                'R' => position.0 += 1,
                'U' => position.1 += 1,
                'D' => position.1 -= 1,
                _ => panic!("Unknown direction: {}", direction),
            }
            *steps += 1;
            let wire_piece = Cell::Wire(wire, *steps);

            if let Some(existing_wire) = self.contents.insert(*position, wire_piece) {
                if let Cell::Wire(existing_num, existing_steps) = existing_wire {
                    if existing_num == wire {
                        self.contents.insert(*position, existing_wire);
                    } else {
                        self.contents.insert(*position, Cell::Intersection(*steps, existing_steps));
                        self.intersections.push(*steps + existing_steps);
                    }
                }
            }
        }
    }

    fn find_nearest_intersection(&self) -> u32 {
        *self.intersections.iter().min().unwrap()
    }
}

#[test]
fn example1() {
    let mut field = Field::new();
    field.draw_wire("R8,U5,L5,D3");
    field.draw_wire("U7,R6,D4,L4");
    assert_eq!(field.find_nearest_intersection(), 30);
}

#[test]
fn example2() {
    let mut field = Field::new();
    field.draw_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    field.draw_wire("U62,R66,U55,R34,D71,R55,D58,R83");
    assert_eq!(field.find_nearest_intersection(), 610);
}

#[test]
fn example3() {
    let mut field = Field::new();
    field.draw_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
    field.draw_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
    assert_eq!(field.find_nearest_intersection(), 410);
}

fn main() -> Result<(), Box<dyn Error>> {
    let buffered = BufReader::new(io::stdin());
    let mut field = Field::new();

    for line in buffered.lines() {
        field.draw_wire(&line?)
    }

    println!("Nearest intersection is {} away.", field.find_nearest_intersection());

    Ok(())
}
