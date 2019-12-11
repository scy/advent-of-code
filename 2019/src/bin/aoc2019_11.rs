use std::collections::HashMap;
use std::ops::Add;
use aoc2019::{IntcodeMachine,MachineState};


enum Direction {
    Up,
    Left,
    Down,
    Right
}

impl Direction {
    fn left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    fn right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position (i32, i32);

impl Add<(i32,i32)> for Position {
    type Output = Self;
    fn add(self, other: (i32,i32)) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}


struct Robot {
    position: Position,
    direction: Direction,
}

impl Robot {
    fn advance_left(&mut self) {
        self.direction = self.direction.left();
        self.advance();
    }

    fn advance_right(&mut self) {
        self.direction = self.direction.right();
        self.advance();
    }

    fn advance(&mut self) {
        self.position = self.position + match self.direction {
            Direction::Up => (0,1),
            Direction::Left => (-1,0),
            Direction::Down => (0,-1),
            Direction::Right => (1,0),
        }
    }
}


#[derive(Clone, Copy)]
enum Color {
    Black,
    White,
}


struct Hull {
    panels: HashMap<Position, Color>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Hull {
    fn new() -> Self {
        Hull { panels: HashMap::new(), min_x: 0, max_x: 0, min_y:0, max_y: 0 }
    }

    fn color_at(&self, pos: Position) -> Color {
        *self.panels.get(&pos).unwrap_or(&Color::Black)
    }

    fn set_color_at(&mut self, pos: &Position, color: &Color) {
        if pos.0 < self.min_x { self.min_x = pos.0; }
        if pos.0 > self.max_x { self.max_x = pos.0; }
        if pos.1 < self.min_y { self.min_y = pos.1; }
        if pos.1 > self.max_y { self.max_y = pos.1; }
        self.panels.insert(*pos, *color);
    }

    fn count_painted(&self) -> usize {
        self.panels.len()
    }

    fn print(&self) {
        for y in self.min_y..=self.max_y {
            let mut line = String::new();
            for x in self.min_x..=self.max_x {
                line.push(match self.color_at(Position(x, y)) {
                    Color::Black => '.',
                    Color::White => '#',
                });
            }
            println!("{}", line);
        }
    }
}


fn main() {
    let mut machine = IntcodeMachine::from_stdin();
    let mut hull = Hull::new();
    let mut robot = Robot { position: Position(0,0), direction: Direction::Up };

    loop {
        machine.push_input(match hull.color_at(robot.position) {
            Color::Black => 0,
            Color::White => 1,
        });
        match machine.compute() {
            MachineState::Done => break,
            MachineState::Waiting => {
                let output = machine.get_outputs_and_clear();
                if output.len() != 2 {
                    panic!("Unexpected robot output length {}", output.len());
                }
                hull.set_color_at(&robot.position, match output[0] {
                    0 => &Color::Black,
                    1 => &Color::White,
                    _ => panic!("Unexpected color output {}", output[0]),
                });
                match output[1] {
                    0 => robot.advance_left(),
                    1 => robot.advance_right(),
                    _ => panic!("Unexpected direction output {}", output[1]),
                }
            }
            _ => unimplemented!(),
        }
    }
    println!("The robot would paint {} panels like this:", hull.count_painted());
    hull.print();

}
