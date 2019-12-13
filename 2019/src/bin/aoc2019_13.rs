use std::collections::HashMap;
use aoc2019::IntcodeMachine;


#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}


#[derive(PartialEq)]
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
}

impl Screen {
    fn new() -> Self {
        Screen { tiles: HashMap::new() }
    }

    fn set_tile(&mut self, pos: Position, tile: Tile) {
        self.tiles.insert(pos, tile);
    }

    fn set_tile_from_chunk(&mut self, chunk: &[i64]) {
        self.set_tile(Position { x: chunk[0] as usize, y: chunk[1] as usize }, Tile::from_id(chunk[2]))
    }

    fn get_tile(&self, pos: Position) -> &Tile {
        self.tiles.get(&pos).unwrap_or(&Tile::Empty)
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
}


fn main() {
    let mut machine = IntcodeMachine::from_stdin();
    let mut screen = Screen::new();

    machine.compute();

    for chunk in machine.get_outputs().chunks_exact(3) {
        screen.set_tile_from_chunk(chunk);
    }
    screen.print();

    println!("This final screen contains {} block tiles.", screen.count_tiles(&Tile::Block));
}
