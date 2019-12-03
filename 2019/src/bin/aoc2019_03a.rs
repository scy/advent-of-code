use std::error::Error;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point(i32, i32);

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Wire1,
    Wire2,
    Intersection,
}

struct Field {
    num_wires: u8,
    contents: HashMap<Point, Cell>,
    intersections: Vec<Point>,
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
        let wire = if self.num_wires == 1 { Cell::Wire1 } else { Cell::Wire2 };

        let mut position = Point(0, 0);
        for instruction in instructions.split(',') {
            self.draw_step(&instruction, &wire, &mut position);
        }
    }

    fn draw_step(&mut self, instruction: &str, wire: &Cell, position: &mut Point) {
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
            if let Some(existing_wire) = self.contents.insert(*position, *wire) {
                if existing_wire != *wire && existing_wire != Cell::Intersection {
                    self.intersections.push(*position);
                }
                self.contents.insert(*position, Cell::Intersection);
            }
        }
    }

    fn find_nearest_intersection(&self) -> i32 {
        self.intersections.iter().map(|point| point.0.abs() + point.1.abs()).min().unwrap()
    }
}

#[test]
fn example1() {
    let mut field = Field::new();
    field.draw_wire("R8,U5,L5,D3");
    field.draw_wire("U7,R6,D4,L4");
    assert_eq!(field.find_nearest_intersection(), 6);
}

#[test]
fn example2() {
    let mut field = Field::new();
    field.draw_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    field.draw_wire("U62,R66,U55,R34,D71,R55,D58,R83");
    assert_eq!(field.find_nearest_intersection(), 159);
}

#[test]
fn example3() {
    let mut field = Field::new();
    field.draw_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
    field.draw_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
    assert_eq!(field.find_nearest_intersection(), 135);
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
