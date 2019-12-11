use std::convert::TryInto;
use std::io::{self, BufRead, BufReader};

#[derive(Clone,Copy,Debug,PartialEq)]
struct Asteroid {
    x: u16,
    y: u16,
}

struct ShootingTarget {
    asteroid: Asteroid,
    angle: f64,
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
    vaporized: Vec<Asteroid>,
}

impl Map {
    fn from_stdin() -> Map {
        let mut map = Map { asteroids: vec![], vaporized: vec![] };
        let buffered = BufReader::new(io::stdin());
        for (y, line) in buffered.lines().enumerate() {
            map.read_line(y.try_into().unwrap(), &line.unwrap());
        }
        map
    }

    fn from_string(input: &str) -> Map {
        let mut map = Map { asteroids: vec![], vaporized: vec![] };
        for (y, line) in input.trim().lines().enumerate() {
            map.read_line(y.try_into().unwrap(), line);
        }
        map
    }

    fn to_intuitive_angle(angle: f64) -> f64 {
        if angle < -90.0 {
            return angle + 450.0;
        }
        angle + 90.0
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
            if ((angle == dest_angle) && (distance < dest_distance)) && distance > 0.0  {
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

    fn get_observables_by_angle(&self, observer: &Asteroid) -> Vec<ShootingTarget> {
        let mut result = vec![];
        for target in self.asteroids.iter() {
            if observer != target && self.can_see(&observer, &target) {
                result.push(ShootingTarget { asteroid: *target, angle: observer.angle_to(&target) });
            }
        }
        result.sort_by(|a, b| {
            Self::to_intuitive_angle(a.angle).partial_cmp(&Self::to_intuitive_angle(b.angle)).unwrap()
            //if ((a.angle >= -90.0 && a.angle <= 180.0) && (b.angle >= -90.0 && b.angle <= 180.0))
            // || ((a.angle >= -180.0 && a.angle < -90.0) && (b.angle >= -180.0 && b.angle < -90.0)) {
            //     a.angle.partial_cmp(&b.angle).unwrap()
            //} else if (a.angle >= -90.0 && a.angle <= 180.0) && (b.angle >= -180.0 && b.angle < -90.0) {
            //    Ordering::Greater
            //} else if (a.angle >= -180.0 && a.angle < -90.0) && (b.angle >= -90.0 && b.angle <= 180.0) {
            //    Ordering::Less
            //} else {
            //    panic!("Sort is broken xD")
            //}
        });
        result
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

    fn shoot_asteroids(&mut self, observer: &Asteroid) -> &Vec<Asteroid> {
        loop {
            let mut vaporized = vec![];
            let observables = self.get_observables_by_angle(&observer);
            if observables.is_empty() {
                return &self.vaporized;
            }
            //println!("Shooting {} observable asteroids", observables.len());
            for target in observables {
                vaporized.push(target.asteroid);
                //println!("Shooting {:?} in direction {}", target.asteroid, target.angle);
            }
            self.asteroids.retain(|asteroid| !vaporized.contains(asteroid));
            self.vaporized.append(&mut vaporized);
        }
    }
}

fn main() {
    let mut map = Map::from_stdin();

    println!("Working...");
    let (&best, count) = map.find_best_observer();
    println!("{:?} sees {} others", best, count);

    println!("Shooting!");
    let vaporized = map.shoot_asteroids(&best);
    println!("The 200th vaporized asteroid is {:?}, answer is {}", vaporized[199], 100 * vaporized[199].x + vaporized[199].y);
}

#[test]
fn test_angle() {
    // Left: 90, Right: -90, Up: 0, Down: 180
    let observer = Asteroid { x: 1, y: 1 };
    assert_eq!(observer.angle_to(&Asteroid { x: 2, y: 1 }),    0.0); // Right (90)
    assert_eq!(observer.angle_to(&Asteroid { x: 1, y: 0 }),  -90.0); // Up (0)
    assert_eq!(observer.angle_to(&Asteroid { x: 0, y: 0 }), -135.0); // Left Up (315)
    assert_eq!(observer.angle_to(&Asteroid { x: 0, y: 1 }),  180.0); // Left (270)
    assert_eq!(observer.angle_to(&Asteroid { x: 0, y: 2 }),  135.0); // Left Down (225)
    assert_eq!(observer.angle_to(&Asteroid { x: 1, y: 2 }),   90.0); // Down (180)
    assert_eq!(observer.angle_to(&Asteroid { x: 2, y: 2 }),   45.0); // Right Down (135)
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

#[test]
fn test_example_b1() {
    let mut map = Map::from_string("
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
    ");
    let vaporized = map.shoot_asteroids(&Asteroid { x: 11, y: 13 });
    assert_eq!(vaporized[0..3], [Asteroid{x:11,y:12},Asteroid{x:12,y:1},Asteroid{x:12,y:2}]);
    assert_eq!(vaporized[9], Asteroid{x:12,y:8});
    assert_eq!(vaporized[19], Asteroid{x:16,y:0});
    assert_eq!(vaporized[49], Asteroid{x:16,y:9});
    assert_eq!(vaporized[99], Asteroid{x:10,y:16});
    assert_eq!(vaporized[198], Asteroid{x:9,y:6});
    assert_eq!(vaporized[199], Asteroid{x:8,y:2});
    assert_eq!(vaporized[200], Asteroid{x:10,y:9});
    assert_eq!(vaporized[298], Asteroid{x:11,y:1});
}
