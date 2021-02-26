use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use ggez::{
    graphics::{spritebatch::SpriteBatch, Image},
    Context,
};
use rand::random;

/// 2-Dimensional vector of tiles.
/// The outer vector is a vector of the game columns.
/// Tiles at coordinates (x, y) can be accessed via `TileArray[x][y]`
type TileArray = Vec<Vec<Tile>>;

/// Game tile
#[derive(Copy, Clone, Debug)]
pub(crate) struct Tile {
    pub(crate) is_revealed: bool,
    is_a_mine: bool,
    pub(crate) is_flagged: bool,
    pub(crate) number: Option<u8>,
}
impl Default for Tile {
    /// Creates a new, uninitialized tile with no 'active values'
    fn default() -> Self {
        Tile {
            is_revealed: false,
            is_a_mine: false,
            is_flagged: false,
            number: None,
        }
    }
}

/// Contains the data relevant to the game
pub(crate) struct GameContainer {
    pub(crate) tile_array: Box<TileArray>,
    pub(crate) tiles_revealed: usize,
    pub(crate) game_rows: usize,
    pub(crate) game_cols: usize,
    game_mines: usize,
    pub(crate) sprite_batch: SpriteBatch,
    pub(crate) scaled_tile_size: f32,
}
impl GameContainer {
    /// Creates a new game with the provided settings and seed.
    pub(crate) fn new(
        mut ctx: &mut Context,
        game_rows: usize,
        game_cols: usize,
        game_mines: usize,
        seed: Option<String>,
        scaled_tile_size: f32,
    ) -> GameContainer {
        let tile_array: Box<TileArray> =
            Box::new(vec![vec![Tile::default(); game_rows]; game_cols]);

        let img = Image::new(&mut ctx, "/spritesheet.png").expect("Image loading error");
        let mut gc = GameContainer {
            tile_array,
            tiles_revealed: 0,
            game_rows,
            game_cols,
            game_mines,
            sprite_batch: SpriteBatch::new(img),
            scaled_tile_size,
        };

        if seed.is_some() {
            gc.distribute_mines(None, seed);
        }

        gc
    }

    /// Distributes the mines randomly or according to the game seed (depending on if "seed" is Some(String) or None).
    ///
    /// Leaves the upper-left corner free from mines and reveals some tiles,
    /// because otherwise seeded gameplay would be impossible (since you'd be able to click on a mine on the initial click).
    pub(crate) fn distribute_mines(
        &mut self,
        tile_coords: Option<(usize, usize)>,
        seed: Option<String>,
    ) {
        let (clicked_tile_x, clicked_tile_y) = if let Some(tuple) = tile_coords {
            tuple
        } else {
            (0, 0)
        };

        let mut hasher = DefaultHasher::default();
        let mut token: u64;
        let mut current_number_of_mines: usize = 0;
        let (mut x_index, mut y_index) = (0, 0);

        // Uses the MAD-hashing technique to distribute the mines on the game field.
        'outer: loop {
            'inner: loop {
                let mut tile = &mut self.tile_array[x_index][y_index];
                if let Some(ref s) = seed {
                    s.hash(&mut hasher);
                    token = hasher.finish();
                } else {
                    token = random::<u64>();
                }

                if (token.overflowing_mul(23).0 + 7) % 53 == 0
                    && !tile.is_a_mine
                    && ((seed.is_some() && x_index > 2 && y_index > 2)
                        || (seed.is_none()
                            && (x_index.overflowing_sub(clicked_tile_x).0) > 1
                            && (y_index.overflowing_sub(clicked_tile_y).0) > 1))
                {
                    tile.is_a_mine = true;
                    current_number_of_mines += 1;
                    if current_number_of_mines == self.game_mines {
                        break 'outer;
                    }
                }
                y_index += 1;
                if y_index == self.game_rows {
                    y_index = 0;
                    break 'inner;
                }
            }
            x_index += 1;
            if x_index == self.game_cols {
                x_index = 0;
            }
        }

        // Sets the number for each tile (the amount of mines surrounding the tile)
        for x in 0..self.game_cols {
            for y in 0..self.game_rows {
                self.set_tile_number((x, y));
            }
        }

        // Reveals the upper left corner if playing a seeded game.
        if seed.is_some() {
            self.reveal_tile_at((0, 0));
        }
    }

    /// Reveals the tile at the given coordinates.
    pub(crate) fn reveal_tile_at(&mut self, (tile_x, tile_y): (usize, usize)) {
        let tile = &self.tile_array[tile_x][tile_y];
        if !tile.is_revealed && !tile.is_flagged {
            if tile.is_a_mine {
                self._lose(&tile);
                return;
            } else {
                self.reveal_nearby((tile_x, tile_y));
                self.check_if_win();
            }
        }
    }

    /// Toggles if the tile is flagged or not
    pub(crate) fn toggle_flag_at(&mut self, (tile_x, tile_y): (usize, usize)) {
        if !self.tile_array[tile_x][tile_y].is_revealed {
            // Toggles with bitwise XOR
            self.tile_array[tile_x][tile_y].is_flagged ^= true;
        }
    }

    pub(crate) fn chord_at(&mut self, (tile_x, tile_y): (usize, usize)) {
        let tile = self.tile_array[tile_x][tile_y];
        if tile.is_revealed && tile.number.is_some() {
            let mut acc: u8 = 0;
            self.map_tile_and_surrounding(
                (tile_x, tile_y),
                |sself: &mut Self, (x, y): (usize, usize)| {
                    if sself.tile_array[x][y].is_flagged {
                        acc += 1;
                    }
                },
            );
            if tile.number == Some(acc) {
                self.map_tile_and_surrounding(
                    (tile_x, tile_y),
                    |sself: &mut Self, (x, y): (usize, usize)| sself.reveal_tile_at((x, y)),
                );
            }
        }
    }

    /// Recursively reveals tiles around the argument tile if it is empty `(is_a_mine == False && number == None)`
    fn reveal_nearby(&mut self, (tile_x, tile_y): (usize, usize)) {
        let mut tile: &mut Tile = &mut self.tile_array[tile_x][tile_y];
        if !tile.is_revealed && !tile.is_flagged {
            match tile.number {
                None if tile.is_a_mine => {
                    return;
                }
                None => {
                    tile.is_revealed = true;
                    self.map_tile_and_surrounding((tile_x, tile_y), GameContainer::reveal_nearby);
                }
                Some(_) => {
                    tile.is_revealed = true;
                }
            }
            self.tiles_revealed += 1;
        }
    }

    /// Sets the number for each non-mine tile
    fn set_tile_number(&mut self, (tile_x, tile_y): (usize, usize)) {
        if !self.tile_array[tile_x][tile_y].is_a_mine {
            let mut acc: u8 = 0; // +1 for each mine surrounding the argument tile
            let add_if_mine = |sself: &mut Self, (x, y): (usize, usize)| {
                if sself.tile_array[x][y].is_a_mine {
                    acc += 1;
                }
            };
            self.map_tile_and_surrounding((tile_x, tile_y), add_if_mine);
            let mut tile: &mut Tile = &mut self.tile_array[tile_x][tile_y];
            if acc != 0 {
                tile.number = Some(acc);
            }
        }
    }

    /// A higher-order function that applies the given function to the given tile and to all of the tiles surrounding it.
    fn map_tile_and_surrounding<T>(&mut self, (tile_x, tile_y): (usize, usize), mut func: T)
    where
        T: FnMut(&mut Self, (usize, usize)),
    {
        for xd in -1..=1 {
            for yd in -1..=1 {
                // Makes sure that it doesn't go outside of the tile_array index bounds
                let (x, y) = (tile_x as i16 + xd, tile_y as i16 + yd);
                if x >= 0 && x < self.game_cols as i16 && y >= 0 && y < self.game_rows as i16 {
                    func(self, (x as usize, y as usize))
                }
            }
        }
    }

    fn _lose(&self, clicked_tile: &Tile) {
        todo!()
    }

    fn check_if_win(&self) {
        if self.tiles_revealed == self.game_cols * self.game_rows - self.game_mines {
            todo!()
        }
    }
}
