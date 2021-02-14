use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

mod tests;

const GAME_COLS: usize = 16;
const GAME_ROWS: usize = 16;


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

struct GameContainer {
    tile_array: [[Tile; GAME_ROWS]; GAME_COLS]
}

impl GameContainer {
    fn new(seed: String) -> GameContainer {
        let mut hasher = DefaultHasher::default();
        let number_of_mines = (GAME_ROWS * GAME_COLS) / 6;
        let mut tile_array = [[Tile::new(Coordinate::default()); GAME_ROWS]; GAME_COLS];
        let mut hash;
        let mut current_number_of_mines: usize = 0;
        let (mut x_index, mut y_index) = (0, 0);

        // Uses the MAD-hashing technique to distribute the mines on the game field.
        'outer: loop {
            'inner: loop {
                seed.hash(&mut hasher);
                hash = hasher.finish();
                if (hash.overflowing_mul(23).0 + 7) % 53 == 0 && !tile_array[x_index][y_index].is_a_mine {
                    tile_array[x_index][y_index].is_a_mine = true;
                    current_number_of_mines += 1;
                    if current_number_of_mines == number_of_mines {
                        break 'outer;
                    }
                }
                y_index += 1;
                if y_index == GAME_ROWS {
                    y_index = 0;
                    break 'inner;
                }
            }
            x_index += 1;
            if x_index == GAME_COLS {
                x_index = 0;
            }
        }

        // Set coordinates for each tile in tile_array
        tile_array.iter_mut()
            .enumerate()
            .for_each(|(x, arr)|
                arr.iter_mut()
                    .enumerate()
                    .for_each(|(y, tile)|
                        tile.coordinates = Coordinate::new(x, y)));

        GameContainer {
            tile_array
        }
    }

    fn reveal_tile_at(&mut self, coordinate: Coordinate) {
        let mut tile = self.tile_array[coordinate.x][coordinate.y];
        self.reveal_tile(&mut tile);
    }

    // When the players wants to reveal a tile
    fn reveal_tile(&mut self, mut tile: &mut Tile) {
        if !tile.is_revealed {
            if tile.is_a_mine {
                // self.lose(&tile);
                return;
            }else{
                self.reveal_nearby(&mut tile)
            }
        }
        
    }

    // Recursively reveals tiles around the argument tile if it is empty (no bomb nor number)
    fn reveal_nearby(&mut self, mut tile: &mut Tile) {
        if !tile.is_revealed {
            match tile.number {
                None if tile.is_a_mine => { return; }
                None => {
                    tile.is_revealed = true;
                    for xd in -1..=1 {
                        for yd in -1..=1 {
                            // Makes sure that it doesn't go outside of the tile_array index bounds
                            let (x, y) = (tile.coordinates.x as isize + xd, tile.coordinates.y as isize + yd);
                            if !(x < 0 || x >= GAME_COLS as isize || y < 0 || y > GAME_ROWS as isize) {
                                let mut tile = self.tile_array[x as usize][y as usize];
                                self.reveal_nearby(&mut tile);
                            }
                            
                        }
                    }
                }
                Some(_) => {
                    tile.is_revealed = true;
                }
            }
        }
    }
}


fn main() {}
