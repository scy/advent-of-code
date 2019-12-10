use std::convert::TryInto;
use std::io::{self, BufRead, BufReader};

#[derive(Debug,PartialEq)]
struct Asteroid {
    x: u16,
    y: u16,
}

impl Asteroid {
    fn angle_to(&self, dest: &Asteroid) -> f64 {
        let delta_y = (dest.y as i32) - (self.y as i32);
        let delta_x = (dest.x as i32) - (self.x as i32);
        ((delta_y as f64).atan2(delta_x as f64).to_degrees() * 10_000.0).round() / 10_000.0
    }

    fn distance_to(&self, dest: &Asteroid) -> f64 {
        let a2 = ((self.x as i32) - (dest.x as i32)).pow(2);
        let b2 = ((self.y as i32) - (dest.y as i32)).pow(2);
        (((a2 + b2) as f64).sqrt() * 10_000.0).round() / 10_000.0
    }
}

struct Map {
    asteroids: Vec<Asteroid>,
}

impl Map {
    fn from_stdin() -> Map {
        let mut map = Map { asteroids: vec![] };
        let buffered = BufReader::new(io::stdin());
        for (y, line) in buffered.lines().enumerate() {
            map.read_line(y.try_into().unwrap(), &line.unwrap());
        }
        map
    }

    fn from_string(input: &str) -> Map{
        let mut map = Map { asteroids: vec![] };
        for (y, line) in input.trim().lines().enumerate() {
            map.read_line(y.try_into().unwrap(), line);
        }
        map
    }

    fn read_line(&mut self, y: u16, line: &str) {
        for (x, character) in line.trim().chars().enumerate() {
            match character {
                '#' => self.asteroids.push(Asteroid { x: x.try_into().unwrap(), y: y }),
                _ => (),
            }
        }
    }

    fn can_see(&self, observer: &Asteroid, observing: &Asteroid) -> bool {
        let dest_angle = observer.angle_to(&observing);
        let dest_distance = observer.distance_to(&observing);
        //if debug { println!("Destination angle to {:?}: {}, distance: {}", observing, dest_angle, dest_distance); }
        for block_candidate in self.asteroids.iter() {
            if block_candidate == observer || block_candidate == observing { continue }
            let angle = observer.angle_to(&block_candidate);
            let distance = observer.distance_to(&block_candidate);
            //if debug { println!("    Candidate {:?} angle: {}, distance: {}", block_candidate, angle, distance); }
            if ((observer.angle_to(&block_candidate) == dest_angle) && (distance < dest_distance)) && distance > 0.0  {
                //if debug { println!("Asteroid {:?} blocks view from {:?} to {:?}", block_candidate, observer, observing); }
                return false
            }
        }
        //if debug { println!("View from {:?} to {:?} is unobstructed", observer, observing); }
        true
    }

    fn count_observables(&self, observer: &Asteroid) -> usize {
        self.asteroids.iter().filter(|target| observer != *target && self.can_see(&observer, &target)).count()
    }

    fn find_best_observer(&self) -> (&Asteroid, usize) {
        let mut count = 0;
        let mut best_observer = None;
        for observer in self.asteroids.iter() {
            let observer_count = self.count_observables(&observer);
            if observer_count > count {
                count = observer_count;
                best_observer = Some(observer);
            }
        }
        (best_observer.unwrap(), count)
    }
}

fn main() {
    let map = Map::from_stdin();

    println!("Working...");
    let (best, count) = map.find_best_observer();
    println!("{:?} sees {} others", best, count);
}

#[test]
fn test_angle() {
    // Left: 90, Right: -90, Up: 0, Down: 180
    let observer = Asteroid { x: 1, y: 1 };
    assert_eq!(observer.angle_to(&Asteroid { x: 2, y: 1 }),    0.0); // Right
    assert_eq!(observer.angle_to(&Asteroid { x: 1, y: 0 }),  -90.0); // Up
    assert_eq!(observer.angle_to(&Asteroid { x: 0, y: 0 }), -135.0); // Left Up
    assert_eq!(observer.angle_to(&Asteroid { x: 0, y: 1 }),  180.0); // Left
    assert_eq!(observer.angle_to(&Asteroid { x: 0, y: 2 }),  135.0); // Left Down
    assert_eq!(observer.angle_to(&Asteroid { x: 1, y: 2 }),   90.0); // Down
    assert_eq!(observer.angle_to(&Asteroid { x: 2, y: 2 }),   45.0); // Right Down
}

#[test]
fn example_a1() {
    let map = Map::from_string("
        .#..#
        .....
        #####
        ....#
        ...##
    ");
    //assert!(!map.can_see(&Asteroid { x: 1, y: 0 }, &Asteroid { x: 3, y: 4 }, false));
    //assert!(map.can_see(&Asteroid { x: 1, y: 0 }, &Asteroid { x: 2, y: 2 }, false));
    //assert!(map.can_see(&Asteroid { x: 1, y: 2 }, &Asteroid { x: 0, y: 2 }, false));
    //assert!(map.can_see(&Asteroid { x: 1, y: 2 }, &Asteroid { x: 2, y: 2 }, false));
    //assert!(!map.can_see(&Asteroid { x: 1, y: 2 }, &Asteroid { x: 3, y: 2 }, false));
    //assert_eq!(map.count_observables(&Asteroid { x: 3, y: 4 }), 8);
    assert_eq!(map.find_best_observer(), (&Asteroid { x: 3, y: 4 }, 8));
}

#[test]
fn example_a2() {
    assert_eq!(Map::from_string("
        ......#.#.
        #..#.#....
        ..#######.
        .#.#.###..
        .#..#.....
        ..#....#.#
        #..#....#.
        .##.#..###
        ##...#..#.
        .#....####
    ").find_best_observer(), (&Asteroid { x: 5, y: 8 }, 33));
}

#[test]
fn example_a3() {
    assert_eq!(Map::from_string("
        #.#...#.#.
        .###....#.
        .#....#...
        ##.#.#.#.#
        ....#.#.#.
        .##..###.#
        ..#...##..
        ..##....##
        ......#...
        .####.###.
    ").find_best_observer(), (&Asteroid { x: 1, y: 2 }, 35));
}

#[test]
fn example_a4() {
    assert_eq!(Map::from_string("
        .#..#..###
        ####.###.#
        ....###.#.
        ..###.##.#
        ##.##.#.#.
        ....###..#
        ..#.#..#.#
        #..#.#.###
        .##...##.#
        .....#.#..
    ").find_best_observer(), (&Asteroid { x: 6, y: 3 }, 41));
}

#[test]
fn example_a5() {
    assert_eq!(Map::from_string("
        .#..##.###...#######
        ##.############..##.
        .#.######.########.#
        .###.#######.####.#.
        #####.##.#.##.###.##
        ..#####..#.#########
        ####################
        #.####....###.#.#.##
        ##.#################
        #####.##.###..####..
        ..######..##.#######
        ####.##.####...##..#
        .#####..#.######.###
        ##...#.##########...
        #.##########.#######
        .####.#.###.###.#.##
        ....##.##.###..#####
        .#.#.###########.###
        #.#.#.#####.####.###
        ###.##.####.##.#..##
    ").find_best_observer(), (&Asteroid { x: 11, y: 13 }, 210));
}
