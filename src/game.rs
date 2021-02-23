use std::env::remove_var;
use std::hash::{Hash, Hasher};
use std::{collections::hash_map::DefaultHasher, process::exit};

type TileArray = Vec<Vec<Tile>>;

#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    pub(crate) fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x, y }
    }
    pub(crate) fn default() -> Coordinate {
        Coordinate { x: 0, y: 0 }
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Tile {
    is_revealed: bool,
    is_a_mine: bool,
    number: Option<u8>,
}
impl Default for Tile {
    fn default() -> Self {
        Tile {
            is_revealed: false,
            is_a_mine: false,
            number: None,
        }
    }
}

impl Tile {
    fn reveal(&mut self) {}
}

pub(crate) struct GameContainer {
    tile_array: TileArray,
    tiles_revealed: usize,
    game_rows: usize,
    game_cols: usize,
    game_mines: usize,
}
impl GameContainer {
    pub(crate) fn new(
        game_rows: usize,
        game_cols: usize,
        game_mines: usize,
        seed: String,
    ) -> GameContainer {
        let mut hasher = DefaultHasher::default();
        let tile_array: TileArray = vec![vec![Tile::default(); game_rows]; game_cols];
        let mut hash;
        let mut current_number_of_mines: usize = 0;
        let (mut x_index, mut y_index) = (0, 0);

        // Initializes the tile array

        // Uses the MAD-hashing technique to distribute the mines on the game field.
        'outer: loop {
            'inner: loop {
                seed.hash(&mut hasher);
                hash = hasher.finish();
                let mut tile = tile_array[x_index][y_index];
                if (hash.overflowing_mul(23).0 + 7) % 53 == 0 && !tile.is_a_mine {
                    tile.is_a_mine = true;
                    current_number_of_mines += 1;
                    if current_number_of_mines == game_mines {
                        break 'outer;
                    }
                }
                y_index += 1;
                if y_index == game_rows {
                    y_index = 0;
                    break 'inner;
                }
            }
            x_index += 1;
            if x_index == game_cols {
                x_index = 0;
            }
        }

        // // For each tile
        // tile_array.iter_mut()
        //     .enumerate()
        //     .for_each(|(x, vec)|
        //         vec.iter_mut()
        //             .enumerate()
        //             .for_each(|(y, tile)|{
        //                 tile.coordinates = Coordinate::new(x, y); // Set tile coordinates
        //             }
        //             ));

        let mut gc = GameContainer {
            tile_array,
            tiles_revealed: 0,
            game_rows,
            game_cols,
            game_mines,
        };
        for x in 0..game_cols {
            for y in 0..game_rows {
                gc.set_tile_number(&Coordinate { x, y }); // Set tile number (the amount of nearby mines)
            }
        }

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
        // println!("Reveal tile: {:?}", tile);
        if !tile.is_revealed {
            if tile.is_a_mine {
                self.lose(&tile);
                return;
            } else {
                self.reveal_nearby(coordinate);
                self.check_if_win();
            }
        }
    }

    // Recursively reveals tiles around the argument tile if it is empty (is_a_mine == False && number == None)
    fn reveal_nearby(&mut self, coordinate: &Coordinate) {
        let mut tile: &mut Tile = &mut self.tile_array[coordinate.x][coordinate.y];
        println!("tile = {:?}", tile);
        if !tile.is_revealed {
            match tile.number {
                None if tile.is_a_mine => {
                    return;
                }
                None => {
                    tile.is_revealed = true;
                    self.map_tile_and_surrounding(coordinate, GameContainer::reveal_nearby);
                }
                Some(_) => {
                    tile.is_revealed = true;
                }
            }
            self.tiles_revealed += 1;
        }
    }

    // Sets the number of each non-mine tile
    fn set_tile_number(&mut self, coordinate: &Coordinate) {
        if !self.tile_array[coordinate.x][coordinate.y].is_a_mine {
            let mut acc: u8 = 0; // +1 for each mine surrounding the argument tile
            let add_if_mine = |sself: &mut Self, coordinate: &Coordinate| {
                if sself.tile_array[coordinate.x][coordinate.y].is_a_mine {
                    acc += 1;
                }
            };
            self.map_tile_and_surrounding(coordinate, add_if_mine);
            let mut tile: &mut Tile = &mut self.tile_array[coordinate.x][coordinate.y];
            if acc != 0 {
                tile.number = Some(acc);
            }
        }
    }

    fn map_tile_and_surrounding<T>(&mut self, coordinate: &Coordinate, mut func: T)
    where
        T: FnMut(&mut Self, &Coordinate),
    {
        for xd in -1..=1 {
            for yd in -1..=1 {
                // Makes sure that it doesn't go outside of the tile_array index bounds
                let (x, y) = (coordinate.x as isize + xd, coordinate.y as isize + yd);
                if x >= 0 && x < self.game_cols as isize && y > 0 && y < self.game_rows as isize {
                    func(
                        self,
                        &Coordinate {
                            x: x as usize,
                            y: y as usize,
                        },
                    )
                }
            }
        }
    }

    fn lose(&self, tile: &Tile) {
        println!("Nope, that's a bomb! You lost the game!");
        exit(0);
    }

    fn check_if_win(&self) {
        if self.tiles_revealed == self.game_cols * self.game_rows - self.game_mines {
            println!("You won!")
        }
    }

    pub(crate) fn get_tiles(&self) -> &TileArray {
        &self.tile_array
    }
}
