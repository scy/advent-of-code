use std::io::{self, Read};
use regex::Regex;


#[derive(Clone, Copy, Debug, PartialEq)]
struct Triple {
    x: i32,
    y: i32,
    z: i32,
}

impl Triple {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn from_string(string: &str) -> Triple {
        let re = Regex::new(r"^\s*<x= *(-?\d+), y= *(-?\d+), z= *(-?\d+)>\s*$").unwrap();
        let cap = re.captures(string).expect(&format!("Strange coordinate format: {}", string));
        Self::new(
            cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            cap.get(3).unwrap().as_str().parse::<i32>().unwrap(),
        )
    }

    fn energy(&self) -> u32 {
        (self.x.abs() + self.y.abs() + self.z.abs()) as u32
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
struct Moon {
    position: Triple,
    velocity: Triple,
}

impl Moon {
    fn new(position: Triple) -> Moon {
        Self {
            position,
            velocity: Triple::new(0, 0, 0),
        }
    }

    fn from_ints(x: i32, y: i32, z: i32) -> Moon {
        Self {
            position: Triple::new(x, y, z),
            velocity: Triple::new(0, 0, 0),
        }
    }

    fn apply_gravity(&mut self, other: Moon) {
        self.velocity.x += if other.position.x > self.position.x { 1 } else if other.position.x < self.position.x { -1 } else { 0 };
        self.velocity.y += if other.position.y > self.position.y { 1 } else if other.position.y < self.position.y { -1 } else { 0 };
        self.velocity.z += if other.position.z > self.position.z { 1 } else if other.position.z < self.position.z { -1 } else { 0 };
    }

    fn apply_velocity(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    fn potential_energy(&self) -> u32 {
        self.position.energy()
    }

    fn kinetic_energy(&self) -> u32 {
        self.velocity.energy()
    }

    fn total_energy(&self) -> u32 {
        self.potential_energy() * self.kinetic_energy()
    }
}


#[derive(Debug)]
struct Simulation {
    moons: Vec<Moon>,
}

impl Simulation {
    fn new() -> Self {
        Self { moons: vec![] }
    }

    fn from_moons(moons: Vec<Moon>) -> Self {
        Self { moons }
    }

    fn from_string(string: &str) -> Self {
        Self { moons: string.trim().lines().map(|line| Moon::new(Triple::from_string(&line))).collect() }
    }

    fn from_stdin() -> Self {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer);
        Self::from_string(&buffer)
    }

    fn apply_gravity(&mut self) {
        let moons_copy = self.moons.clone();
        for a in self.moons.iter_mut() {
            for b in moons_copy.iter() {
                a.apply_gravity(*b);
            }
        }
    }

    fn apply_velocity(&mut self) {
        for moon in self.moons.iter_mut() {
            moon.apply_velocity();
        }
    }

    fn do_step(&mut self) {
        self.apply_gravity();
        self.apply_velocity();
    }

    fn do_steps(&mut self, count: u32) {
        for step in 0..count {
            self.do_step();
        }
    }

    fn total_energy(&self) -> u32 {
        self.moons.iter().map(|moon| moon.total_energy()).sum()
    }
}


fn main() {
    let mut sim = Simulation::from_stdin();
    sim.do_steps(1000);
    println!("Total energy after 1000 steps is {}", sim.total_energy());
}


#[test]
fn test_from_string() {
    assert_eq!(Triple::from_string("<x=-1, y=  0, z= 2>"), Triple { x: -1, y: 0, z: 2 });
}

#[test]
fn test_apply_gravity() {
    let mut simulation = Simulation::from_moons(vec![
        Moon::from_ints(1, 2, 3),
        Moon::from_ints(3, 1, 2),
        Moon::from_ints(2, 3, 4),
    ]);
    simulation.apply_gravity();
    assert_eq!(simulation.moons[0].velocity, Triple::new(2, 0, 0));
    assert_eq!(simulation.moons[1].velocity, Triple::new(-2, 2, 2));
    assert_eq!(simulation.moons[2].velocity, Triple::new(0, -2, -2));
}

#[test]
fn test_example_a1() {
    let mut sim = Simulation::from_string("
        <x=-1, y=0, z=2>
        <x=2, y=-10, z=-7>
        <x=4, y=-8, z=8>
        <x=3, y=5, z=-1>
    ");
    sim.do_steps(10);
    assert_eq!(sim.moons[0].potential_energy(), 6);
    assert_eq!(sim.moons[0].kinetic_energy(), 6);
    assert_eq!(sim.moons[0].total_energy(), 36);
    assert_eq!(sim.total_energy(), 179);
}

#[test]
fn test_example_a2() {
    let mut sim = Simulation::from_string("
        <x=-8, y=-10, z=0>
        <x=5, y=5, z=10>
        <x=2, y=-7, z=3>
        <x=9, y=-8, z=-3>
    ");
    sim.do_steps(100);
    assert_eq!(sim.moons[0].potential_energy(), 29);
    assert_eq!(sim.moons[0].kinetic_energy(), 10);
    assert_eq!(sim.moons[0].total_energy(), 290);
    assert_eq!(sim.total_energy(), 1940);
}
