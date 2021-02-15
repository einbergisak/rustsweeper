

mod tests;
mod game_container;



#[derive(Copy, Clone, PartialEq, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Coordinate {
        Coordinate {
            x,
            y,
        }
    }
    fn default() -> Coordinate {
        Coordinate {
            x: 0,
            y: 0,
        }
    }
}

#[derive(Copy, Clone)]
struct Tile {
    is_revealed: bool,
    is_a_mine: bool,
    number: Option<u8>,
    coordinates: Coordinate,
}

impl Tile {
    fn new(coordinates: Coordinate) -> Tile {
        Tile {
            is_revealed: false,
            is_a_mine: false,
            number: None,
            coordinates,
        }
    }
}





fn main() {}
