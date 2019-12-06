use std::error::Error;
use std::collections::HashMap;
use std::io::{self, BufRead, BufReader};

struct SpaceMap {
    orbits: HashMap<String, String>,
}

impl SpaceMap {
    fn new() -> SpaceMap {
        Self { orbits: HashMap::new() }
    }

    fn add(&mut self, def: &String) {
        if let [orbited, orbiting] = def.split(')').collect::<Vec<&str>>()[0..2] {
            self.orbits.insert(orbiting.to_string(), orbited.to_string());
        }
    }

    fn count_orbits(&self, name: &String) -> u32 {
        if let Some(orbiting) = self.orbits.get(name) {
            1 + self.count_orbits(&orbiting)
        } else {
            0
        }
    }

    fn checksum(&self) -> u32 {
        let mut sum = 0;
        for (orbiting, _orbited) in self.orbits.iter() {
            sum += self.count_orbits(&orbiting);
        }
        sum
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let buffered = BufReader::new(io::stdin());
    let mut map = SpaceMap::new();

    for line in buffered.lines() {
        map.add(&line?);
    }

    println!("Checksum is {}.", map.checksum());

    Ok(())
}

#[test]
fn test_single_orbit() {
    let mut map = SpaceMap::new();
    map.add(&"COM)A".to_string());
    let mut expected = HashMap::new();
    expected.insert("A".to_string(), "COM".to_string());
    assert_eq!(map.orbits, expected);
}

#[test]
fn example() {
    let mut map = SpaceMap::new();
    map.add(&"COM)B".to_string());
    map.add(&"B)C".to_string());
    map.add(&"C)D".to_string());
    map.add(&"D)E".to_string());
    map.add(&"E)F".to_string());
    map.add(&"B)G".to_string());
    map.add(&"G)H".to_string());
    map.add(&"D)I".to_string());
    map.add(&"E)J".to_string());
    map.add(&"J)K".to_string());
    map.add(&"K)L".to_string());
    assert_eq!(map.count_orbits(&"COM".to_string()), 0);
    assert_eq!(map.count_orbits(&"B".to_string()), 1);
    assert_eq!(map.count_orbits(&"H".to_string()), 3);
    assert_eq!(map.count_orbits(&"E".to_string()), 4);
    assert_eq!(map.checksum(), 42);
}
