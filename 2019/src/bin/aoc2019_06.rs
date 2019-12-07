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

    fn add(&mut self, def: &str) {
        if let [orbited, orbiting] = def.split(')').collect::<Vec<&str>>()[0..2] {
            self.orbits.insert(orbiting.to_string(), orbited.to_string());
        }
    }

    fn count_orbits(&self, name: &str) -> u32 {
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

    fn find_common_orbiting(&self, a: &str, b: &str) -> &String {
        let mut parents = vec![];
        let mut cur = self.orbits.get(a).unwrap();
        loop {
            parents.push(cur);
            if let Some(new_cur) = self.orbits.get(cur) {
                cur = new_cur;
            } else {
                break;
            }
        }
        cur = self.orbits.get(b).unwrap();
        loop {
            if let Some(common) = parents.iter().find(|&&parent| parent == cur) {
                break *common;
            }
            cur = self.orbits.get(cur).unwrap();
        }
    }

    fn calc_transfers(&self, a: &str, b: &str) -> u32 {
        self.count_orbits(self.orbits.get(a).unwrap()) +
        self.count_orbits(self.orbits.get(b).unwrap()) -
        2 * self.count_orbits(self.find_common_orbiting(self.orbits.get(a).unwrap(), self.orbits.get(b).unwrap()))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let buffered = BufReader::new(io::stdin());
    let mut map = SpaceMap::new();

    for line in buffered.lines() {
        map.add(&line?[..]);
    }

    println!("Checksum is {}.", map.checksum());
    println!("Minimum number of orbital transfers is {}.", map.calc_transfers("YOU", "SAN"));

    Ok(())
}

#[test]
fn test_single_orbit() {
    let mut map = SpaceMap::new();
    map.add("COM)A");
    let mut expected = HashMap::new();
    expected.insert("A".to_string(), "COM".to_string());
    assert_eq!(map.orbits, expected);
}

#[test]
fn example_a() {
    let mut map = SpaceMap::new();
    map.add("COM)B");
    map.add("B)C");
    map.add("C)D");
    map.add("D)E");
    map.add("E)F");
    map.add("B)G");
    map.add("G)H");
    map.add("D)I");
    map.add("E)J");
    map.add("J)K");
    map.add("K)L");
    assert_eq!(map.count_orbits("COM"), 0);
    assert_eq!(map.count_orbits("B"), 1);
    assert_eq!(map.count_orbits("H"), 3);
    assert_eq!(map.count_orbits("E"), 4);
    assert_eq!(map.checksum(), 42);
}

#[test]
fn example_b() {
    let mut map = SpaceMap::new();
    map.add("COM)B");
    map.add("B)C");
    map.add("C)D");
    map.add("D)E");
    map.add("E)F");
    map.add("B)G");
    map.add("G)H");
    map.add("D)I");
    map.add("E)J");
    map.add("J)K");
    map.add("K)L");
    map.add("K)YOU");
    map.add("I)SAN");
    assert_eq!(map.find_common_orbiting("YOU", "SAN"), "D");
    assert_eq!(map.calc_transfers("YOU", "SAN"), 4);
}
