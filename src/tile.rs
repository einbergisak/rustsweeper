#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) struct Coordinate {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Coordinate {
    pub(crate) fn new(x: usize, y: usize) -> Coordinate {
        Coordinate {
            x,
            y,
        }
    }
    pub(crate) fn default() -> Coordinate {
        Coordinate {
            x: 0,
            y: 0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Tile {
    pub(crate) is_revealed: bool,
    pub(crate) is_a_mine: bool,
    pub(crate) number: Option<u8>,
    pub(crate) coordinates: Coordinate,
}

impl Tile {
    pub(crate) fn new(coordinates: Coordinate) -> Tile {
        Tile {
            is_revealed: false,
            is_a_mine: false,
            number: None,
            coordinates,
        }
    }
}