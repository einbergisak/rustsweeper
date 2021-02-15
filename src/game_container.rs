use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::tile::{Tile, Coordinate};
use std::env::remove_var;


pub const GAME_COLS: usize = 16;
pub const GAME_ROWS: usize = 16;
const NUM_GAME_MINES: usize = 40;

type TileArray = [[Tile; GAME_ROWS]; GAME_COLS];

pub(crate) struct GameContainer {
    pub(crate) tile_array: TileArray
}
impl GameContainer {
    pub(crate) fn new(seed: &str) -> GameContainer {
        let mut hasher = DefaultHasher::default();
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
                    if current_number_of_mines == NUM_GAME_MINES {
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

        let mut gc = GameContainer {
            tile_array
        };

        // For each tile
        tile_array.iter_mut()
            .enumerate()
            .for_each(|(x, arr)|
                arr.iter_mut()
                    .enumerate()
                    .for_each(|(y, tile)|{
                        tile.coordinates = Coordinate::new(x, y); // Set tile coordinates
                    }
                    ));
        for x in 0..GAME_COLS{
            for y in 0..GAME_ROWS{
                GameContainer::set_tile_number(&mut tile_array, &Coordinate{x, y}); // Set tile number (the amount of nearby mines)
            }
        }

        gc.tile_array = tile_array;
        gc


    }

    // Calls "reveal_tile" using coordinates instead of tile references
    // pub(crate) fn reveal_tile_at(&mut self, coordinate: Coordinate) {
    //     let mut tile = self.tile_array[coordinate.x][coordinate.y];
    //     self.reveal_tile(&mut tile);
    // }

    // When the players wants to reveal a tile
    pub(crate) fn reveal_tile_at(&mut self, coordinate: &Coordinate) {
        let tile = &self.tile_array[coordinate.x][coordinate.y];

        println!("Reveal tile: {:?}", tile);
        if !tile.is_revealed {
            if tile.is_a_mine {
                // self.lose(&tile);
                return;
            }else{
                GameContainer::reveal_nearby(&mut self.tile_array, coordinate);
            }
        }

    }

    // Recursively reveals tiles around the argument tile if it is empty (no bomb nor number)
    fn reveal_nearby (mut tile_array: &mut TileArray, coordinate: &Coordinate){
        let mut tile: &mut Tile = &mut tile_array[coordinate.x][coordinate.y];
        if !tile.is_revealed{
            match tile.number {
                None if tile.is_a_mine => { return; }
                None => {
                    tile.is_revealed = true;
                    GameContainer::map_tile_and_surrounding(&mut tile_array, coordinate, GameContainer::reveal_nearby);
                }
                Some(_) => {
                    tile.is_revealed = true;
                }
            }
        }
    }


    // Sets the number of each non-mine tile
    fn set_tile_number(mut tile_array: &mut TileArray, coordinate: &Coordinate){
        if !tile_array[coordinate.x][coordinate.y].is_a_mine{
            let mut acc: u8 = 0; // +1 for each mine surrounding the argument tile
            let add_if_mine = |tile_array: &mut TileArray, coordinate: &Coordinate|
                if tile_array[coordinate.x][coordinate.y].is_a_mine{
                    acc += 1;
                };
            GameContainer::map_tile_and_surrounding(&mut tile_array, coordinate, add_if_mine);
            let mut tile: &mut Tile = &mut tile_array[coordinate.x][coordinate.y];
            if acc != 0{
                tile.number = Some(acc);
            }
        }
    }

    fn map_tile_and_surrounding<T>(mut tile_array: &mut TileArray, coordinate: &Coordinate, mut func: T)
        where T: FnMut(&mut TileArray, &Coordinate){
        for xd in -1..=1 {
            for yd in -1..=1 {
                // Makes sure that it doesn't go outside of the tile_array index bounds
                let (x, y) = (coordinate.x as isize + xd, coordinate.y as isize + yd);
                if x >= 0 && x < GAME_COLS as isize && y > 0 && y < GAME_ROWS as isize {
                    func(&mut tile_array, &Coordinate{x: x as usize, y: y as usize})
                }

            }
        }
    }
}