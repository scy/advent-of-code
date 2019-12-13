use std::collections::HashMap;
use std::thread;
use aoc2019::{IntcodeMachine,MachineState,ParamMode};


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}


#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl Tile {
    fn from_id(id: i64) -> Self {
        match id {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            _ => panic!("Unknown tile id: {}", id),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::Wall => '█',
            Tile::Block => '▒',
            Tile::HorizontalPaddle => '─',
            Tile::Ball => '•',
        }
    }
}


struct Screen {
    tiles: HashMap<Position, Tile>,
    score: i64,
}

impl Screen {
    fn new() -> Self {
        Screen { tiles: HashMap::new(), score: 0 }
    }

    fn set_tile(&mut self, pos: Position, tile: Tile) {
        self.tiles.insert(pos, tile);
    }

    fn set_tile_from_chunk(&mut self, chunk: &[i64]) {
        if chunk[0] == -1 && chunk[1] == 0 {
            self.score = chunk[2];
        } else {
            self.set_tile(Position { x: chunk[0] as usize, y: chunk[1] as usize }, Tile::from_id(chunk[2]));
        }
    }

    fn get_tile(&self, pos: Position) -> &Tile {
        self.tiles.get(&pos).unwrap_or(&Tile::Empty)
    }

    fn get_position_of(&self, look_for: &Tile) -> Option<Position> {
        let found: Vec<Position> = self.tiles.iter().filter(|&kv| kv.1 == look_for).map(|(&pos, &tile)| pos).collect();
        if found.len() != 1 {
            return None;
        }
        Some(found[0])
    }

    fn get_bounds(&self) -> (Position, Position) {
        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_x = 0;
        let mut max_y = 0;
        for position in self.tiles.keys() {
            if position.x < min_x { min_x = position.x }
            if position.x > max_x { max_x = position.x }
            if position.y < min_y { min_y = position.y }
            if position.y > max_y { max_y = position.y }
        }
        (Position { x: min_x, y: min_y }, Position { x: max_x, y: max_y })
    }

    fn count_tiles(&self, look_for: &Tile) -> usize {
        self.tiles.values().filter(|&tile| tile == look_for).count()
    }

    fn print(&self) {
        let (min, max) = self.get_bounds();
        for y in min.y..=max.y {
            let mut line = String::new();
            for x in min.x..=max.x {
                line.push(self.get_tile(Position { x, y }).to_char());
            }
            println!("{}", line);
        }
    }

    fn clear(&self) {
        print!("{}[2J", 27 as char);
    }

    fn clear_and_print(&self) {
        self.clear();
        self.print();
    }
}


fn draw_screen_from_outputs(machine: &IntcodeMachine, screen: &mut Screen) {
    for chunk in machine.get_outputs().chunks_exact(3) {
        screen.set_tile_from_chunk(chunk);
    }
    screen.clear_and_print();
}


fn main() {
    let mut machine = IntcodeMachine::from_stdin();
    let mut screen = Screen::new();

    machine.set_memory(ParamMode::Position, 0, 2);

    loop {
        if let Some(ball) = screen.get_position_of(&Tile::Ball) {
            if let Some(paddle) = screen.get_position_of(&Tile::HorizontalPaddle) {
                if ball.x > paddle.x {
                    machine.set_input(vec![1]);
                } else if ball.x < paddle.x {
                    machine.set_input(vec![-1]);
                } else {
                    machine.set_input(vec![0]);
                }
            }
        }
        match machine.compute() {
            MachineState::Waiting => {
                draw_screen_from_outputs(&machine, &mut screen);
                thread::sleep_ms(5);
            },
            MachineState::Done => {
                draw_screen_from_outputs(&machine, &mut screen);
                break;
            },
            _ => panic!("Unexpected machine state."),
        }
    }
    println!("Game over! Your score is: {}", screen.score);
}
